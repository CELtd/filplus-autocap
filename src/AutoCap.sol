// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title AutoCap
/// @notice Registry and Coordination Layer for the Burn-to-Earn program
/// @dev Multi-round registry contract - does not handle DataCap distribution directly
contract AutoCap is Ownable {
    // ============ Structs ============

    /// @notice Round configuration
    struct Round {
        uint256 startTime;
        uint256 endTime;
        uint256 registrationFee;
        uint256 totalDatacap; // Total DataCap to be distributed in this round (for reference only)
    }

    // ============ State Variables ============

    /// @notice Address of the Filecoin Pay contract (for reference)
    address public paymentContract;

    /// @notice Counter for the latest round ID
    uint256 public currentRoundId;

    /// @notice Round configurations: roundId => Round
    mapping(uint256 => Round) public rounds;

    /// @notice Registration data: roundId => participant => datacapActorId (0 = not registered)
    mapping(uint256 => mapping(address => uint64)) public roundRegistrations;

    /// @notice Participant list for enumeration: roundId => address[]
    /// @dev Can be replaced by off-chain indexer but for now we keep it here
    mapping(uint256 => address[]) internal _roundParticipantList;

    // ============ Events ============

    event RoundCreated(uint256 indexed roundId, uint256 fee, uint256 startTime, uint256 endTime, uint256 totalDatacap);
    event RoundClosed(uint256 indexed roundId);
    event Registered(uint256 indexed roundId, address indexed spAddress, uint64 datacapActorId);
    event FeesWithdrawn(address indexed owner, uint256 amount);
    event PaymentContractUpdated(address indexed oldAddress, address indexed newAddress);

    // ============ Errors ============

    error InvalidTimeRange();
    error RoundNotOpen();
    error InvalidRegistrationFee(uint256 required, uint256 sent);
    error AlreadyRegistered();
    error InvalidActorId();
    error NoFeesToWithdraw();
    error TransferFailed();

    // ============ Modifiers ============

    /// @notice Ensures the round is currently open for registration
    modifier onlyOpenRound(uint256 _roundId) {
        if (block.timestamp < rounds[_roundId].startTime || block.timestamp > rounds[_roundId].endTime) {
            revert RoundNotOpen();
        }
        _;
    }

    // ============ Constructor ============

    /// @notice Initializes the AutoCap registry
    /// @param _paymentContract Address of the Filecoin Pay contract
    constructor(address _paymentContract) Ownable(msg.sender) {
        paymentContract = _paymentContract;
    }

    // ============ Round Management (Owner Only) ============

    /// @notice Create a new round
    /// @param _startTime Unix timestamp when registration opens
    /// @param _endTime Unix timestamp when registration closes
    /// @param _registrationFee Amount of FIL required to register (in wei)
    /// @param _totalDatacap Total DataCap to be distributed in this round (for reference only)
    /// @return roundId The ID of the created round
    function createRound(uint256 _startTime, uint256 _endTime, uint256 _registrationFee, uint256 _totalDatacap)
        external
        onlyOwner
        returns (uint256 roundId)
    {
        if (_endTime <= _startTime) {
            revert InvalidTimeRange();
        }

        currentRoundId++;
        roundId = currentRoundId;

        rounds[roundId] = Round({
            startTime: _startTime,
            endTime: _endTime,
            registrationFee: _registrationFee,
            totalDatacap: _totalDatacap
        });

        emit RoundCreated(roundId, _registrationFee, _startTime, _endTime, _totalDatacap);
    }

    /// @notice Close a round early
    /// @param _roundId The ID of the round to close
    function closeRound(uint256 _roundId) external onlyOwner onlyOpenRound(_roundId) {
        rounds[_roundId].endTime = block.timestamp;

        emit RoundClosed(_roundId);
    }

    // ============ Registration ============

    /// @notice Register for a round
    /// @param _roundId The ID of the round to register for
    /// @param _datacapActorId The Actor ID where DataCap should be sent
    function register(uint256 _roundId, uint64 _datacapActorId) external payable onlyOpenRound(_roundId) {
        if (msg.value != rounds[_roundId].registrationFee) {
            revert InvalidRegistrationFee(rounds[_roundId].registrationFee, msg.value);
        }

        if (roundRegistrations[_roundId][msg.sender] != 0) {
            revert AlreadyRegistered();
        }

        if (_datacapActorId == 0) {
            revert InvalidActorId();
        }
        // @dev: Do actor id ownership check here if needed

        roundRegistrations[_roundId][msg.sender] = _datacapActorId;
        _roundParticipantList[_roundId].push(msg.sender);

        emit Registered(_roundId, msg.sender, _datacapActorId);
    }

    // ============ View Functions ============

    /// @notice Get participants for a round (paginated)
    /// @param _roundId The round ID
    /// @param _cursor Starting index
    /// @param _limit Maximum number of participants to return
    /// @return participants Array of participant addresses
    /// @return nextCursor The next cursor position (0 if end reached)
    function getParticipants(uint256 _roundId, uint256 _cursor, uint256 _limit)
        external
        view
        returns (address[] memory participants, uint256 nextCursor)
    {
        address[] storage list = _roundParticipantList[_roundId];
        uint256 length = list.length;

        if (_cursor >= length) {
            return (new address[](0), 0);
        }

        uint256 remaining = length - _cursor;
        uint256 count = remaining < _limit ? remaining : _limit;

        participants = new address[](count);
        for (uint256 i = 0; i < count; i++) {
            participants[i] = list[_cursor + i];
        }

        nextCursor = _cursor + count;
        if (nextCursor >= length) {
            nextCursor = 0;
        }
    }

    /// @notice Get the datacap actor ID for a participant in a round
    /// @param _roundId The round ID
    /// @param _participant The participant address
    /// @return datacapActorId The actor ID (0 if not registered)
    function getParticipantDetails(uint256 _roundId, address _participant)
        external
        view
        returns (uint64 datacapActorId)
    {
        return roundRegistrations[_roundId][_participant];
    }

    /// @notice Get the total number of registrants for a round
    /// @param _roundId The round ID
    /// @return count The number of registrants
    function getTotalRegistrants(uint256 _roundId) external view returns (uint256 count) {
        return _roundParticipantList[_roundId].length;
    }

    /// @notice Check if a round is currently open for registration
    /// @param _roundId The round ID
    /// @return isOpen True if round is open
    function isRoundOpen(uint256 _roundId) external view returns (bool isOpen) {
        return block.timestamp >= rounds[_roundId].startTime && block.timestamp <= rounds[_roundId].endTime;
    }

    /// @notice Get round details
    /// @param _roundId The round ID
    /// @return startTime The round start time
    /// @return endTime The round end time
    /// @return registrationFee The registration fee
    /// @return totalDatacap The total DataCap for the round
    function getRound(uint256 _roundId)
        external
        view
        returns (uint256 startTime, uint256 endTime, uint256 registrationFee, uint256 totalDatacap)
    {
        Round storage round = rounds[_roundId];
        return (round.startTime, round.endTime, round.registrationFee, round.totalDatacap);
    }

    // ============ Administrative ============

    /// @notice Withdraw collected registration fees
    function withdrawFees() external onlyOwner {
        uint256 balance = address(this).balance;

        if (balance == 0) {
            revert NoFeesToWithdraw();
        }

        (bool success,) = payable(owner()).call{value: balance}("");
        if (!success) {
            revert TransferFailed();
        }

        emit FeesWithdrawn(owner(), balance);
    }

    /// @notice Update the payment contract address
    /// @param _newPaymentContract The new payment contract address
    function updatePaymentContract(address _newPaymentContract) external onlyOwner {
        address oldAddress = paymentContract;
        paymentContract = _newPaymentContract;

        emit PaymentContractUpdated(oldAddress, _newPaymentContract);
    }
}
