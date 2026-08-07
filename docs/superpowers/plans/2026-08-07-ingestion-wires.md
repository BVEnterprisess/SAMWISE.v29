# Ingestion Wires Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move OmniRoute's execution evidence into YantrikDB before OmniRoute's 7-day retention deletes it, and classify each execution's originating source so HITL coordination tax becomes computable.

**Architecture:** A stateless projector process reads OmniRoute's SQLite **read-only**, converts four-stage request artifacts into immutable typed assets keyed by their existing SHA-256, and writes them idempotently to YantrikDB. The projector holds no durable state of its own — its ingest cursor lives in YantrikDB's KV index, so the axiom holds: YantrikDB is the only persistent continuity. Wire 2 rides the same pass: each execution is classified `human` / `agent` / `unknown` from turn-type headers, and `unknown` stays explicit rather than being guessed.

**Tech Stack:** Python 3.11+, `pytest`, stdlib `sqlite3` (read-only URI mode), YantrikDB client (contract established in Task 1).

## Global Constraints

- **Never write to OmniRoute's database.** Open it read-only. OmniRoute owns that file; the projector is a reader.
- **Ingest only at the immutable typed-asset boundary** — the four-stage artifacts, which already carry SHA-256. Never raw conversational material. `buffer_turn` was deleted for exactly this violation.
- **Idempotency is mandatory.** Re-ingesting the same artifact must be a no-op. Duplicate source events may not inflate support, confidence, or importance.
- **Partial evidence stays explicit.** An execution whose source cannot be determined is `unknown`. Never default it to `human` or `agent`.
- **The projector is stateless.** No local cursor file, no local cache. The cursor lives in YantrikDB.
- **Retention is the deadline.** `DEFAULT_CALL_LOG_RETENTION_DAYS = 7` and `DEFAULT_CALL_LOGS_TABLE_MAX_ROWS = 100000` (`src/lib/logEnv.ts:5,9`). Ingest must run far more often than either bound.
- Commit after every task. Do not batch.

---

### Task 1: Establish and record the YantrikDB ingest contract

**Files:**
- Create: `docs/INGEST-CONTRACT.md`
- Create: `samwise-ingest/pyproject.toml`
- Create: `samwise-ingest/tests/test_contract.py`

**Interfaces:**
- Consumes: nothing.
- Produces: `docs/INGEST-CONTRACT.md` recording the verified write/read/cursor calls; a Python package skeleton importable as `samwise_ingest`.

**Why this task exists:** the exact YantrikDB write surface is not known to this plan's author. Guessing it would produce code that cannot run. This task establishes it by observation and writes it down, so every later task consumes a recorded contract instead of an assumption.

- [ ] **Step 1: Determine which YantrikDB surface is reachable**

```bash
cd /home/johnh/services/yantrikdb
ls crates/
grep -rn "fn remember\|fn recall\|pub fn think" crates/yantrikdb-core/src/*.rs | head -20
pip show yantrikdb-client 2>/dev/null || echo "client SDK not installed"
curl -s -m 5 -o /dev/null -w "7438 raft: %{http_code}\n" http://127.0.0.1:7438/v1/cluster/raft
```

Record which of these is available: embedded Rust, Python bindings (`crates/yantrikdb-python`), the `yantrikdb-client` SDK, or an HTTP server.

- [ ] **Step 2: Identify the four required operations**

From whichever surface Step 1 found, locate and record the call for each:

| Need | Purpose |
|---|---|
| write one record with explicit id/hash | idempotent asset ingest |
| read one record by id | idempotency check and verification |
| KV get / KV set | the ingest cursor |
| list or query by a metadata field | verification queries in later tasks |

If a needed operation does not exist on the available surface, stop and report which one. Do not substitute an approximation.

- [ ] **Step 3: Write `docs/INGEST-CONTRACT.md`**

Record, with exact signatures observed in Step 2, one section per operation: the call, its parameters, its return shape, and its idempotency behavior when the same id is written twice. State the surface chosen and why. End with a "Verified on" line carrying today's date and the YantrikDB version.

- [ ] **Step 4: Create the package skeleton**

`samwise-ingest/pyproject.toml`:

```toml
[project]
name = "samwise-ingest"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = []

[build-system]
requires = ["setuptools>=68"]
build-backend = "setuptools.build_meta"

[tool.pytest.ini_options]
testpaths = ["tests"]
```

`samwise-ingest/tests/test_contract.py`:

```python
def test_contract_document_exists():
    from pathlib import Path
    doc = Path(__file__).resolve().parents[2] / "docs" / "INGEST-CONTRACT.md"
    text = doc.read_text()
    assert "Verified on" in text, "contract must record when it was verified"
    for op in ("write", "read", "cursor"):
        assert op in text.lower(), f"contract must document the {op} operation"
```

- [ ] **Step 5: Run the test**

```bash
cd /home/johnh/services/samwise/samwise-ingest
python -m pytest tests/test_contract.py -v
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
cd /home/johnh/services/samwise
git add docs/INGEST-CONTRACT.md samwise-ingest/
git commit -m "docs: record the verified YantrikDB ingest contract"
```

---

### Task 2: Read OmniRoute artifacts read-only

**Files:**
- Create: `samwise-ingest/src/samwise_ingest/omniroute_reader.py`
- Create: `samwise-ingest/tests/test_omniroute_reader.py`

**Interfaces:**
- Consumes: nothing.
- Produces:
  - `ExecutionRecord` dataclass with fields `request_id: str`, `timestamp: int`, `api_key_id: str | None`, `session_id: str | None`, `provider: str | None`, `model: str | None`, `status_code: int | None`, `input_tokens: int | None`, `output_tokens: int | None`, `artifact_sha256: str | None`, `artifact_path: str | None`, `turn_type: str | None`
  - `read_since(db_path: str, after_timestamp: int, limit: int = 1000) -> list[ExecutionRecord]`
  - `OmniRouteReadError(Exception)`

- [ ] **Step 1: Write the failing test**

`samwise-ingest/tests/test_omniroute_reader.py`:

```python
import sqlite3
import pytest
from samwise_ingest.omniroute_reader import read_since, OmniRouteReadError

def _fixture_db(path):
    con = sqlite3.connect(path)
    con.execute("""CREATE TABLE call_logs (
        id TEXT PRIMARY KEY, timestamp INTEGER, api_key_id TEXT, session_id TEXT,
        provider TEXT, model TEXT, status_code INTEGER,
        input_tokens INTEGER, output_tokens INTEGER,
        artifact_sha256 TEXT, artifact_path TEXT, turn_type TEXT)""")
    con.executemany("INSERT INTO call_logs VALUES (?,?,?,?,?,?,?,?,?,?,?,?)", [
        ("req-1", 1000, "key-a", "sess-1", "openai", "gpt", 200, 10, 20, "aa"*32, "/a", "human"),
        ("req-2", 2000, "key-a", "sess-1", "openai", "gpt", 200, 11, 21, "bb"*32, "/b", "agent"),
        ("req-3", 3000, "key-b", "sess-2", "anthropic", "claude", 500, 12, 0, "cc"*32, "/c", None),
    ])
    con.commit(); con.close()

def test_reads_only_records_after_cursor(tmp_path):
    db = tmp_path / "omni.db"; _fixture_db(db)
    got = read_since(str(db), after_timestamp=1000)
    assert [r.request_id for r in got] == ["req-2", "req-3"]

def test_returns_records_in_timestamp_order(tmp_path):
    db = tmp_path / "omni.db"; _fixture_db(db)
    got = read_since(str(db), after_timestamp=0)
    assert [r.timestamp for r in got] == [1000, 2000, 3000]

def test_preserves_null_turn_type_as_none(tmp_path):
    db = tmp_path / "omni.db"; _fixture_db(db)
    got = read_since(str(db), after_timestamp=2000)
    assert got[0].turn_type is None

def test_respects_limit(tmp_path):
    db = tmp_path / "omni.db"; _fixture_db(db)
    assert len(read_since(str(db), after_timestamp=0, limit=2)) == 2

def test_connection_is_read_only(tmp_path):
    db = tmp_path / "omni.db"; _fixture_db(db)
    read_since(str(db), after_timestamp=0)
    from samwise_ingest.omniroute_reader import _connect
    con = _connect(str(db))
    with pytest.raises(sqlite3.OperationalError):
        con.execute("INSERT INTO call_logs (id, timestamp) VALUES ('x', 1)")

def test_missing_database_raises_typed_error(tmp_path):
    with pytest.raises(OmniRouteReadError):
        read_since(str(tmp_path / "nope.db"), after_timestamp=0)
```

- [ ] **Step 2: Run to verify it fails**

```bash
cd /home/johnh/services/samwise/samwise-ingest
python -m pytest tests/test_omniroute_reader.py -v
```

Expected: FAIL — `ModuleNotFoundError: No module named 'samwise_ingest.omniroute_reader'`.

- [ ] **Step 3: Implement the reader**

`samwise-ingest/src/samwise_ingest/omniroute_reader.py`:

```python
"""Read-only reader over OmniRoute's SQLite. OmniRoute owns this file."""
from __future__ import annotations

import sqlite3
from dataclasses import dataclass
from pathlib import Path


class OmniRouteReadError(Exception):
    """The OmniRoute database could not be read."""


@dataclass(frozen=True)
class ExecutionRecord:
    request_id: str
    timestamp: int
    api_key_id: str | None
    session_id: str | None
    provider: str | None
    model: str | None
    status_code: int | None
    input_tokens: int | None
    output_tokens: int | None
    artifact_sha256: str | None
    artifact_path: str | None
    turn_type: str | None


_COLUMNS = (
    "id, timestamp, api_key_id, session_id, provider, model, status_code, "
    "input_tokens, output_tokens, artifact_sha256, artifact_path, turn_type"
)


def _connect(db_path: str) -> sqlite3.Connection:
    """Open read-only. mode=ro makes writes fail rather than relying on discipline."""
    if not Path(db_path).exists():
        raise OmniRouteReadError(f"OmniRoute database not found: {db_path}")
    try:
        return sqlite3.connect(f"file:{db_path}?mode=ro", uri=True)
    except sqlite3.Error as exc:
        raise OmniRouteReadError(str(exc)) from exc


def read_since(db_path: str, after_timestamp: int, limit: int = 1000) -> list[ExecutionRecord]:
    con = _connect(db_path)
    try:
        rows = con.execute(
            f"SELECT {_COLUMNS} FROM call_logs WHERE timestamp > ? ORDER BY timestamp ASC LIMIT ?",
            (after_timestamp, limit),
        ).fetchall()
    except sqlite3.Error as exc:
        raise OmniRouteReadError(str(exc)) from exc
    finally:
        con.close()
    return [ExecutionRecord(*row) for row in rows]
```

- [ ] **Step 4: Run to verify it passes**

```bash
python -m pytest tests/test_omniroute_reader.py -v
```

Expected: 6 passed.

- [ ] **Step 5: Confirm the real schema matches**

```bash
sqlite3 "file:$HOME/.omniroute/omniroute.db?mode=ro" ".schema call_logs" 2>/dev/null | head -30
```

If real column names differ from the fixture, update `_COLUMNS` and the fixture together, then re-run Step 4. Record any deviation in `docs/INGEST-CONTRACT.md`.

- [ ] **Step 6: Commit**

```bash
cd /home/johnh/services/samwise
git add samwise-ingest/
git commit -m "feat(ingest): read-only reader over OmniRoute call_logs"
```

---

### Task 3: Classify execution source (Wire 2)

**Files:**
- Create: `samwise-ingest/src/samwise_ingest/source.py`
- Create: `samwise-ingest/tests/test_source.py`

**Interfaces:**
- Consumes: `ExecutionRecord` from Task 2.
- Produces: `Source` (str enum: `HUMAN = "human"`, `AGENT = "agent"`, `UNKNOWN = "unknown"`), `classify(record: ExecutionRecord) -> Source`.

**Why this task exists:** HITL coordination tax is a graph query over human-sourced perturbations. That query is only possible if each execution carries a source. OmniRoute cannot infer this — it is set by the agent harness via a turn-type header. Where the header is absent, the answer is `unknown` and must stay `unknown`; guessing would fabricate the very metric the benchmark depends on.

- [ ] **Step 1: Write the failing test**

`samwise-ingest/tests/test_source.py`:

```python
import pytest
from samwise_ingest.omniroute_reader import ExecutionRecord
from samwise_ingest.source import classify, Source

def _rec(turn_type):
    return ExecutionRecord("r", 1, "k", "s", "p", "m", 200, 1, 1, "h", "/p", turn_type)

@pytest.mark.parametrize("raw", ["human", "HUMAN", " Human "])
def test_human_turn_types_classify_as_human(raw):
    assert classify(_rec(raw)) is Source.HUMAN

@pytest.mark.parametrize("raw", ["agent", "AGENT", "assistant", "tool"])
def test_agent_turn_types_classify_as_agent(raw):
    assert classify(_rec(raw)) is Source.AGENT

def test_missing_turn_type_is_unknown_not_guessed():
    assert classify(_rec(None)) is Source.UNKNOWN

def test_empty_turn_type_is_unknown():
    assert classify(_rec("   ")) is Source.UNKNOWN

def test_unrecognised_turn_type_is_unknown_not_agent():
    assert classify(_rec("cron")) is Source.UNKNOWN
```

- [ ] **Step 2: Run to verify it fails**

```bash
cd /home/johnh/services/samwise/samwise-ingest
python -m pytest tests/test_source.py -v
```

Expected: FAIL — module does not exist.

- [ ] **Step 3: Implement the classifier**

`samwise-ingest/src/samwise_ingest/source.py`:

```python
"""Classify what originated an execution.

The human is another perturbation source: intents arrive as graph nodes with a
`source: human` edge and are treated like any other. An execution whose origin cannot
be established is UNKNOWN and stays UNKNOWN — HITL coordination tax is computed only
over executions whose source is known.
"""
from __future__ import annotations

from enum import Enum

from .omniroute_reader import ExecutionRecord


class Source(str, Enum):
    HUMAN = "human"
    AGENT = "agent"
    UNKNOWN = "unknown"


_HUMAN = frozenset({"human", "user"})
_AGENT = frozenset({"agent", "assistant", "tool", "toolresult"})


def classify(record: ExecutionRecord) -> Source:
    raw = (record.turn_type or "").strip().lower()
    if raw in _HUMAN:
        return Source.HUMAN
    if raw in _AGENT:
        return Source.AGENT
    return Source.UNKNOWN
```

- [ ] **Step 4: Run to verify it passes**

```bash
python -m pytest tests/test_source.py -v
```

Expected: 8 passed.

- [ ] **Step 5: Commit**

```bash
cd /home/johnh/services/samwise
git add samwise-ingest/
git commit -m "feat(ingest): classify execution source, keeping unknown explicit"
```

---

### Task 4: Build immutable assets with stable identity

**Files:**
- Create: `samwise-ingest/src/samwise_ingest/asset.py`
- Create: `samwise-ingest/tests/test_asset.py`

**Interfaces:**
- Consumes: `ExecutionRecord` (Task 2), `Source` (Task 3).
- Produces: `Asset` frozen dataclass with fields `asset_id: str`, `asset_type: str`, `schema_version: int`, `canonical_payload: dict`, `content_hash: str`, `source: str`, `observed_at: int`; and `build_asset(record: ExecutionRecord, source: Source) -> Asset`, `canonical_bytes(payload: dict, schema_version: int) -> bytes`.

- [ ] **Step 1: Write the failing test**

`samwise-ingest/tests/test_asset.py`:

```python
import pytest
from samwise_ingest.omniroute_reader import ExecutionRecord
from samwise_ingest.source import Source
from samwise_ingest.asset import build_asset, canonical_bytes

def _rec(rid="r1", sha="ab" * 32, ts=1000):
    return ExecutionRecord(rid, ts, "k", "s", "openai", "gpt", 200, 10, 20, sha, "/p", "human")

def test_same_record_yields_same_asset_id():
    a, b = build_asset(_rec(), Source.HUMAN), build_asset(_rec(), Source.HUMAN)
    assert a.asset_id == b.asset_id

def test_different_artifact_hash_yields_different_asset_id():
    a = build_asset(_rec(sha="ab" * 32), Source.HUMAN)
    b = build_asset(_rec(sha="cd" * 32), Source.HUMAN)
    assert a.asset_id != b.asset_id

def test_source_is_part_of_identity():
    a = build_asset(_rec(), Source.HUMAN)
    b = build_asset(_rec(), Source.AGENT)
    assert a.asset_id != b.asset_id

def test_canonicalization_is_key_order_independent():
    assert canonical_bytes({"b": 1, "a": 2}, 1) == canonical_bytes({"a": 2, "b": 1}, 1)

def test_canonicalization_includes_schema_version():
    assert canonical_bytes({"a": 1}, 1) != canonical_bytes({"a": 1}, 2)

def test_rejects_non_finite_numbers():
    with pytest.raises(ValueError):
        canonical_bytes({"x": float("nan")}, 1)

def test_asset_is_immutable():
    a = build_asset(_rec(), Source.HUMAN)
    with pytest.raises(Exception):
        a.asset_id = "tampered"

def test_source_recorded_on_asset():
    assert build_asset(_rec(), Source.UNKNOWN).source == "unknown"
```

- [ ] **Step 2: Run to verify it fails**

```bash
cd /home/johnh/services/samwise/samwise-ingest
python -m pytest tests/test_asset.py -v
```

Expected: FAIL — module does not exist.

- [ ] **Step 3: Implement**

`samwise-ingest/src/samwise_ingest/asset.py`:

```python
"""Immutable, content-addressed assets derived from OmniRoute artifacts.

Identity is computed, never assigned. The same artifact always produces the same
asset_id, which is what makes ingest idempotent: re-ingesting is a no-op.
"""
from __future__ import annotations

import hashlib
import json
import math
import unicodedata
from dataclasses import dataclass

from .omniroute_reader import ExecutionRecord
from .source import Source

SCHEMA_VERSION = 1
CANONICALIZATION_VERSION = "canon-v1"
_DOMAIN = b"samwise/asset/"


def _normalize(value):
    if isinstance(value, float):
        if math.isnan(value) or math.isinf(value):
            raise ValueError("NaN and Infinity are not representable in a canonical payload")
        return value
    if isinstance(value, str):
        return unicodedata.normalize("NFC", value)
    if isinstance(value, dict):
        return {_normalize(k): _normalize(v) for k, v in value.items()}
    if isinstance(value, list):
        return [_normalize(v) for v in value]  # array order is preserved
    return value


def canonical_bytes(payload: dict, schema_version: int) -> bytes:
    envelope = {
        "schema_version": schema_version,
        "canonicalization_version": CANONICALIZATION_VERSION,
        "payload": _normalize(payload),
    }
    return _DOMAIN + json.dumps(
        envelope, sort_keys=True, separators=(",", ":"), ensure_ascii=False,
        allow_nan=False,
    ).encode("utf-8")


@dataclass(frozen=True)
class Asset:
    asset_id: str
    asset_type: str
    schema_version: int
    canonical_payload: dict
    content_hash: str
    source: str
    observed_at: int


def build_asset(record: ExecutionRecord, source: Source) -> Asset:
    payload = {
        "request_id": record.request_id,
        "artifact_sha256": record.artifact_sha256,
        "artifact_path": record.artifact_path,
        "session_id": record.session_id,
        "api_key_id": record.api_key_id,
        "provider": record.provider,
        "model": record.model,
        "status_code": record.status_code,
        "input_tokens": record.input_tokens,
        "output_tokens": record.output_tokens,
        "source": source.value,
        "observed_at": record.timestamp,
    }
    digest = hashlib.sha256(canonical_bytes(payload, SCHEMA_VERSION)).hexdigest()
    return Asset(
        asset_id=digest,
        asset_type="observation",
        schema_version=SCHEMA_VERSION,
        canonical_payload=payload,
        content_hash=digest,
        source=source.value,
        observed_at=record.timestamp,
    )
```

- [ ] **Step 4: Run to verify it passes**

```bash
python -m pytest tests/test_asset.py -v
```

Expected: 8 passed.

- [ ] **Step 5: Commit**

```bash
cd /home/johnh/services/samwise
git add samwise-ingest/
git commit -m "feat(ingest): content-addressed immutable assets"
```

---

### Task 5: Idempotent writer and YantrikDB-resident cursor

**Files:**
- Create: `samwise-ingest/src/samwise_ingest/writer.py`
- Create: `samwise-ingest/tests/test_writer.py`

**Interfaces:**
- Consumes: `Asset` (Task 4), the contract from `docs/INGEST-CONTRACT.md` (Task 1).
- Produces:
  - `YantrikStore` protocol with `write(asset_id: str, payload: dict) -> None`, `exists(asset_id: str) -> bool`, `kv_get(key: str) -> str | None`, `kv_set(key: str, value: str) -> None`
  - `ingest_assets(store: YantrikStore, assets: list[Asset]) -> IngestResult`
  - `IngestResult` with `written: int`, `skipped: int`
  - `CURSOR_KEY = "samwise/ingest/cursor"`, `get_cursor(store) -> int`, `set_cursor(store, ts: int) -> None`

**Why the cursor lives in YantrikDB:** the projector must be stateless. A local cursor file would be original state outside YantrikDB, which the axiom forbids, and would desynchronise if the projector is moved or restarted on another node.

- [ ] **Step 1: Write the failing test**

`samwise-ingest/tests/test_writer.py`:

```python
from samwise_ingest.asset import build_asset
from samwise_ingest.omniroute_reader import ExecutionRecord
from samwise_ingest.source import Source
from samwise_ingest.writer import ingest_assets, get_cursor, set_cursor, CURSOR_KEY

class FakeStore:
    def __init__(self):
        self.records, self.kv, self.write_calls = {}, {}, 0
    def write(self, asset_id, payload):
        self.write_calls += 1
        self.records[asset_id] = payload
    def exists(self, asset_id):
        return asset_id in self.records
    def kv_get(self, key):
        return self.kv.get(key)
    def kv_set(self, key, value):
        self.kv[key] = value

def _asset(rid="r1", sha="ab" * 32, ts=1000):
    rec = ExecutionRecord(rid, ts, "k", "s", "openai", "gpt", 200, 1, 1, sha, "/p", "human")
    return build_asset(rec, Source.HUMAN)

def test_writes_new_assets():
    s = FakeStore()
    assert ingest_assets(s, [_asset()]).written == 1

def test_reingesting_same_asset_is_a_noop():
    s = FakeStore()
    ingest_assets(s, [_asset()])
    result = ingest_assets(s, [_asset()])
    assert (result.written, result.skipped) == (0, 1)
    assert s.write_calls == 1

def test_duplicates_within_one_batch_written_once():
    s = FakeStore()
    result = ingest_assets(s, [_asset(), _asset()])
    assert (result.written, result.skipped) == (1, 1)

def test_cursor_defaults_to_zero_when_absent():
    assert get_cursor(FakeStore()) == 0

def test_cursor_round_trips_through_the_store():
    s = FakeStore()
    set_cursor(s, 4242)
    assert get_cursor(s) == 4242
    assert s.kv[CURSOR_KEY] == "4242"

def test_cursor_is_not_stored_locally(tmp_path, monkeypatch):
    monkeypatch.chdir(tmp_path)
    s = FakeStore()
    set_cursor(s, 99)
    assert list(tmp_path.iterdir()) == []
```

- [ ] **Step 2: Run to verify it fails**

```bash
cd /home/johnh/services/samwise/samwise-ingest
python -m pytest tests/test_writer.py -v
```

Expected: FAIL — module does not exist.

- [ ] **Step 3: Implement**

`samwise-ingest/src/samwise_ingest/writer.py`:

```python
"""Idempotent asset writer. The cursor lives in YantrikDB, never on local disk."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Protocol

from .asset import Asset

CURSOR_KEY = "samwise/ingest/cursor"


class YantrikStore(Protocol):
    def write(self, asset_id: str, payload: dict) -> None: ...
    def exists(self, asset_id: str) -> bool: ...
    def kv_get(self, key: str) -> str | None: ...
    def kv_set(self, key: str, value: str) -> None: ...


@dataclass(frozen=True)
class IngestResult:
    written: int
    skipped: int


def ingest_assets(store: YantrikStore, assets: list[Asset]) -> IngestResult:
    written = skipped = 0
    seen: set[str] = set()
    for asset in assets:
        if asset.asset_id in seen or store.exists(asset.asset_id):
            skipped += 1
            continue
        store.write(asset.asset_id, asset.canonical_payload)
        seen.add(asset.asset_id)
        written += 1
    return IngestResult(written=written, skipped=skipped)


def get_cursor(store: YantrikStore) -> int:
    raw = store.kv_get(CURSOR_KEY)
    return int(raw) if raw else 0


def set_cursor(store: YantrikStore, ts: int) -> None:
    store.kv_set(CURSOR_KEY, str(ts))
```

- [ ] **Step 4: Run to verify it passes**

```bash
python -m pytest tests/test_writer.py -v
```

Expected: 6 passed.

- [ ] **Step 5: Commit**

```bash
cd /home/johnh/services/samwise
git add samwise-ingest/
git commit -m "feat(ingest): idempotent writer with YantrikDB-resident cursor"
```

---

### Task 6: The projection pass and its CLI

**Files:**
- Create: `samwise-ingest/src/samwise_ingest/pass_runner.py`
- Create: `samwise-ingest/src/samwise_ingest/cli.py`
- Create: `samwise-ingest/tests/test_pass_runner.py`

**Interfaces:**
- Consumes: everything from Tasks 2–5.
- Produces: `run_pass(db_path: str, store: YantrikStore, batch: int = 1000) -> PassResult`; `PassResult` with `read: int`, `written: int`, `skipped: int`, `cursor: int`; console entry point `samwise-ingest`.

- [ ] **Step 1: Write the failing test**

`samwise-ingest/tests/test_pass_runner.py`:

```python
import sqlite3
from samwise_ingest.pass_runner import run_pass
from tests.test_writer import FakeStore

def _db(path, rows):
    con = sqlite3.connect(path)
    con.execute("""CREATE TABLE call_logs (
        id TEXT PRIMARY KEY, timestamp INTEGER, api_key_id TEXT, session_id TEXT,
        provider TEXT, model TEXT, status_code INTEGER, input_tokens INTEGER,
        output_tokens INTEGER, artifact_sha256 TEXT, artifact_path TEXT, turn_type TEXT)""")
    con.executemany("INSERT INTO call_logs VALUES (?,?,?,?,?,?,?,?,?,?,?,?)", rows)
    con.commit(); con.close()

def _rows(n, start=1):
    return [(f"r{i}", i * 1000, "k", "s", "openai", "gpt", 200, 1, 1, f"{i:064x}", "/p",
             "human" if i % 2 else "agent") for i in range(start, start + n)]

def test_pass_ingests_all_and_advances_cursor(tmp_path):
    db = tmp_path / "o.db"; _db(db, _rows(3))
    s = FakeStore()
    r = run_pass(str(db), s)
    assert (r.read, r.written, r.cursor) == (3, 3, 3000)

def test_second_pass_ingests_nothing_new(tmp_path):
    db = tmp_path / "o.db"; _db(db, _rows(3))
    s = FakeStore()
    run_pass(str(db), s)
    r = run_pass(str(db), s)
    assert (r.read, r.written) == (0, 0)

def test_pass_picks_up_records_added_after_the_cursor(tmp_path):
    db = tmp_path / "o.db"; _db(db, _rows(2))
    s = FakeStore()
    run_pass(str(db), s)
    con = sqlite3.connect(db)
    con.executemany("INSERT INTO call_logs VALUES (?,?,?,?,?,?,?,?,?,?,?,?)", _rows(1, start=3))
    con.commit(); con.close()
    r = run_pass(str(db), s)
    assert (r.read, r.written, r.cursor) == (1, 1, 3000)

def test_cursor_not_advanced_when_nothing_read(tmp_path):
    db = tmp_path / "o.db"; _db(db, [])
    s = FakeStore()
    assert run_pass(str(db), s).cursor == 0

def test_source_is_carried_into_the_written_payload(tmp_path):
    db = tmp_path / "o.db"; _db(db, _rows(2))
    s = FakeStore()
    run_pass(str(db), s)
    assert {p["source"] for p in s.records.values()} == {"human", "agent"}
```

- [ ] **Step 2: Run to verify it fails**

```bash
cd /home/johnh/services/samwise/samwise-ingest
python -m pytest tests/test_pass_runner.py -v
```

Expected: FAIL — module does not exist.

- [ ] **Step 3: Implement the pass**

`samwise-ingest/src/samwise_ingest/pass_runner.py`:

```python
"""One projection pass: read OmniRoute, build assets, write to YantrikDB, advance cursor.

The cursor advances only after a successful write. A crash mid-pass re-reads the same
window on the next run, and idempotent identity makes the replay harmless.
"""
from __future__ import annotations

from dataclasses import dataclass

from .asset import build_asset
from .omniroute_reader import read_since
from .source import classify
from .writer import YantrikStore, get_cursor, ingest_assets, set_cursor


@dataclass(frozen=True)
class PassResult:
    read: int
    written: int
    skipped: int
    cursor: int


def run_pass(db_path: str, store: YantrikStore, batch: int = 1000) -> PassResult:
    cursor = get_cursor(store)
    records = read_since(db_path, after_timestamp=cursor, limit=batch)
    if not records:
        return PassResult(read=0, written=0, skipped=0, cursor=cursor)

    assets = [build_asset(r, classify(r)) for r in records]
    result = ingest_assets(store, assets)

    new_cursor = max(r.timestamp for r in records)
    set_cursor(store, new_cursor)
    return PassResult(
        read=len(records), written=result.written,
        skipped=result.skipped, cursor=new_cursor,
    )
```

- [ ] **Step 4: Implement the CLI**

`samwise-ingest/src/samwise_ingest/cli.py`:

```python
"""Console entry point.

Run far more often than OmniRoute's retention bounds — DEFAULT_CALL_LOG_RETENTION_DAYS
is 7 and DEFAULT_CALL_LOGS_TABLE_MAX_ROWS is 100000. Hourly is a safe default; a
high-throughput day can hit the row cap long before the day cap.
"""
from __future__ import annotations

import argparse
import os
import sys

from .pass_runner import run_pass


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="samwise-ingest")
    parser.add_argument(
        "--omniroute-db",
        default=os.path.expanduser("~/.omniroute/omniroute.db"),
        help="path to OmniRoute's SQLite database (opened read-only)",
    )
    parser.add_argument("--batch", type=int, default=1000)
    args = parser.parse_args(argv)

    from .store import open_store  # built per docs/INGEST-CONTRACT.md

    with open_store() as store:
        result = run_pass(args.omniroute_db, store, batch=args.batch)

    print(
        f"read={result.read} written={result.written} "
        f"skipped={result.skipped} cursor={result.cursor}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
```

Add to `samwise-ingest/pyproject.toml`:

```toml
[project.scripts]
samwise-ingest = "samwise_ingest.cli:main"
```

- [ ] **Step 5: Implement the concrete store**

Create `samwise-ingest/src/samwise_ingest/store.py` implementing `open_store()` as a context manager returning a `YantrikStore`, using **exactly** the calls recorded in `docs/INGEST-CONTRACT.md` from Task 1. Do not invent calls; if the contract lacks one, return to Task 1.

- [ ] **Step 6: Run the full suite**

```bash
python -m pytest tests/ -v
```

Expected: all tests pass (33 across the five test modules).

- [ ] **Step 7: Run one real pass and verify idempotency against live data**

```bash
samwise-ingest --batch 50
samwise-ingest --batch 50
```

Expected: the first prints `written=` greater than 0; the second prints `read=0 written=0` with an unchanged cursor.

- [ ] **Step 8: Commit**

```bash
cd /home/johnh/services/samwise
git add samwise-ingest/
git commit -m "feat(ingest): projection pass and CLI

Reads OmniRoute read-only, builds content-addressed assets, writes them
idempotently to YantrikDB, and advances a cursor held in YantrikDB. Wire 2
rides the same pass: each execution carries source human/agent/unknown."
```

---

### Task 7: Schedule the pass and backfill existing history

**Files:**
- Create: `samwise-ingest/README.md`
- Create: `scripts/samwise-ingest.timer.example`

**Interfaces:**
- Consumes: the CLI from Task 6.
- Produces: operating instructions and a scheduling unit.

**Why this task exists:** every hour the pass does not run, OmniRoute is one hour closer to deleting evidence that has never been ingested. History older than the retention window is already unrecoverable — the compounding record begins at first successful backfill.

- [ ] **Step 1: Backfill everything currently retained**

```bash
while true; do
  out=$(samwise-ingest --batch 1000)
  echo "$out"
  echo "$out" | grep -q "read=0" && break
done
```

Expected: successive lines with an advancing cursor, terminating on `read=0`.

- [ ] **Step 2: Record what the backfill captured**

```bash
sqlite3 "file:$HOME/.omniroute/omniroute.db?mode=ro" \
  "SELECT COUNT(*), MIN(timestamp), MAX(timestamp) FROM call_logs;"
```

Note the oldest retained timestamp. Anything before it was deleted before ingest existed and cannot be recovered. State this in the README rather than implying full history.

- [ ] **Step 3: Write the schedule unit**

`scripts/samwise-ingest.timer.example`:

```ini
# Install: copy both units to ~/.config/systemd/user/, then
#   systemctl --user enable --now samwise-ingest.timer
#
# --- samwise-ingest.service ---
# [Unit]
# Description=SAMWISE evidence projection pass
# [Service]
# Type=oneshot
# ExecStart=%h/.local/bin/samwise-ingest --batch 1000
#
# --- samwise-ingest.timer ---
[Unit]
Description=Run the SAMWISE evidence projection pass hourly

[Timer]
OnBootSec=5min
OnUnitActiveSec=1h
Persistent=true

[Install]
WantedBy=timers.target
```

- [ ] **Step 4: Write the README**

`samwise-ingest/README.md` must state: what the projector does; that it opens OmniRoute read-only and never writes to it; that its cursor lives in YantrikDB so the process is stateless; that it must run more often than both `DEFAULT_CALL_LOG_RETENTION_DAYS = 7` and `DEFAULT_CALL_LOGS_TABLE_MAX_ROWS = 100000`; the oldest timestamp captured by the backfill from Step 2; and that executions with `source = unknown` are excluded from HITL coordination-tax queries rather than assumed.

- [ ] **Step 5: Verify the timer parses**

```bash
systemd-analyze verify scripts/samwise-ingest.timer.example 2>&1 | head -5 || \
  echo "systemd unavailable; schedule with cron: 0 * * * * ~/.local/bin/samwise-ingest --batch 1000"
```

- [ ] **Step 6: Commit**

```bash
cd /home/johnh/services/samwise
git add samwise-ingest/README.md scripts/samwise-ingest.timer.example
git commit -m "docs(ingest): backfill record and hourly schedule"
```

---

## Spec coverage review

- Wire 1, evidence reaching YantrikDB inside the retention window: Tasks 2, 4, 5, 6, 7.
- Wire 2, human intents as `source: human` nodes: Task 3, carried through Tasks 4 and 6.
- Read-only against OmniRoute: Task 2, Step 3 (`mode=ro`) and its test.
- Ingest only at the typed-asset boundary: Task 4 payload is built from artifact metadata, never message content.
- Idempotency, duplicates cannot inflate support: Task 5, Steps 1 and 3.
- Partial evidence explicit: Task 3 (`UNKNOWN` never guessed), Task 7 Step 4 (excluded from queries).
- Projector statelessness, cursor in YantrikDB: Task 5 (`CURSOR_KEY`, `test_cursor_is_not_stored_locally`).
- Retention deadline honoured: Task 6 Step 4 docstring, Task 7 Steps 1 and 3.
- Unknown YantrikDB API handled without guessing: Task 1, referenced by Task 6 Step 5.

Deliberately out of scope: the governor control law, CEI enforcement, skill promotion, and benchmark confound control. Each depends on accumulated history that these wires must first produce.
