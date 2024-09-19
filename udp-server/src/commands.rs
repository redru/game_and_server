use std::str;

const HANDSHAKE_UUID_IDX: usize = 2;
const HANDSHAKE_UUID_SIZE: usize = 36;
const HANDSHAKE_PORT_IDX: usize = 38;

const TOTAL_TIME_TIME_IDX: usize = 38;

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Handshake,
    TotalTime,
}

pub fn parse_command(data: &[u8]) -> Option<Command> {
    let command_code = str::from_utf8(&data[0..2]).expect("error parsing command code");

    match command_code {
        "00" => Some(Command::Handshake),
        "01" => Some(Command::TotalTime),
        _ => None,
    }
}

pub fn get_uuid(data: &[u8]) -> &str {
    str::from_utf8(&data[HANDSHAKE_UUID_IDX..HANDSHAKE_UUID_IDX + HANDSHAKE_UUID_SIZE])
        .expect("error parsing uuid")
}

pub fn get_port(data: &[u8]) -> u16 {
    str::from_utf8(&data[HANDSHAKE_PORT_IDX..])
        .expect("error parsing the port")
        .parse()
        .expect("error casting the port to u16")
}

pub fn get_time(data: &[u8]) -> &[u8] {
    &data[TOTAL_TIME_TIME_IDX..]
}
