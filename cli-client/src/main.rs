#[macro_use]
extern crate simple_error;
use simple_error::SimpleError;
#[macro_use]
extern crate  clap;

// use failure::ResultExt;
use exitfailure::ExitFailure;

// pub enum Error<E> {
//     UnexpectedError,
//     UserError,
//     OtherError(E)
// }
fn usage_error() -> Result<(), SimpleError>{
    Err(SimpleError::new("Usage error"))
}

fn delete_remote_file_command(remote_server:&str,filename:&str)-> Result<(), ExitFailure>
{
    // let url = remote_server.to_string()+"/files/"+&filename;
    // // let rest = reqwest::blocking:: 
    // // (&url)?;


    // let client = reqwest::Client::new();
    // let res = client.delete(&url)
    // .body("the exact body that is sent")
    // .send();
    // // res.w
    Ok(())

}
fn push_remote_file_command(remote_server:&str,filename:&str)-> Result<(), ExitFailure>
{
    let url = remote_server.to_string()+"/files/"+&filename;
    // let rest = reqwest::blocking:: 
    // (&url)?; 

    let x = async{
        println!("Try push url {}",url);
        let client = reqwest::Client::new();
        let res = client.post(&url)
        .body("the exact body that is sent")
        .send().await;
        println!("Awaited!");
        res
    };
    x.await();
    Ok(())
}

fn list_files_command(remote_server:&str)-> Result<(), ExitFailure>
{
    // let url = remote_server.to_string()+"files";
    // println!("{}",url);
    // let rest = reqwest::blocking::get(&url)?.text()?;
    // println!("{}",rest);
    Ok(())
}

fn main()-> Result<(), ExitFailure>  {
    let matches = clap_app!(cli_client =>
        (version: "1.0")
        (@setting SubcommandRequiredElseHelp)
        (author: "Author: Fredrik SIMONSSON")
        (about: "Command line tool for simple file server project")
        // (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg SERVER: -s --server +takes_value "Remote server base url - default http://127.1:5000/")
        // (@arg debug: -d ... "Sets the level of debugging information")
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
                println!("push");
                push_remote_file_command(server,args.value_of("FILENAME").unwrap())?;

            },
        _ => { usage_error()?; }
    }

    Ok(())
}
