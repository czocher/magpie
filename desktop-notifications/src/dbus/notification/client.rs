use zbus::dbus_proxy;

use super::types::{
    Actions, Capability, Hints, ImagePath, NotificationCloseReason, NotificationId,
    ServerInformation, Timeout,
};

#[dbus_proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    #[dbus_proxy(name = "CloseNotification")]
    fn close_notification(&self, id: NotificationId) -> zbus::Result<()>;

    #[dbus_proxy(name = "GetCapabilities")]
    fn get_capabilities(&self) -> zbus::Result<Vec<Capability>>;

    #[dbus_proxy(name = "GetServerInformation")]
    fn get_server_information(&self) -> zbus::Result<ServerInformation>;

    #[dbus_proxy(name = "Notify")]
    fn notify(
        &self,
        app_name: &str,
        replaces_id: NotificationId,
        app_icon: ImagePath,
        summary: &str,
        body: &str,
        actions: Actions,
        hints: Hints,
        timeout: Timeout,
    ) -> zbus::Result<NotificationId>;

    /// ActionInvoked signal
    #[dbus_proxy(signal, name = "ActionInvoked")]
    fn action_invoked(&self, id: NotificationId, action_key: &str) -> zbus::Result<()>;

    /// NotificationClosed signal
    #[dbus_proxy(signal, name = "NotificationClosed")]
    fn notification_closed(
        &self,
        id: NotificationId,
        reason: NotificationCloseReason,
    ) -> zbus::Result<()>;
}
