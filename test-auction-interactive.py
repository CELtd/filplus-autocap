import asyncio
import aioconsole
from filplus_autocap.utils.setup import initialize
from filplus_autocap.utils.constants import GAS_PRICE
from filplus_autocap.blockchain_utils.transaction import Tx
from filplus_autocap.blockchain_utils.storage_provider import initialize_sp
from filplus_autocap.utils.logger import get_logger


logger = get_logger("MasterBotLogger")

async def run_auction_in_background(master_bot, time_vector):
    """Run the auction in the background."""
    try:
        await master_bot.run_auction(time_vector)
    except asyncio.CancelledError:
        logger.info("[🛑] Auction loop was cancelled.")
        return  # Let the cancellation propagate cleanly

async def listen_for_commands(env, processor, verified_list, revenue_bot, auction_task):
    """Listen for user input and handle commands for SP registration and revenue declaration."""
    while True:
        command = await aioconsole.ainput("Enter command (register, declare, exit): ")

        if command == "register":
            sp_address = await aioconsole.ainput("Enter SP address: ")
            sp_owner = await aioconsole.ainput("Enter SP owner: ")
            fil_balance = float(await aioconsole.ainput("Enter SP FIL balance: "))

            sp = initialize_sp(
                address=sp_address,
                owner=sp_owner,
                fil_balance=fil_balance,
                wallets=env.wallets,
                processor=processor
            )

            registration_tx = Tx(
                sender=sp.address,
                recipient=verified_list.address,
                fil_amount=0.0,
                datacap_amount=0.0,
                signers=[sp.address],
                message="Registering SP"
            )
            logger.info(f"    └─ Registration TX: {registration_tx}")
            processor.send([registration_tx])

        elif command == "declare":
            sp_address = await aioconsole.ainput("Enter SP address: ")
            amount = float(await aioconsole.ainput("Enter revenue amount (FIL): "))

            revenue_tx = Tx(
                sender=sp_address,
                recipient=revenue_bot.address,
                fil_amount=amount,
                signers=[sp_address],
                message="Revenue declaration"
            )
            logger.info(f"    └─ Revenue TX: {revenue_tx}")
            processor.send([revenue_tx])

            resulting_txs = revenue_bot.process_incoming_tx(revenue_tx)
            for tx in resulting_txs:
                logger.info(f"    └─ RevenueBot Processed → {tx}")
                processor.send([tx])

        elif command == "exit":
            logger.info("Exiting...")
            auction_task.cancel()  # 🔥 Cancel the background auction loop
            break

        else:
            logger.warning(f"Unknown command: '{command}'")

async def main():
    # --- Initialize environment ---
    env = initialize()
    wallets = env.wallets
    processor = env.processor
    revenue_bot = env.revenue_bot
    master_bot = env.master_bot
    verified_list = env.verified_list

    # --- Create a time vector for auction simulation ---
    time_vector = list(range(0, 101, 1))  # [0, 1, ..., 10]

    # Launch the background auction loop
    auction_task = asyncio.create_task(run_auction_in_background(master_bot, time_vector))

    # --- Start command listener concurrently with the auction ---
    await asyncio.gather(
        listen_for_commands(env, processor, verified_list, revenue_bot, auction_task),
        auction_task
    )

if __name__ == "__main__":
    asyncio.run(main())
