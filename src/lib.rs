mod apple_smart_bettery;
mod error;

use std::io::Cursor;

use error::Result;

pub struct IOReg();

impl IOReg {
    pub fn apple_smart_battery() -> Result<apple_smart_bettery::AppleSmartBattery> {
        let res = std::process::Command::new("ioreg")
            .arg("-a")
            .arg("-r")
            .arg("-c")
            .arg("AppleSmartBattery")
            .output()?
            .stdout;
        let cursor = Cursor::new(res);
        let reader = plist::stream::Reader::new(cursor);
        let mut deserializer = plist::Deserializer::new(reader);
        let mut res: Vec<_> = serde_path_to_error::deserialize(&mut deserializer).unwrap();
        Ok(res.pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apple_smart_battery() {
        let battery = IOReg::apple_smart_battery();
        println!("{:?}", battery);
    }

    #[test]
    fn print_value() {
        let res = std::process::Command::new("ioreg")
            .arg("-a")
            .arg("-r")
            .arg("-c")
            .arg("AppleSmartBattery")
            .output()
            .unwrap()
            .stdout;
        let value: plist::Value = plist::from_bytes(&res).unwrap();
        println!("{:?}", value);
    }
}
