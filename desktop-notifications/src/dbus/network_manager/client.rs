use zbus::dbus_proxy;

use super::types::Connectivity;
use super::types::Metered;

#[dbus_proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    #[dbus_proxy(property)]
    fn connectivity(&self) -> zbus::Result<Connectivity>;

    #[dbus_proxy(property)]
    fn metered(&self) -> zbus::Result<Metered>;
}
