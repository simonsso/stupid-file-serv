#!/bin/bash

export FLASK_APP=./microservice/flaskfs.py
export FLASK_ENV=development
export FLASK_RUN_HOST='::'
export FLASK_RUN_PORT=8000

python3 -m flask run
