#!/bin/bash

for i in $*
do
    curl -X DELETE http://127.0.0.1:5000/files/$i
done
