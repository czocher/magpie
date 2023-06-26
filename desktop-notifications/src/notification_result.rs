use std::str::FromStr;

use crate::{Action, NotificationCloseReason};

#[derive(Debug)]
pub enum NotificationResult<T: Action + FromStr + ToString + Default> {
    Close(NotificationCloseReason),
    Action(T),
}
