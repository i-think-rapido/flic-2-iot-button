pub mod stream_mapper;

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

use super::enums::*;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum OpCode {
    AdvertisementPacket = 0,
    CreateConnectionChannelResponse = 1,
    ConnectionStatusChanged = 2,
    ConnectionChannelRemoved = 3,
    ButtonUpOrDown = 4,
    ButtonClickOrHold = 5,
    ButtonSingleOrDoubleClick = 6,
    ButtonSingleOrDoubleClickOrHold = 7,
    NewVerifiedButton = 8,
    GetInfoResponse = 9,
    NoSpaceForNewConnection = 10,
    GotSpaceForNewConnection = 11,
    BluetoothControllerStateChange = 12,
    PingResponse = 13,
    GetButtonInfoResponse = 14,
    ScanWizardFoundPrivateButton = 15,
    ScanWizardFoundPublicButton = 16,
    ScanWizardButtonConnected = 17,
    ScanWizardCompleted = 18,
    ButtonDeleted = 19,
    BatteryStatus = 20,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    NoOp,
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

    ButtonUpOrDown {
        conn_id: u32,
        click_type: ClickType,
        was_queued: bool,
        time_diff: i32,
    },
    ButtonClickOrHold {
        conn_id: u32,
        click_type: ClickType,
        was_queued: bool,
        time_diff: i32,
    },
    ButtonSingleOrDoubleClick {
        conn_id: u32,
        click_type: ClickType,
        was_queued: bool,
        time_diff: i32,
    },
    ButtonSingleOrDoubleClickOrHold {
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
