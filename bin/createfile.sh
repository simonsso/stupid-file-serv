#!/bin/bash
curl -s --data-binary @- -X POST http://127.0.0.1:8000/files/$1 -H "Content-Type: text/plain"
