extern crate ws;

use rocket_contrib::serve::StaticFiles;
use rand::Rng;
use std::{fs, thread, env};
use ws::{listen, Message, Handler, Sender};
use serde::{Deserialize, Serialize};
use serde_json;

struct Error {
    message: String
}

struct Server {
    ws: Sender,
    file_path: String,
    max_plays: usize
}

#[derive(Serialize, Deserialize)]
struct ClientPayload {
    max_plays: usize,
    file_path: String,
    error: Option<String>
}

type Result<T> = std::result::Result<T, Error>;

impl<T: std::error::Error> From<T> for Error {
    fn from(err: T) -> Self {
        Error { message: err.description().to_owned() }
    }
}

impl Handler for Server {
    fn on_message(&mut self, _msg: Message) -> Result<String> {
       match get_file(self.file_path.to_owned()) {
           Some(file) => {
            let item = ClientPayload {
                max_plays: self.max_plays,
                file_path: file,
                error: None
            };
            self.ws.send(serde_json::to_string(&item).unwrap()).unwrap();
            Ok("Ok".to_owned())
           },
           Err(error) => {
            println!("Error: {:?}", error.message);
            Ok("Error".to_owned())
           }
       }
        
    }
}

fn generate_random_number(count: usize) -> usize {
    rand::thread_rng().gen_range(0, count)
}

fn get_file(file_path: String) -> Result<String> {
    let paths = fs::read_dir(file_path).unwrap();
    let vec: Vec<_> = paths.map(|path| path.unwrap().path()).collect();
    let random_path = vec.get(generate_random_number(vec.len()));
    Ok(random_path.unwrap().display().to_string())
}

fn init_rocket() {
    rocket::ignite()
    .mount("/", StaticFiles::from("public"))
    .launch();
}

fn init_websockets(file_path: String, max_plays: usize) {
    listen("127.0.0.1:3012", |out| {
        Server {
            ws: out,
            file_path: file_path.to_owned(),
            max_plays,
        }
    }).unwrap();
}

fn main () {
    
    let args: Vec<String> = env::args().collect();
    let file_path = args[2].to_string();
    let max_plays = args[4].to_string();

    let mut threads = vec![];

    threads.push(thread::spawn(move || {
        init_rocket();
    }));

    threads.push(thread::spawn(move || {
        init_websockets(file_path, max_plays.parse::<usize>().unwrap());
    }));

    for thread in threads {
        let _ = thread.join();
    }

}