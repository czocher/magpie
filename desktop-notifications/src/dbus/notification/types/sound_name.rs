use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Default)]
pub enum SoundName {
    #[default]
    None,
    Other(String),
    Alert(Alert),
    Notification(Notification),
    Action(Action),
    InputFeedback(InputFeedback),
    Game(Game),
}

impl From<&str> for SoundName {
    fn from(value: &str) -> Self {
        SoundName::Other(value.to_string())
    }
}

impl ToString for SoundName {
    fn to_string(&self) -> String {
        match self {
            SoundName::None => "".to_string(),
            SoundName::Other(value) => value.to_string(),
            SoundName::Alert(alert) => alert.to_string(),
            SoundName::Notification(notification) => notification.to_string(),
            SoundName::Action(action) => action.to_string(),
            SoundName::InputFeedback(input_feedback) => input_feedback.to_string(),
            SoundName::Game(game) => game.to_string(),
        }
    }
}

#[derive(Debug, EnumString, Display, Clone, Default)]
#[strum(serialize_all = "kebab-case")]
pub enum Alert {
    #[default]
    #[strum(serialize = "")]
    None,
    NetworkConnectivityLost,
    NetworkConnectivityError,
    DialogError,
    BatteryLow,
    SuspendError,
    SoftwareUpdateUrgent,
    PowerUnplugBatteryLow,
}

#[derive(Debug, EnumString, Display, Clone, Default)]
#[strum(serialize_all = "kebab-case")]
pub enum Notification {
    #[default]
    #[strum(serialize = "")]
    None,
    MessageNewInstant,
    MessageNewEmail,
    CompleteMediaBurn,
    CompleteMediaBurnTest,
    CompleteMediaRip,
    CompleteMediaFormat,
    CompleteDownload,
    CompleteCopy,
    CompleteScan,
    PhoneIncomingCall,
    PhoneOutgoingBusy,
    PhoneHangup,
    PhoneFailure,
    NetworkConnectivityEstablished,
    SystemBootup,
    SystemReady,
    SystemShutdown,
    SearchResults,
    SearchResultsEmpty,
    DesktopLogin,
    DesktopLogout,
    DesktopScreenLock,
    ServiceLogin,
    ServiceLogout,
    BatteryCaution,
    BatteryFull,
    DialogWarning,
    DialogInformation,
    DialogQuestion,
    SoftwareUpdateAvailable,
    DeviceAdded,
    DeviceAddedAudio,
    DeviceAddedMedia,
    DeviceRemoved,
    DeviceRemovedMedia,
    DeviceRemovedAudio,
    WindowNew,
    PowerPlug,
    PowerUnplug,
    SuspendStart,
    SuspendResume,
    LidOpen,
    LidClose,
    AlarmClockElapsed,
    WindowAttentionActive,
    WindowAttentionInactive,
}

#[derive(Debug, EnumString, Display, Clone, Default)]
#[strum(serialize_all = "kebab-case")]
pub enum Action {
    #[default]
    #[strum(serialize = "")]
    None,
    PhoneOutgoingCalling,
    MessageSentInstant,
    MessageSentEmail,
    BellTerminal,
    BellWindowSystem,
    TrashEmpty,
    ItemDeleted,
    FileTrash,
    CameraShutter,
    CameraFocus,
    ScreenCapture,
    CountDown,
    CompletionSucess,
    CompletionFail,
    CompletionPartial,
    CompletionRotation,
    AudioVolumeChange,
    AudioChannelLeft,
    AudioChannelRight,
    AudioChannelFrontLeft,
    AudioChannelFrontRight,
    AudioChannelFrontCenter,
    AudioChannelRearLeft,
    AudioChannelRearRight,
    AudioChannelRearCenter,
    AudioChannelLfe,
    AudioChannelSideLeft,
    AudioChannelSideRight,
    AudioTestSignal,
    ThemeDemo,
}

#[derive(Debug, EnumString, Display, Clone, Default)]
#[strum(serialize_all = "kebab-case")]
pub enum InputFeedback {
    #[default]
    #[strum(serialize = "")]
    None,
    WindowClose,
    WindowSlideIn,
    WindowSlideOut,
    WindowMinimized,
    WindowUnminimized,
    WindowMaximized,
    WindowUnmaximized,
    WindowInactiveClick,
    WindowMoveStart,
    WindowMoveEnd,
    WindowResizeStart,
    WindowResizeEnd,
    DesktopSwitchLeft,
    DesktopSwitchRight,
    WindowSwitch,
    NotebookTabChanged,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    ScrollUpEnd,
    ScrollDownEnd,
    ScrollLeftEnd,
    ScrollRightEnd,
    DialogOk,
    DialogCancel,
    DragStart,
    DragAccept,
    DragFail,
    LinkPressed,
    LinkReleased,
    ButtonPressed,
    ButtonReleased,
    MenuClick,
    ButtonToggleOn,
    ButtonToggleOff,
    ExpanderToggleOn,
    ExpanderToggleOff,
    MenuPopup,
    MenuPopdown,
    MenuReplace,
    TooltipPopup,
    TooltipPopdown,
    ItemSelected,
}

#[derive(Debug, EnumString, Display, Clone, Default)]
#[strum(serialize_all = "kebab-case")]
pub enum Game {
    #[default]
    #[strum(serialize = "")]
    None,
    GameOverWinner,
    GameOverLoser,
    GameCardShuffle,
    GameHumanMove,
    GameComputerMove,
}
