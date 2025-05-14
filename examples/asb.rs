use ioreg::IOReg;

fn main() {
    println!(
        "{:?}",
        IOReg::apple_smart_battery().unwrap().power_telemetry_data
    );
}
