ENDPOINT ?= eos.firehose.eosnation.io:9001
STARTBLOCK ?= 316729079

.PHONY: all
all:
	make build
	substreams pack
	substreams graph
	substreams info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_transfers -s $(STARTBLOCK) -t +1000

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_transfers -s $(STARTBLOCK) -t +1000

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: schema_events
schema_events: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_schema_events -s 317182530 -t +10

.PHONY: collections
collections: build
	substreams run -e $(ENDPOINT) substreams.yaml map_collections -s 313817420  -t +1

.PHONY: templates
templates: build
	substreams run -e $(ENDPOINT) substreams.yaml map_templates -s 317648437  -t +1

.PHONY: schemas
schemas: build
	substreams run -e $(ENDPOINT) substreams.yaml map_schemas -s 317377836  -t +1

.PHONY: transfers
transfers: build
	substreams run -e $(ENDPOINT) substreams.yaml map_transfers -s 313477147  -t +1

.PHONY: assets
assets: build
	substreams run -e $(ENDPOINT) substreams.yaml map_assets -s 317928585  -t +1

.PHONY: offers
offers: build
	substreams run -e $(ENDPOINT) substreams.yaml map_offers -s 318586021  -t +1

.PHONY: balances
balances: build
	substreams run -e $(ENDPOINT) substreams.yaml map_balances -s 318382446  -t +1

# Not used for now \
.PHONY: collection_events \
collection_events: build \
	substreams run -e $(ENDPOINT) substreams.yaml map_collection_events -s 317811050 -t +1