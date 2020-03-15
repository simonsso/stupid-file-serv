#!/bin/bash

export FLASK_APP=./flaskfs 
export FLASK_ENV=development
export FLASK_RUN_HOST='::'
export FLASK_RUN_PORT=8000

python3 -m flask run