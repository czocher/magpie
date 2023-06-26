use zbus::Connection;

use crate::{
    dbus::notification::types::{Hint, Hints, ImagePath},
    Action, Actions, NotificationHandle, NotificationId, Timeout,
};

#[derive(Default)]
pub struct NotificationBuilder {
    connection: Option<Connection>,
    app_name: String,
    replaces_id: NotificationId,
    app_icon: ImagePath,
    summary: String,
    body: String,
    actions: Actions,
    hints: Hints,
    timeout: Timeout,
}

// TODO: Proper errors
impl NotificationBuilder {
    pub async fn new() -> zbus::Result<Self> {
        Ok(Self {
            connection: Some(Connection::session().await?),
            ..Default::default()
        })
    }

    pub async fn new_with_connection(connection: &Connection) -> zbus::Result<Self> {
        Ok(Self {
            connection: Some(connection.clone()),
            ..Default::default()
        })
    }

    pub fn build(self) -> zbus::Result<NotificationHandle> {
        Ok(NotificationHandle {
            connection: self.connection.ok_or(zbus::Error::ExcessData)?, // TODO: Proper error
            app_name: self.app_name,
            replaces_id: self.replaces_id,
            app_icon: self.app_icon,
            summary: self.summary,
            body: self.body,
            actions: self.actions,
            hints: self.hints,
            timeout: self.timeout,
        })
    }

    pub fn app_name(mut self, app_name: impl AsRef<str>) -> Self {
        self.app_name = app_name.as_ref().to_string();
        self
    }

    pub fn id(mut self, replaces_id: NotificationId) -> Self {
        self.replaces_id = replaces_id;
        self
    }

    pub fn app_icon(mut self, app_icon: ImagePath) -> Self {
        self.app_icon = app_icon;
        self
    }

    pub fn summary(mut self, summary: impl AsRef<str>) -> Self {
        self.summary = summary.as_ref().to_string();
        self
    }

    pub fn body(mut self, body: impl AsRef<str>) -> Self {
        self.body = body.as_ref().to_string();
        self
    }

    pub fn actions(mut self, actions: &[impl Action]) -> Self {
        self.actions = actions.into();
        self
    }

    pub fn hints(mut self, hints: &[Hint]) -> Self {
        self.hints = hints.into();
        self
    }

    pub fn timeout(mut self, timeout: Timeout) -> Self {
        self.timeout = timeout;
        self
    }
}
