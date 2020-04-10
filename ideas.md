# Ideas

- Server/Client
- Server does everything, client is just UI/visualization
- Data storage via sled + bincode
  - important: save keys as big endian
- Client only sends commands and data requests (+ watch requests?)
  - all visible/cached data must be watched to allow
- RPC via tonic/gRPC http/2
  - no tarpc since streaming/notifications aren't supported

## Undo/Redo

- Generate the inverse of a Command and keep last n changes?
- and/or snapshot system

## Extensibility

- ?