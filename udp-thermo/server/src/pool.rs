use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::Mutex;

#[derive(Default, Debug)]
pub struct ClientPool {
    clients: Mutex<HashSet<SocketAddr>>,
}

impl ClientPool {
    pub fn store_client(&self, addr: SocketAddr) {
        self.clients.lock().unwrap().insert(addr);
    }

    pub fn remove_client(&self, addr: SocketAddr) {
        self.clients.lock().unwrap().remove(&addr);
    }

    pub fn get_clients(&self) -> Vec<SocketAddr> {
        let set = self.clients.lock().unwrap();
        set.iter().copied().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::{Ipv4Addr, SocketAddrV4};
    use std::str::FromStr;

    #[test]
    fn test_add_remove_clients() {
        let mut pool = ClientPool::default();
        pool.store_client(SocketAddr::from_str("127.0.0.1:5000").unwrap());
        pool.store_client(SocketAddr::from_str("127.0.0.1:5001").unwrap());
        assert_eq!(pool.clients.lock().unwrap().len(), 2);
        pool.remove_client(SocketAddr::from_str("127.0.0.1:5001").unwrap());
        assert_eq!(pool.clients.lock().unwrap().len(), 1);
        assert!(pool
            .clients
            .lock()
            .unwrap()
            .contains(&SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::new(127, 0, 0, 1),
                5000
            ))));
    }

    #[test]
    fn test_get_clients() {
        let mut pool = ClientPool::default();
        pool.store_client(SocketAddr::from_str("127.0.0.1:5000").unwrap());
        pool.store_client(SocketAddr::from_str("127.0.0.1:5001").unwrap());
        let clients = pool.get_clients();
        println!("clients: {:?}", clients);
    }
}
