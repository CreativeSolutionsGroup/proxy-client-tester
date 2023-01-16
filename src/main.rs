use chrono::{self, Local};

fn main() {
    let args: Vec<String> = std::env::args().collect();


    match args[1].as_str() {
        "-p" => {
            let mut handles = Vec::new();
            for _i in 0..10 {
                let h = std::thread::spawn(move || loop {
                    let context = zmq::Context::new();
                    let p = context.socket(zmq::REQ).unwrap();
                    p.connect("tcp://localhost:9951").unwrap();
                    println!("sending");
                    p.send(format!("heartbeat 00:00:00:00:00:00").as_bytes(), 0)
                        .unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                });
                handles.push(h);
            }

            for h in handles {
                h.join().unwrap();
            }
        }
        "-s" => {
            let context = zmq::Context::new();
            let sub = context.socket(zmq::REP).unwrap();
            sub.connect("tcp://localhost:9950").unwrap();
            let mut msg = zmq::Message::new();
            loop {
                println!("waiting for message...");
                sub.recv(&mut msg, 0).unwrap();
                match msg.as_str() {
                    Some(m) => {
                        dbg!(msg.as_str().unwrap());
                        sub.send(format!("ACK {}", &m[10..]).as_bytes(), 0).unwrap();
                    },
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
            sub.connect("tcp://localhost:9952").unwrap();
            let mut msg = zmq::Message::new();
            println!("waiting to log messages...");
            loop {
                sub.recv(&mut msg, 0).unwrap();
                match msg.as_str() {
                    Some(m) => {
                        if !m.trim().is_empty() {
                            println!("{}: {}", Local::now().format("%Y-%m-%d %H:%M:%S.%3f").to_string(), m);
                        } 
                    },
                    None => (),
                }
            }
        }
        _ => unreachable!(),
    }
}
