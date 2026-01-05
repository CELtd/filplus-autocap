// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console} from "forge-std/Script.sol";
import {AutoCap} from "../src/AutoCap.sol";

/// @title DeployAutoCap
/// @notice Deployment script for AutoCap contract
contract DeployAutoCap is Script {
    function run() external {
        // Get deployment parameters from environment or use defaults
        address paymentContract = vm.envOr("PAYMENT_CONTRACT", address(0x1234567890123456789012345678901234567890));

        console.log("Deploying AutoCap...");
        console.log("Payment Contract:", paymentContract);

        vm.startBroadcast();

        AutoCap autocap = new AutoCap(paymentContract);

        vm.stopBroadcast();

        console.log("AutoCap deployed at:", address(autocap));
        console.log("Owner:", autocap.owner());
        console.log("Payment Contract:", autocap.paymentContract());
    }
}

/// @title DeployAutoCapLocal
/// @notice Local deployment script for testing
contract DeployAutoCapLocal is Script {
    function run() external {
        // Use a mock payment contract address for local testing
        address paymentContract = address(0x1111111111111111111111111111111111111111);

        console.log("Deploying AutoCap locally...");
        console.log("Payment Contract:", paymentContract);

        vm.startBroadcast();

        AutoCap autocap = new AutoCap(paymentContract);

        vm.stopBroadcast();

        console.log("AutoCap deployed at:", address(autocap));
        console.log("Owner:", autocap.owner());
        console.log("Payment Contract:", autocap.paymentContract());

        // Example: Create a test round
        uint256 startTime = block.timestamp;
        uint256 endTime = block.timestamp + 7 days;
        uint256 registrationFee = 0.1 ether;
        uint256 totalDatacap = 1000 * 1024 * 1024 * 1024 * 1024; // 1000 TiB

        console.log("\nCreating test round...");
        uint256 roundId = autocap.createRound(startTime, endTime, registrationFee, totalDatacap);
        console.log("Round created with ID:", roundId);
    }
}
