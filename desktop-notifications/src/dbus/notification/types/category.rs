use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, Display, Clone, Default)]
pub enum Category {
    #[default]
    #[strum(serialize = "")]
    None,
    Other(String),
    #[strum(serialize = "device")]
    Device,
    #[strum(serialize = "device.added")]
    DeviceAdded,
    #[strum(serialize = "device.error")]
    DeviceError,
    #[strum(serialize = "device.removed")]
    DeviceRemoved,
    #[strum(serialize = "email")]
    Email,
    #[strum(serialize = "email.arrived")]
    EmailArrived,
    #[strum(serialize = "email.bounced")]
    EmailBounced,
    #[strum(serialize = "im")]
    Im,
    #[strum(serialize = "im.error")]
    ImError,
    #[strum(serialize = "im.received")]
    ImReceived,
    #[strum(serialize = "network")]
    Network,
    #[strum(serialize = "network.connected")]
    NetworkConnected,
    #[strum(serialize = "network.disconnected")]
    NetworkDisconnected,
    #[strum(serialize = "network.error")]
    NetworkError,
    #[strum(serialize = "presence")]
    Presence,
    #[strum(serialize = "presence.offline")]
    PresenceOffline,
    #[strum(serialize = "presence.online")]
    PresenceOnline,
    #[strum(serialize = "transfer")]
    Transfer,
    #[strum(serialize = "transfer.complete")]
    TransferComplete,
    #[strum(serialize = "transfer.error")]
    TransferError,
}
