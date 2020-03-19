#[macro_use]
extern crate simple_error;
use simple_error::SimpleError;
#[macro_use]
extern crate  clap;

extern crate serde;
use exitfailure::ExitFailure;

use std::collections::HashMap;

#[macro_use]
use serde::{Deserialize,Serialize};
// #[derive(Deserialize)]
// struct ListResponse {
//     files: Vec<String>
// }

fn usage_error() -> Result<(), SimpleError>{
    Err(SimpleError::new("Usage error"))
}

fn delete_remote_file_command(remote_server:&str,filename:&str)-> Result<(), ExitFailure>
{
    let url = remote_server.to_string()+"/files/"+&filename;

    let client = reqwest::blocking::Client::new();
    let resp = client.delete(&url).send()?.error_for_status()?;

    Ok(())
}
fn push_remote_file_command(remote_server:&str,filename:&str,remotefilename:&str)-> Result<(), ExitFailure>
{
    let url = remote_server.to_string()+"/files/"+&filename;
    let client = reqwest::blocking::Client::new();
    let resp = client.post(&url)
        .body("the exact body that is sent")
        .send()?;

    resp.error_for_status()?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Data{
    files: Vec<String>
}

fn list_files_command(remote_server:&str)-> Result<(), ExitFailure>
{
    let url = remote_server.to_string()+"files";
    let client = reqwest::blocking::Client::new();
    // let resp = client.get(&url).send()?.json::<HashMap<String,Vec<String>>>()?;
    let resp = client.get(&url).send()?.json::<Data>()?;
    for file in resp.files {
        println!("{}",file);
    }
    // let files:Vec<String> = resp.entry("files") ;
    // for (key,val) in resp.iter(){
    //     println!("{}",key);
    // }
    Ok(())
}

fn main()-> Result<(), ExitFailure>  {
    let matches = clap_app!(cli_client =>
        (version: "1.0")
        (@setting SubcommandRequiredElseHelp)
        (author: "Author: Fredrik SIMONSSON")
        (about: "Command line tool for simple file server project")
        (@arg SERVER: -s --server +takes_value "Remote server base url - default http://127.1:5000/")
        (@subcommand list =>
            (about: "list files on server")
        )
        (@subcommand delete =>
            (about: "remove a file on server")
            (@arg REMOTENAME: +required "Remote file to remove")
            (@arg verbose: -v --verbose "Print test information verbosely")
        )
        (@subcommand upload =>
            (about: "upload a file on to the server")
            (@arg FILENAME: +required "Local filename")
            (@arg REMOTENAME:  "Remote filename")
            (@arg verbose: -v --verbose "Print test information verbosely")
        )
    ).get_matches();


    let server = matches.value_of("SERVER").unwrap_or("http://127.0.0.1:5000/");
    if server.chars().last() != Some('/'){
        println!("Expting last char in url to be a /");
        usage_error()?;
    }
    println!("{}",server);

    match matches.subcommand(){
        ("delete",Some(args)) => {
            delete_remote_file_command(server,args.value_of("REMOTENAME").unwrap())?;
        },
        ("list",_args) =>{ list_files_command(server)?;},
        ("upload",Some(args)) =>{ 
                let localfilename = args.value_of("FILENAME").unwrap();
                let remotefilename = match args.value_of("REMOTENAME") {
                    Some(s) => {s}
                    None => {localfilename}
                };
                push_remote_file_command(server,localfilename,remotefilename )?;

            },
        _ => { usage_error()?; }
    }

    Ok(())
}
