extern crate ws;

use rocket_contrib::serve::StaticFiles;
use rand::Rng;
use std::{fs, thread, env};
use ws::{listen, Message, Handler, Sender};
use serde::{Deserialize, Serialize};
use serde_json;


struct Server {
    ws: Sender,
    file_path: String,
    max_plays: u8
}

#[derive(Serialize, Deserialize)]
struct ClientPayload {
    max_plays: u8,
    file_path: String
}

impl Handler for Server {
    fn on_message(&mut self, _msg: Message) -> ws::Result<()> {
        let item = ClientPayload {
            max_plays: self.max_plays,
            file_path: get_file(self.file_path.to_owned())
        };
        self.ws.send(serde_json::to_string(&item).unwrap()).unwrap();
        Ok(())
    }
}



fn generate_random_number(count: usize) -> usize {
    rand::thread_rng().gen_range(0, count)
}

fn get_file(file_path: String) -> String {
    let paths = fs::read_dir(file_path).unwrap();
    let vec: Vec<_> = paths.map(|path| path.unwrap().path()).collect();
    let random_path = vec.get(generate_random_number(vec.len()));
    random_path.unwrap().display().to_string()
}

fn init_rocket() {
    rocket::ignite()
    .mount("/", StaticFiles::from("public"))
    .launch();
}

fn init_websockets(file_path: String, max_plays: u8) {
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
        init_websockets(file_path, max_plays.parse::<u8>().unwrap());
    }));

    for thread in threads {
        let _ = thread.join();
    }

}