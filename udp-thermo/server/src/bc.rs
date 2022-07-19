use crate::pool::ClientPool;
use crate::thermo::Thermometer;
use std::net::UdpSocket;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Default)]
pub struct Broadcast;

impl Broadcast {
    pub fn run(pool: Arc<ClientPool>) {
        thread::spawn(move || loop {
            let clients = pool.get_clients();
            let socket = UdpSocket::bind("127.0.0.1:5000").expect("couldn't bind to address");

            let temp = Thermometer::generate();

            let buf = temp.to_be_bytes();
            for client in clients {
                println!("Send temperature: {:.2} to: {:?}", temp, client);
                socket.send_to(&buf, client).expect("Can't send data");
            }

            thread::sleep(Duration::from_secs(1));
        });
    }
}
