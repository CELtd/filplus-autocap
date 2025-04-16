import asyncio
from filplus_autocap.utils.setup import initialize
from filplus_autocap.utils.constants import GAS_PRICE
from filplus_autocap.utils.console import listen_for_commands

async def main():
    # --- Initialize environment ---
    env = initialize()
    wallets = env.wallets
    processor = env.processor
    revenue_bot = env.revenue_bot
    master_bot = env.master_bot
    verified_list = env.verified_list

    # --- Create a time vector for auction simulation ---
    time_vector = list(range(0, 10001, 1))  # [0, 1, ..., 10]

    # Launch the background auction loop
    auction_task = asyncio.create_task(master_bot.run_auction_in_background(time_vector))

    # --- Start command listener concurrently with the auction ---
    await asyncio.gather(
        listen_for_commands(env, master_bot.logger, processor, verified_list, revenue_bot, auction_task),
        auction_task
    )

if __name__ == "__main__":
    asyncio.run(main())
