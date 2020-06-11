extern crate simple_error;
use simple_error::SimpleError;
#[macro_use]
extern crate clap;
extern crate serde;

use serde::Deserialize;

fn usage_error(s: &str) -> Result<(), SimpleError> {
    Err(SimpleError::new(s))
}


fn delete_remote_file_command(remote_server: &str, filename: &str) -> Result<(), Box<dyn std::error::Error> > {
    let url = remote_server.to_string() + "files/" + &filename;

    let client = reqwest::blocking::Client::new();
    client.delete(&url).send()?.error_for_status()?;

    Ok(())
}
fn push_remote_file_command(
    remote_server: &str,
    filename: &str,
    remotefilename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = remote_server.to_string() + "files/" + &remotefilename;
    let client = reqwest::blocking::Client::new();
    client
        .post(&url)
        .body(std::fs::read_to_string(filename)?)
        .send()?
        .error_for_status()?;
    Ok(())
}

fn list_files_command(remote_server: &str) -> Result<(), Box<dyn std::error::Error>> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        usage_error("Expting last char in url to be a /")?;
    }

    match matches.subcommand() {
        ("delete", Some(args)) => {
            delete_remote_file_command(server, args.value_of("REMOTEFILE").unwrap())?;
        }
        ("list", _args) => {
            list_files_command(server)?;
        }
        ("upload", Some(args)) => {
            let localfilename = args.value_of("FILENAME").unwrap();
            let remotefilename = match args.value_of("REMOTEFILE") {
                Some(s) => s,
                None => localfilename,
            };
            push_remote_file_command(server, localfilename, remotefilename)?;
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
