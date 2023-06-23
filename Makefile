ENDPOINT ?= eos.firehose.eosnation.io:9001
START_BLOCK ?= 12567146
STOP_BLOCK ?= +10

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_transfers -s -1

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_transfers -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: stream
package: build
	substreams package substreams.yaml
