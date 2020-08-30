pub mod stream_mapper;

use super::enums::LatencyMode;

/// Commands

pub enum Command {
    GetInfo,
    CreateScanner {
        scan_id: u32,
    },
    RemoveScanner {
        scan_id: u32,
    },
    CreateConnectionChannel {
        conn_id: u32,
        bd_addr: String,
        latency_mode: LatencyMode,
        auto_disconnect_time: i16,
    },
    RemoveConnectionChannel {
        conn_id: u32,
    },
    ForceDisconnect {
        bd_addr: String,
    },
    ChangeModeParameters {
        conn_id: u32,
        latency_mode: LatencyMode,
        auto_disconnect_time: i16,
    },
    Ping {
        ping_id: u32,
    },
    GetButtonInfo {
        bd_addr: String,
    },
    CreateScanWizard {
        scan_wizard_id: u32,
    },
    CancelScanWizard {
        scan_wizard_id: u32,
    },
    DeleteButton {
        bd_addr: String,
    },
    CreateBatteryStatusListener {
        listener_id: u32,
        bd_addr: String,
    },
    RemoveBatteryStatusListener {
        listener_id: u32,
    },
}

impl Command {
    pub fn opcode(&self) -> u8 {
        match self {
            Self::GetInfo { .. } => 0,
            Self::CreateScanner { .. } => 1,
            Self::RemoveScanner { .. } => 2,
            Self::CreateConnectionChannel { .. } => 3,
            Self::RemoveConnectionChannel { .. } => 4,
            Self::ForceDisconnect { .. } => 5,
            Self::ChangeModeParameters { .. } => 6,
            Self::Ping { .. } => 7,
            Self::GetButtonInfo { .. } => 8,
            Self::CreateScanWizard { .. } => 9,
            Self::CancelScanWizard { .. } => 10,
            Self::DeleteButton { .. } => 11,
            Self::CreateBatteryStatusListener { .. } => 12,
            Self::RemoveBatteryStatusListener { .. } => 13,
        }
    }
}
