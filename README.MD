# Building the private chain

compile with
`cargo build`

Generate new user for chain admin
`./target/debug/myChain-node key generate --scheme Sr25519 --password-interactive`

Modify chain specification in the JSON created by this command. Includes account perms:

`./target/debug/myChain-node build-spec --disable-default-bootnode --chain local > customSpec.json`


Start first node:

`./target/debug/myChain-node \
--base-path /tmp/myChain01 \
--chain ./customSpecRaw.json \
--port 30333 \
--ws-port 9945 \
--rpc-port 9933 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--name MyNode01 \
--validator`


Add additional node:

`./target/debug/myChain-node \
--base-path /tmp/myChain02 \
--chain ./customSpecRaw.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--name MyNode02 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/ //ADD MASTER NODE IDENTIFIER HERE`


| :<div style="min-width:110pt;font-weight:bold;">Option</div> | :<div style="font-weight:bold;">Description</div>                                                                                                                                                                                          |
| :----------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--base-path`                                                | Specifies the directory for storing all of the data related to this chain.                                                                                                                                                                 |
| `--chain ./customSpecRaw.json`                                              | Points at our customSpec JSON.
                                                                     |
| `--port 30333`                                               | Specifies the port to listen on for peer-to-peer (`p2p`) traffic. Because this tutorial uses two nodes running on the same physical computer to simulate a network, you must explicitly specify a different port for at least one account. |
| `--ws-port 9945`                                             | Specifies the port to listen on for incoming WebSocket traffic. The default port is `9944`. This tutorial uses a custom web socket port number (`9945`).                                                                                   |
| `--rpc-port 9933`                                            | Specifies the port to listen on for incoming RPC traffic. The default port is `9933`.                                                                                                                                                      |
| `--Name`                                           | The Name.                                                                                                             |
| `--telemetry-url`                                            | Specifies where to send telemetry data. For this tutorial, you can send telemetry data to a server hosted by Parity that is available for anyone to use.                                                                                   |
| `--validator`                                                | Specifies that this node participates in block production and finalization for the network.   

The `--bootnodes` option specifies the following information:

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
