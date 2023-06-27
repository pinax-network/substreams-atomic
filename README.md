# Substream for Antelope NFTs
Antelope NFTs transfers

## Quick Start
```bash
make
make gui
```
### Mermaid graph

```mermaid
graph TD;
  map_transfers[map: map_transfers];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_transfers;
```
## Modules
```yaml
Package name: eosio_nfts
Version: v0.1.0
Doc: Antelope `atomicassets` based action traces.
Modules:
----
Name: map_transfers
Kind: map
Output Type: proto:antelope.eosio.nfts.v1.TransferEvents
```
