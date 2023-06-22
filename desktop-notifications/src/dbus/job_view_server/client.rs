use zbus::dbus_proxy;

use super::types::{Capabilities, JobPath};

#[dbus_proxy(
    default_service = "org.kde.kuiserver",
    interface = "org.kde.JobViewServer",
    default_path = "/JobViewServer"
)]
pub trait JobViewServer {
    #[dbus_proxy(name = "requestView")]
    fn request_view(
        &self,
        app_name: &str,
        app_icon_name: &str,
        capabilities: Capabilities,
    ) -> zbus::Result<JobPath>;
}
