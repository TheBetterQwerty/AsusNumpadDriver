use std::io::{BufReader, BufRead};

pub fn find_event() -> Option<(i32, u32)> {
    let fd = std::fs::File::open("/proc/bus/input/devices").ok()?;
    let reader = BufReader::new(fd);

    let mut touchpad = 0;
    let mut keyboard = 0;
    let mut touchpad_event_idx = -1;
    let mut _keyboard_event_idx = -1;
    let mut device_id = 0;

    for line in reader.lines() {
        let line = line.ok()?;

        if touchpad == 0
            && (line.contains("Name=\"ASUE") || line.contains("Name=\"ELAN"))
                && line.contains("Touchpad")
        {
            touchpad = 1;
        }

        if touchpad == 1 {
            if line.starts_with("Sysfs=") {
                let num = line
                    .split('/')
                    .find_map(|part| part.strip_prefix("i2c-"))
                    .and_then(|rest| rest.parse::<u32>().ok());
                if num.is_some() {
                    device_id = num.unwrap();
                }
            }

            if let Some(rest) = line.split("event").nth(1) {
                if let Ok(idx) = rest
                    .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<i32>()
                {
                    touchpad_event_idx = idx;
                    touchpad = 2;
                }
            }
        }

        if keyboard == 0
            && (line.contains("Name=\"AT Translated Set 2 keyboard")
                || line.contains("Name=\"Asus Keyboard"))
        {
            keyboard = 1;
        }

        if keyboard == 1 {
            if let Some(rest) = line.split("event").nth(1) {
                if let Ok(idx) = rest
                    .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<i32>()
                {
                    _keyboard_event_idx = idx;
                    keyboard = 2;
                }
            }
        }

        if touchpad == 2 && keyboard == 2 {
            break;
        }
    }

    Some((touchpad_event_idx, device_id))
}
