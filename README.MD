
# My Chain

## HOW TO

* Build with `cargo build`

* Add your node with :
 ```
./target/debug/myChain-node \
--base-path /tmp/node11 \
--chain ./myChainRaw.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode11 \
--bootnodes /ip4/172.31.2.247/tcp/30333/p2p/12D3KooWS9NN5x9x4UwBtGZc3rawc3yavGtj2TxPjd3RY7jHZYx6
```

* connect to http `mychain-dev.mynft.com` / `172.31.2.247`

* remote bootnode `12D3KooWBjaKUwjTQoEoz4z5RqcvoerAHKEwbpji93v1kjM8QtA7`

* Private netowrk tutorial `https://docs.substrate.io/tutorials/v3/private-network/`

## HOW TO LOCAL

* Build with `Cargo build`

* Run with `./target/debug/myChain-node --dev`

## Information

* ChainID 7777777

* Default local http 127.0.0.1:9933

* Default local ws 127.0.0.1:9944

* for help run `./target/debug/myChain-node --help`