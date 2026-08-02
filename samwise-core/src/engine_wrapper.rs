use crate::errors::SidecarError;
use compact_str::CompactString;
use serde::Serialize;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::mpsc;
use yantrikdb::YantrikDB;

#[derive(Debug, Clone, Serialize)]
pub struct SkillRecord {
    pub skill_id: CompactString,
    pub body: CompactString,
    pub importance: f64,
}

pub struct EngineWrapper {
    db: Arc<YantrikDB>,
    state_pool: SqlitePool,
    epoch: AtomicU64,
    think_tx: mpsc::Sender<()>,
}

impl EngineWrapper {
    pub async fn new<P: AsRef<Path>>(
        db_path: P,
        state_db_path: P,
    ) -> Result<Arc<Self>, SidecarError> {
        let db_path_str = db_path.as_ref().to_str().ok_or_else(|| {
            SidecarError::Permanent(CompactString::const_new("Invalid UTF-8 in db_path"))
        })?;
        let state_path_str = state_db_path.as_ref().to_str().ok_or_else(|| {
            SidecarError::Permanent(CompactString::const_new("Invalid UTF-8 in state_db_path"))
        })?;

        if let Some(parent) = Path::new(state_path_str).parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                SidecarError::Permanent(CompactString::from(format!("Dir creation failed: {e}")))
            })?;
        }

        let db = Arc::new(
            YantrikDB::with_default(db_path_str)
                .map_err(|e| SidecarError::Permanent(CompactString::from(e.to_string())))?,
        );

        let state_pool = SqlitePoolOptions::new()
            .max_connections(16)
            .connect(&format!("sqlite://{state_path_str}?mode=rwc"))
            .await
            .map_err(|e| {
                SidecarError::Permanent(CompactString::from(format!("State DB init failed: {e}")))
            })?;

        sqlx::query("PRAGMA journal_mode = WAL")
            .execute(&state_pool)
            .await
            .map_err(|e| {
                SidecarError::Permanent(CompactString::from(format!(
                    "State DB WAL setup failed: {e}"
                )))
            })?;

        sqlx::migrate!("./migrations")
            .run(&state_pool)
            .await
            .map_err(|e| {
                SidecarError::Permanent(CompactString::from(format!(
                    "Migration execution failed: {e}"
                )))
            })?;

        let (think_tx, think_rx) = mpsc::channel::<()>(64);

        let engine = Arc::new(Self {
            db,
            state_pool,
            epoch: AtomicU64::new(0),
            think_tx,
        });

        engine.spawn_think_loop(think_rx);
        Ok(engine)
    }

    fn spawn_think_loop(&self, mut rx: mpsc::Receiver<()>) {
        let db = Arc::clone(&self.db);
        tokio::spawn(async move {
            while rx.recv().await.is_some() {
                let db_clone = Arc::clone(&db);
                match tokio::task::spawn_blocking(move || {
                    db_clone.think(&yantrikdb::ThinkConfig::default())?;
                    yantrikdb::conflict::scan_conflicts(&db_clone)
                })
                .await
                {
                    Ok(Ok(conflicts)) if !conflicts.is_empty() => {
                        tracing::warn!(
                            count = conflicts.len(),
                            "Conflicts detected in background think loop"
                        );
                    }
                    Ok(Err(e)) => tracing::error!(error = %e, "YantrikDB cognition loop failed"),
                    Err(e) => {
                        tracing::error!(error = %e, "Think loop worker task panicked");
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                    _ => {}
                }
            }
        });
    }

    #[inline]
    pub fn get_governance_epoch(&self) -> u64 {
        self.epoch.load(Ordering::Acquire)
    }

    #[inline]
    pub fn increment_governance_epoch(&self) -> u64 {
        self.epoch.fetch_add(1, Ordering::AcqRel) + 1
    }

    pub async fn buffer_turn(
        &self,
        content: CompactString,
        memory_type: CompactString,
        importance: f64,
    ) -> Result<(), SidecarError> {
        let db = Arc::clone(&self.db);
        let pool = self.state_pool.clone();

        let memory_id = tokio::task::spawn_blocking(move || {
            let metadata = serde_json::json!({"source": "samwise-sidecar"});
            db.record_text(
                content.as_str(),
                memory_type.as_str(),
                importance.clamp(0.0, 1.0),
                0.0,
                604800.0,
                &metadata,
                "default",
                importance.clamp(0.0, 1.0),
                "cross_department",
                "user",
                None,
            )
            .map_err(|e| SidecarError::Permanent(CompactString::from(e.to_string())))
        })
        .await
        .map_err(|e| {
            SidecarError::Permanent(CompactString::from(format!("Blocking task failed: {e}")))
        })??;

        let now = unix_timestamp();
        sqlx::query(
            "INSERT OR IGNORE INTO evolver_state (memory_id, state, updated_at) VALUES (?, 'unprocessed', ?)",
        )
        .bind(&memory_id)
        .bind(now)
        .execute(&pool)
        .await?;

        let _ = self.think_tx.try_send(());
        Ok(())
    }

    pub async fn search_skills(&self, context: &str) -> Result<Vec<SkillRecord>, SidecarError> {
        let db = Arc::clone(&self.db);
        let ctx = context.to_owned();

        let results = tokio::task::spawn_blocking(move || {
            let embedding = db
                .embed(&ctx)
                .map_err(|e| SidecarError::Permanent(CompactString::from(e.to_string())))?;
            db.surface_procedural(&embedding, Some(&ctx), None, 10, Some("default"))
                .map_err(|e| SidecarError::Permanent(CompactString::from(e.to_string())))
        })
        .await
        .map_err(|e| {
            SidecarError::Permanent(CompactString::from(format!("Skill search failed: {e}")))
        })??;

        Ok(results
            .into_iter()
            .map(|r| SkillRecord {
                skill_id: CompactString::from(r.rid),
                body: CompactString::from(r.text),
                importance: r.importance,
            })
            .collect())
    }

    pub async fn define_skill(
        &self,
        skill_id: CompactString,
        body: CompactString,
        skill_type: CompactString,
        applies_to: Vec<CompactString>,
        cluster_id: CompactString,
    ) -> Result<(), SidecarError> {
        let db = Arc::clone(&self.db);
        let pool = self.state_pool.clone();
        let skill_id_for_mapping = skill_id.clone();

        let rid = tokio::task::spawn_blocking(move || {
            let meta = serde_json::json!({
                "skill_id": skill_id.as_str(),
                "type": skill_type.as_str(),
                "applies_to": applies_to.iter().map(|s| s.as_str()).collect::<Vec<_>>()
            });
            let text = format!("[METADATA: {}]\n\n{}", meta, body);
            let embedding = db
                .embed(&text)
                .map_err(|e| SidecarError::Permanent(CompactString::from(e.to_string())))?;
            db.record_procedural(
                &text,
                &embedding,
                "cross_department",
                cluster_id.as_str(),
                0.8,
                "default",
            )
            .map_err(|e| SidecarError::Permanent(CompactString::from(e.to_string())))
        })
        .await
        .map_err(|e| {
            SidecarError::Permanent(CompactString::from(format!("Skill define failed: {e}")))
        })??;

        sqlx::query("INSERT OR REPLACE INTO skill_mapping (skill_id, engine_rid) VALUES (?, ?)")
            .bind(skill_id_for_mapping.as_str())
            .bind(&rid)
            .execute(&pool)
            .await?;

        self.increment_governance_epoch();
        Ok(())
    }

    pub async fn claim_unprocessed_traces(
        &self,
        batch_size: i64,
    ) -> Result<Vec<CompactString>, SidecarError> {
        let now = unix_timestamp();
        let rows: Vec<(String,)> = sqlx::query_as(
            r#"
            UPDATE evolver_state
            SET state = 'drafted', updated_at = ?
            WHERE memory_id IN (
                SELECT memory_id FROM evolver_state
                WHERE state = 'unprocessed'
                LIMIT ?
            )
            RETURNING memory_id
            "#,
        )
        .bind(now)
        .bind(batch_size)
        .fetch_all(&self.state_pool)
        .await?;

        Ok(rows.into_iter().map(|r| CompactString::from(r.0)).collect())
    }
}

fn unix_timestamp() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}
