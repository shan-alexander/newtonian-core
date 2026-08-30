---
tags: [otp, gateway, supervision]
node_type: edge_case
---
# Executive and gateway die independently

If one process owns chart, beliefs, *and* the socket, a panic in I/O wipes committed stance. OTP already knew this.

The persistable intention is `{config, context, history}` on disk (or in the parent). The gateway may be a sibling task that only admits and writes. The Executive may restart, restore the snapshot, then *reconcile* via messages (`PeerGone`, `Killed`, `Resync`). Restore is not reconcile. See [[docs/edge_cases/restore-snapshot-is-not-beliefs]].

Skills at L0 (estop) must outlive both.
