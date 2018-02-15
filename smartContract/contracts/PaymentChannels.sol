pragma solidity ^0.4.14;
import "./ECVerify.sol";
import "./ETHWallet.sol";


contract PaymentChannels is ECVerify, ETHWallet {
    struct Channel {
        bytes32 channelId;
        address address0;
        address address1;
        uint256 totalBalance;

        uint256 balance0;
        uint256 balance1;
        bytes hashlocks;
        uint256 sequenceNumber;

        uint256 settlingPeriodLength;
        bool settlingPeriodStarted;
        uint256 settlingPeriodEnd;
        bool closed;
    }

    mapping (bytes32 => Channel) public channels;
    mapping (bytes32 => bool) seenPreimage;

    function channelDoesNotExist (bytes32 _channelId) {
        require(channels[_channelId].channelId != _channelId);
    }

    function channelExists (bytes32 _channelId) {
        require(channels[_channelId].channelId == _channelId);
    }

    function channelSettlingPeriodStarted (bytes32 _channelId) {
        require(channels[_channelId].settlingPeriodStarted);
    }

    function channelSettlingPeriodNotStarted (bytes32 _channelId) {
        require(!channels[_channelId].settlingPeriodStarted);
    }

    function channelIsNotClosed (bytes32 _channelId) {
        require(!channels[_channelId].closed);
    }

    function channelIsSettled (bytes32 _channelId) {
        require(
            channels[_channelId].settlingPeriodStarted && // If the settling period has started
            block.number >= channels[_channelId].settlingPeriodEnd // And ended
        );
    }

    function channelIsNotSettled (bytes32 _channelId) {
        require(!( // Negate the below
            channels[_channelId].settlingPeriodStarted && // If the settling period is started
            block.number >= channels[_channelId].settlingPeriodEnd // And ended
        ));
    }

    function balancesEqualTotal (bytes32 _channelId, uint256 _balance0, uint256 _balance1) {
        require(_balance0.add(_balance1) == channels[_channelId].totalBalance);
    }

    function sequenceNumberIsHighest (bytes32 _channelId, uint256 _sequenceNumber) {
        require(_sequenceNumber > channels[_channelId].sequenceNumber);
    }

    function signedBy (
        bytes32 _fingerprint,
        bytes _signature,
        address _address
    ) {
        require(ecverify(_fingerprint, _signature, _address));
    }

    function signedBys (
        bytes32 _fingerprint,
        bytes _signature,
        address _address
    ) returns (bool){
        return ecverify(_fingerprint, _signature, _address);
    }
    
    function signedByBoth (
        bytes32 _fingerprint,
        bytes _signature0,
        bytes _signature1,
        address _address0,
        address _address1
    ) {
        require(
            ecverify(_fingerprint, _signature0, _address0) &&
            ecverify(_fingerprint, _signature1, _address1)
        );
    }

    function signedByBoths (
        bytes32 _fingerprint,
        bytes _signature0,
        bytes _signature1,
        address _address0,
        address _address1
    ) returns (bool){
            return ecverify(_fingerprint, _signature0, _address0) &&
            ecverify(_fingerprint, _signature1, _address1);
    }

    function signedByOne (
        bytes32 _fingerprint,
        bytes _signature,
        address _address0,
        address _address1
    ) {
        require(
            ecverify(_fingerprint, _signature, _address0) ||
            ecverify(_fingerprint, _signature, _address1)
        );
    }

    function signedByOnes (
        bytes32 _fingerprint,
        bytes _signature,
        address _address0,
        address _address1
    ) returns (bool){
        return
            ecverify(_fingerprint, _signature, _address0) ||
            ecverify(_fingerprint, _signature, _address1);
    }

	//Added for testing purposes
    function recoverAddr(bytes32 msgHash, uint8 v, bytes32 r, bytes32 s) returns (address) {
        return ecrecover(msgHash, v, r, s);
    }

	//Added for testing purposes
    function isSigned(address _addr, int256 num1, int256 num2 , uint8 v, bytes32 r, bytes32 s) returns (bool) {
        bytes32 msgHash = keccak256(num1, num2);
       return ecrecover(msgHash, v, r, s) == _addr;
	}

    function incrementBalance(address _addr, uint _value)
        internal
    {
        ethBalances[_addr] = ethBalances[_addr].add(_value);
    }

    function decrementBalance(address _addr, uint _value)
        internal
    {
        ethBalances[_addr] = ethBalances[_addr].sub(_value);
    }

    function newChannel(
        bytes32 _channelId,

        address _address0,
        address _address1,

        uint256 _balance0,
        uint256 _balance1,

        uint256 _settlingPeriodLength,

        bytes _signature0,
        bytes _signature1
    ) {
        channelDoesNotExist(_channelId);
        bytes32 fingerprint = sha3(
            "newChannel",
            _channelId,

            _address0,
            _address1,

            _balance0,
            _balance1,

            _settlingPeriodLength
        );

        signedByBoth(
            fingerprint,
            _signature0,
            _signature1,
            _address0,
            _address1
        );

        decrementBalance(_address0, _balance0);
        decrementBalance(_address1, _balance1);

        channels[_channelId] = Channel(
            _channelId,                  // bytes32 channelId;
            _address0,                   // address address0;
            _address1,                   // address address1;
            _balance0.add(_balance1),    // uint256 totalBalance;

            _balance0,                   // uint256 balance0;
            _balance1,                   // uint256 balance1;
            new bytes(0),                // bytes hashlocks
            0,                           // uint256 sequenceNumber;

            _settlingPeriodLength,       // uint256 settlingPeriodLength;
            false,                       // bool settlingPeriodStarted;
            0,                           // uint256 settlingPeriodEnd;
            false                        // bool closed;

        );
    }

    function updateState(
        bytes32 _channelId,
        uint256 _sequenceNumber,

        uint256 _balance0,
        uint256 _balance1,

        bytes _hashlocks,

        bytes _signature0,
        bytes _signature1
    ) {
        channelExists(_channelId);
        channelIsNotSettled(_channelId);
        channelIsNotClosed(_channelId);
        sequenceNumberIsHighest(_channelId, _sequenceNumber);

        bytes32 fingerprint = sha3(
            "updateState",
            _channelId,
            _sequenceNumber,
            _balance0,
            _balance1,
            _hashlocks
        );

        signedByBoth(
            fingerprint,
            _signature0,
            _signature1,
            channels[_channelId].address0,
            channels[_channelId].address1
        );

        updateStateInternal(
            _channelId,
            _sequenceNumber,

            _balance0,
            _balance1,

            _hashlocks
        );
    }

    function updateStateInternal (
        bytes32 _channelId,
        uint256 _sequenceNumber,

        uint256 _balance0,
        uint256 _balance1,

        bytes _hashlocks
    )
        internal
    {
        channels[_channelId].sequenceNumber = _sequenceNumber;
        channels[_channelId].balance0 = _balance0;
        channels[_channelId].balance1 = _balance1;
        channels[_channelId].hashlocks = _hashlocks;
    }

    function updateStateWithBounty(
        bytes32 _channelId,
        uint256 _sequenceNumber,

        uint256 _balance0,
        uint256 _balance1,

        bytes _hashlocks,

        bytes _signature0,
        bytes _signature1,

        uint256 _bountyAmount,
        bytes _bountySignature
    ) {
        channelSettlingPeriodStarted(_channelId);

        bytes32 fingerprint = sha3(
            "updateStateWithBounty",
            _channelId,
            _sequenceNumber,
            _balance0,
            _balance1,
            _hashlocks,
            _signature0,
            _signature1,
            _bountyAmount
        );

        address bountyPayer = ecrecovery(fingerprint, _bountySignature);

        decrementBalance(bountyPayer, _bountyAmount);
        incrementBalance(msg.sender, _bountyAmount);

        updateState(
            _channelId,
            _sequenceNumber,

            _balance0,
            _balance1,

            _hashlocks,

            _signature0,
            _signature1
        );
    }

    function submitPreimage (
        bytes32 _hashed,
        bytes32 _preimage
    ) {
        require(_hashed == sha3(_preimage));
        seenPreimage[_hashed] = true;
    }

    function submitPreimages (
        bytes pairs
    ) {
        bytes32 hashed;
        bytes32 preimage;

        for (uint256 i = 0; i < pairs.length; i += 64) {
            uint256 hashedOffset = i + 32;
            uint256 preimageOffset = i + 64;

            assembly {
                hashed := mload(add(pairs, hashedOffset))
                preimage := mload(add(pairs, preimageOffset))
            }

            submitPreimage(hashed, preimage);
        }
    }

    function startSettlingPeriod (
        bytes32 _channelId,
        bytes _signature
    ) {
        channelExists(_channelId);
        channelSettlingPeriodNotStarted(_channelId);

        bytes32 fingerprint = sha3(
            "startSettlingPeriod",
            _channelId
        );

        signedByOne(
            fingerprint,
            _signature,
            channels[_channelId].address0,
            channels[_channelId].address1
        );

        channels[_channelId].settlingPeriodStarted = true;
        channels[_channelId].settlingPeriodEnd = block.number + channels[_channelId].settlingPeriodLength;
    }

    function closeChannel (
        bytes32 _channelId
    ) {
        channelExists(_channelId);
        channelIsSettled(_channelId);
        channelIsNotClosed(_channelId);

        closeChannelInternal(_channelId);
    }

    function closeChannelFast (
        bytes32 _channelId,

        uint256 _sequenceNumber,
        uint256 _balance0,
        uint256 _balance1,
        bytes _hashlocks,

        bytes _signature0,
        bytes _signature1
    ) {
        channelExists(_channelId);
        sequenceNumberIsHighest(_channelId, _sequenceNumber);
        balancesEqualTotal(_channelId, _balance0, _balance1);

        bytes32 fingerprint = sha3(
            "closeChannelFast",
            _channelId,
            _sequenceNumber,
            _balance0,
            _balance1,
            _hashlocks
        );

        signedByBoth(
            fingerprint,
            _signature0,
            _signature1,
            channels[_channelId].address0,
            channels[_channelId].address1
        );

        updateStateInternal(
            _channelId,
            _sequenceNumber,
            _balance0,
            _balance1,
            _hashlocks
        );

        closeChannelInternal(_channelId);
    }

    function closeChannelInternal (
        bytes32 _channelId
    )
        internal
    {
        channels[_channelId].closed = true;

        int256 adjustment = getHashlockAdjustment(channels[_channelId].hashlocks);

        uint256 balance0;
        uint256 balance1;
        (balance0, balance1) = applyHashlockAdjustment(
            _channelId,
            channels[_channelId].balance0,
            channels[_channelId].balance1,
            adjustment
        );

        incrementBalance(channels[_channelId].address0, balance0);
        incrementBalance(channels[_channelId].address1, balance1);
    }

    function getHashlockAdjustment (
        bytes _hashlocks
    )
        internal
        returns (int256)
    {
        bytes32 hashed;
        int256 adjustment;
        int256 totalAdjustment;

        for (uint256 i = 0; i < _hashlocks.length; i += 64) {
            uint256 hashedOffset = i + 32;
            uint256 adjustmentOffset = i + 64;

            assembly {
                hashed := mload(add(_hashlocks, hashedOffset))
                adjustment := mload(add(_hashlocks, adjustmentOffset))
            }

            if (seenPreimage[hashed]) {
                totalAdjustment += adjustment;
            }
        }

        return totalAdjustment;
    }

    function applyHashlockAdjustment (
        bytes32 _channelId,
        uint256 _currentBalance0,
        uint256 _currentBalance1,
        int256 _totalAdjustment
    )
        internal
        returns (uint256, uint256)
    {
        uint256 balance0;
        uint256 balance1;

        if (_totalAdjustment > 0) {
            balance0 = _currentBalance0.add(uint256(_totalAdjustment));
            balance1 = _currentBalance1.sub(uint256(_totalAdjustment));
        }

        if (_totalAdjustment < 0) {
            balance0 = _currentBalance0.sub(uint256(-_totalAdjustment));
            balance1 = _currentBalance1.add(uint256(-_totalAdjustment));
        }

        if (_totalAdjustment == 0) {
            balance0 = _currentBalance0;
            balance1 = _currentBalance1;
        }

        balancesEqualTotal(_channelId, balance0, balance1);
        return (balance0, balance1);
    }
}
