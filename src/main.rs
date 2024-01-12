use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct BatteryInfoPayload {
    deviceSlug: String,
    batteryLevel: f32,
    interruptMode: String,
    power: bool,
}

struct BatteryInfo {
    battery_level: f32,
    power: bool,
}

fn get_battery_info() -> Result<BatteryInfo, battery::Error> {
    let manager = battery::Manager::new();

    // handle errors
    let manager = match manager {
        Ok(m) => m,
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    };

    let batteries = match manager.batteries() {
        Ok(b) => b,
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    };

    for battery in batteries {
        let battery = match battery {
            Ok(b) => b,
            Err(e) => {
                println!("Error: {}", e);
                return Err(e);
            }
        };

        let battery_level = battery.state_of_charge().value * 100.0;
        let power = battery.state() != battery::State::Discharging;

        return Ok(BatteryInfo {
            battery_level: battery_level,
            power,
        });
    }

    return Ok(BatteryInfo {
        battery_level: 0.0,
        power: false,
    });
}

fn main() {
    let api_endpoint = std::env::var("API_ENDPOINT").expect("API_ENDPOINT not set");
    let device_slug = std::env::var("DEVICE_SLUG").expect("DEVICE_SLUG not set");

    let info = get_battery_info().expect("Error getting battery level'");
    println!("Battery level: {}%", info.battery_level);
    println!("Power: {}", info.power);

    let payload = BatteryInfoPayload {
        deviceSlug: device_slug,
        batteryLevel: info.battery_level,
        power: info.power,
        interruptMode: "none".to_string(),
    };

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(api_endpoint)
        .json(&payload)
        .send()
        .expect("Error sending request");

    println!("Success: {}", res.status().is_success());
}
