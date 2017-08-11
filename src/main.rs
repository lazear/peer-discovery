use std::net::{UdpSocket, SocketAddr};
use std::thread;
use std::collections::HashSet;


fn main() {
    println!("Hello, world!");

    let h = thread::spawn(move || {
        let mut peers: HashSet<SocketAddr> = HashSet::new();
        let listener = UdpSocket::bind("0.0.0.0:8096").expect("Could not bind port 8096");

        listener.set_read_timeout(Some(std::time::Duration::from_millis(500)))
                .expect("Could not set read timeout");
        listener.set_broadcast(true)
                .expect("Could not get broadcast access for socket");

        println!("listener bound on {:?}", listener.local_addr().unwrap());

        loop {
            let mut buf = [0; 256];
            match listener.recv_from(&mut buf) {
                Ok((_, addr)) => {
                    if peers.insert(addr) {
                         println!("received ping from new peer {}", String::from_utf8_lossy(&buf));
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
    h.join().unwrap();
}
