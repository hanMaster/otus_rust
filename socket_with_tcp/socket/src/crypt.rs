pub trait Crypt {
    fn encrypt(&self) -> Vec<u8>;
    fn decrypt(&self) -> Vec<u8>;
}

impl Crypt for &[u8] {
    fn encrypt(&self) -> Vec<u8> {
        let key = 252;
        self.iter().map(|i| i ^ key).collect()
    }

    fn decrypt(&self) -> Vec<u8> {
        self.encrypt()
    }
}
