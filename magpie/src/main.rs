use std::{process::Stdio, str::FromStr, time::Duration};

use desktop_notifications::dbus::{
    job_view::client::JobViewProxy,
    job_view_server::{client::JobViewServerProxy, types::Capability},
    network_manager::{
        client::NetworkManagerProxy,
        types::{Connectivity, Metered},
    },
    notification::{
        client::{ActionInvoked, NotificationsProxy},
        types::{Action, Actions, Hints, ImagePath, NotificationId, Timeout},
    },
};
use futures::StreamExt;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};
use zbus::Connection;

#[derive(Display, Debug, AsRefStr, EnumString, Clone, Default, EnumIter)]
enum Choice {
    #[strum(to_string = "Cancel", serialize = "__closed")]
    Cancel,
    #[default]
    Backup,
}

impl Action for Choice {
    fn key(&self) -> String {
        self.as_ref().to_string()
    }

    fn value(&self) -> String {
        self.to_string()
    }
}

impl From<ActionInvoked> for Choice {
    fn from(value: ActionInvoked) -> Self {
        if let Ok(value) = value.args().map(|a| a.action_key) {
            Choice::from_str(value).unwrap_or_default()
        } else {
            Choice::Cancel
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = zbus::Connection::session().await?;
    let system = zbus::Connection::system().await?;

    let connected = is_connected(&system).await?;

    if !connected {
        // TODO: Change to error
        return Ok(());
    }

    let metered = is_metered(&system).await?;

    let mut message = "Do you want to perform a backup?".to_string();

    if metered {
        message += "\n<i>Note: The network connection seems to be metered.</i>";
    }

    let proxy = NotificationsProxy::new(&session).await?;
    proxy
        .notify(
            "magpie",
            NotificationId::None,
            ImagePath::Name("backup".to_string()),
            "Create backup?",
            &message,
            Actions::from_iter(Choice::iter()),
            Hints::default(),
            Timeout::Duration(Duration::from_secs(1)),
        )
        .await?;

    let mut notification_closed_stream = proxy.receive_notification_closed().await?;
    let mut action_invoked_stream = proxy.receive_action_invoked().await?;

    tokio::select! {
        Some(action) = action_invoked_stream.next() =>  match Choice::from(action) {
            Choice::Cancel => return Ok(()),
            Choice::Backup => {
                perform_backup(&session).await?;
            },
        },
        Some(_) = notification_closed_stream.next() => return Ok(())
    }

    Ok(())
}

async fn is_metered(system: &Connection) -> zbus::Result<bool> {
    let metered = NetworkManagerProxy::new(system).await?.metered().await?;

    Ok(matches!(
        metered,
        Metered::GuessYes | Metered::Yes | Metered::Unknown
    ))
}

async fn is_connected(system: &Connection) -> zbus::Result<bool> {
    let connectivity = NetworkManagerProxy::new(system)
        .await?
        .connectivity()
        .await?;

    Ok(matches!(connectivity, Connectivity::Full))
}

async fn perform_backup(session: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let job_view_server = JobViewServerProxy::new(session).await?;
    let path = job_view_server
        .request_view(
            "magpie",
            "backup",
            vec![Capability::Killable, Capability::Suspendable].into(),
        )
        .await?;
    let job_view = JobViewProxy::new_with_path(session, path).await?;
    job_view.set_info_message("Backing up").await?;

    let mut cmd = Command::new("restic");
    let cmd = cmd
        .arg("--json")
        .arg("backup")
        .arg("--files-from")
        .arg("/home/czocher/.backup_list")
        .stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().unwrap();

    let mut reader = BufReader::new(stdout).lines();

    tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .expect("child process encountered an error");

        println!("child status was: {}", status);
    });

    while let Some(line) = reader.next_line().await? {
        println!("Line: {}", line);
    }

    Ok(())
}
