use client::{errors::Result, HttpClient};
use models::device::DeviceType;

fn main() -> Result<()> {
    let mut client = HttpClient::new("http://localhost:8080/")?;

    client.add_room("hall")?;
    client.add_room("kitchen")?;
    client.add_room("bedroom")?;

    client.add_device("hall", "hallSocket1", DeviceType::Socket)?;
    client.add_device("hall", "hallThermo1", DeviceType::Thermometer)?;
    client.add_device("kitchen", "kitchenSocket1", DeviceType::Socket)?;
    client.add_device("kitchen", "kitchenThermo1", DeviceType::Thermometer)?;
    client.add_device("bedroom", "bedroomSocket1", DeviceType::Socket)?;
    client.add_device("bedroom", "bedroomThermo1", DeviceType::Thermometer)?;
    client.get_house_structure()?;
    client.house_summary()?;
    Ok(())
}

// cargo run --example smart_house
