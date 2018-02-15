pragma solidity ^0.4.14;
import "./SafeMath.sol";

contract ETHWallet {
    using SafeMath for uint256;

    mapping (address => uint256) ethBalances;

    function quickDeposit () public payable {
        require(msg.value != 0);
        ethBalances[msg.sender] = ethBalances[msg.sender].add(msg.value);
    }


    function depositToAddress (address _address) public payable {
        require(msg.value != 0 && _address != address(0));
        ethBalances[_address] = ethBalances[_address].add(msg.value);
    }


    function withdraw (uint256 amount) public {
        ethBalances[msg.sender] = ethBalances[msg.sender].sub(amount);
        msg.sender.transfer(amount);
    }

    function balanceOf (address _address) public view returns (uint256 balance) {
        return ethBalances[_address];
    }
}
