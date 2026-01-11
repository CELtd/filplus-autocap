// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {AutoCap} from "../src/AutoCap.sol";

contract AutoCapTest is Test {
    AutoCap public autocap;
    address public owner = address(0x1);
    address public user1 = address(0x2);
    address public user2 = address(0x3);
    address public user3 = address(0x4);
    address public paymentContract = address(0x100);

    uint256 public constant REGISTRATION_FEE = 0.1 ether;
    uint256 public constant TOTAL_DATACAP = 1000 ether; // 1000 TiB for testing
    uint64 public constant ACTOR_ID_1 = 1024;
    uint64 public constant ACTOR_ID_2 = 2048;
    uint64 public constant ACTOR_ID_3 = 3072;

    event RoundCreated(uint256 indexed roundId, uint256 fee, uint256 startTime, uint256 endTime, uint256 totalDatacap);
    event RoundClosed(uint256 indexed roundId);
    event Registered(uint256 indexed roundId, address indexed spAddress, uint64 datacapActorId);
    event FeesWithdrawn(address indexed owner, uint256 amount);
    event PaymentContractUpdated(address indexed oldAddress, address indexed newAddress);

    function setUp() public {
        // Deal ETH to test accounts
        vm.deal(owner, 100 ether);
        vm.deal(user1, 100 ether);
        vm.deal(user2, 100 ether);
        vm.deal(user3, 100 ether);

        // Etch code at paymentContract to pass NotAContract check
        vm.etch(paymentContract, hex"00");

        vm.prank(owner);
        autocap = new AutoCap(paymentContract);
    }

    // ============ Constructor Tests ============

    function test_Constructor() public {
        assertEq(autocap.owner(), owner);
        assertEq(autocap.paymentContract(), paymentContract);
        assertEq(autocap.currentRoundId(), 0);
    }

    function test_Constructor_ZeroAddress() public {
        vm.prank(owner);
        vm.expectRevert(AutoCap.ZeroAddress.selector);
        new AutoCap(address(0));
    }

    function test_Constructor_NotAContract() public {
        vm.prank(owner);
        vm.expectRevert(AutoCap.NotAContract.selector);
        new AutoCap(address(0xdead));
    }

    // ============ Round Creation Tests ============

    function test_CreateRound() public {
        uint256 startTime = block.timestamp + 1 days;
        uint256 endTime = block.timestamp + 7 days;

        vm.prank(owner);
        vm.expectEmit(true, false, false, true);
        emit RoundCreated(1, REGISTRATION_FEE, startTime, endTime, TOTAL_DATACAP);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);

        assertEq(roundId, 1);
        assertEq(autocap.currentRoundId(), 1);

        (uint256 storedStartTime, uint256 storedEndTime, uint256 storedFee, uint256 storedDatacap) =
            autocap.getRound(roundId);
        assertEq(storedStartTime, startTime);
        assertEq(storedEndTime, endTime);
        assertEq(storedFee, REGISTRATION_FEE);
        assertEq(storedDatacap, TOTAL_DATACAP);
    }

    function test_CreateRound_OnlyOwner() public {
        uint256 startTime = block.timestamp + 1 days;
        uint256 endTime = block.timestamp + 7 days;

        vm.prank(user1);
        vm.expectRevert();
        autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
    }

    function test_CreateRound_InvalidTimeRange() public {
        uint256 startTime = block.timestamp + 7 days;
        uint256 endTime = block.timestamp + 1 days; // endTime <= startTime

        vm.prank(owner);
        vm.expectRevert(AutoCap.InvalidTimeRange.selector);
        autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
    }

    function test_CreateRound_EqualTimes() public {
        uint256 time = block.timestamp + 1 days;

        vm.prank(owner);
        vm.expectRevert(AutoCap.InvalidTimeRange.selector);
        autocap.createRound(time, time, REGISTRATION_FEE, TOTAL_DATACAP);
    }

    function test_CreateMultipleRounds() public {
        uint256 startTime1 = block.timestamp + 1 days;
        uint256 endTime1 = block.timestamp + 7 days;

        uint256 startTime2 = block.timestamp + 10 days;
        uint256 endTime2 = block.timestamp + 14 days;

        vm.startPrank(owner);
        uint256 roundId1 = autocap.createRound(startTime1, endTime1, REGISTRATION_FEE, TOTAL_DATACAP);
        uint256 roundId2 = autocap.createRound(startTime2, endTime2, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        assertEq(roundId1, 1);
        assertEq(roundId2, 2);
        assertEq(autocap.currentRoundId(), 2);
    }

    // ============ Registration Tests ============

    function test_Register() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        vm.expectEmit(true, true, false, true);
        emit Registered(roundId, user1, ACTOR_ID_1);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_1);

        assertEq(autocap.getParticipantDetails(roundId, user1), ACTOR_ID_1);
        assertEq(autocap.getTotalRegistrants(roundId), 1);
        assertTrue(autocap.isRoundOpen(roundId));
    }

    function test_Register_MultipleUsers() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_1);

        vm.prank(user2);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_2);

        vm.prank(user3);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_3);

        assertEq(autocap.getTotalRegistrants(roundId), 3);
        assertEq(autocap.getParticipantDetails(roundId, user1), ACTOR_ID_1);
        assertEq(autocap.getParticipantDetails(roundId, user2), ACTOR_ID_2);
        assertEq(autocap.getParticipantDetails(roundId, user3), ACTOR_ID_3);
    }

    function test_Register_RoundNotOpen_BeforeStart() public {
        uint256 startTime = block.timestamp + 1 days;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        vm.expectRevert(AutoCap.RoundNotOpen.selector);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_1);
    }

    function test_Register_RoundNotOpen_AfterEnd() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 1 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.warp(endTime + 1);

        vm.prank(user1);
        vm.expectRevert(AutoCap.RoundNotOpen.selector);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_1);
    }

    function test_Register_InvalidFee() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        vm.expectRevert(
            abi.encodeWithSelector(AutoCap.InvalidRegistrationFee.selector, REGISTRATION_FEE, REGISTRATION_FEE / 2)
        );
        autocap.register{value: REGISTRATION_FEE / 2}(roundId, ACTOR_ID_1);
    }

    function test_Register_AlreadyRegistered() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.startPrank(user1);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_1);
        vm.expectRevert(AutoCap.AlreadyRegistered.selector);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_2);
        vm.stopPrank();
    }

    function test_Register_InvalidActorId() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        vm.expectRevert(AutoCap.InvalidActorId.selector);
        autocap.register{value: REGISTRATION_FEE}(roundId, 0);
    }

    function test_Register_DifferentRounds() public {
        uint256 startTime1 = block.timestamp;
        uint256 endTime1 = block.timestamp + 7 days;

        uint256 startTime2 = block.timestamp + 10 days;
        uint256 endTime2 = block.timestamp + 14 days;

        vm.startPrank(owner);
        uint256 roundId1 = autocap.createRound(startTime1, endTime1, REGISTRATION_FEE, TOTAL_DATACAP);
        uint256 roundId2 = autocap.createRound(startTime2, endTime2, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        autocap.register{value: REGISTRATION_FEE}(roundId1, ACTOR_ID_1);

        vm.warp(startTime2);
        vm.prank(user1);
        autocap.register{value: REGISTRATION_FEE}(roundId2, ACTOR_ID_2);

        assertEq(autocap.getParticipantDetails(roundId1, user1), ACTOR_ID_1);
        assertEq(autocap.getParticipantDetails(roundId2, user1), ACTOR_ID_2);
        assertEq(autocap.getTotalRegistrants(roundId1), 1);
        assertEq(autocap.getTotalRegistrants(roundId2), 1);
    }

    // ============ Close Round Tests ============

    function test_CloseRound() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_1);

        assertTrue(autocap.isRoundOpen(roundId));

        vm.prank(owner);
        vm.expectEmit(true, false, false, false);
        emit RoundClosed(roundId);
        autocap.closeRound(roundId);

        // After closing, endTime is set to block.timestamp, so round is still open at that exact moment
        // But advancing time by 1 second should make it closed
        vm.warp(block.timestamp + 1);
        assertFalse(autocap.isRoundOpen(roundId));

        // Should not be able to register after closing
        vm.prank(user2);
        vm.expectRevert(AutoCap.RoundNotOpen.selector);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_2);
    }

    function test_CloseRound_OnlyOwner() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        vm.expectRevert();
        autocap.closeRound(roundId);
    }

    function test_CloseRound_AlreadyClosed() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.warp(endTime + 1);

        vm.prank(owner);
        vm.expectRevert(AutoCap.RoundNotOpen.selector);
        autocap.closeRound(roundId);
    }

    // ============ View Function Tests ============

    function test_GetParticipants_Paginated() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_1);

        vm.prank(user2);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_2);

        vm.prank(user3);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_3);

        // Get first 2 participants
        (address[] memory participants, uint256 nextCursor) = autocap.getParticipants(roundId, 0, 2);
        assertEq(participants.length, 2);
        assertEq(participants[0], user1);
        assertEq(participants[1], user2);
        assertEq(nextCursor, 2);

        // Get remaining participants
        (participants, nextCursor) = autocap.getParticipants(roundId, 2, 2);
        assertEq(participants.length, 1);
        assertEq(participants[0], user3);
        assertEq(nextCursor, 0); // End reached
    }

    function test_GetParticipants_EmptyRound() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        (address[] memory participants, uint256 nextCursor) = autocap.getParticipants(roundId, 0, 10);
        assertEq(participants.length, 0);
        assertEq(nextCursor, 0);
    }

    function test_GetParticipants_InvalidCursor() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_1);

        // Cursor beyond length
        (address[] memory participants, uint256 nextCursor) = autocap.getParticipants(roundId, 10, 10);
        assertEq(participants.length, 0);
        assertEq(nextCursor, 0);
    }

    function test_GetParticipantDetails_NotRegistered() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        assertEq(autocap.getParticipantDetails(roundId, user1), 0);
    }

    function test_GetTotalRegistrants() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        assertEq(autocap.getTotalRegistrants(roundId), 0);

        vm.prank(user1);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_1);
        assertEq(autocap.getTotalRegistrants(roundId), 1);

        vm.prank(user2);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_2);
        assertEq(autocap.getTotalRegistrants(roundId), 2);
    }

    function test_IsRoundOpen() public {
        uint256 startTime = block.timestamp + 1 days;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        assertFalse(autocap.isRoundOpen(roundId)); // Before start

        vm.warp(startTime);
        assertTrue(autocap.isRoundOpen(roundId)); // At start

        vm.warp(endTime);
        assertTrue(autocap.isRoundOpen(roundId)); // At end

        vm.warp(endTime + 1);
        assertFalse(autocap.isRoundOpen(roundId)); // After end
    }

    // ============ Administrative Tests ============

    function test_WithdrawFees() public {
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_1);

        vm.prank(user2);
        autocap.register{value: REGISTRATION_FEE}(roundId, ACTOR_ID_2);

        uint256 balanceBefore = owner.balance;

        vm.prank(owner);
        vm.expectEmit(true, false, false, true);
        emit FeesWithdrawn(owner, REGISTRATION_FEE * 2);
        autocap.withdrawFees();

        assertEq(owner.balance, balanceBefore + REGISTRATION_FEE * 2);
        assertEq(address(autocap).balance, 0);
    }

    function test_WithdrawFees_OnlyOwner() public {
        vm.prank(user1);
        vm.expectRevert();
        autocap.withdrawFees();
    }

    function test_WithdrawFees_NoFees() public {
        vm.prank(owner);
        vm.expectRevert(AutoCap.NoFeesToWithdraw.selector);
        autocap.withdrawFees();
    }

    function test_UpdatePaymentContract() public {
        address newPaymentContract = address(0x200);
        vm.etch(newPaymentContract, hex"00");

        vm.prank(owner);
        vm.expectEmit(true, true, false, false);
        emit PaymentContractUpdated(paymentContract, newPaymentContract);
        autocap.updatePaymentContract(newPaymentContract);

        assertEq(autocap.paymentContract(), newPaymentContract);
    }

    function test_UpdatePaymentContract_ZeroAddress() public {
        vm.prank(owner);
        vm.expectRevert(AutoCap.ZeroAddress.selector);
        autocap.updatePaymentContract(address(0));
    }

    function test_UpdatePaymentContract_NotAContract() public {
        vm.prank(owner);
        vm.expectRevert(AutoCap.NotAContract.selector);
        autocap.updatePaymentContract(address(0xdead));
    }

    function test_UpdatePaymentContract_OnlyOwner() public {
        address newPaymentContract = address(0x200);

        vm.prank(user1);
        vm.expectRevert();
        autocap.updatePaymentContract(newPaymentContract);
    }

    // ============ Edge Cases ============

    function test_Fuzz_CreateRound(uint256 startTime, uint256 endTime, uint256 fee, uint256 datacap) public {
        // Bound inputs to reasonable ranges
        startTime = bound(startTime, block.timestamp, block.timestamp + 365 days);
        endTime = bound(endTime, startTime + 1, startTime + 365 days);
        fee = bound(fee, 0, 1000 ether);
        datacap = bound(datacap, 0, type(uint256).max);

        vm.prank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, fee, datacap);

        (uint256 storedStartTime, uint256 storedEndTime, uint256 storedFee, uint256 storedDatacap) =
            autocap.getRound(roundId);
        assertEq(storedStartTime, startTime);
        assertEq(storedEndTime, endTime);
        assertEq(storedFee, fee);
        assertEq(storedDatacap, datacap);
    }

    function test_Fuzz_Register(uint64 actorId) public {
        vm.assume(actorId > 0); // Actor ID must be non-zero

        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;

        vm.startPrank(owner);
        uint256 roundId = autocap.createRound(startTime, endTime, REGISTRATION_FEE, TOTAL_DATACAP);
        vm.stopPrank();

        vm.prank(user1);
        autocap.register{value: REGISTRATION_FEE}(roundId, actorId);

        assertEq(autocap.getParticipantDetails(roundId, user1), actorId);
    }
}
