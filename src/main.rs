use std::net::{UdpSocket};
use std::thread;
use std::collections::HashMap;


fn main() {
    println!("Hello, world!");

    let h = thread::spawn(move || {
        let mut peers: HashMap<String, ()> = HashMap::new();
        let listener = UdpSocket::bind("0.0.0.0:8096").expect("Could not bind port 8096");

        listener.set_read_timeout(Some(std::time::Duration::from_millis(5000)))
                .expect("Could not set read timeout");
        listener.set_broadcast(true)
                .expect("Could not get broadcast access for socket");

        // println!("listener bound on {:?}", listener.local_addr().unwrap());
        //             listener.connect("255.255.255.255:8096")
        //                     .expect("Could not connect to broadcast socket");
        //             listener.send(listener.local_addr().unwrap().to_string().as_bytes())
        //                     .expect("Could not send broadcast");
        loop {
            let mut buf = [0; 256];
            match listener.recv_from(&mut buf) {
                Ok((_, addr)) => {
                    let remote = String::from_utf8_lossy(&buf);
                    println!("ping from {}",addr);
                    if let None = peers.insert(remote.into_owned(), ()) {
                         println!("received ping from new peer {} ({})", String::from_utf8_lossy(&buf), peers.len());
                    } else {
                        println!("Found {} peers", peers.len());
                        //thread::sleep(std::time::Duration::from_millis(500))
                    }
                },
                Err(_) => {
                    println!("Sending broadcast...");   
                    listener.connect("255.255.255.255:8096")
                            .expect("Could not connect to broadcast socket");
                    listener.send(listener.local_addr().unwrap().to_string().as_bytes())
                            .expect("Could not send broadcast");
                            //break;
                },
            }
        }

    });

    thread::sleep(std::time::Duration::from_millis(300));
    let udp = UdpSocket::bind("0.0.0.0:0").unwrap();
    println!("bound to on port {:?}", udp.local_addr().unwrap().port());
    udp.set_broadcast(true).unwrap();
    udp.connect("255.255.255.255:8096").unwrap();

    //for _ in 0..10 {
        let addr = format!("{}", udp.local_addr().unwrap());
        udp.send(addr.as_bytes()).unwrap();
        thread::sleep(std::time::Duration::from_millis(500));
    //}
    

    h.join().unwrap();
}
