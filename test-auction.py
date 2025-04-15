import asyncio
from filplus_autocap.utils.setup import initialize
from filplus_autocap.utils.constants import GAS_PRICE
from filplus_autocap.blockchain_utils.transaction import Tx
from filplus_autocap.blockchain_utils.storage_provider import initialize_sp


async def test_async_auction_with_delayed_sp():
    # --- Setup environment ---
    env = initialize()
    wallets = env.wallets
    processor = env.processor
    revenue_bot = env.revenue_bot
    master_bot = env.master_bot
    verified_list = env.verified_list

    # --- Create a time vector and auction task ---
    time_vector = list(range(0, 11, 1))  # [0, 1, 2, ..., 10]
    auction_task = asyncio.create_task(master_bot.run_auction(time_vector))

    # --- Simulate delay (t = 5) before SP registers and sends revenue ---
    await asyncio.sleep(5)

    # Step 1: Initialize and register SP
    sp_1 = initialize_sp(
        address="f1sp001",
        owner="sp001",
        fil_balance=150.0,
        wallets=wallets,
        processor=processor
    )

    # Enhanced output for event detection
    print("\n[time: 5 epochs]️ 🛰️ EVENT DETECTED: New SP registration & revenue contribution")
    registration_tx = Tx(
        sender=sp_1.address,
        recipient=verified_list.address,
        fil_amount=0.0,
        datacap_amount=0.0,
        signers=[sp_1.address],
        message="Registering"
    )
    print("    └─ Registration TX:", registration_tx)
    processor.send([registration_tx])

    # Step 2: SP sends revenue to RevenueBot
    incoming = Tx(
        sender=sp_1.address,
        recipient=revenue_bot.address,
        fil_amount=100.0,
        signers=[sp_1.address]
    )
    print("    └─ Revenue TX to RevenueBot:", incoming)
    processor.send([incoming])

    # Process the incoming transaction
    resulting_txs = revenue_bot.process_incoming_tx(incoming)
    for tx in resulting_txs:
        print("    └─ RevenueBot Processed →", tx)
        processor.send([tx])
    print(" ")

    # --- Wait for auction loop to complete ---
    await auction_task



if __name__ == "__main__":
    asyncio.run(test_async_auction_with_delayed_sp())
