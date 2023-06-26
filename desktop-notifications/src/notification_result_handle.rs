use std::str::FromStr;

use tokio::sync::oneshot;

use crate::{notification_handle::Message, Action, NotificationCloseReason, NotificationResult};

pub struct NotificationResultHandle {
    channel: oneshot::Receiver<Message>,
}

impl NotificationResultHandle {
    pub(crate) fn new(channel: oneshot::Receiver<Message>) -> Self {
        Self { channel }
    }

    pub async fn action_result<T: Action + FromStr + ToString + Default>(
        self,
    ) -> zbus::Result<NotificationResult<T>> {
        let message = self.channel.await.unwrap();
        match message {
            Message::Close((_id, reason)) => Ok(NotificationResult::Close(reason)),
            Message::Action((_id, action_key)) => Ok(NotificationResult::Action(
                T::from_str(&action_key).map_err(|_| zbus::Error::ExcessData)?,
            )),
            Message::Error(e) => Err(e),
        }
    }

    pub async fn close_result(self) -> zbus::Result<NotificationCloseReason> {
        let message = self.channel.await.unwrap();
        match message {
            Message::Close((_id, reason)) => Ok(reason),
            Message::Action((_id, _action_key)) => Err(zbus::Error::ExcessData), // TODO
            Message::Error(e) => Err(e),
        }
    }
}
