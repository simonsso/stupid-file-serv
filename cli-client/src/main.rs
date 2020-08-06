use std::fmt;
extern crate simple_error;
use simple_error::SimpleError;
#[macro_use]
extern crate clap;
extern crate serde;

use serde::Deserialize;

#[derive(Debug)]

/// Error handling
/// Map errors to custom error type
///
/// # 1 The error type is an enum with an bracket implantation for std::error::Error
/// # 2 The error typ must implement fmt::Display
/// # 3 Create from conversion methods on all errors which could be encountered and a
///     which customized error it should map to.
enum CustomError {
    HTTPError,
    IOError,
    GenericError(String),
}
impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::HTTPError => write!(f, "HTTP Error"),
            CustomError::IOError => write!(f, "IO Error"),
            CustomError::GenericError(s) => write!(f, "Wrap error: {}", s),
        }
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(_: reqwest::Error) -> Self {
        CustomError::HTTPError
    }
}

impl From<std::io::Error> for CustomError {
    fn from(_: std::io::Error) -> Self {
        CustomError::IOError
    }
}
impl From<simple_error::SimpleError> for CustomError {
    fn from(e: simple_error::SimpleError) -> Self {
        CustomError::GenericError(e.as_str().to_string())
    }
}

// Helper function
fn usage_error(s: &str) -> Result<(), SimpleError> {
    Err(SimpleError::new(s))
}

fn delete_remote_file_command(remote_server: &str, filename: &str) -> Result<(), CustomError> {
    let url = remote_server.to_string() + "files/" + &filename;

    let client = reqwest::blocking::Client::new();
    client.delete(&url).send()?.error_for_status()?;

    Ok(())
}
fn push_remote_file_command(
    remote_server: &str,
    filename: &str,
    remotefilename: &str,
) -> Result<(), CustomError> {
    let url = remote_server.to_string() + "files/" + &remotefilename;
    let client = reqwest::blocking::Client::new();
    client
        .post(&url)
        .body(std::fs::read_to_string(filename)?)
        .send()?
        .error_for_status()?;
    Ok(())
}

fn list_files_command(remote_server: &str) -> Result<(), CustomError> {
    #[derive(Deserialize)]
    struct Data {
        files: Vec<String>,
    }

    let url = remote_server.to_string() + "files";
    let client = reqwest::blocking::Client::new();
    let resp = client.get(&url).send()?.json::<Data>()?;

    for file in resp.files {
        println!("{}", file);
    }
    Ok(())
}

fn main() -> Result<(), CustomError> {
    let matches = clap_app!(cli_client =>
        (version: "1.0")
        (@setting SubcommandRequiredElseHelp)
        (author: "Author: Fredrik SIMONSSON")
        (about: "Command line tool for simple file server project")
        (@arg SERVER: -s --server +takes_value "Remote server base url - default http://127.1:8000/")
        (@arg verbose: -v --verbose "Verbosely report final result")
        (@subcommand list =>
            (about: "list files on server")
        )
        (@subcommand delete =>
            (about: "remove a file on server")
            (@arg REMOTEFILE: +required "Remote file to remove")
        )
        (@subcommand upload =>
            (about: "upload a file on to the server")
            (@arg FILENAME: +required "Local filename")
            (@arg REMOTEFILE:  "Remote filename (optional remote filename, default is the local name )")
        )
    ).get_matches();

    let server = matches
        .value_of("SERVER")
        .unwrap_or("http://127.0.0.1:8000/");
    let verbose = match matches.value_of("verbose") {
        Some(_) => true,
        None => false,
    };
    if server.chars().last() != Some('/') {
        usage_error("Expecting last char in url to be a /")?;
    }

    match matches.subcommand() {
        ("delete", Some(args)) => {
            if let Some(remotefilename) = args.value_of("REMOTEFILE") {
                delete_remote_file_command(server, remotefilename)?;
            }
        }
        ("list", _args) => {
            list_files_command(server)?;
        }
        ("upload", Some(args)) => {
            if let Some(localfilename) = args.value_of("FILENAME") {
                let remotefilename = match args.value_of("REMOTEFILE") {
                    Some(s) => s,
                    None => localfilename,
                };
                push_remote_file_command(server, localfilename, remotefilename)?;
            }
        }
        _ => {
            usage_error("Unexpected error")?;
        }
    }

    if verbose {
        println!("OK");
    }
    Ok(())
}
