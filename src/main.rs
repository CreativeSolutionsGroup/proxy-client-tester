use chrono::{self, Local};
use std::env;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let parsed_url = env::var("HEARTBEAT_URL").unwrap_or("localhost".to_string());

    println!("{parsed_url}");

    match args[1].as_str() {
        "-p" => {
            for _i in 0..10 {
                loop {
                    let context = zmq::Context::new();
                    let p = context.socket(zmq::REQ).unwrap();
                    p.connect(&format!("tcp://{}:9951", parsed_url.clone()))
                        .unwrap();
                    println!("sending");
                    p.send(format!("heartbeat 00:00:00:00:00:0{}", _i).as_bytes(), 0)
                        .unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            }
        }
        "-s" => {
            let context = zmq::Context::new();
            let sub = context.socket(zmq::REP).unwrap();
            sub.connect(&format!("tcp://{}:9950", parsed_url).clone())
                .unwrap();
            let mut msg = zmq::Message::new();
            loop {
                println!("waiting for message...");
                sub.recv(&mut msg, 0).unwrap();
                match msg.as_str() {
                    Some(m) => {
                        dbg!(msg.as_str().unwrap());
                        sub.send(format!("ACK {}", &m[10..]).as_bytes(), 0).unwrap();
                    }
                    None => {
                        dbg!("recieved None");
                        sub.send("ACK".as_bytes(), 0).unwrap();
                    }
                }
            }
        }
        "-l" => {
            let context = zmq::Context::new();
            let sub = context.socket(zmq::PAIR).unwrap();
            sub.connect(&format!("tcp://{}:9952", parsed_url.clone()))
                .unwrap();
            let mut msg = zmq::Message::new();
            println!("waiting to log messages...");
            loop {
                sub.recv(&mut msg, 0).unwrap();
                match msg.as_str() {
                    Some(m) => {
                        if !m.trim().is_empty() {
                            println!(
                                "{}: {}",
                                Local::now().format("%Y-%m-%d %H:%M:%S.%3f").to_string(),
                                m
                            );
                        }
                    }
                    None => (),
                }
            }
        }
        _ => unreachable!(),
    }
}
