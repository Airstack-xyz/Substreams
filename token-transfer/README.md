# Substreams
Repo contains all substreams build by airstack.

# Steps to run the substream in terminal

## Download and install all the dependencies
### Install the substreams CLI
Use this doc to install the CLI - https://substreams.streamingfast.io/getting-started/installing-the-cli

Use this doc to download and install all the dependencies required to develop on the substream - https://substreams.streamingfast.io/developer-guide/installation-requirements

## Setting up the env
For getting the block data we are using the substream API "api-dev.streaming-fast.io:443" which is using their particular hosted firehose. For accessing the API we need the streaming-fast token which you can get following this doc https://substreams.streamingfast.io/reference-and-specs/authentication

Once you have setup the SUBSTREAMS_API_TOKEN, you are good to run the substream.

## Build the project
### Generate rust code with protogen
```
substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"
```

### Install wasm32-unknown-unknown
```
rustup target add wasm32-unknown-unknown
```

### Use this command to build the project and install the rust dependencies
```
cargo build --target wasm32-unknown-unknown --release
```

## Running the substreams
Just move to the root directory of your substream folder and run this command

```
substreams run -e api-dev.streamingfast.io:443 \
   substreams.yaml \
   map_transfers,store_transfers \
   --start-block 12369621 \
   --stop-block 20000000
```

# Steps to run the consumer

## Move to the consumer directory of the project and create the env folder
Install what is required to build protobufs:

```bash
python3 -m venv env
source env/bin/activate
pip3 install grpcio-tools protobuf==3.20.1
# important note, 3.20.1 works newer updated protobuf seem to cause issues -> https://github.com/protocolbuffers/protobuf/issues/10571
```

## Activate the python environment using
```
source env/bin/activate
```
Before running the consumer make sure you set the SUBSTREAMS_API_TOKEN, after that run the python file "main.py" using
```
python3 main.py
```