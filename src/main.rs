
use std::net::TcpListener;
use std::thread;

use http::api;
pub mod preprocess;
pub mod math;
pub mod http;

fn main() {
    println!("START");

    let listener = match TcpListener::bind("127.0.0.1:3000") {
        Ok(list) => {
            list
        }
        Err(e) => {
            println!("{}", e);
            return
        }
    };
    println!("Server Started");

    for stream in listener.incoming() {
        println!("New Connection");
        let stream = match stream {
            Ok(s) => {
                s
            }
            Err(e) => {
                println!("{}", e);
                return
            }
        };
    
        thread::spawn(|| {
            api::handle(stream);
        });
    }
}
