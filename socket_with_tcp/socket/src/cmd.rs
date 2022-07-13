use crate::crypt::Crypt;

pub struct Commands {}

impl Commands {
    pub fn get_status_cmd() -> Vec<u8> {
        let cmd = "cmd0".as_bytes();
        cmd.encrypt()
    }

    pub fn switch_on_cmd() -> Vec<u8> {
        let cmd = "cmd1".as_bytes();
        cmd.encrypt()
    }

    pub fn switch_off_cmd() -> Vec<u8> {
        let cmd = "cmd2".as_bytes();
        cmd.encrypt()
    }
}
