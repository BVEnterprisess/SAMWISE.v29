# Component Roles

SAMWISE is the composition of three independently shipped systems. Nothing here is built by
this repository; these documents define **who owns what**, so no component silently
duplicates another's authority.

```text
OmniRoute observes and canonicalizes
    → YantrikDB governs evidence and projections
        → MetaClaw governs procedures
            → OmniRoute applies approved decisions during execution
```

OmniRoute is an **active authority on every execution**, never a passive upstream logger — all
inference continues to route through it.

| Component | Speed | Owns | Role document |
|---|---|---|---|
| **OmniRoute** | hot, O(1) | Execution observation, canonicalization, routing, enforcement | *not yet written* — see `../source/omniroute-forensic-autopsy.md` and `../source/omniroute-trace-inventory.md` |
| **YantrikDB** | slow, async | Durable evidence, decay, recall, links, conflicts, correction, deterministic projection, `think()` | [`yantrikdb.md`](yantrikdb.md) |
| **MetaClaw** | hot, O(1) | Procedure validation, bounded injection, usage evaluation, skill evolution | *not yet written* — see `../source/metaclaw-skills-only.md` |

**The gap is real, not an oversight.** Only YantrikDB has a role document. Until OmniRoute and
MetaClaw have theirs, their authority boundaries are asserted in the specs but not stated in one
place, and the source briefs above are the closest thing to a definition.

The governing axiom, which every role document must respect:

> **YantrikDB = Persistent Continuity. Everything else → Stateless.**

Other components may hold state, never **original** state. Falsifiable form — the deletion test:
delete OmniRoute's SQLite or MetaClaw's skill directory and the system rebuilds from YantrikDB
with only warm-up cost. Delete YantrikDB and nothing recovers.
