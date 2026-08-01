CREATE TABLE IF NOT EXISTS evolver_state (
    memory_id TEXT PRIMARY KEY,
    state TEXT NOT NULL DEFAULT 'unprocessed',
    updated_at REAL NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_evolver_state_unprocessed 
ON evolver_state(state, updated_at) 
WHERE state = 'unprocessed';

CREATE TABLE IF NOT EXISTS skill_mapping (
    skill_id TEXT PRIMARY KEY,
    engine_rid TEXT NOT NULL
);
