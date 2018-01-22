pragma solidity ^0.4.13;

contract Store {

    function recoverAddr(bytes32 msgHash, uint8 v, bytes32 r, bytes32 s) returns (address) {
        return ecrecover(msgHash, v, r, s);
    }

    function isSigned(address _addr, int256 num1, int256 num2 , uint8 v, bytes32 r, bytes32 s) returns (bool) {
        bytes32 msgHash = keccak256(num1, num2);
       return ecrecover(msgHash, v, r, s) == _addr;
}

}
