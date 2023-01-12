fn main() {
    let args: Vec<String> = std::env::args().collect();


    match args[1].as_str() {
        "-p" => {
            let mut handles = Vec::new();
            for _i in 0..10 {
                let h = std::thread::spawn(move || loop {
                    let context = zmq::Context::new();
                    let p = context.socket(zmq::PUB).unwrap();
                    p.connect("tcp://localhost:9950").unwrap();
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
            let sub = context.socket(zmq::SUB).unwrap();
            sub.connect("tcp://localhost:9951").unwrap();
            sub.set_subscribe("heartbeat".as_bytes()).unwrap();
            let mut msg = zmq::Message::new();
            loop {
                println!("waiting for message...");
                sub.recv(&mut msg, 0).unwrap();
                dbg!(msg.as_str().unwrap());
            }
        }
        _ => unreachable!(),
    }
}
