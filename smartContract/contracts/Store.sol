pragma solidity ^0.4.13;

contract SmartContract {
	uint myVariable;


	function set(uint x) public {
		myVariable = x;
	}


	function get() constant public returns (uint) {
		return myVariable;
	}

}
