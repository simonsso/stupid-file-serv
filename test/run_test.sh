#!/bin/bash

echo Simple test script which will delete all files on server and then upload list and delete files
echo start server with run_server.sh if it is not already running

(cd ../cli-client ; cargo build --release )

# clear all files
 ../cli-client/target/release/cli-client list | xargs -r -n1 ../cli-client/target/release/cli-client delete
echo Next line should be 0
 ../cli-client/target/release/cli-client list |wc -l
../cli-client/target/release/cli-client upload $0 file1
../cli-client/target/release/cli-client upload $0 file2
../cli-client/target/release/cli-client upload $0 file3
../cli-client/target/release/cli-client upload $0 file4
echo Next line should be 4
 ../cli-client/target/release/cli-client list |wc -l
if [[ "file1_file2_file3_file4_" == `../cli-client/target/release/cli-client list | tr "\n" "_"` ]];
then
    echo OK
else
    echo NOK
fi
# remove two files
../cli-client/target/release/cli-client delete file1
../cli-client/target/release/cli-client delete file2
# and the same test shoudl fail
if [[ "file1_file2_file3_file4_" == `../cli-client/target/release/cli-client  list | tr "\n" "_"` ]];
then
    echo NOK
else
    echo OK
fi
if [[ "file3_file4_" == `../cli-client/target/release/cli-client list | tr "\n" "_"` ]];
then
    echo OK
else
    echo NOK
fi

echo Next lines should be file3 and file4
../cli-client/target/release/cli-client list
echo Next 3 lines should fail
../cli-client/target/release/cli-client delete file2
../cli-client/target/release/cli-client upload $0 file3
../cli-client/target/release/cli-client upload /bin/sh
