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
--rpc-methods Unsafe \
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



ChainID: 55555

Default http: 127.0.0.1:9933

for help run `./target/debug/myChain-node --help`
Private netowrk tutorial: https://docs.substrate.io/tutorials/v3/private-network/
