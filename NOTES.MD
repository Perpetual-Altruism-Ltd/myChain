# Building the private chain

compile with
`cargo build`

Generate new user for chain admin
`./target/debug/myChain-node key generate --scheme Sr25519 --password-interactive`


## Start first node:

`./target/debug/myChain-node \
--base-path /tmp/node01 \
--chain ./customSpecRaw.json \
--port 30333 \
--ws-port 9945 \
--rpc-port 9933 \
--telemetry-url "ws://cennznet-telemetry.centrality.me:8000/submit 0" \
--rpc-methods Unsafe \
--name MyNode01 \
--validator`


### Add additional node:

`./target/debug/myChain-node \
--base-path /tmp/node11 \
--chain ./myChainRaw.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "ws://cennznet-telemetry.centrality.me:8000/submit 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode11 \
--bootnodes /ip4/18.130.42.208/tcp/30333/p2p/12D3KooWBjaKUwjTQoEoz4z5RqcvoerAHKEwbpji93v1kjM8QtA7`


### ADD KEYS

`./target/debug/myChain-node key insert --base-path /tmp/node01 \

--chain customSpecRaw.json \

--scheme Sr25519 \

--suri "capable meat travel addict size armor giant oppose poverty awkward morning asthma" \

--password-interactive \

--key-type aura`


`./target/debug/myChain-node key insert --base-path /tmp/node01 \

--chain customSpecRaw.json \

--scheme Ed25519 \

--suri "capable meat travel addict size armor giant oppose poverty awkward morning asthma" \

--password-interactive \

--key-type gran`


`./target/debug/myChain-node key insert --base-path /tmp/node02 \

--chain customSpecRaw.json \

--scheme Sr25519 \

--suri "supreme faith private wing render stem chronic direct equip three exhaust liberty" \

--password-interactive \

--key-type aura`


`./target/debug/myChain-node key insert --base-path /tmp/node02 \

--chain customSpecRaw.json \

--scheme Ed25519 \

--suri "supreme faith private wing render stem chronic direct equip three exhaust liberty" \

--password-interactive \

--key-type gran`


### KEYS:

  Secret phrase `capable meat travel addict size armor giant oppose poverty awkward morning asthma` is account:

  Secret seed:       0x615dee7d2e0e9330d3affb947195f260a714bdccc9e44b0a160f1f0c76eb4678

  Public key (hex):  0x6afbff2acf7942bac3acc2874840a62d442a3ec036e9c043bab1175ebd0e2241

  Public key (SS58): 5EUynsE7m1aN3qNBMFcyB2xcXXLjWsR2m5CZQUVV1ARHRDVf

  Account ID:        0x6afbff2acf7942bac3acc2874840a62d442a3ec036e9c043bab1175ebd0e2241

  SS58 Address:      5EUynsE7m1aN3qNBMFcyB2xcXXLjWsR2m5CZQUVV1ARHRDVf

  Secret phrase `capable meat travel addict size armor giant oppose poverty awkward morning asthma` is account:


  Secret seed:       0x615dee7d2e0e9330d3affb947195f260a714bdccc9e44b0a160f1f0c76eb4678

  Public key (hex):  0x7d509e116f453e3e6c4ad305a4e8a920fe690d978eeccae3a6eeef5f5ca5c958

  Public key (SS58): 5Eu1nqdUoQkSXP5zsYEpLgWLCABf27hDv2DRj4e5ewzKUNVo

  Account ID:        0x7d509e116f453e3e6c4ad305a4e8a920fe690d978eeccae3a6eeef5f5ca5c958

  SS58 Address:      5Eu1nqdUoQkSXP5zsYEpLgWLCABf27hDv2DRj4e5ewzKUNVo


  Secret phrase `supreme faith private wing render stem chronic direct equip three exhaust liberty` is account:

  Secret seed:       0x9b562e287088846026b305504379b48e99521006f0daea083df9510516de449b

  Public key (hex):  0x76d712503053ec46d63d0c0f45c4cc70f38915c5f5eb111cb0a0a76a7b843450

  Public key (SS58): 5EkXPzFmqEQ6wvBe6ggw9xFJBvq4i6No2ynPfosxYVbExRqD

  Account ID:        0x76d712503053ec46d63d0c0f45c4cc70f38915c5f5eb111cb0a0a76a7b843450

  SS58 Address:      5EkXPzFmqEQ6wvBe6ggw9xFJBvq4i6No2ynPfosxYVbExRqD


  Secret phrase `supreme faith private wing render stem chronic direct equip three exhaust liberty` is account:

  Secret seed:       0x9b562e287088846026b305504379b48e99521006f0daea083df9510516de449b

  Public key (hex):  0x7fdd0c074ae8778474e7989d567ca67eaafa4bcee0cff9d5f313e9d25353b423

  Public key (SS58): 5ExMbwP1JE1eKSK1RcPT1YdmnUhPSLfZ9iN8eDW9tN1EeuNx

  Account ID:        0x7fdd0c074ae8778474e7989d567ca67eaafa4bcee0cff9d5f313e9d25353b423

  SS58 Address:      5ExMbwP1JE1eKSK1RcPT1YdmnUhPSLfZ9iN8eDW9tN1EeuNx

./target/debug/myChain-node build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json

### Params:

- `--base-path` Specifies the directory for storing all of the data related to this chain.
- `--chain ./customSpecRaw.json` oints at our customSpec JSON.
- `--port 30333` Specifies the port to listen on for peer-to-peer (`p2p`) traffic. Because this tutorial uses two nodes running on the same physical computer to simulate a network, you must explicitly specify a different port for at least one account.
- `--ws-port 9945` Specifies the port to listen on for incoming WebSocket traffic. The default port is `9944`. This tutorial uses a custom web socket port number (`9945`).
- `--rpc-port 9933` Specifies the port to listen on for incoming RPC traffic. The default port is `9933`.
- `--Name` The name.
- `--telemetry-url` Specifies where to send telemetry data. For this tutorial, you can send telemetry data to a server hosted by Parity that is available for anyone to use.
- `--validator` Specifies that this node participates in block production and finalization for the network.



### The `--bootnodes` option specifies the following information:

- `ip4` indicates that the IP address for the node uses the IPv4 format
- `127.0.0.1` specifies the IP address for the running node..
- `tcp` specifies TCP as the protocol used for peer-to-peer communication.
- `30333` specifies the port number used for peer-to-peer communication.
    In this case, the port number for TCP traffic.
- `12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp` identifies the running node to communicate with for this network.

ChainID: 55555

Default http: 127.0.0.1:9933

for help run `./target/debug/myChain-node --help`
Private netowrk tutorial: https://docs.substrate.io/tutorials/v3/private-network/
