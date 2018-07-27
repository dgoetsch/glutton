extern crate rand;

use rand::prelude::*;
use std::thread;
use std::io::Write;
use std::iter::Map;

struct Glutton {
    dir: String,
    at_a_time: usize
}

impl Glutton {
    fn consume(&self) {

        let mut handlers: Vec<std::thread::JoinHandle<()>> = vec![];

        for idx in  (0..self.at_a_time) {
            handlers.push(self.just_one_of_them())
        }

        for handler in handlers {
            handler.join();
        }



    }


    fn write_dir(&self) -> String {
        let name = thread_rng().gen_ascii_chars().take(10) .collect::<String>();
        let mut sub_dir = String::from("");
        sub_dir.push_str(self.dir.as_str());
        sub_dir.push_str("/");
        sub_dir.push_str(name.as_str());

        let cd_res = std::fs::create_dir_all(sub_dir.as_str());

        let mut file_name = String::new();
        file_name.push_str(self.dir.as_str());
        file_name.push_str("/");
        file_name.push_str(name.as_str());
        file_name.push_str(".dat");

        let cf_res = std::fs::File::create(file_name.as_str())
            .and_then(|mut new_file|{ new_file.write(name.as_bytes()) });

        println!("done: {}", sub_dir);
        name
    }

    fn just_one_of_them(&self) -> std::thread::JoinHandle<()> {

        let dir = self.write_dir();
        let mut new_path = String::new();
        new_path.push_str(self.dir.as_str());
        new_path.push_str("/");
        new_path.push_str(dir.as_str());
        let at_a_time = self.at_a_time;
        let handler = thread::spawn(move ||{
            Glutton { dir: new_path, at_a_time: at_a_time}.consume()
        });

        return handler
    }
}

fn main() {
    Glutton{dir: String::from("."), at_a_time: 10}.consume();
}


