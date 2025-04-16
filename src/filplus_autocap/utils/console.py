import asyncio
import aioconsole
from filplus_autocap.blockchain_utils.storage_provider import initialize_sp
from filplus_autocap.blockchain_utils.transaction import Tx
from filplus_autocap.blockchain_utils.currencies import FIL, DAT

async def listen_for_commands(env, logger, processor, verified_list, revenue_bot, auction_task):
    """
    Listens for user commands and handles Storage Provider registration and revenue declaration.

    This function runs an asynchronous loop that waits for user input, processes commands,
    and performs actions based on the user's choice such as registering a Storage Provider (SP)
    or declaring revenue to the RevenueBot. It interacts with various blockchain components
    such as the `VerifiedSPList` and `RevenueBot`.

    Args:
        env (Environment): The environment context, which includes wallet and other system state.
        logger (logging.Logger): The logger instance for logging events.
        processor (TxProcessor): The transaction processor responsible for handling transactions.
        verified_list (VerifiedSPList): The list of verified Storage Providers to register SPs.
        revenue_bot (RevenueBot): The bot that processes revenue declarations.
        auction_task (asyncio.Task): The task managing the ongoing auction process to be canceled when exiting.
    """
    while True:
        # Await user input for a command to process.
        command = (await aioconsole.ainput("Enter command (register, declare, exit): ")).strip()

        if command == "register":
            # Handle the SP registration command
            sp_address = await aioconsole.ainput("Enter SP address: ")
            sp_owner = await aioconsole.ainput("Enter SP owner: ")
            fil_balance = FIL(await aioconsole.ainput("Enter SP FIL balance: "))

            # Initialize the SP and add to the environment wallet
            sp = initialize_sp(
                address=sp_address,
                owner=sp_owner,
                fil_balance=fil_balance,
                wallets=env.wallets,
                processor=processor
            )

            # Create the transaction to register the SP with the VerifiedSPList
            registration_tx = Tx(
                sender=sp.address,
                recipient=verified_list.address,
                fil_amount=FIL(0.0),
                datacap_amount=DAT(0.0),
                signers=[sp.address],
                message=f"Registering SP with wallet {sp}"
            )
            # Log and send the transaction
            logger.info(f"       └─ 🛰️ EVENT DETECTED: SP registration TX: {registration_tx}")
            processor.send([registration_tx])

        elif command == "declare":
            # Handle the revenue declaration command
            sp_address = await aioconsole.ainput("Enter SP address: ")
            amount = FIL(await aioconsole.ainput("Enter revenue amount (FIL): "))

            # Create the revenue declaration transaction
            revenue_tx = Tx(
                sender=sp_address,
                recipient=revenue_bot.address,
                fil_amount=amount,
                signers=[sp_address],
                message="Revenue declaration"
            )
            # Log and send the revenue declaration transaction
            logger.info(f"       └─ 🛰️ EVENT DETECTED: Revenue declaration TX: {revenue_tx}")
            processor.send([revenue_tx])

            # Process the transaction with the RevenueBot and log the resulting transactions
            resulting_txs = revenue_bot.process_incoming_tx(revenue_tx)
            for tx in resulting_txs:
                logger.info(f"    └─ RevenueBot Processed → {tx}")
                processor.send([tx])

        elif command == "exit":
            # Exit the loop and cancel the auction task
            logger.info("Exiting...")
            auction_task.cancel()  # 🔥 Cancel the background auction loop
            break

        else:
            # Handle unrecognized commands
            logger.warning(f"Unknown command: '{command}'")
