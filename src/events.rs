
use super::enums::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    AdvertisementPacket {
        opcode: u8,
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

    ButtonEvent {
        conn_id: u32,
        click_type: ClickType,
        was_queued: bool,
        time_diff: i32,
    },

    NewVerifiedButton {
        opcode: u8,
        bd_addr: String,
    },

    GetInfoResponse {
        opcode: u8,
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
        opcode: u8,
        max_concurrently_connected_buttons: u8,
    },

    GotSpaceForNewConnection {
        opcode: u8,
        max_concurrently_connected_buttons: u8,
    },

    BluetoothControllerStateChange {
        opcode: u8,
        state: BluetoothControllerState,
    },

    PingResponse {
        opcode: u8,
        ping_id: u32,
    },

    GetButtonInfoResponse {
        opcode: u8,
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
        opcode: u8,
        bd_addr: String,
        deleted_by_this_client: bool,
    },

    BatteryStatus {
        opcode: u8,
        listener_id: u32,
        battery_percentage: i8,
        timestamp: u64,
    },
}
