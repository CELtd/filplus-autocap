import logging
import os

def get_logger(name="MasterBotLogger"):
    """Returns a logger with a specific name, configured for the MasterBot."""
    logger = logging.getLogger(name)
    logger.setLevel(logging.INFO)

    # Clear existing handlers before adding new ones to avoid duplicates
    for handler in logger.handlers[:]:
        handler.close()
        logger.removeHandler(handler)

    # Remove the log file if it exists, to clear the file each time
    log_file = "data/masterbot.log"
    if os.path.exists(log_file):
        os.remove(log_file)

    # Add a new handler to log to file
    file_handler = logging.FileHandler(log_file)
    file_handler.setFormatter(logging.Formatter("%(asctime)s - %(message)s"))
    logger.addHandler(file_handler)
    
    return logger
