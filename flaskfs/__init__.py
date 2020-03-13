import os
from flask import Flask
import json

globalfilesystem = {}
def create_app(test_config=None):
    # create and configure the app
    app = Flask(__name__, instance_relative_config=True)
    app.config.from_mapping(
        SECRET_KEY='dev',
    )

    if test_config is None:
        # load the instance config, if it exists, when not testing
        app.config.from_pyfile('config.py', silent=True)
    else:
        # load the test config if passed in
        app.config.from_mapping(test_config)

    # ensure the instance folder exists
    try:
        os.makedirs(app.instance_path)
    except OSError:
        pass


    # Persistency was not part of the requirements
    # store all files in this global and forget all when retarted.
    globalfilesystem = {}

    #create some files present on restart.
    globalfilesystem["index.html"] = "<body>"


    @app.route('/files', methods=['GET'])
    def hello():
        response = {}
        response ['files']= list (globalfilesystem.keys())
        return json.JSONEncoder().encode(response)

    @app.route('/files/<filename>', methods=['DELETE'] )
    def deletefile(filename):
        del globalfilesystem[filename]
        return "File server:" + filename + "\r\n"

    @app.route('/files/<filename>', methods=['POST'] )
    def createfile(filename):
        #Since there is no way of geting the data there is no need to store it
        globalfilesystem[filename] = "dummydata"
        return "File server created:" + filename + "\r\n" 

    return app