use libsrt_sys as srt;

use std::{
    borrow::Cow,
    convert::From,
    error::Error,
    fmt::{self, Display, Formatter},
    io::{self, ErrorKind},
    os::raw::c_int,
};

#[derive(Clone, Copy, Debug)]
pub enum SrtError {
    Unknown,
    Success,
    ConnSetup,
    NoServer,
    ConnRej(SrtRejectReason),
    SockFail,
    SecFail,
    Closed,
    ConnFail,
    ConnLost,
    NoConn,
    Resource,
    Thread,
    NoBuf,
    SysObj,
    File,
    InvRdOff,
    RdPerm,
    InvWrOff,
    WrPerm,
    InvOp,
    BoundSock,
    ConnSock,
    InvParam,
    InvSock,
    UnboundSock,
    NoListen,
    RdvNoServ,
    RdvUnbound,
    InvalMsgApi,
    InvalBufferApi,
    DupListen,
    LargeMsg,
    InvPollId,
    PollEmpty,
    AsyncFail,
    AsyncSnd,
    AsyncRcv,
    Timeout,
    Congest,
    PeerErr,
    BindConflict,
}

impl Display for SrtError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", error_msg(self))
    }
}

impl Error for SrtError {}

pub fn handle_result<T>(ok: T, return_code: i32) -> Result<T, SrtError> {
    match return_code {
        0 => Ok(ok),
        -1 => {
            let mut _errno_loc = 0;
            let err_no = unsafe { srt::srt_getlasterror(&mut _errno_loc as *mut c_int) };
            let err = srt::SRT_ERRNO(err_no);
            match SrtError::from(err) {
                SrtError::Success => Ok(ok),
                e => Err(e),
            }
        }
        e => unreachable!("unrecognized return code {}", e),
    }
}

impl From<SrtError> for io::Error {
    fn from(e: SrtError) -> Self {
        io::Error::new(
            match e {
                SrtError::Unknown => ErrorKind::Other,
                SrtError::Success => ErrorKind::Other,
                SrtError::ConnSetup => ErrorKind::ConnectionRefused,
                SrtError::NoServer => ErrorKind::ConnectionRefused,
                SrtError::ConnRej(_) => ErrorKind::ConnectionRefused,
                SrtError::SockFail => ErrorKind::AddrNotAvailable,
                SrtError::SecFail => ErrorKind::ConnectionRefused,
                SrtError::ConnFail => ErrorKind::ConnectionRefused,
                SrtError::Closed => ErrorKind::AddrNotAvailable,
                SrtError::ConnLost => ErrorKind::ConnectionAborted,
                SrtError::NoConn => ErrorKind::NotConnected,
                SrtError::Resource => ErrorKind::Other,
                SrtError::Thread => ErrorKind::Other,
                SrtError::NoBuf => ErrorKind::Other,
                SrtError::SysObj => ErrorKind::Other,
                SrtError::File => ErrorKind::NotFound,
                SrtError::InvRdOff => ErrorKind::InvalidInput,
                SrtError::RdPerm => ErrorKind::PermissionDenied,
                SrtError::InvWrOff => ErrorKind::InvalidInput,
                SrtError::WrPerm => ErrorKind::PermissionDenied,
                SrtError::InvOp => ErrorKind::InvalidInput,
                SrtError::BoundSock => ErrorKind::AddrInUse,
                SrtError::ConnSock => ErrorKind::AddrInUse,
                SrtError::InvParam => ErrorKind::InvalidInput,
                SrtError::InvSock => ErrorKind::AddrNotAvailable,
                SrtError::UnboundSock => ErrorKind::NotConnected,
                SrtError::NoListen => ErrorKind::InvalidInput,
                SrtError::RdvNoServ => ErrorKind::ConnectionRefused,
                SrtError::RdvUnbound => ErrorKind::ConnectionRefused,
                SrtError::InvalMsgApi => ErrorKind::InvalidInput,
                SrtError::InvalBufferApi => ErrorKind::InvalidInput,
                SrtError::DupListen => ErrorKind::AddrInUse,
                SrtError::BindConflict => ErrorKind::AddrInUse,
                SrtError::LargeMsg => ErrorKind::Other,
                SrtError::InvPollId => ErrorKind::AddrNotAvailable,
                SrtError::PollEmpty => ErrorKind::Other,
                SrtError::AsyncFail => ErrorKind::WouldBlock,
                SrtError::AsyncSnd => ErrorKind::WouldBlock,
                SrtError::AsyncRcv => ErrorKind::WouldBlock,
                SrtError::Timeout => ErrorKind::TimedOut,
                SrtError::Congest => ErrorKind::Other,
                SrtError::PeerErr => ErrorKind::Other,
            },
            e,
        )
    }
}

impl From<srt::SRT_ERRNO> for SrtError {
    fn from(err_no: srt::SRT_ERRNO) -> Self {
        match err_no {
            srt::SRT_ERRNO::SRT_EUNKNOWN => SrtError::Unknown,
            srt::SRT_ERRNO::SRT_SUCCESS => SrtError::Success,
            srt::SRT_ERRNO::SRT_ECONNSETUP => SrtError::ConnSetup,
            srt::SRT_ERRNO::SRT_ENOSERVER => SrtError::NoServer,
            srt::SRT_ERRNO::SRT_ECONNREJ => SrtError::ConnRej(SrtRejectReason::Unknown),
            srt::SRT_ERRNO::SRT_ESOCKFAIL => SrtError::SockFail,
            srt::SRT_ERRNO::SRT_ESECFAIL => SrtError::SecFail,
            srt::SRT_ERRNO::SRT_ESCLOSED => SrtError::Closed,
            srt::SRT_ERRNO::SRT_ECONNFAIL => SrtError::ConnFail,
            srt::SRT_ERRNO::SRT_ECONNLOST => SrtError::ConnLost,
            srt::SRT_ERRNO::SRT_ENOCONN => SrtError::NoConn,
            srt::SRT_ERRNO::SRT_ERESOURCE => SrtError::Resource,
            srt::SRT_ERRNO::SRT_ETHREAD => SrtError::Thread,
            srt::SRT_ERRNO::SRT_ENOBUF => SrtError::NoBuf,
            srt::SRT_ERRNO::SRT_ESYSOBJ => SrtError::SysObj,
            srt::SRT_ERRNO::SRT_EFILE => SrtError::File,
            srt::SRT_ERRNO::SRT_EINVRDOFF => SrtError::InvRdOff,
            srt::SRT_ERRNO::SRT_ERDPERM => SrtError::RdPerm,
            srt::SRT_ERRNO::SRT_EINVWROFF => SrtError::InvWrOff,
            srt::SRT_ERRNO::SRT_EWRPERM => SrtError::WrPerm,
            srt::SRT_ERRNO::SRT_EINVOP => SrtError::InvOp,
            srt::SRT_ERRNO::SRT_EBOUNDSOCK => SrtError::BoundSock,
            srt::SRT_ERRNO::SRT_ECONNSOCK => SrtError::ConnSock,
            srt::SRT_ERRNO::SRT_EINVPARAM => SrtError::InvParam,
            srt::SRT_ERRNO::SRT_EINVSOCK => SrtError::InvSock,
            srt::SRT_ERRNO::SRT_EUNBOUNDSOCK => SrtError::UnboundSock,
            srt::SRT_ERRNO::SRT_ENOLISTEN => SrtError::NoListen,
            srt::SRT_ERRNO::SRT_ERDVNOSERV => SrtError::RdvNoServ,
            srt::SRT_ERRNO::SRT_ERDVUNBOUND => SrtError::RdvUnbound,
            srt::SRT_ERRNO::SRT_EINVALMSGAPI => SrtError::InvalMsgApi,
            srt::SRT_ERRNO::SRT_EINVALBUFFERAPI => SrtError::InvalBufferApi,
            srt::SRT_ERRNO::SRT_EDUPLISTEN => SrtError::DupListen,
            srt::SRT_ERRNO::SRT_ELARGEMSG => SrtError::LargeMsg,
            srt::SRT_ERRNO::SRT_EINVPOLLID => SrtError::InvPollId,
            srt::SRT_ERRNO::SRT_EPOLLEMPTY => SrtError::PollEmpty,
            srt::SRT_ERRNO::SRT_EASYNCFAIL => SrtError::AsyncFail,
            srt::SRT_ERRNO::SRT_EASYNCSND => SrtError::AsyncSnd,
            srt::SRT_ERRNO::SRT_EASYNCRCV => SrtError::AsyncRcv,
            srt::SRT_ERRNO::SRT_ETIMEOUT => SrtError::Timeout,
            srt::SRT_ERRNO::SRT_ECONGEST => SrtError::Congest,
            srt::SRT_ERRNO::SRT_EPEERERR => SrtError::PeerErr,
            srt::SRT_ERRNO::SRT_EBINDCONFLICT => SrtError::BindConflict,
            _ => SrtError::Unknown,
        }
    }
}

fn error_msg(err: &SrtError) -> Cow<'static, str> {
    match err {
        SrtError::Unknown => Cow::Borrowed("Internal error when setting the right error code"),
        SrtError::Success => Cow::Borrowed("The value set when the last error was cleared and no error has occurred since then"),
        SrtError::ConnSetup => Cow::Borrowed("General setup error resulting from internal system state"),
        SrtError::NoServer => Cow::Borrowed("Connection timed out while attempting to connect to the remote address"),
        SrtError::ConnRej(reason) => Cow::Owned(format!("Connection has been rejected: {:?}", reason)),
        SrtError::SockFail => Cow::Borrowed("An error occurred when trying to call a system function on an internally used UDP socket"),
        SrtError::SecFail => Cow::Borrowed("A possible tampering with the handshake packets was detected, or encryption request wasn't properly fulfilled."),
        SrtError::Closed => Cow::Borrowed("A socket that was vital for an operation called in blocking mode has been closed during the operation"),
        SrtError::ConnFail => Cow::Borrowed("General connection failure of unknown details"),
        SrtError::ConnLost => Cow::Borrowed("The socket was properly connected, but the connection has been broken"),
        SrtError::NoConn => Cow::Borrowed("The socket is not connected"),
        SrtError::Resource => Cow::Borrowed("System or standard library error reported unexpectedly for unknown purpose"),
        SrtError::Thread => Cow::Borrowed("System was unable to spawn a new thread when required"),
        SrtError::NoBuf => Cow::Borrowed("System was unable to allocate memory for buffers"),
        SrtError::SysObj => Cow::Borrowed("System was unable to allocate system specific objects"),
        SrtError::File => Cow::Borrowed("General filesystem error (for functions operating with file transmission)"),
        SrtError::InvRdOff => Cow::Borrowed("Failure when trying to read from a given position in the file"),
        SrtError::RdPerm => Cow::Borrowed("Read permission was denied when trying to read from file"),
        SrtError::InvWrOff => Cow::Borrowed("Failed to set position in the written file"),
        SrtError::WrPerm => Cow::Borrowed("Write permission was denied when trying to write to a file"),
        SrtError::InvOp => Cow::Borrowed("Invalid operation performed for the current state of a socket"),
        SrtError::BoundSock => Cow::Borrowed("The socket is currently bound and the required operation cannot be performed in this state"),
        SrtError::ConnSock => Cow::Borrowed("The socket is currently connected and therefore performing the required operation is not possible"),
        SrtError::InvParam => Cow::Borrowed("Call parameters for API functions have some requirements that were not satisfied"),
        SrtError::InvSock => Cow::Borrowed("The API function required an ID of an entity (socket or group) and it was invalid"),
        SrtError::UnboundSock => Cow::Borrowed("The operation to be performed on a socket requires that it first be explicitly bound"),
        SrtError::NoListen => Cow::Borrowed("The socket passed for the operation is required to be in the listen state"),
        SrtError::RdvNoServ => Cow::Borrowed("The required operation cannot be performed when the socket is set to rendezvous mode"),
        SrtError::RdvUnbound => Cow::Borrowed("An attempt was made to connect to a socket set to rendezvous mode that was not first bound"),
        SrtError::InvalMsgApi => Cow::Borrowed("The function was used incorrectly in the message API"),
        SrtError::InvalBufferApi => Cow::Borrowed("The function was used incorrectly in the stream (buffer) API"),
        SrtError::DupListen => Cow::Borrowed("The port tried to be bound for listening is already busy"),
        SrtError::BindConflict => Cow::Borrowed("Binding specification conflicts with existing one"),
        SrtError::LargeMsg => Cow::Borrowed("Size exceeded"),
        SrtError::InvPollId => Cow::Borrowed("The epoll ID passed to an epoll function is invalid"),
        SrtError::PollEmpty => Cow::Borrowed("The epoll container currently has no subscribed sockets"),
        SrtError::AsyncFail => Cow::Borrowed("General asynchronous failure (not in use currently)"),
        SrtError::AsyncSnd => Cow::Borrowed("Sending operation is not ready to perform"),
        SrtError::AsyncRcv => Cow::Borrowed("Receiving operation is not ready to perform"),
        SrtError::Timeout => Cow::Borrowed("The operation timed out"),
        SrtError::Congest => Cow::Borrowed("With SRTO_TSBPDMODE and SRTO_TLPKTDROP set to true, some packets were dropped by sender"),
        SrtError::PeerErr => Cow::Borrowed("Receiver peer is writing to a file that the agent is sending"),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SrtRejectReason {
    Unknown,         // initial set when in progress
    System,          // broken due to system function error
    Peer,            // connection was rejected by peer
    Resource,        // internal problem with resource allocation
    Rogue,           // incorrect data in handshake messages
    Backlog,         // listener's backlog exceeded
    IPE,             // internal program error
    Close,           // socket is closing
    Version,         // peer is older version than agent's minimum set
    RdvCookie,       // rendezvous cookie collision
    BadSecret,       // wrong password
    Unsecure,        // password required or unexpected
    MessageAPI,      // streamapi/messageapi collision
    Congestion,      // incompatible congestion-controller type
    Filter,          // incompatible packet filter
    Group,           // incompatible group
    Timeout,         // connection timeout
    Predefined(u32), // adopted HTTP codes
    UserDefined(u32) // freely defined by application
}

impl SrtRejectReason {
    pub fn raw(&self) -> u32 {
        match self {
            SrtRejectReason::Unknown => srt::SRT_REJECT_REASON::SRT_REJ_UNKNOWN.0,
            SrtRejectReason::System => srt::SRT_REJECT_REASON::SRT_REJ_SYSTEM.0,
            SrtRejectReason::Peer => srt::SRT_REJECT_REASON::SRT_REJ_PEER.0,
            SrtRejectReason::Resource => srt::SRT_REJECT_REASON::SRT_REJ_RESOURCE.0,
            SrtRejectReason::Rogue => srt::SRT_REJECT_REASON::SRT_REJ_ROGUE.0,
            SrtRejectReason::Backlog => srt::SRT_REJECT_REASON::SRT_REJ_BACKLOG.0,
            SrtRejectReason::IPE => srt::SRT_REJECT_REASON::SRT_REJ_IPE.0,
            SrtRejectReason::Close => srt::SRT_REJECT_REASON::SRT_REJ_CLOSE.0,
            SrtRejectReason::Version => srt::SRT_REJECT_REASON::SRT_REJ_VERSION.0,
            SrtRejectReason::RdvCookie => srt::SRT_REJECT_REASON::SRT_REJ_RDVCOOKIE.0,
            SrtRejectReason::BadSecret => srt::SRT_REJECT_REASON::SRT_REJ_BADSECRET.0,
            SrtRejectReason::Unsecure => srt::SRT_REJECT_REASON::SRT_REJ_UNSECURE.0,
            SrtRejectReason::MessageAPI => srt::SRT_REJECT_REASON::SRT_REJ_MESSAGEAPI.0,
            SrtRejectReason::Congestion => srt::SRT_REJECT_REASON::SRT_REJ_CONGESTION.0,
            SrtRejectReason::Filter => srt::SRT_REJECT_REASON::SRT_REJ_FILTER.0,
            SrtRejectReason::Group => srt::SRT_REJECT_REASON::SRT_REJ_GROUP.0,
            SrtRejectReason::Timeout => srt::SRT_REJECT_REASON::SRT_REJ_TIMEOUT.0,
            SrtRejectReason::Predefined(x) => srt::SRT_REJC_PREDEFINED + x,
            SrtRejectReason::UserDefined(x) => srt::SRT_REJC_USERDEFINED + 1 + x
        }
    }
}

impl From<srt::SRT_REJECT_REASON> for SrtRejectReason {
    fn from(reject_reason: srt::SRT_REJECT_REASON) -> Self {
        match reject_reason {
            srt::SRT_REJECT_REASON::SRT_REJ_UNKNOWN => SrtRejectReason::Unknown, // initial set when in progress
            srt::SRT_REJECT_REASON::SRT_REJ_SYSTEM => SrtRejectReason::System,
            srt::SRT_REJECT_REASON::SRT_REJ_PEER => SrtRejectReason::Peer,
            srt::SRT_REJECT_REASON::SRT_REJ_RESOURCE => SrtRejectReason::Resource,
            srt::SRT_REJECT_REASON::SRT_REJ_ROGUE => SrtRejectReason::Rogue,
            srt::SRT_REJECT_REASON::SRT_REJ_BACKLOG => SrtRejectReason::Backlog,
            srt::SRT_REJECT_REASON::SRT_REJ_IPE => SrtRejectReason::IPE,
            srt::SRT_REJECT_REASON::SRT_REJ_CLOSE => SrtRejectReason::Close,
            srt::SRT_REJECT_REASON::SRT_REJ_VERSION => SrtRejectReason::Version,
            srt::SRT_REJECT_REASON::SRT_REJ_RDVCOOKIE => SrtRejectReason::RdvCookie,
            srt::SRT_REJECT_REASON::SRT_REJ_BADSECRET => SrtRejectReason::BadSecret,
            srt::SRT_REJECT_REASON::SRT_REJ_UNSECURE => SrtRejectReason::Unsecure,
            srt::SRT_REJECT_REASON::SRT_REJ_MESSAGEAPI => SrtRejectReason::MessageAPI,
            srt::SRT_REJECT_REASON::SRT_REJ_CONGESTION => SrtRejectReason::Congestion,
            srt::SRT_REJECT_REASON::SRT_REJ_FILTER => SrtRejectReason::Filter,
            srt::SRT_REJECT_REASON::SRT_REJ_GROUP => SrtRejectReason::Group,
            srt::SRT_REJECT_REASON::SRT_REJ_TIMEOUT => SrtRejectReason::Timeout,
            _ => match reject_reason.0 {
                srt::SRT_REJC_PREDEFINED..=srt::SRT_REJC_USERDEFINED => SrtRejectReason::Predefined(reject_reason.0 - srt::SRT_REJC_PREDEFINED),
                _ => SrtRejectReason::UserDefined(reject_reason.0 - srt::SRT_REJC_USERDEFINED + 1),
            }
        }
    }
}
