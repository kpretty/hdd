import logging
import os
from logging.config import fileConfig
from pathlib import Path

fileConfig(Path(Path(__file__).parent, '../logging.ini'))
logger = logging.getLogger("hdd")
logger.setLevel(os.environ.get("LOG_LEVEL", logger.level))