from bot import Bot


class MasterBot(Bot):
    def __init__(self, address: str):
        super().__init__(address)

    def start_auction(self):
        print("Auction started")

    def stop_auction(self):
        print("Auction ended")
