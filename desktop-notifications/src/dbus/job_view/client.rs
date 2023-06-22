use zbus::{dbus_proxy, zvariant::OwnedObjectPath, Connection};

use crate::dbus::job_view_server::types::JobPath;

use super::types::Unit;

#[dbus_proxy(default_service = "org.kde.kuiserver", interface = "org.kde.JobViewV2")]
pub trait JobView {
    #[dbus_proxy(name = "clearDescriptionField")]
    fn clear_description_field(&self, number: u32) -> zbus::Result<()>;

    #[dbus_proxy(name = "setDescriptionField")]
    fn set_description_field(&self, number: u32, name: &str, value: &str) -> zbus::Result<bool>;

    #[dbus_proxy(name = "setDestUrl")]
    fn set_dest_url(&self, dest_url: &zbus::zvariant::Value<'_>) -> zbus::Result<()>;

    #[dbus_proxy(name = "setError")]
    fn set_error(&self, error_code: u32) -> zbus::Result<()>;

    #[dbus_proxy(name = "setInfoMessage")]
    fn set_info_message(&self, message: &str) -> zbus::Result<()>;

    #[dbus_proxy(name = "setPercent")]
    fn set_percent(&self, percent: u32) -> zbus::Result<()>;

    #[dbus_proxy(name = "setProcessedAmount")]
    fn set_processed_amount(&self, amount: u64, unit: Unit) -> zbus::Result<()>;

    #[dbus_proxy(name = "setSpeed")]
    fn set_speed(&self, bytes_per_second: u64) -> zbus::Result<()>;

    #[dbus_proxy(name = "setSuspended")]
    fn set_suspended(&self, suspended: bool) -> zbus::Result<()>;

    #[dbus_proxy(name = "setTotalAmount")]
    fn set_total_amount(&self, amount: u64, unit: Unit) -> zbus::Result<()>;

    #[dbus_proxy(name = "terminate")]
    fn terminate(&self, error_message: &str) -> zbus::Result<()>;

    #[dbus_proxy(signal, name = "cancelRequested")]
    fn cancel_requested(&self) -> zbus::Result<()>;

    #[dbus_proxy(signal, name = "resumeRequested")]
    fn resume_requested(&self) -> zbus::Result<()>;

    #[dbus_proxy(signal, name = "suspendRequested")]
    fn suspend_requested(&self) -> zbus::Result<()>;
}

impl<'a> JobViewProxy<'a> {
    pub async fn new_with_path(conn: &Connection, path: JobPath) -> zbus::Result<JobViewProxy<'a>> {
        let path: OwnedObjectPath = path.into();
        JobViewProxy::builder(conn).path(path)?.build().await
    }
}
