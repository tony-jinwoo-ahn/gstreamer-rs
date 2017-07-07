// This file was generated by gir (33e9567) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib_ffi;
use glib::error::ErrorDomain;
use glib::translate::*;
use std;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BufferingMode {
    Stream,
    Download,
    Timeshift,
    Live,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for BufferingMode {
    type GlibType = ffi::GstBufferingMode;

    fn to_glib(&self) -> ffi::GstBufferingMode {
        match *self {
            BufferingMode::Stream => ffi::GST_BUFFERING_STREAM,
            BufferingMode::Download => ffi::GST_BUFFERING_DOWNLOAD,
            BufferingMode::Timeshift => ffi::GST_BUFFERING_TIMESHIFT,
            BufferingMode::Live => ffi::GST_BUFFERING_LIVE,
            BufferingMode::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstBufferingMode> for BufferingMode {
    fn from_glib(value: ffi::GstBufferingMode) -> Self {
        match value as i32 {
            0 => BufferingMode::Stream,
            1 => BufferingMode::Download,
            2 => BufferingMode::Timeshift,
            3 => BufferingMode::Live,
            value => BufferingMode::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CoreError {
    Failed,
    TooLazy,
    NotImplemented,
    StateChange,
    Pad,
    Thread,
    Negotiation,
    Event,
    Seek,
    Caps,
    Tag,
    MissingPlugin,
    Clock,
    Disabled,
    NumErrors,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for CoreError {
    type GlibType = ffi::GstCoreError;

    fn to_glib(&self) -> ffi::GstCoreError {
        match *self {
            CoreError::Failed => ffi::GST_CORE_ERROR_FAILED,
            CoreError::TooLazy => ffi::GST_CORE_ERROR_TOO_LAZY,
            CoreError::NotImplemented => ffi::GST_CORE_ERROR_NOT_IMPLEMENTED,
            CoreError::StateChange => ffi::GST_CORE_ERROR_STATE_CHANGE,
            CoreError::Pad => ffi::GST_CORE_ERROR_PAD,
            CoreError::Thread => ffi::GST_CORE_ERROR_THREAD,
            CoreError::Negotiation => ffi::GST_CORE_ERROR_NEGOTIATION,
            CoreError::Event => ffi::GST_CORE_ERROR_EVENT,
            CoreError::Seek => ffi::GST_CORE_ERROR_SEEK,
            CoreError::Caps => ffi::GST_CORE_ERROR_CAPS,
            CoreError::Tag => ffi::GST_CORE_ERROR_TAG,
            CoreError::MissingPlugin => ffi::GST_CORE_ERROR_MISSING_PLUGIN,
            CoreError::Clock => ffi::GST_CORE_ERROR_CLOCK,
            CoreError::Disabled => ffi::GST_CORE_ERROR_DISABLED,
            CoreError::NumErrors => ffi::GST_CORE_ERROR_NUM_ERRORS,
            CoreError::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstCoreError> for CoreError {
    fn from_glib(value: ffi::GstCoreError) -> Self {
        match value as i32 {
            1 => CoreError::Failed,
            2 => CoreError::TooLazy,
            3 => CoreError::NotImplemented,
            4 => CoreError::StateChange,
            5 => CoreError::Pad,
            6 => CoreError::Thread,
            7 => CoreError::Negotiation,
            8 => CoreError::Event,
            9 => CoreError::Seek,
            10 => CoreError::Caps,
            11 => CoreError::Tag,
            12 => CoreError::MissingPlugin,
            13 => CoreError::Clock,
            14 => CoreError::Disabled,
            15 => CoreError::NumErrors,
            value => CoreError::__Unknown(value),
        }
    }
}

impl ErrorDomain for CoreError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::gst_core_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            1 => Some(CoreError::Failed),
            2 => Some(CoreError::TooLazy),
            3 => Some(CoreError::NotImplemented),
            4 => Some(CoreError::StateChange),
            5 => Some(CoreError::Pad),
            6 => Some(CoreError::Thread),
            7 => Some(CoreError::Negotiation),
            8 => Some(CoreError::Event),
            9 => Some(CoreError::Seek),
            10 => Some(CoreError::Caps),
            11 => Some(CoreError::Tag),
            12 => Some(CoreError::MissingPlugin),
            13 => Some(CoreError::Clock),
            14 => Some(CoreError::Disabled),
            15 => Some(CoreError::NumErrors),
            _ => Some(CoreError::Failed),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FlowReturn {
    CustomSuccess2,
    CustomSuccess1,
    CustomSuccess,
    Ok,
    NotLinked,
    Flushing,
    Eos,
    NotNegotiated,
    Error,
    NotSupported,
    CustomError,
    CustomError1,
    CustomError2,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for FlowReturn {
    type GlibType = ffi::GstFlowReturn;

    fn to_glib(&self) -> ffi::GstFlowReturn {
        match *self {
            FlowReturn::CustomSuccess2 => ffi::GST_FLOW_CUSTOM_SUCCESS_2,
            FlowReturn::CustomSuccess1 => ffi::GST_FLOW_CUSTOM_SUCCESS_1,
            FlowReturn::CustomSuccess => ffi::GST_FLOW_CUSTOM_SUCCESS,
            FlowReturn::Ok => ffi::GST_FLOW_OK,
            FlowReturn::NotLinked => ffi::GST_FLOW_NOT_LINKED,
            FlowReturn::Flushing => ffi::GST_FLOW_FLUSHING,
            FlowReturn::Eos => ffi::GST_FLOW_EOS,
            FlowReturn::NotNegotiated => ffi::GST_FLOW_NOT_NEGOTIATED,
            FlowReturn::Error => ffi::GST_FLOW_ERROR,
            FlowReturn::NotSupported => ffi::GST_FLOW_NOT_SUPPORTED,
            FlowReturn::CustomError => ffi::GST_FLOW_CUSTOM_ERROR,
            FlowReturn::CustomError1 => ffi::GST_FLOW_CUSTOM_ERROR_1,
            FlowReturn::CustomError2 => ffi::GST_FLOW_CUSTOM_ERROR_2,
            FlowReturn::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstFlowReturn> for FlowReturn {
    fn from_glib(value: ffi::GstFlowReturn) -> Self {
        match value as i32 {
            102 => FlowReturn::CustomSuccess2,
            101 => FlowReturn::CustomSuccess1,
            100 => FlowReturn::CustomSuccess,
            0 => FlowReturn::Ok,
            -1 => FlowReturn::NotLinked,
            -2 => FlowReturn::Flushing,
            -3 => FlowReturn::Eos,
            -4 => FlowReturn::NotNegotiated,
            -5 => FlowReturn::Error,
            -6 => FlowReturn::NotSupported,
            -100 => FlowReturn::CustomError,
            -101 => FlowReturn::CustomError1,
            -102 => FlowReturn::CustomError2,
            value => FlowReturn::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Format {
    Undefined,
    Default,
    Bytes,
    Time,
    Buffers,
    Percent,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for Format {
    type GlibType = ffi::GstFormat;

    fn to_glib(&self) -> ffi::GstFormat {
        match *self {
            Format::Undefined => ffi::GST_FORMAT_UNDEFINED,
            Format::Default => ffi::GST_FORMAT_DEFAULT,
            Format::Bytes => ffi::GST_FORMAT_BYTES,
            Format::Time => ffi::GST_FORMAT_TIME,
            Format::Buffers => ffi::GST_FORMAT_BUFFERS,
            Format::Percent => ffi::GST_FORMAT_PERCENT,
            Format::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstFormat> for Format {
    fn from_glib(value: ffi::GstFormat) -> Self {
        match value as i32 {
            0 => Format::Undefined,
            1 => Format::Default,
            2 => Format::Bytes,
            3 => Format::Time,
            4 => Format::Buffers,
            5 => Format::Percent,
            value => Format::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum LibraryError {
    Failed,
    TooLazy,
    Init,
    Shutdown,
    Settings,
    Encode,
    NumErrors,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for LibraryError {
    type GlibType = ffi::GstLibraryError;

    fn to_glib(&self) -> ffi::GstLibraryError {
        match *self {
            LibraryError::Failed => ffi::GST_LIBRARY_ERROR_FAILED,
            LibraryError::TooLazy => ffi::GST_LIBRARY_ERROR_TOO_LAZY,
            LibraryError::Init => ffi::GST_LIBRARY_ERROR_INIT,
            LibraryError::Shutdown => ffi::GST_LIBRARY_ERROR_SHUTDOWN,
            LibraryError::Settings => ffi::GST_LIBRARY_ERROR_SETTINGS,
            LibraryError::Encode => ffi::GST_LIBRARY_ERROR_ENCODE,
            LibraryError::NumErrors => ffi::GST_LIBRARY_ERROR_NUM_ERRORS,
            LibraryError::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstLibraryError> for LibraryError {
    fn from_glib(value: ffi::GstLibraryError) -> Self {
        match value as i32 {
            1 => LibraryError::Failed,
            2 => LibraryError::TooLazy,
            3 => LibraryError::Init,
            4 => LibraryError::Shutdown,
            5 => LibraryError::Settings,
            6 => LibraryError::Encode,
            7 => LibraryError::NumErrors,
            value => LibraryError::__Unknown(value),
        }
    }
}

impl ErrorDomain for LibraryError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::gst_library_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            1 => Some(LibraryError::Failed),
            2 => Some(LibraryError::TooLazy),
            3 => Some(LibraryError::Init),
            4 => Some(LibraryError::Shutdown),
            5 => Some(LibraryError::Settings),
            6 => Some(LibraryError::Encode),
            7 => Some(LibraryError::NumErrors),
            _ => Some(LibraryError::Failed),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PadDirection {
    Unknown,
    Src,
    Sink,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PadDirection {
    type GlibType = ffi::GstPadDirection;

    fn to_glib(&self) -> ffi::GstPadDirection {
        match *self {
            PadDirection::Unknown => ffi::GST_PAD_UNKNOWN,
            PadDirection::Src => ffi::GST_PAD_SRC,
            PadDirection::Sink => ffi::GST_PAD_SINK,
            PadDirection::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPadDirection> for PadDirection {
    fn from_glib(value: ffi::GstPadDirection) -> Self {
        match value as i32 {
            0 => PadDirection::Unknown,
            1 => PadDirection::Src,
            2 => PadDirection::Sink,
            value => PadDirection::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PadLinkReturn {
    Ok,
    WrongHierarchy,
    WasLinked,
    WrongDirection,
    Noformat,
    Nosched,
    Refused,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PadLinkReturn {
    type GlibType = ffi::GstPadLinkReturn;

    fn to_glib(&self) -> ffi::GstPadLinkReturn {
        match *self {
            PadLinkReturn::Ok => ffi::GST_PAD_LINK_OK,
            PadLinkReturn::WrongHierarchy => ffi::GST_PAD_LINK_WRONG_HIERARCHY,
            PadLinkReturn::WasLinked => ffi::GST_PAD_LINK_WAS_LINKED,
            PadLinkReturn::WrongDirection => ffi::GST_PAD_LINK_WRONG_DIRECTION,
            PadLinkReturn::Noformat => ffi::GST_PAD_LINK_NOFORMAT,
            PadLinkReturn::Nosched => ffi::GST_PAD_LINK_NOSCHED,
            PadLinkReturn::Refused => ffi::GST_PAD_LINK_REFUSED,
            PadLinkReturn::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPadLinkReturn> for PadLinkReturn {
    fn from_glib(value: ffi::GstPadLinkReturn) -> Self {
        match value as i32 {
            0 => PadLinkReturn::Ok,
            -1 => PadLinkReturn::WrongHierarchy,
            -2 => PadLinkReturn::WasLinked,
            -3 => PadLinkReturn::WrongDirection,
            -4 => PadLinkReturn::Noformat,
            -5 => PadLinkReturn::Nosched,
            -6 => PadLinkReturn::Refused,
            value => PadLinkReturn::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ParseError {
    Syntax,
    NoSuchElement,
    NoSuchProperty,
    Link,
    CouldNotSetProperty,
    EmptyBin,
    Empty,
    DelayedLink,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ParseError {
    type GlibType = ffi::GstParseError;

    fn to_glib(&self) -> ffi::GstParseError {
        match *self {
            ParseError::Syntax => ffi::GST_PARSE_ERROR_SYNTAX,
            ParseError::NoSuchElement => ffi::GST_PARSE_ERROR_NO_SUCH_ELEMENT,
            ParseError::NoSuchProperty => ffi::GST_PARSE_ERROR_NO_SUCH_PROPERTY,
            ParseError::Link => ffi::GST_PARSE_ERROR_LINK,
            ParseError::CouldNotSetProperty => ffi::GST_PARSE_ERROR_COULD_NOT_SET_PROPERTY,
            ParseError::EmptyBin => ffi::GST_PARSE_ERROR_EMPTY_BIN,
            ParseError::Empty => ffi::GST_PARSE_ERROR_EMPTY,
            ParseError::DelayedLink => ffi::GST_PARSE_ERROR_DELAYED_LINK,
            ParseError::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstParseError> for ParseError {
    fn from_glib(value: ffi::GstParseError) -> Self {
        match value as i32 {
            0 => ParseError::Syntax,
            1 => ParseError::NoSuchElement,
            2 => ParseError::NoSuchProperty,
            3 => ParseError::Link,
            4 => ParseError::CouldNotSetProperty,
            5 => ParseError::EmptyBin,
            6 => ParseError::Empty,
            7 => ParseError::DelayedLink,
            value => ParseError::__Unknown(value),
        }
    }
}

impl ErrorDomain for ParseError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::gst_parse_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(ParseError::Syntax),
            1 => Some(ParseError::NoSuchElement),
            2 => Some(ParseError::NoSuchProperty),
            3 => Some(ParseError::Link),
            4 => Some(ParseError::CouldNotSetProperty),
            5 => Some(ParseError::EmptyBin),
            6 => Some(ParseError::Empty),
            7 => Some(ParseError::DelayedLink),
            value => Some(ParseError::__Unknown(value)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PluginError {
    Module,
    Dependencies,
    NameMismatch,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PluginError {
    type GlibType = ffi::GstPluginError;

    fn to_glib(&self) -> ffi::GstPluginError {
        match *self {
            PluginError::Module => ffi::GST_PLUGIN_ERROR_MODULE,
            PluginError::Dependencies => ffi::GST_PLUGIN_ERROR_DEPENDENCIES,
            PluginError::NameMismatch => ffi::GST_PLUGIN_ERROR_NAME_MISMATCH,
            PluginError::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPluginError> for PluginError {
    fn from_glib(value: ffi::GstPluginError) -> Self {
        match value as i32 {
            0 => PluginError::Module,
            1 => PluginError::Dependencies,
            2 => PluginError::NameMismatch,
            value => PluginError::__Unknown(value),
        }
    }
}

impl ErrorDomain for PluginError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::gst_plugin_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(PluginError::Module),
            1 => Some(PluginError::Dependencies),
            2 => Some(PluginError::NameMismatch),
            value => Some(PluginError::__Unknown(value)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ProgressType {
    Start,
    Continue,
    Complete,
    Canceled,
    Error,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ProgressType {
    type GlibType = ffi::GstProgressType;

    fn to_glib(&self) -> ffi::GstProgressType {
        match *self {
            ProgressType::Start => ffi::GST_PROGRESS_TYPE_START,
            ProgressType::Continue => ffi::GST_PROGRESS_TYPE_CONTINUE,
            ProgressType::Complete => ffi::GST_PROGRESS_TYPE_COMPLETE,
            ProgressType::Canceled => ffi::GST_PROGRESS_TYPE_CANCELED,
            ProgressType::Error => ffi::GST_PROGRESS_TYPE_ERROR,
            ProgressType::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstProgressType> for ProgressType {
    fn from_glib(value: ffi::GstProgressType) -> Self {
        match value as i32 {
            0 => ProgressType::Start,
            1 => ProgressType::Continue,
            2 => ProgressType::Complete,
            3 => ProgressType::Canceled,
            4 => ProgressType::Error,
            value => ProgressType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ResourceError {
    Failed,
    TooLazy,
    NotFound,
    Busy,
    OpenRead,
    OpenWrite,
    OpenReadWrite,
    Close,
    Read,
    Write,
    Seek,
    Sync,
    Settings,
    NoSpaceLeft,
    NotAuthorized,
    NumErrors,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ResourceError {
    type GlibType = ffi::GstResourceError;

    fn to_glib(&self) -> ffi::GstResourceError {
        match *self {
            ResourceError::Failed => ffi::GST_RESOURCE_ERROR_FAILED,
            ResourceError::TooLazy => ffi::GST_RESOURCE_ERROR_TOO_LAZY,
            ResourceError::NotFound => ffi::GST_RESOURCE_ERROR_NOT_FOUND,
            ResourceError::Busy => ffi::GST_RESOURCE_ERROR_BUSY,
            ResourceError::OpenRead => ffi::GST_RESOURCE_ERROR_OPEN_READ,
            ResourceError::OpenWrite => ffi::GST_RESOURCE_ERROR_OPEN_WRITE,
            ResourceError::OpenReadWrite => ffi::GST_RESOURCE_ERROR_OPEN_READ_WRITE,
            ResourceError::Close => ffi::GST_RESOURCE_ERROR_CLOSE,
            ResourceError::Read => ffi::GST_RESOURCE_ERROR_READ,
            ResourceError::Write => ffi::GST_RESOURCE_ERROR_WRITE,
            ResourceError::Seek => ffi::GST_RESOURCE_ERROR_SEEK,
            ResourceError::Sync => ffi::GST_RESOURCE_ERROR_SYNC,
            ResourceError::Settings => ffi::GST_RESOURCE_ERROR_SETTINGS,
            ResourceError::NoSpaceLeft => ffi::GST_RESOURCE_ERROR_NO_SPACE_LEFT,
            ResourceError::NotAuthorized => ffi::GST_RESOURCE_ERROR_NOT_AUTHORIZED,
            ResourceError::NumErrors => ffi::GST_RESOURCE_ERROR_NUM_ERRORS,
            ResourceError::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstResourceError> for ResourceError {
    fn from_glib(value: ffi::GstResourceError) -> Self {
        match value as i32 {
            1 => ResourceError::Failed,
            2 => ResourceError::TooLazy,
            3 => ResourceError::NotFound,
            4 => ResourceError::Busy,
            5 => ResourceError::OpenRead,
            6 => ResourceError::OpenWrite,
            7 => ResourceError::OpenReadWrite,
            8 => ResourceError::Close,
            9 => ResourceError::Read,
            10 => ResourceError::Write,
            11 => ResourceError::Seek,
            12 => ResourceError::Sync,
            13 => ResourceError::Settings,
            14 => ResourceError::NoSpaceLeft,
            15 => ResourceError::NotAuthorized,
            16 => ResourceError::NumErrors,
            value => ResourceError::__Unknown(value),
        }
    }
}

impl ErrorDomain for ResourceError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::gst_resource_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            1 => Some(ResourceError::Failed),
            2 => Some(ResourceError::TooLazy),
            3 => Some(ResourceError::NotFound),
            4 => Some(ResourceError::Busy),
            5 => Some(ResourceError::OpenRead),
            6 => Some(ResourceError::OpenWrite),
            7 => Some(ResourceError::OpenReadWrite),
            8 => Some(ResourceError::Close),
            9 => Some(ResourceError::Read),
            10 => Some(ResourceError::Write),
            11 => Some(ResourceError::Seek),
            12 => Some(ResourceError::Sync),
            13 => Some(ResourceError::Settings),
            14 => Some(ResourceError::NoSpaceLeft),
            15 => Some(ResourceError::NotAuthorized),
            16 => Some(ResourceError::NumErrors),
            _ => Some(ResourceError::Failed),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SeekType {
    None,
    Set,
    End,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for SeekType {
    type GlibType = ffi::GstSeekType;

    fn to_glib(&self) -> ffi::GstSeekType {
        match *self {
            SeekType::None => ffi::GST_SEEK_TYPE_NONE,
            SeekType::Set => ffi::GST_SEEK_TYPE_SET,
            SeekType::End => ffi::GST_SEEK_TYPE_END,
            SeekType::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstSeekType> for SeekType {
    fn from_glib(value: ffi::GstSeekType) -> Self {
        match value as i32 {
            0 => SeekType::None,
            1 => SeekType::Set,
            2 => SeekType::End,
            value => SeekType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum State {
    VoidPending,
    Null,
    Ready,
    Paused,
    Playing,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for State {
    type GlibType = ffi::GstState;

    fn to_glib(&self) -> ffi::GstState {
        match *self {
            State::VoidPending => ffi::GST_STATE_VOID_PENDING,
            State::Null => ffi::GST_STATE_NULL,
            State::Ready => ffi::GST_STATE_READY,
            State::Paused => ffi::GST_STATE_PAUSED,
            State::Playing => ffi::GST_STATE_PLAYING,
            State::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstState> for State {
    fn from_glib(value: ffi::GstState) -> Self {
        match value as i32 {
            0 => State::VoidPending,
            1 => State::Null,
            2 => State::Ready,
            3 => State::Paused,
            4 => State::Playing,
            value => State::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StateChange {
    NullToReady,
    ReadyToPaused,
    PausedToPlaying,
    PlayingToPaused,
    PausedToReady,
    ReadyToNull,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for StateChange {
    type GlibType = ffi::GstStateChange;

    fn to_glib(&self) -> ffi::GstStateChange {
        match *self {
            StateChange::NullToReady => ffi::GST_STATE_CHANGE_NULL_TO_READY,
            StateChange::ReadyToPaused => ffi::GST_STATE_CHANGE_READY_TO_PAUSED,
            StateChange::PausedToPlaying => ffi::GST_STATE_CHANGE_PAUSED_TO_PLAYING,
            StateChange::PlayingToPaused => ffi::GST_STATE_CHANGE_PLAYING_TO_PAUSED,
            StateChange::PausedToReady => ffi::GST_STATE_CHANGE_PAUSED_TO_READY,
            StateChange::ReadyToNull => ffi::GST_STATE_CHANGE_READY_TO_NULL,
            StateChange::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstStateChange> for StateChange {
    fn from_glib(value: ffi::GstStateChange) -> Self {
        match value as i32 {
            10 => StateChange::NullToReady,
            19 => StateChange::ReadyToPaused,
            28 => StateChange::PausedToPlaying,
            35 => StateChange::PlayingToPaused,
            26 => StateChange::PausedToReady,
            17 => StateChange::ReadyToNull,
            value => StateChange::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StateChangeReturn {
    Failure,
    Success,
    Async,
    NoPreroll,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for StateChangeReturn {
    type GlibType = ffi::GstStateChangeReturn;

    fn to_glib(&self) -> ffi::GstStateChangeReturn {
        match *self {
            StateChangeReturn::Failure => ffi::GST_STATE_CHANGE_FAILURE,
            StateChangeReturn::Success => ffi::GST_STATE_CHANGE_SUCCESS,
            StateChangeReturn::Async => ffi::GST_STATE_CHANGE_ASYNC,
            StateChangeReturn::NoPreroll => ffi::GST_STATE_CHANGE_NO_PREROLL,
            StateChangeReturn::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstStateChangeReturn> for StateChangeReturn {
    fn from_glib(value: ffi::GstStateChangeReturn) -> Self {
        match value as i32 {
            0 => StateChangeReturn::Failure,
            1 => StateChangeReturn::Success,
            2 => StateChangeReturn::Async,
            3 => StateChangeReturn::NoPreroll,
            value => StateChangeReturn::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StreamError {
    Failed,
    TooLazy,
    NotImplemented,
    TypeNotFound,
    WrongType,
    CodecNotFound,
    Decode,
    Encode,
    Demux,
    Mux,
    Format,
    Decrypt,
    DecryptNokey,
    NumErrors,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for StreamError {
    type GlibType = ffi::GstStreamError;

    fn to_glib(&self) -> ffi::GstStreamError {
        match *self {
            StreamError::Failed => ffi::GST_STREAM_ERROR_FAILED,
            StreamError::TooLazy => ffi::GST_STREAM_ERROR_TOO_LAZY,
            StreamError::NotImplemented => ffi::GST_STREAM_ERROR_NOT_IMPLEMENTED,
            StreamError::TypeNotFound => ffi::GST_STREAM_ERROR_TYPE_NOT_FOUND,
            StreamError::WrongType => ffi::GST_STREAM_ERROR_WRONG_TYPE,
            StreamError::CodecNotFound => ffi::GST_STREAM_ERROR_CODEC_NOT_FOUND,
            StreamError::Decode => ffi::GST_STREAM_ERROR_DECODE,
            StreamError::Encode => ffi::GST_STREAM_ERROR_ENCODE,
            StreamError::Demux => ffi::GST_STREAM_ERROR_DEMUX,
            StreamError::Mux => ffi::GST_STREAM_ERROR_MUX,
            StreamError::Format => ffi::GST_STREAM_ERROR_FORMAT,
            StreamError::Decrypt => ffi::GST_STREAM_ERROR_DECRYPT,
            StreamError::DecryptNokey => ffi::GST_STREAM_ERROR_DECRYPT_NOKEY,
            StreamError::NumErrors => ffi::GST_STREAM_ERROR_NUM_ERRORS,
            StreamError::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstStreamError> for StreamError {
    fn from_glib(value: ffi::GstStreamError) -> Self {
        match value as i32 {
            1 => StreamError::Failed,
            2 => StreamError::TooLazy,
            3 => StreamError::NotImplemented,
            4 => StreamError::TypeNotFound,
            5 => StreamError::WrongType,
            6 => StreamError::CodecNotFound,
            7 => StreamError::Decode,
            8 => StreamError::Encode,
            9 => StreamError::Demux,
            10 => StreamError::Mux,
            11 => StreamError::Format,
            12 => StreamError::Decrypt,
            13 => StreamError::DecryptNokey,
            14 => StreamError::NumErrors,
            value => StreamError::__Unknown(value),
        }
    }
}

impl ErrorDomain for StreamError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::gst_stream_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            1 => Some(StreamError::Failed),
            2 => Some(StreamError::TooLazy),
            3 => Some(StreamError::NotImplemented),
            4 => Some(StreamError::TypeNotFound),
            5 => Some(StreamError::WrongType),
            6 => Some(StreamError::CodecNotFound),
            7 => Some(StreamError::Decode),
            8 => Some(StreamError::Encode),
            9 => Some(StreamError::Demux),
            10 => Some(StreamError::Mux),
            11 => Some(StreamError::Format),
            12 => Some(StreamError::Decrypt),
            13 => Some(StreamError::DecryptNokey),
            14 => Some(StreamError::NumErrors),
            _ => Some(StreamError::Failed),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StreamStatusType {
    Create,
    Enter,
    Leave,
    Destroy,
    Start,
    Pause,
    Stop,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for StreamStatusType {
    type GlibType = ffi::GstStreamStatusType;

    fn to_glib(&self) -> ffi::GstStreamStatusType {
        match *self {
            StreamStatusType::Create => ffi::GST_STREAM_STATUS_TYPE_CREATE,
            StreamStatusType::Enter => ffi::GST_STREAM_STATUS_TYPE_ENTER,
            StreamStatusType::Leave => ffi::GST_STREAM_STATUS_TYPE_LEAVE,
            StreamStatusType::Destroy => ffi::GST_STREAM_STATUS_TYPE_DESTROY,
            StreamStatusType::Start => ffi::GST_STREAM_STATUS_TYPE_START,
            StreamStatusType::Pause => ffi::GST_STREAM_STATUS_TYPE_PAUSE,
            StreamStatusType::Stop => ffi::GST_STREAM_STATUS_TYPE_STOP,
            StreamStatusType::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstStreamStatusType> for StreamStatusType {
    fn from_glib(value: ffi::GstStreamStatusType) -> Self {
        match value as i32 {
            0 => StreamStatusType::Create,
            1 => StreamStatusType::Enter,
            2 => StreamStatusType::Leave,
            3 => StreamStatusType::Destroy,
            8 => StreamStatusType::Start,
            9 => StreamStatusType::Pause,
            10 => StreamStatusType::Stop,
            value => StreamStatusType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StructureChangeType {
    Link,
    Unlink,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for StructureChangeType {
    type GlibType = ffi::GstStructureChangeType;

    fn to_glib(&self) -> ffi::GstStructureChangeType {
        match *self {
            StructureChangeType::Link => ffi::GST_STRUCTURE_CHANGE_TYPE_PAD_LINK,
            StructureChangeType::Unlink => ffi::GST_STRUCTURE_CHANGE_TYPE_PAD_UNLINK,
            StructureChangeType::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstStructureChangeType> for StructureChangeType {
    fn from_glib(value: ffi::GstStructureChangeType) -> Self {
        match value as i32 {
            0 => StructureChangeType::Link,
            1 => StructureChangeType::Unlink,
            value => StructureChangeType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum URIError {
    UnsupportedProtocol,
    BadUri,
    BadState,
    BadReference,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for URIError {
    type GlibType = ffi::GstURIError;

    fn to_glib(&self) -> ffi::GstURIError {
        match *self {
            URIError::UnsupportedProtocol => ffi::GST_URI_ERROR_UNSUPPORTED_PROTOCOL,
            URIError::BadUri => ffi::GST_URI_ERROR_BAD_URI,
            URIError::BadState => ffi::GST_URI_ERROR_BAD_STATE,
            URIError::BadReference => ffi::GST_URI_ERROR_BAD_REFERENCE,
            URIError::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstURIError> for URIError {
    fn from_glib(value: ffi::GstURIError) -> Self {
        match value as i32 {
            0 => URIError::UnsupportedProtocol,
            1 => URIError::BadUri,
            2 => URIError::BadState,
            3 => URIError::BadReference,
            value => URIError::__Unknown(value),
        }
    }
}

impl ErrorDomain for URIError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::gst_uri_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(URIError::UnsupportedProtocol),
            1 => Some(URIError::BadUri),
            2 => Some(URIError::BadState),
            3 => Some(URIError::BadReference),
            value => Some(URIError::__Unknown(value)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum URIType {
    Unknown,
    Sink,
    Src,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for URIType {
    type GlibType = ffi::GstURIType;

    fn to_glib(&self) -> ffi::GstURIType {
        match *self {
            URIType::Unknown => ffi::GST_URI_UNKNOWN,
            URIType::Sink => ffi::GST_URI_SINK,
            URIType::Src => ffi::GST_URI_SRC,
            URIType::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstURIType> for URIType {
    fn from_glib(value: ffi::GstURIType) -> Self {
        match value as i32 {
            0 => URIType::Unknown,
            1 => URIType::Sink,
            2 => URIType::Src,
            value => URIType::__Unknown(value),
        }
    }
}

