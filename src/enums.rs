#![allow(dead_code)]

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

// Enums

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum CreateConnectionChannelError {
    NoError,
    MaxPendingConnectionsReached,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum ConnectionStatus {
    Disconnected,
    Connected,
    Ready,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum DisconnectReason {
    Unspecified,
    ConnectionEstablishmentFailed,
    TimedOut,
    BondingKeysMismatch,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum RemovedReason {
    RemovedByThisClient,
    ForceDisconnectedByThisClient,
    ForceDisconnectedByOtherClient,

    ButtonIsPrivate,
    VerifyTimeout,
    InternetBackendError,
    InvalidData,

    CouldntLoadDevice,

    DeletedByThisClient,
    DeletedByOtherClient,
    ButtonBelongsToOtherPartner,
    DeletedFromButton,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum ClickType {
    ButtonDown,
    ButtonUp,
    ButtonClick,
    ButtonSingleClick,
    ButtonDoubleClick,
    ButtonHold,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum BdAddrType {
    PublicBdAddrType,
    RandomBdAddrType,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum LatencyMode {
    NormalLatency,
    LowLatency,
    HighLatency,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum ScanWizardResult {
    WizardSuccess,
    WizardCancelledByUser,
    WizardFailedTimeout,
    WizardButtonIsPrivate,
    WizardBluetoothUnavailable,
    WizardInternetBackendError,
    WizardInvalidData,
    WizardButtonBelongsToOtherPartner,
    WizardButtonAlreadyConnectedToOtherDevice,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum BluetoothControllerState {
    Detached,
    Resetting,
    Attached,
}
