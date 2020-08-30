use regex::Regex;

use std::collections::vec_deque::Drain;
use std::collections::VecDeque;
use std::convert::TryInto;

use super::*;

pub struct CommandToByteMapper {
    buffer: VecDeque<u8>,
}
impl CommandToByteMapper {
    pub fn new() -> CommandToByteMapper {
        CommandToByteMapper {
            buffer: VecDeque::new(),
        }
    }

    pub fn map(&mut self, command: Command) -> Drain<u8> {
        self.clear_buffer();

        self.write_u8(command.opcode());
        match command {
            Command::GetInfo => {}
            Command::CreateScanner { scan_id } => {
                self.write_u32(scan_id);
            }
            Command::RemoveScanner { scan_id } => {
                self.write_u32(scan_id);
            }
            Command::CreateConnectionChannel {
                conn_id,
                bd_addr,
                latency_mode,
                auto_disconnect_time,
            } => {
                self.write_u32(conn_id);
                self.write_bdaddr(&bd_addr[..]);
                if let Ok(latency_mode) = latency_mode.try_into() {
                    self.write_u8(latency_mode);
                }
                self.write_i16(auto_disconnect_time);
            }
            Command::RemoveConnectionChannel { conn_id } => {
                self.write_u32(conn_id);
            }
            Command::ForceDisconnect { bd_addr } => {
                self.write_bdaddr(&bd_addr[..]);
            }
            Command::ChangeModeParameters {
                conn_id,
                latency_mode,
                auto_disconnect_time,
            } => {
                self.write_u32(conn_id);
                if let Ok(latency_mode) = latency_mode.try_into() {
                    self.write_u8(latency_mode);
                }
                self.write_i16(auto_disconnect_time);
            }
            Command::Ping { ping_id } => {
                self.write_u32(ping_id);
            }
            Command::GetButtonInfo { bd_addr } => {
                self.write_bdaddr(&bd_addr[..]);
            }
            Command::CreateScanWizard { scan_wizard_id } => {
                self.write_u32(scan_wizard_id);
            }
            Command::CancelScanWizard { scan_wizard_id } => {
                self.write_u32(scan_wizard_id);
            }
            Command::DeleteButton { bd_addr } => {
                self.write_bdaddr(&bd_addr[..]);
            }
            Command::CreateBatteryStatusListener {
                listener_id,
                bd_addr,
            } => {
                self.write_u32(listener_id);
                self.write_bdaddr(&bd_addr[..]);
            }
            Command::RemoveBatteryStatusListener { listener_id } => {
                self.write_u32(listener_id);
            }
        }

        self.prepend_size();

        self.buffer.drain(..)
    }

    fn clear_buffer(&mut self) {
        self.buffer.drain(..);
    }

    fn prepend_size(&mut self) {
        let len = self.buffer.len();
        self.buffer.push_front((len >> 8) as u8);
        self.buffer.push_front((len & 255) as u8);
    }
    fn write_u8(&mut self, value: u8) {
        let mut buf = vec![value].into_iter().collect();
        self.buffer.append(&mut buf);
    }
    /*
        fn write_bool(&mut self, value: bool) {
            if value {
                self.write_u8(1);
            }
            else {
                self.write_u8(0);
            }
        }
    */
    fn write_u16(&mut self, value: u16) {
        self.write_u8(value as u8);
        self.write_u8((value >> 8) as u8);
    }
    fn write_i16(&mut self, value: i16) {
        self.write_u8(value as u8);
        self.write_u8((value >> 8) as u8);
    }
    fn write_u32(&mut self, value: u32) {
        self.write_u16(value as u16);
        self.write_u16((value >> 16) as u16);
    }
    /*
        fn write_i32(&mut self, value: i32) {
            self.write_i16(value as i16);
            self.write_i16((value >> 16) as i16);
        }
    */
    fn write_bdaddr(&mut self, str: &str) {
        let re = Regex::new(r"([0-9a-z]{2}:){5}[0-9a-z]{2}").unwrap();
        if re.is_match(str) {
            if let Some(b) = hex_to_u8(&str[15..]) {
                self.write_u8(b);
            }
            if let Some(b) = hex_to_u8(&str[12..]) {
                self.write_u8(b);
            }
            if let Some(b) = hex_to_u8(&str[9..]) {
                self.write_u8(b);
            }
            if let Some(b) = hex_to_u8(&str[6..]) {
                self.write_u8(b);
            }
            if let Some(b) = hex_to_u8(&str[3..]) {
                self.write_u8(b);
            }
            if let Some(b) = hex_to_u8(&str[..]) {
                self.write_u8(b);
            }
        }
    }
}

fn hex_to_u8(buffer: &str) -> Option<u8> {
    let mut char_indices = buffer.char_indices();
    match (char_indices.next(), char_indices.next()) {
        (Some((0, upper)), Some((1, lower))) => {
            let mut b = 0u8;
            if let Some(v) = upper.to_digit(16) {
                b += (v as u8) << 4;
            }
            if let Some(v) = lower.to_digit(16) {
                b += v as u8;
            }
            Some(b)
        }
        _ => None,
    }
}
