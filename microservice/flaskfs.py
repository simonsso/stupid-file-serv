from flask import Flask
from flask import request

globalfilesystem = {}
def create_app(test_config=None):
    # create and configure the app
    app = Flask(__name__, instance_relative_config=True)
   
    # Persistency was not part of the requirements
    # store all files in this global and forget all when restarted.
    globalfilesystem = {}

    # uncomment to create some files present on restart.
    # globalfilesystem["index.html"] = "<body>Nothing <b>to see</b></body>"
    # globalfilesystem["startup.sys"] = "Nothing here"


    @app.route('/files', methods=['GET'])
    def hello():
        files = list (globalfilesystem.keys())
        files.sort()
        return {
            'files': files
        }

    @app.route('/files/<filename>', methods=['GET'] )
    def filecontents(filename):
        if filename in globalfilesystem.keys():
            return globalfilesystem[filename]
        else:
            return "Error No such file\r\n",410

    @app.route('/files/<filename>', methods=['DELETE'] )
    def deletefile(filename):
        if filename in globalfilesystem.keys():
            del globalfilesystem[filename]
            return "File server removed: " + filename + "\r\n"
        else:
            return "Error No such file "+filename+"\r\n",410

    @app.route('/files/<filename>', methods=['POST','PUT'] )
    def createfile(filename):
        # PUT allows for data to be put multiple times, POST will check and fail
        if request.method == 'POST' and filename in globalfilesystem.keys():
            return "Error file already exists\r\n",409
        # Data supplied with POST as text/plain is found in data
        # needs to be changed if other mimetypes are used for upload.
        globalfilesystem[filename] = request.data
        return "File server created:" + filename + "\r\n",201


    # Hack to serve the UI from same origin
    @app.route('/',methods=['GET'])
    @app.route('/index.html', methods=['GET'])
    def root_index():
        f = open ("client/index.html")
        return f.read()

    return app