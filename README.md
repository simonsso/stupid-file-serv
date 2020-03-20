# Simple file server

This project contains a simple volatile storage file server, a web client and a commandline client.
 
* The microservice is built on python and flask framework.
* The web client uses some small portion of react and,
* the command line is built in rust and based on `clap`,`reqwest`,`serde` and other standard crates from crate.io,
* as a bonus the curl scripts used while developing is also included.

The solution have been built and tested on Ubuntu, Windows 7 and Android, install varies from platform to platform

# Install commands on Ubuntu

    sudo apt-get install python3
    pip3 install flask
    pip3 install python-dotenv

run server with script run_server.sh  or theese lines:

    export FLASK_APP=./microservice/flaskfs.py
    export FLASK_ENV=development
    python3 -m flask run


Output will look something like this:

    $ python3 -m flask run
    * Serving Flask app "./flaskfs" (lazy loading)
    * Environment: development
    * Debug mode: on
    * Running on http://127.0.0.1:8000/ (Press CTRL+C to quit)
    * Restarting with stat
    * Debugger is active!
    * Debugger PIN: 196-948-859

In this case the webinterface is running on http://localhost:8000 and can be accessed with a browser.


# Command line tools

## Option1, designated client

Precompiled binaries for a few platforms exists in dir ./prebuilt/xx/cli-client

To build from source:

install a working rust toolchain - on Ubuntu stable works fine, but on Android and Windows nightly was needed.

    cd cli-client
    cargo build --release
    cp target/release/cli-client ../bin/

`run cli-client` --help to show the basic usage instructions below.
There are three subcommands for upload, delete and list files on server. List and upload require arguments and list prints to stdout.


    Command line tool for simple file server project

    USAGE:
        cli-client [FLAGS] [OPTIONS] <SUBCOMMAND>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
        -v, --verbose    Verbosely report final result

    OPTIONS:
        -s, --server <SERVER>    Remote server base url - default http://127.1:8000/

    SUBCOMMANDS:
        delete    remove a file on server
        help      Prints this message or the help of the given subcommand(s)
        list      list files on server
        upload    upload a file on to the server




## option 2, cURL based scripts
install curl with
    sudo apt-get install curl

The scripts are stored in ./bin/ dir

### createfile.sh
arg: filename  and read content from stdin
example:
echo Hello | ./createfile.sh  hello_world.txt

### deletefile.sh

removes a list of files from server
example:
./deletefile.sh myfile.txt myotherfile.html

### listfiles.sh

Prints file list in json format as received from server, output could be piped to any tool to process json for instance jq

example:
    listfiles.sh | jq .files | jq '.[]'

To remove all files simply run
    ./deletefile.sh ` ./listfiles.sh | jq .files | jq '.[]'| tr '["]' '[ ]' `

