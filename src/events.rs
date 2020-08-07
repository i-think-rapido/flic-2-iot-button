
use tokio::net::TcpStream;

use super::enums::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    CorruptEvent,

    AdvertisementPacket {
        scan_id: u32,
        bd_addr: String,
        name: String,
        rssi: u8,
        is_private: bool,
        already_verified: bool,
        already_connected_to_this_device: bool,
        already_connected_to_other_device: bool,
    },

    CreateConnectionChannelResponse {
        conn_id: u32,
        error: CreateConnectionChannelError,
        connection_status: ConnectionStatus,
    },

    ConnectionStatusChanged {
        conn_id: u32,
        connection_status: ConnectionStatus,
        disconnect_reason: DisconnectReason,
    },

    ConnectionChannelRemoved {
        conn_id: u32,
        removed_reason: RemovedReason,
    },

    ButtonUpOrDown{
        conn_id: u32,
        click_type: ClickType,
        was_queued: bool,
        time_diff: i32,
    },
    ButtonClickOrHold{
        conn_id: u32,
        click_type: ClickType,
        was_queued: bool,
        time_diff: i32,
    },
    ButtonSingleOrDoubleClick{
        conn_id: u32,
        click_type: ClickType,
        was_queued: bool,
        time_diff: i32,
    },
    ButtonSingleOrDoubleClickOrHold{
        conn_id: u32,
        click_type: ClickType,
        was_queued: bool,
        time_diff: i32,
    },

    NewVerifiedButton {
        bd_addr: String,
    },

    GetInfoResponse {
        bluetooth_controller_state: BluetoothControllerState,
        my_bd_addr: String,
        my_bd_addr_type: BdAddrType,
        max_pending_connections: u8,
        max_concurrently_connected_buttons: i16,
        current_pending_connections: u8,
        currently_no_space_for_new_connection: bool,
        bd_addr_of_verified_buttons: Vec<String>,
    },

    NoSpaceForNewConnection {
        max_concurrently_connected_buttons: u8,
    },

    GotSpaceForNewConnection {
        max_concurrently_connected_buttons: u8,
    },

    BluetoothControllerStateChange {
        state: BluetoothControllerState,
    },

    PingResponse {
        ping_id: u32,
    },

    GetButtonInfoResponse {
        bd_addr: String,
        uuid: String,
        color: Option<String>,
        serial_number: Option<String>,
    },

    ScanWizardFoundPrivateButton {
        scan_wizard_id: u32,
    },

    ScanWizardFoundPublicButton {
        scan_wizard_id: u32,
        bd_addr: String,
        name: String,
    },

    ScanWizardButtonConnected {
        scan_wizard_id: u32,
    },

    ScanWizardCompleted {
        scan_wizard_id: u32,
        result: ScanWizardResult,
    },

    ButtonDeleted {
        bd_addr: String,
        deleted_by_this_client: bool,
    },

    BatteryStatus {
        listener_id: u32,
        battery_percentage: i8,
        timestamp: u64,
    },
}

impl Event {
    pub fn opcode(&self) -> u8 {
        match self {
            Self::CorruptEvent => 255,
            Self::AdvertisementPacket{..} => 0,
            Self::CreateConnectionChannelResponse{..} => 1,
            Self::ConnectionStatusChanged{..} => 2,
            Self::ConnectionChannelRemoved{..} => 3,
            Self::ButtonUpOrDown{..} => 4,
            Self::ButtonClickOrHold{..} => 5,
            Self::ButtonSingleOrDoubleClick{..} => 6,
            Self::ButtonSingleOrDoubleClickOrHold{..} => 7,
            Self::NewVerifiedButton{..} => 8,
            Self::GetInfoResponse{..} => 9,
            Self::NoSpaceForNewConnection{..} => 10,
            Self::GotSpaceForNewConnection{..} => 11,
            Self::BluetoothControllerStateChange{..} => 12,
            Self::PingResponse{..} => 13,
            Self::GetButtonInfoResponse{..} => 14,
            Self::ScanWizardFoundPrivateButton{..} => 15,
            Self::ScanWizardFoundPublicButton{..} => 16,
            Self::ScanWizardButtonConnected{..} => 17,
            Self::ScanWizardCompleted{..} => 18,
            Self::ButtonDeleted{..} => 19,
            Self::BatteryStatus{..} => 20,
        }
    }

    pub fn read_event(opcode: u8, reader: &mut TcpStream) -> Event {
        match opcode {
            13 => Self::PingResponse{ping_id: 8},
            _ => Self::CorruptEvent,
        }
    }
        
}

