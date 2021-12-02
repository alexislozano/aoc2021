use std::fs;

pub fn read(exo: u8) -> String {
    match fs::read_to_string(format!("src/inputs/e{:02}.txt", exo)) {
        Ok(file) => file.to_string(),
        _ => panic!("Cannot read e{:02}.txt", exo),
    }
}

pub fn write(exo: u8, part: u8, result: &str) {
    match fs::write(format!("src/outputs/e{:02}p{}.txt", exo, part), result) {
        Ok(_) => println!("e{:02}p{} ok", exo, part),
        _ => panic!("Cannot write in e{:02}p{}.txt", exo, part),
    }
}
