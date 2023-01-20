
pub fn parse2u64(value: &String)->u64 {
    match value.parse::<u64>() {
        Ok(i) => i,
        Err(err) => {
            panic!("Error occur while parsing string to u64. {}", err);
        },
    }
}

pub fn parse2u16(value: &String)->u16 {
    match value.parse::<u16>() {
        Ok(i) => i,
        Err(err) => {
            panic!("Error occur while parsing string to u16. {}", err);
        }
    }
}