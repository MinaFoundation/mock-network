# mock-network

This is a simple mock network for Mina's [integration test abstract engine](https://github.com/MinaFoundation/mina/tree/abstract_engine/src/lib/integration_test_abstract_engine). All it does it return the expected network start up data to Lucy and bails before any node interactions begin.

## Usage with the [test executive](https://github.com/MinaProtocol/mina/tree/develop/src/app/test_executive)

Clone and enter the repo

```sh
git clone git@github.com:MinaFoundation/mock-network.git
cd mock-network
```

Build the binary

```sh
cargo build
```

Set the `MINA_NETWORK_RUNNER` environment variable

```sh
export MINA_NETWORK_RUNNER=$(pwd)/target/debug/mock
```

Navigate to your local `mina` repo and run the abstract engine with the mock network from the project root

```sh
dune exec src/app/test_executive/test_executive.exe abstract mock \
  --mina-image mock \
  --config ./integration_tests/mock_config.json \
  | tee mina.log | logproc -i inline -f '!(.level in ["Spam", "Debug"])'
```

By default, the test executive uses the value of the `MINA_NETWORK_RUNNER` env var. Alternatively, the mock network binary can be passed explicitly to the test executive via the `--network-runner` flag like so

```sh
dune exec src/app/test_executive/test_executive.exe abstract mock \
  --mina-image mock \
  --config ./integration_tests/mock_config.json \
  --network-runner absolute/path/to/mock-network/binary \
  | tee mina.log | logproc -i inline -f '!(.level in ["Spam", "Debug"])'
```
