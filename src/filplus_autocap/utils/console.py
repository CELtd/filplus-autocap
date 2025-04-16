import asyncio
import aioconsole
from filplus_autocap.blockchain_utils.storage_provider import initialize_sp
from filplus_autocap.blockchain_utils.transaction import Tx
from filplus_autocap.blockchain_utils.currencies import FIL, DAT

async def listen_for_commands(env, logger, processor, verified_list, revenue_bot, auction_task):
    """Listen for user input and handle commands for SP registration and revenue declaration."""
    while True:
        # Listen to commands
        command = (await aioconsole.ainput("Enter command (register, declare, exit): ")).strip()

        if command == "register":
            sp_address = await aioconsole.ainput("Enter SP address: ")
            sp_owner = await aioconsole.ainput("Enter SP owner: ")
            fil_balance = FIL(await aioconsole.ainput("Enter SP FIL balance: "))

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
                fil_amount=FIL(0.0),
                datacap_amount=DAT(0.0),
                signers=[sp.address],
                message=f"Registering SP with wallet {sp}"
            )
            logger.info(f"       └─ 🛰️ EVENT DETECTED: SP registration TX: {registration_tx}")
            processor.send([registration_tx])

        elif command == "declare":
            sp_address = await aioconsole.ainput("Enter SP address: ")
            amount = FIL(await aioconsole.ainput("Enter revenue amount (FIL): "))

            revenue_tx = Tx(
                sender=sp_address,
                recipient=revenue_bot.address,
                fil_amount=amount,
                signers=[sp_address],
                message="Revenue declaration"
            )
            logger.info(f"       └─ 🛰️ EVENT DETECTED: Revenue declaration TX: {revenue_tx}")
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
