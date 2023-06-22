use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Default)]
pub enum SoundContext {
    #[default]
    None,
    Alert(Alert),
    Notification(Notification),
    Action(Action),
    InputFeedback(InputFeedback),
    Game(Game),
}

impl ToString for SoundContext {
    fn to_string(&self) -> String {
        match self {
            SoundContext::None => "".to_string(),
            SoundContext::Alert(alert) => alert.to_string(),
            SoundContext::Notification(notification) => notification.to_string(),
            SoundContext::Action(action) => action.to_string(),
            SoundContext::InputFeedback(input_feedback) => input_feedback.to_string(),
            SoundContext::Game(game) => game.to_string(),
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
