use super::*;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::usize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EventResult {
    Some(Event),
    None,
    Failure(Event),
    Drained,
    Pending,
    CorruptPackage,
}
enum HasPacketResult {
    Yes,
    NotYet,
    Failure(Event),
}

pub struct ByteToEventMapper {
    fifo: VecDeque<u8>,
}
impl ByteToEventMapper {
    pub fn new() -> ByteToEventMapper {
        ByteToEventMapper {
            fifo: VecDeque::new(),
        }
    }

    pub fn map(&mut self, value: u8) -> EventResult {
        self.fifo.push_back(value);

        match self.fifo.len() {
            0..=2 => EventResult::None,
            3..=2047 => match self.has_packet() {
                HasPacketResult::NotYet => EventResult::Pending,
                HasPacketResult::Failure(event) => {
                    self.fifo.drain(..);
                    EventResult::Failure(event)
                }
                HasPacketResult::Yes => match self.read_event() {
                    Event::CorruptEvent => EventResult::CorruptPackage,
                    event => EventResult::Some(event),
                },
            },
            _ => {
                self.fifo.drain(..);
                EventResult::Drained
            }
        }
    }

    fn has_packet(&self) -> HasPacketResult {
        match (self.fifo.get(0), self.fifo.get(1), self.fifo.get(2)) {
            (Some(&lower), Some(&upper), Some(&opcode)) => {
                let len = ((upper as usize) << 8) + (lower as usize);
                if OpCode::try_from(opcode).is_err() {
                    HasPacketResult::Failure(Event::CorruptEvent)
                } else if self.fifo.len() >= len + 2 {
                    HasPacketResult::Yes
                } else {
                    HasPacketResult::NotYet
                }
            }
            _ => HasPacketResult::NotYet,
        }
    }
    fn read_u8(&mut self) -> Option<u8> {
        if self.fifo.is_empty() {
            return None;
        }
        let mut iter = self.fifo.drain(..1);
        iter.next()
    }
    fn read_u16(&mut self) -> Option<u16> {
        match (self.read_u8(), self.read_u8()) {
            (Some(lower), Some(higher)) => Some((lower as u16) | (higher as u16) << 8),
            _ => None,
        }
    }
    fn read_u32(&mut self) -> Option<u32> {
        match (self.read_u16(), self.read_u16()) {
            (Some(lower), Some(higher)) => Some((lower as u32) | (higher as u32) << 16),
            _ => None,
        }
    }
    fn read_u64(&mut self) -> Option<u64> {
        match (self.read_u32(), self.read_u32()) {
            (Some(lower), Some(higher)) => Some((lower as u64) | (higher as u64) << 32),
            _ => None,
        }
    }
    fn read_i8(&mut self) -> Option<i8> {
        match self.read_u8() {
            Some(expr) => Some(expr as i8),
            None => None,
        }
    }
    fn read_i16(&mut self) -> Option<i16> {
        match self.read_u16() {
            Some(expr) => Some(expr as i16),
            None => None,
        }
    }
    fn read_i32(&mut self) -> Option<i32> {
        match self.read_u32() {
            Some(expr) => Some(expr as i32),
            None => None,
        }
    }
    fn read_bool(&mut self) -> Option<bool> {
        match self.read_u8() {
            Some(expr) => Some(expr != 0),
            None => None,
        }
    }
    fn read_enum(&mut self) -> Option<u8> {
        self.read_u8()
    }
    fn read_bdaddr(&mut self) -> Option<String> {
        let mut out = String::new();
        let mut buffer = vec![
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
        ];
        buffer.reverse();
        for (idx, b) in buffer.iter().enumerate() {
            if idx > 0 {
                out.push(':');
            }
            match b {
                Some(v) => {
                    out.push(u8_to_hex(v >> 4));
                    out.push(u8_to_hex(v << 4 >> 4));
                }
                None => return None,
            }
        }
        Some(out)
    }
    fn read_string(&mut self) -> Option<String> {
        match self.read_u8() {
            Some(len) => {
                let mut buf = vec![];
                for _ in 0..len {
                    match self.read_u8() {
                        Some(b) => buf.push(b),
                        _ => return None,
                    }
                }
                String::from_utf8(buf).ok()
            }
            _ => None,
        }
    }
    fn read_uuid(&mut self) -> Option<String> {
        let mut out = String::new();
        for _ in 0..16 {
            let b = self.read_u8();
            match b {
                Some(v) => {
                    out.push(u8_to_hex(v >> 4));
                    out.push(u8_to_hex(v << 4 >> 4));
                }
                None => return None,
            }
        }
        if &out[..] == "00000000000000000000000000000000" {
            None
        } else {
            Some(out)
        }
    }

    fn read_event(&mut self) -> Event {
        self.read_u8();
        self.read_u8();
        match self.read_u8() {
            Some(opcode) => match opcode.try_into().ok() {
                Some(OpCode::AdvertisementPacket) => match (
                    self.read_u32(),
                    self.read_bdaddr(),
                    self.read_string(),
                    self.read_u8(),
                    self.read_bool(),
                    self.read_bool(),
                    self.read_bool(),
                    self.read_bool(),
                ) {
                    (
                        Some(scan_id),
                        Some(bd_addr),
                        Some(name),
                        Some(rssi),
                        Some(is_private),
                        Some(already_verified),
                        Some(already_connected_to_this_device),
                        Some(already_connected_to_other_device),
                    ) => Event::AdvertisementPacket {
                        scan_id,
                        bd_addr,
                        name,
                        rssi,
                        is_private,
                        already_verified,
                        already_connected_to_this_device,
                        already_connected_to_other_device,
                    },
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::CreateConnectionChannelResponse) => {
                    match (self.read_u32(), self.read_enum(), self.read_enum()) {
                        (Some(conn_id), Some(error), Some(connection_status)) => {
                            match (error.try_into().ok(), connection_status.try_into().ok()) {
                                (Some(error), Some(connection_status)) => {
                                    Event::CreateConnectionChannelResponse {
                                        conn_id,
                                        error,
                                        connection_status,
                                    }
                                }
                                _ => Event::CorruptEvent,
                            }
                        }
                        _ => Event::CorruptEvent,
                    }
                }
                Some(OpCode::ConnectionStatusChanged) => {
                    match (self.read_u32(), self.read_enum(), self.read_enum()) {
                        (Some(conn_id), Some(connection_status), Some(disconnect_reason)) => {
                            match (
                                connection_status.try_into().ok(),
                                disconnect_reason.try_into().ok(),
                            ) {
                                (Some(connection_status), Some(disconnect_reason)) => {
                                    Event::ConnectionStatusChanged {
                                        conn_id,
                                        connection_status,
                                        disconnect_reason,
                                    }
                                }
                                _ => Event::CorruptEvent,
                            }
                        }
                        _ => Event::CorruptEvent,
                    }
                }
                Some(OpCode::ConnectionChannelRemoved) => match (self.read_u32(), self.read_enum())
                {
                    (Some(conn_id), Some(removed_reason)) => match removed_reason.try_into().ok() {
                        Some(removed_reason) => Event::ConnectionChannelRemoved {
                            conn_id,
                            removed_reason,
                        },
                        _ => Event::CorruptEvent,
                    },
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::ButtonUpOrDown) => match (
                    self.read_u32(),
                    self.read_enum(),
                    self.read_bool(),
                    self.read_i32(),
                ) {
                    (Some(conn_id), Some(click_type), Some(was_queued), Some(time_diff)) => {
                        match click_type.try_into().ok() {
                            Some(click_type) => Event::ButtonUpOrDown {
                                conn_id,
                                click_type,
                                was_queued,
                                time_diff,
                            },
                            _ => Event::CorruptEvent,
                        }
                    }
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::ButtonClickOrHold) => match (
                    self.read_u32(),
                    self.read_enum(),
                    self.read_bool(),
                    self.read_i32(),
                ) {
                    (Some(conn_id), Some(click_type), Some(was_queued), Some(time_diff)) => {
                        match click_type.try_into().ok() {
                            Some(click_type) => Event::ButtonClickOrHold {
                                conn_id,
                                click_type,
                                was_queued,
                                time_diff,
                            },
                            _ => Event::CorruptEvent,
                        }
                    }
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::ButtonSingleOrDoubleClick) => match (
                    self.read_u32(),
                    self.read_enum(),
                    self.read_bool(),
                    self.read_i32(),
                ) {
                    (Some(conn_id), Some(click_type), Some(was_queued), Some(time_diff)) => {
                        match click_type.try_into().ok() {
                            Some(click_type) => Event::ButtonSingleOrDoubleClick {
                                conn_id,
                                click_type,
                                was_queued,
                                time_diff,
                            },
                            _ => Event::CorruptEvent,
                        }
                    }
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::ButtonSingleOrDoubleClickOrHold) => match (
                    self.read_u32(),
                    self.read_enum(),
                    self.read_bool(),
                    self.read_i32(),
                ) {
                    (Some(conn_id), Some(click_type), Some(was_queued), Some(time_diff)) => {
                        match click_type.try_into().ok() {
                            Some(click_type) => Event::ButtonSingleOrDoubleClickOrHold {
                                conn_id,
                                click_type,
                                was_queued,
                                time_diff,
                            },
                            _ => Event::CorruptEvent,
                        }
                    }
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::NewVerifiedButton) => match (self.read_bdaddr(),) {
                    (Some(bd_addr),) => Event::NewVerifiedButton { bd_addr },
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::GetInfoResponse) => match (
                    self.read_enum(),
                    self.read_bdaddr(),
                    self.read_enum(),
                    self.read_u8(),
                    self.read_i16(),
                    self.read_u8(),
                    self.read_bool(),
                    self.read_u16(),
                ) {
                    (
                        Some(bluetooth_controller_state),
                        Some(my_bd_addr),
                        Some(my_bd_addr_type),
                        Some(max_pending_connections),
                        Some(max_concurrently_connected_buttons),
                        Some(current_pending_connections),
                        Some(currently_no_space_for_new_connection),
                        Some(buttons_size),
                    ) => {
                        let mut bd_addr_of_verified_buttons = vec![];
                        for _ in 0..buttons_size {
                            match self.read_bdaddr() {
                                Some(bd_addr) => bd_addr_of_verified_buttons.push(bd_addr),
                                None => return Event::CorruptEvent,
                            }
                        }
                        match (
                            bluetooth_controller_state.try_into().ok(),
                            my_bd_addr_type.try_into().ok(),
                        ) {
                            (Some(bluetooth_controller_state), Some(my_bd_addr_type)) => {
                                Event::GetInfoResponse {
                                    bluetooth_controller_state,
                                    my_bd_addr,
                                    my_bd_addr_type,
                                    max_pending_connections,
                                    max_concurrently_connected_buttons,
                                    current_pending_connections,
                                    currently_no_space_for_new_connection,
                                    bd_addr_of_verified_buttons,
                                }
                            }
                            _ => Event::CorruptEvent,
                        }
                    }
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::NoSpaceForNewConnection) => match (self.read_u8(),) {
                    (Some(max_concurrently_connected_buttons),) => Event::NoSpaceForNewConnection {
                        max_concurrently_connected_buttons,
                    },
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::GotSpaceForNewConnection) => match (self.read_u8(),) {
                    (Some(max_concurrently_connected_buttons),) => {
                        Event::GotSpaceForNewConnection {
                            max_concurrently_connected_buttons,
                        }
                    }
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::BluetoothControllerStateChange) => match (self.read_enum(),) {
                    (Some(state),) => match state.try_into().ok() {
                        Some(state) => Event::BluetoothControllerStateChange { state },
                        _ => Event::CorruptEvent,
                    },
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::PingResponse) => match (self.read_u32(),) {
                    (Some(ping_id),) => Event::PingResponse { ping_id },
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::GetButtonInfoResponse) => match (
                    self.read_bdaddr(),
                    self.read_uuid(),
                    self.read_string(),
                    self.read_string(),
                ) {
                    (Some(bd_addr), Some(uuid), color, serial_number) => {
                        Event::GetButtonInfoResponse {
                            bd_addr,
                            uuid,
                            color,
                            serial_number,
                        }
                    }
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::ScanWizardFoundPrivateButton) => match (self.read_u32(),) {
                    (Some(scan_wizard_id),) => {
                        Event::ScanWizardFoundPrivateButton { scan_wizard_id }
                    }
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::ScanWizardButtonConnected) => match (self.read_u32(),) {
                    (Some(scan_wizard_id),) => Event::ScanWizardButtonConnected { scan_wizard_id },
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::ScanWizardFoundPublicButton) => {
                    match (self.read_u32(), self.read_bdaddr(), self.read_string()) {
                        (Some(scan_wizard_id), Some(bd_addr), Some(name)) => {
                            Event::ScanWizardFoundPublicButton {
                                scan_wizard_id,
                                bd_addr,
                                name,
                            }
                        }
                        _ => Event::CorruptEvent,
                    }
                }
                Some(OpCode::ScanWizardCompleted) => match (self.read_u32(), self.read_enum()) {
                    (Some(scan_wizard_id), Some(result)) => match result.try_into().ok() {
                        Some(result) => Event::ScanWizardCompleted {
                            scan_wizard_id,
                            result,
                        },
                        _ => Event::CorruptEvent,
                    },
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::ButtonDeleted) => match (self.read_bdaddr(), self.read_bool()) {
                    (Some(bd_addr), Some(deleted_by_this_client)) => Event::ButtonDeleted {
                        bd_addr,
                        deleted_by_this_client,
                    },
                    _ => Event::CorruptEvent,
                },
                Some(OpCode::BatteryStatus) => {
                    match (self.read_u32(), self.read_i8(), self.read_u64()) {
                        (Some(listener_id), Some(battery_percentage), Some(timestamp)) => {
                            Event::BatteryStatus {
                                listener_id,
                                battery_percentage,
                                timestamp,
                            }
                        }
                        _ => Event::CorruptEvent,
                    }
                }
                _ => Event::CorruptEvent,
            },
            None => Event::NoOp,
        }
    }
}
fn u8_to_hex(value: u8) -> char {
    match value {
        0u8..=9u8 => (value + b'0') as char,
        _ => (value - 10u8 + b'a') as char,
    }
}
