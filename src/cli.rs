pub enum Command{
    Get(String),
    Set(String,String),
    Delete(String),
    Keys,

    Unknown
}

pub fn parse(input: &str) -> Command{
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts.as_slice() {
        ["get", key] => Command::Get((*key).into()),
        ["set", key, value]=> Command::Set((*key).into(), (*value).into()),
        ["keys"] => Command::Keys,
        _ => Command::Unknown,
    }
}