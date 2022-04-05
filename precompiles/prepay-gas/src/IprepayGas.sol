//@dev Will convert to interface.
//@dev Estimation of the rust/precompile implementation

//SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract feeLess {
    address owner = msg.sender;

    mapping(address => bool) whiteList;

    mapping(uint256 => bool) usedNonces;

    uint256 balance;
    

    constructor() {
      whiteList[owner] = true;
    }

    function fund() public payable { 
      balance = balance + msg.value;
    }

    function withdraw(uint256 amount) public {
      require(msg.sender == owner);
      require(balance >= amount);
      payable(msg.sender).transfer(amount);
    }

    function updateWhitelist(address account, bool value) public {
    whiteList[account] = value;
    }


    function getGas(uint256 gasAmount, uint256 nonce, bytes memory sig) public {
        require(!usedNonces[nonce]);
        usedNonces[nonce] = true;

        bytes32 message = getHash(keccak256(abi.encodePacked(msg.sender, gasAmount, nonce, this)));

        require(whiteList[recoverSigner(message, sig)]);

        payable(msg.sender).transfer(gasAmount);
        balance = balance - gasAmount;
    }

    // Destroy contract and reclaim leftover funds.
    function kill() public {
        require(msg.sender == owner);
        selfdestruct(payable(msg.sender));
    }


    function splitSignature(bytes memory sig)
        internal
        pure
        returns (uint8, bytes32, bytes32)
    {
        require(sig.length == 65);

        bytes32 r;
        bytes32 s;
        uint8 v;

        assembly {
            // first 32 bytes, after the length prefix
            r := mload(add(sig, 32))
            // second 32 bytes
            s := mload(add(sig, 64))
            // final byte (first byte of the next 32 bytes)
            v := byte(0, mload(add(sig, 96)))
        }

        return (v, r, s);
    }

    function recoverSigner(bytes32 message, bytes memory sig)
        internal
        pure
        returns (address)
    {
        uint8 v;
        bytes32 r;
        bytes32 s;

        (v, r, s) = splitSignature(sig);

        return ecrecover(message, v, r, s);
    }

    // Builds a prefixed hash to mimic the behavior of eth_sign.
    function getHash(bytes32 hash) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", hash));
    }
}