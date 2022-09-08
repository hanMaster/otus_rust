pub trait DeviceInfo {
    fn get_info(&self) -> String {
        String::from("Device info")
    }

    fn get_info_by_room_and_device(&self, _room_name: &str, _device_name: &str) -> Option<String> {
        Some(String::from("Device info"))
    }
}
