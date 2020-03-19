#!/bin/bash

for i in $*
do
    curl -s -X DELETE http://127.0.0.1:8000/files/$i
done
