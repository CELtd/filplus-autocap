import logging
import os

def get_logger(name="MasterBotLogger"):
    """
    Returns a logger configured for the MasterBot with a specified name.
    
    The logger is configured to log to a file (`data/masterbot.log`), with a default logging level of INFO.
    If the log file already exists, it will be removed to ensure a fresh log each time.

    Args:
        name (str): The name of the logger. Default is "MasterBotLogger".

    Returns:
        logging.Logger: A logger instance configured with a file handler.
    """
    # Create a logger with the specified name
    logger = logging.getLogger(name)
    logger.setLevel(logging.INFO)

    # Clear any existing handlers to prevent duplicate logs
    for handler in logger.handlers[:]:
        handler.close()  # Close the handler to free up resources
        logger.removeHandler(handler)

    # Define the path for the log file
    log_file = "data/masterbot.log"

    # Remove the log file if it exists to start with a fresh file each time
    if os.path.exists(log_file):
        os.remove(log_file)

    # Set up a new file handler for logging
    file_handler = logging.FileHandler(log_file)
    file_handler.setFormatter(logging.Formatter("%(asctime)s - %(message)s"))  # Format log entries

    # Add the file handler to the logger
    logger.addHandler(file_handler)
    
    return logger
