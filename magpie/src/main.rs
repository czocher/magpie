use std::{process::Stdio, str::FromStr, time::Duration};

use desktop_notifications::{
    hints::{Category, Hint, SoundName},
    Action, ImagePath, NotificationBuilder,
};
use futures::StreamExt;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    time::sleep,
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

    let mut notification = NotificationBuilder::new()
        .await?
        .app_name("magpie")
        .app_icon(ImagePath::Name("backup".to_string()))
        .summary("Create backup?")
        .body(message)
        .hints(&[Hint::SoundContext(SoundName::Action(
            desktop_notifications::hints::Action::AudioChannelFrontCenter,
        ))])
        .timeout(Duration::from_secs(1).into())
        .actions(&[Choice::Cancel, Choice::Backup])
        .build()?;

    let mut notification = notification.show().await?;

    Ok(())
}

async fn is_metered(system: &Connection) -> zbus::Result<bool> {
    // let metered = NetworkManagerProxy::new(system).await?.metered().await?;

    // Ok(matches!(
    //     metered,
    //     Metered::GuessYes | Metered::Yes | Metered::Unknown
    // ))
    Ok(true)
}

async fn is_connected(system: &Connection) -> zbus::Result<bool> {
    // let connectivity = NetworkManagerProxy::new(system)
    //     .await?
    //     .connectivity()
    //     .await?;

    // Ok(matches!(connectivity, Connectivity::Full))
    Ok(true)
}

async fn perform_backup(session: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // let job_view_server = JobViewServerProxy::new(session).await?;
    // let path = job_view_server
    //     .request_view(
    //         "magpie",
    //         "backup",
    //         vec![Capability::Killable, Capability::Suspendable].into(),
    //     )
    //     .await?;
    // let job_view = JobViewProxy::new_with_path(session, path).await?;
    // job_view.set_info_message("Backing up").await?;

    // let mut cmd = Command::new("restic");
    // let cmd = cmd
    //     .arg("--json")
    //     .arg("backup")
    //     .arg("--files-from")
    //     .arg("/home/czocher/.backup_list")
    //     .stdout(Stdio::piped());

    // let mut child = cmd.spawn()?;
    // let stdout = child.stdout.take().unwrap();

    // let mut reader = BufReader::new(stdout).lines();

    // tokio::spawn(async move {
    //     let status = child
    //         .wait()
    //         .await
    //         .expect("child process encountered an error");

    //     println!("child status was: {}", status);
    // });

    // while let Some(line) = reader.next_line().await? {
    //     println!("Line: {}", line);
    // }

    Ok(())
}
