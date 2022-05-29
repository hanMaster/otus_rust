use smart_house::device::Device;
use smart_house::house::House;
use smart_house::room::Room;
use smart_house::switcher::Switcher;
use smart_house::thermometer::Thermometer;

fn main() {
    let mut house = House::new(String::from("myHouse"));

    house.add_room("room1", Room::new());
    house.add_room("room2", Room::new());
    house.add_room("room3", Room::new());

    let rooms = house.get_rooms_list();
    println!("Rooms list:");
    for item in rooms {
        println!("{}", item);
    }

    println!("remove room 2");
    house.remove_room("room2");

    let rooms = house.get_rooms_list();
    for item in rooms {
        println!("{}", item);
    }

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
            "room3",
            "thermometer2",
            Device::DevThermometer(thermometer2),
        )
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
        });

    let devices = house.get_room_devices("room1").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        vec![]
    });

    println!("Device list for room1:");
    for item in devices {
        println!("{}", item);
    }

    let devices = house.get_room_devices("room3").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        vec![]
    });

    println!("Device list for room3:");
    for item in devices {
        println!("{}", item);
    }

    println!("Remove thermometer1 from room1");

    house
        .unset_room_device("room1", "thermometer1")
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
        });

    let devices = house.get_room_devices("room1").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        vec![]
    });

    println!("Device list for room1:");
    for item in devices {
        println!("{}", item);
    }

    // println!("Device: {}", switcher.get_description());
    // println!("Device state: {}", switcher.get_state());
    // switcher.toggle_switch();
    // println!("Device state: {}", switcher.get_state());
    // let thermometer = Thermometer::default();
    // println!("Current temperature: {}", thermometer.get_temperature());
}
