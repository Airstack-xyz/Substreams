# Substreams
Repo contains all substreams build by airstack.

# Steps to run the substream with substream cli

## Prerequisite

- [Install the substreams CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli
)

- [Setup development env](https://substreams.streamingfast.io/developers-guide/installation-requirements)

- You will need firehose node to run substream, or you can use firehose node provided by streaming fast team for development. [Get a token to access firehose node](https://substreams.streamingfast.io/reference-and-specs/authentication). 


## Run a substream

1. Generate rust code with protogen
```
substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"
```

2. Add target  wasm32-unknown-unknown
```
rustup target add wasm32-unknown-unknown
```

3. Build the project and install the rust dependencies
```
cargo build --target wasm32-unknown-unknown --release
```

If you face any authentication error on downloading dependencies, add below code to your ~/.gitconfig by using `code . ~/.gitconfig`
```
[url "https://github.com/rust-lang/crates.io-index"]
	insteadOf = https://github.com/rust-lang/crates.io-index
```

4. Run the substream. Make sure to change module name & block number based on substream.

```
substreams run -e mainnet.eth.streamingfast.io:443 \
   substreams.yaml \
   map_transfers \
   --start-block 10000000 \
   --stop-block 20000000
```

## Create a substream package (.spkg)

To create substream package, run `substreams pack <path_to_substreams.yaml>`


