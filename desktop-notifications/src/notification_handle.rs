use tokio::sync::oneshot;
use tokio_stream::StreamExt;
use zbus::Connection;

use crate::{
    dbus::notification::{
        client::NotificationsProxy,
        types::{Hints, ImagePath},
    },
    notification_result_handle::NotificationResultHandle,
    Actions, NotificationCloseReason, NotificationId, Timeout,
};

#[derive(Debug)]
pub(crate) enum Message {
    Close((NotificationId, NotificationCloseReason)),
    Action((NotificationId, String)),
    Error(zbus::Error),
}

pub struct NotificationHandle {
    pub(crate) connection: Connection,
    pub app_name: String,
    pub replaces_id: NotificationId,
    pub app_icon: ImagePath,
    pub summary: String,
    pub body: String,
    pub actions: Actions,
    pub hints: Hints,
    pub timeout: Timeout,
}

impl NotificationHandle {
    pub async fn show(&mut self) -> zbus::Result<NotificationResultHandle> {
        let proxy = NotificationsProxy::new(&self.connection).await?;
        self.replaces_id = proxy
            .notify(
                &self.app_name,
                &self.replaces_id,
                &self.app_icon,
                &self.summary,
                &self.body,
                &self.actions,
                &self.hints,
                &self.timeout,
            )
            .await?;
        let (sender, receiver) = oneshot::channel::<Message>();

        let id = self.replaces_id.clone();

        // Immediately expect some kind of result if the user wants it later
        tokio::spawn(async move {
            let mut notification_closed_stream = proxy.receive_notification_closed().await.unwrap();

            let mut action_invoked_stream = proxy.receive_action_invoked().await.unwrap(); // TODO

            loop {
                let message = tokio::select! {
                    Some(action_invoked) = action_invoked_stream.next() => {
                        match action_invoked.args() {
                            Ok(args) if args.id == id => Message::Action((args.id, args.action_key.to_string())),
                            Err(e) => Message::Error(e),
                            _ => continue
                        }
                     },
                    Some(notification_closed) = notification_closed_stream.next() => {
                        match notification_closed.args() {
                            Ok(args) if args.id == id => Message::Close((id, args.reason)),
                            Err(e) => Message::Error(e),
                            _ => continue
                        }
                    }
                };

                // Send the result to the channel
                sender
                    .send(message)
                    .expect("sending to channel should always succeed");
                return;
            }
        });

        return Ok(NotificationResultHandle::new(receiver));
    }
}
