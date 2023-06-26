ENDPOINT ?= eos.firehose.eosnation.io:9001
STARTBLOCK ?= 316729079
.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_transfers -s $(STARTBLOCK) -t +1000

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_transfers -s $(STARTBLOCK) -t +1000

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

