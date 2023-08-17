# mock-network

This is a simple mock network for Mina's [integration test abstract engine](https://github.com/MinaProtocol/mina/tree/develop/src/lib/integration_test_abstract_engine).

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

Set the `MOCK_NETWORK` environment variable

```sh
export MOCK_NETWORK=$(pwd)/target/debug/mock
```

Navigate to your local `mina` repo and run the abstract engine with the mock network from the project root (assuming you have an alias `test_executive` for `src/app/test_executive/test_executive.exe`)

```sh
test_executive abstract mock --mina-image mock --config ./integration_tests/mock_config.json \
  | tee mina.log \
  | logproc -i inline -f '!(.level in ["Spam", "Debug"])'
```

By default, the test executive uses the value of the `MOCK_NETWORK` env var. Alternatively, the mock network binary can be passed explicitly to the test executive via the `--mock` flag like so

```sh
test_executive abstract mock --mina-image mock --config ./integration_tests/mock_config.json --mock 'MOCK_NETWORK=/absolute/path/to/mock-network/binary' \
  | tee mina.log \
  | logproc -i inline -f '!(.level in ["Spam", "Debug"])'
```
