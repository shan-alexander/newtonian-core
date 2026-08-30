---
tags: [snapshot, restore]
node_type: edge_case
---
# Restore snapshot is not beliefs

`IntentionMachine::snapshot` is the machine’s phase space. It is not the belief store, not the mandate, not in-flight `Cmd`s, not the gateway lock.

Restoring a chart onto a live world without reconciling beliefs is how you get “we think we are Online” while the socket is dead. Procedure: restore machine snapshot → restore mandate → *do not* trust stale beliefs → sensors write fresh beliefs → lift may emit `Resync` / `PeerGone` / `FeedStale` → then the configuration is honest.

A later *program* snapshot may be a product type. It is still not an excuse to skip reconcile.
