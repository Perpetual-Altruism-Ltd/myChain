
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
--telemetry-url "ws://cennznet-telemetry.centrality.me:8000/submit 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode11 \
--bootnodes /ip4/mychain-dev.mynft.com/tcp/30333/p2p/12D3KooWBjaKUwjTQoEoz4z5RqcvoerAHKEwbpji93v1kjM8QtA7
```

* remote http `mychain-dev.mynft.com`

* remote bootnode `12D3KooWBjaKUwjTQoEoz4z5RqcvoerAHKEwbpji93v1kjM8QtA7`

* Private netowrk tutorial `https://docs.substrate.io/tutorials/v3/private-network/`

## HOW TO LOCAL

* Build with `Cargo build`

* Run with `./target/debug/myChain-node --dev`

* Default http 127.0.0.1:9933

* Default ws 127.0.0.1:9944

## Information

* Build with `cargo build`

* ChainID 7777777

* for help run `./target/debug/myChain-node --help`