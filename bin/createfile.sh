#!/bin/bash
cat| curl -X POST http://127.0.0.1:5000/files/$1
