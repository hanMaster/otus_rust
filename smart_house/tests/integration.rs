use smart_house::device::Device;
use smart_house::house::House;
use smart_house::room::Room;
use smart_house::switcher::Switcher;
use smart_house::thermometer::Thermometer;

#[test]
fn test_summary_report() {
    let mut house = House::new(String::from("myHouse"));

    house.add_room("room1", Room::new());
    house.add_room("room2", Room::new());
    house.add_room("room3", Room::new());

    let switcher = Switcher::default();
    house
        .set_room_device("room1", "switcher1", Device::DevSwitcher(switcher))
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
        });

    let thermometer = Thermometer::default();
    house
        .set_room_device("room1", "thermometer1", Device::DevThermometer(thermometer))
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
        });


    let thermometer2 = Thermometer::default();
    house
        .set_room_device(
            "room2",
            "thermometer2",
            Device::DevThermometer(thermometer2),
        )
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
        });

    let mut switcher2 = Switcher::default();
    switcher2.set_description("Main switcher");
    house
        .set_room_device("room3", "switcher2", Device::DevSwitcher(switcher2))
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
        });

    house.print_house_summary();
}
