# Pulse 08: ARCHITECTURE

Settled. Authored `docs/vtrace/ARCHITECTURE.md`: 7 components (packet-network / corpus /
score / tier / gap / cli + docs review layer), scale allocated to the corpus and gap
layers, downward-only dependency direction, data flow, dependencies, and failure modes
(incl. performance-basis-unknown and coverage-overstatement → hold/label). Fixed point:
removed a potential `corpus→score` cycle. Next: INTERFACES.
