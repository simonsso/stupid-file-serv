import os
from flask import Flask
from flask import request

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
        return {
            'files': list (globalfilesystem.keys())
        }

    @app.route('/files/<filename>', methods=['DELETE'] )
    def deletefile(filename):
        if filename in globalfilesystem.keys():
            del globalfilesystem[filename]
            return "File server:" + filename + "\r\n"
        else:
            return "Error No such file\r\n",410

    @app.route('/files/<filename>', methods=['POST','PUT'] )
    def createfile(filename):
        # PUT allows for data to be put mutliple times, POST will check and fail
        if request.method == 'POST' and filename in globalfilesystem.keys():
            return "Error file already exists",409
        #Since there is no way of geting the data there is no need to store it
        globalfilesystem[filename] = "dummydata"
        return "File server created:" + filename + "\r\n",201


    # Hack to serve the UI from same orgin
    @app.route('/index.html', methods=['GET'])
    def root_index():
        f = open ("client/index.html")
        return f.read()

    @app.route('/button.js', methods=['GET'])
    def serve_button():
        f = open ("client/button.js")
        return f.read()
    return app