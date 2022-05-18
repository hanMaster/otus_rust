use smart_house::switcher::Switcher;
use smart_house::thermometer::Thermometer;

fn main() {
    let mut switcher = Switcher::default();
    println!("Device: {}", switcher.get_description());
    switcher.set_description("Cool switcher");
    println!("Device: {}", switcher.get_description());
    println!("Device state: {}", switcher.get_state());
    switcher.toggle_switch();
    println!("Device state: {}", switcher.get_state());
    let thermometer = Thermometer::default();
    println!("Current temperature: {}", thermometer.get_temperature());
}
