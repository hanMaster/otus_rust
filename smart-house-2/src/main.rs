use smart_house_2::device::Device;
use smart_house_2::house::House;
use smart_house_2::store::DeviceStore;
use smart_house_2::switcher::Switcher;
use smart_house_2::thermometer::Thermometer;

fn main() {
    let mut house = House::with_name("myHouse");
    let mut store = DeviceStore::new();

    let dining_room_list = house.get_mut_devices_list_for_room("dining room");
    dining_room_list.insert("switch1".to_string());
    store.add_device(
        "dining room",
        "switch1",
        Device::DevSwitcher(Switcher::new()),
    );
    dining_room_list.insert("thermometer1".to_string());
    store.add_device(
        "dining room",
        "thermometer1",
        Device::DevThermometer(Thermometer::new()),
    );

    let kitchen_list = house.get_mut_devices_list_for_room("kitchen");
    kitchen_list.insert("switch1".to_string());
    store.add_device("kitchen", "switch1", Device::DevSwitcher(Switcher::new()));

    house.summary(store);
}
