use std::{thread::sleep, time::Duration};

use ioreg::IOReg;

fn main() {
    loop {
        let battery = IOReg::apple_smart_battery().unwrap();
        println!(
            "{}W",
            battery.voltage as f32 / 1000.0 * battery.amperage as f32 / 1000.0
        );
        sleep(Duration::from_secs(1));
    }
}
