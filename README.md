# mock-network

This is a simple mock network for Mina's [integration test abstract engine](https://github.com/MinaProtocol/mina/tree/develop/src/lib/integration_test_abstract_engine).

## Usage

Clone and enter the repo

```sh
git clone git@github.com:MinaFoundation/mock-network.git
cd mock-network
```

Build the binary

```sh
cargo build
```

Set the `MOCK_NETWORK` environment variable

```sh
export MOCK_NETWORK=$(pwd)/target/debug/mock
```

Navigate to your local `mina` repo and run the abstract engine with the mock network from the project root

```sh
test_executive abstract mock --mina-image mock --config ./integration_tests/mock_config.json \
  | tee mina.log \
  | logproc -i inline -f '!(.level in ["Spam", "Debug"])'
```
