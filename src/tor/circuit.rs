use std::fmt::{self, Write};
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::{escaped, tag};
use nom::character::complete::{alphanumeric1, digit1, none_of, one_of, space1};
use nom::combinator::{map, map_opt, opt, verify};
use nom::error::{context, ContextError, ParseError};
use nom::multi::{count, separated_list1};
use nom::sequence::tuple;

use crate::tor::utils::{base32_word, parse_hex, word};
use crate::tor::NomParse;

#[derive(Debug, Eq, PartialEq)]
pub enum CircuitStatus {
    /// circuit ID assigned to new circuit
    Launched,

    /// all hops finished, can now accept streams
    Built,

    /// all hops finished, waiting to see if a circuit with a better guard will be usable.
    GuardWait,

    /// one more hop has been completed
    Extended,

    /// circuit closed (was not built)
    Failed,

    /// circuit closed (was built)
    Closed,
}

impl NomParse for CircuitStatus {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "Circuit status",
            alt((
                map(tag("LAUNCHED"), |_| Self::Launched),
                map(tag("BUILT"), |_| Self::Built),
                map(tag("GUARD_WAIT"), |_| Self::GuardWait),
                map(tag("EXTENDED"), |_| Self::Extended),
                map(tag("FAILED"), |_| Self::Failed),
                map(tag("CLOSED"), |_| Self::Closed),
            )),
        )(input)
    }
}
impl_from_str!(CircuitStatus);

impl fmt::Display for CircuitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Launched => f.write_str("LAUNCHED"),
            Self::Built => f.write_str("BUILT"),
            Self::GuardWait => f.write_str("GUARD_WAIT"),
            Self::Extended => f.write_str("EXTENDED"),
            Self::Failed => f.write_str("FAILED"),
            Self::Closed => f.write_str("CLOSED"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CircuitBuildFlag {
    /// One-hop circuit, used for tunneled directory conns
    OneHopTunnel,

    /// Internal circuit, not to be used for exiting streams
    IsInternal,

    /// This circuit must use only high-capacity nodes
    NeedCapacity,

    /// This circuit must use only high-uptime nodes
    NeedUptime,
}

impl NomParse for CircuitBuildFlag {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "Circuit build flag",
            alt((
                map(tag("ONEHOP_TUNNEL"), |_| Self::OneHopTunnel),
                map(tag("IS_INTERNAL"), |_| Self::IsInternal),
                map(tag("NEED_CAPACITY"), |_| Self::NeedCapacity),
                map(tag("NEED_UPTIME"), |_| Self::NeedUptime),
            )),
        )(input)
    }
}
impl_from_str!(CircuitBuildFlag);

impl fmt::Display for CircuitBuildFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneHopTunnel => f.write_str("ONEHOP_TUNNEL"),
            Self::IsInternal => f.write_str("IS_INTERNAL"),
            Self::NeedCapacity => f.write_str("NEED_CAPACITY"),
            Self::NeedUptime => f.write_str("NEED_UPTIME"),
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CircuitBuildFlags(Vec<CircuitBuildFlag>);

impl NomParse for CircuitBuildFlags {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, flags) = separated_list1(tag(","), CircuitBuildFlag::parse)(input)?;
        Ok((rest, Self(flags)))
    }
}
impl_from_str!(CircuitBuildFlags);

impl fmt::Display for CircuitBuildFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, flag) in self.0.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", flag)?;
            } else {
                write!(f, "|{}", flag)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CircuitPurpose {
    /// Circuit for AP and/or directory request streams
    General,

    /// HS client-side introduction-point circuit
    HsClientIntro,

    /// HS client-side rendezvous circuit; carries AP streams
    HsClientRend,

    /// HS service-side introduction-point circuit
    HsServiceIntro,

    /// HS service-side rendezvous circuit
    HsServiceRend,

    HsClientHsDir,

    /// Reachability-testing circuit; carries no traffic
    Testing,

    /// Circuit built by a controller
    Controller,

    /// Circuit being kept around to see how long it takes
    MeasureTimeout,

    /// Circuit created ahead of time when using HS vanguards, and later repurposed as needed
    HsVanguards,

    /// Circuit used to probe whether our circuits are being deliberately closed by an attacker
    PathBiasTesting,

    /// Circuit that is being held open to disguise its true close time
    CircuitPadding,
}

impl NomParse for CircuitPurpose {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "Circuit purpose",
            alt((
                map(tag("GENERAL"), |_| Self::General),
                map(tag("HS_CLIENT_INTRO"), |_| Self::HsClientIntro),
                map(tag("HS_CLIENT_REND"), |_| Self::HsClientRend),
                map(tag("HS_CLIENT_HSDIR"), |_| Self::HsClientHsDir),
                map(tag("HS_SERVICE_INTRO"), |_| Self::HsServiceIntro),
                map(tag("HS_SERVICE_REND"), |_| Self::HsServiceRend),
                map(tag("TESTING"), |_| Self::Testing),
                map(tag("CONTROLLER"), |_| Self::Controller),
                map(tag("MEASURE8TIMEOUT"), |_| Self::MeasureTimeout),
                map(tag("HS_VANGUARDS"), |_| Self::HsVanguards),
                map(tag("PATH_BIAS_TESTING"), |_| Self::PathBiasTesting),
                map(tag("CIRCUIT_PADDING"), |_| Self::CircuitPadding),
            )),
        )(input)
    }
}
impl_from_str!(CircuitPurpose);

impl fmt::Display for CircuitPurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::General => f.write_str("GENERAL"),
            Self::HsClientIntro => f.write_str("HS_CLIENT_INTRO"),
            Self::HsClientRend => f.write_str("HS_CLIENT_REND"),
            Self::HsClientHsDir => f.write_str("HS_CLIENT_HSDIR"),
            Self::HsServiceIntro => f.write_str("HS_SERVICE_INTRO"),
            Self::HsServiceRend => f.write_str("HS_SERVICE_REND"),
            Self::Testing => f.write_str("TESTING"),
            Self::Controller => f.write_str("CONTROLLER"),
            Self::MeasureTimeout => f.write_str("MEASURE8TIMEOUT"),
            Self::HsVanguards => f.write_str("HS_VANGUARDS"),
            Self::PathBiasTesting => f.write_str("PATH_BIAS_TESTING"),
            Self::CircuitPadding => f.write_str("CIRCUIT_PADDING"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum HsState {
    /// Client-side introduction-point circuit states, connecting to intro point
    HSCIConnecting,

    /// Client-side introduction-point circuit states, sent INTRODUCE1; waiting for reply from IP
    HSCIIntroSent,

    /// Client-side introduction-point circuit states, received reply from IP relay; closing
    HSCIDone,

    /// Client-side rendezvous-point circuit states, connecting to or waiting for reply from RP
    HSCRConnecting,

    /// Client-side rendezvous-point circuit states, established RP; waiting for introduction
    HSCREstablishedIdle,

    /// Client-side rendezvous-point circuit states, introduction sent to HS; waiting for rend
    HSCREstablishedWaiting,

    /// Client-side rendezvous-point circuit states, connected to HS
    HSCRJoined,

    /// Service-side introduction-point circuit states, connecting to intro point
    HSSIConnecting,

    /// Service-side introduction-point circuit states, established intro point
    HSSIEstablished,

    /// Service-side rendezvous-point circuit states, connecting to client's rend point
    HSSRConnecting,

    /// Service-side rendezvous-point circuit states, connected to client's RP circuit
    HSSRJoined,
}

impl NomParse for HsState {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "HS state",
            alt((
                map(tag("HSCI_CONNECTING"), |_| Self::HSCIConnecting),
                map(tag("HSCI_INTROSENT"), |_| Self::HSCIIntroSent),
                map(tag("HSCI_DONE"), |_| Self::HSCIDone),
                map(tag("HSCR_CONNECTING"), |_| Self::HSCRConnecting),
                map(tag("HSCR_ESTABLISHED_IDLE"), |_| Self::HSCREstablishedIdle),
                map(tag("HSCR_ESTABLISHED_WAITING"), |_| {
                    Self::HSCREstablishedWaiting
                }),
                map(tag("HSCR_JOINED"), |_| Self::HSCRJoined),
                map(tag("HSSI_CONNECTING"), |_| Self::HSSIConnecting),
                map(tag("HSSI_ESTABLISHED"), |_| Self::HSSIEstablished),
                map(tag("HSSR_CONNECTING"), |_| Self::HSSRConnecting),
                map(tag("HSSR_JOINED"), |_| Self::HSSRJoined),
            )),
        )(input)
    }
}
impl_from_str!(HsState);

impl fmt::Display for HsState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HSCIConnecting => f.write_str("HSCI_CONNECTING"),
            Self::HSCIIntroSent => f.write_str("HSCI_INTROSENT"),
            Self::HSCIDone => f.write_str("HSCI_DONE"),
            Self::HSCRConnecting => f.write_str("HSCR_CONNECTING"),
            Self::HSCREstablishedIdle => f.write_str("HSCR_ESTABLISHED_IDLE"),
            Self::HSCREstablishedWaiting => f.write_str("HSCR_ESTABLISHED_WAITING"),
            Self::HSCRJoined => f.write_str("HSCR_JOINED"),
            Self::HSSIConnecting => f.write_str("HSSI_CONNECTING"),
            Self::HSSIEstablished => f.write_str("HSSI_ESTABLISHED"),
            Self::HSSRConnecting => f.write_str("HSSR_CONNECTING"),
            Self::HSSRJoined => f.write_str("HSSR_JOINED"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CircuitReason {
    None,
    TorProtocol,
    Internal,
    Requested,
    Hibernating,
    ResourceLimit,
    ConnectFailed,
    OrIdentity,
    OrConnClosed,
    Timeout,
    Finished,
    Destroyed,
    Nopath,
    Nosuchservice,
    MeasurementExpired,
}

impl NomParse for CircuitReason {
    fn parse<'a, E>(s: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "Circuit reason",
            alt((
                map(tag("NONE"), |_| Self::None),
                map(tag("TORPROTOCOL"), |_| Self::TorProtocol),
                map(tag("INTERNAL"), |_| Self::Internal),
                map(tag("REQUESTED"), |_| Self::Requested),
                map(tag("HIBERNATING"), |_| Self::Hibernating),
                map(tag("RESOURCELIMIT"), |_| Self::ResourceLimit),
                map(tag("CONNECTFAILED"), |_| Self::ConnectFailed),
                map(tag("OR_IDENTITY"), |_| Self::OrIdentity),
                map(tag("OR_CONN_CLOSED"), |_| Self::OrConnClosed),
                map(tag("TIMEOUT"), |_| Self::Timeout),
                map(tag("FINISHED"), |_| Self::Finished),
                map(tag("DESTROYED"), |_| Self::Destroyed),
                map(tag("NOPATH"), |_| Self::Nopath),
                map(tag("NOSUCHSERVICE"), |_| Self::Nosuchservice),
                map(tag("MEASUREMENT_EXPIRED"), |_| Self::MeasurementExpired),
            )),
        )(s)
    }
}
impl_from_str!(CircuitReason);

impl fmt::Display for CircuitReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("NONE"),
            Self::TorProtocol => f.write_str("TORPROTOCOL"),
            Self::Internal => f.write_str("INTERNAL"),
            Self::Requested => f.write_str("REQUESTED"),
            Self::Hibernating => f.write_str("HIBERNATING"),
            Self::ResourceLimit => f.write_str("RESOURCELIMIT"),
            Self::ConnectFailed => f.write_str("CONNECTFAILED"),
            Self::OrIdentity => f.write_str("OR_IDENTITY"),
            Self::OrConnClosed => f.write_str("OR_CONN_CLOSED"),
            Self::Timeout => f.write_str("TIMEOUT"),
            Self::Finished => f.write_str("FINISHED"),
            Self::Destroyed => f.write_str("DESTROYED"),
            Self::Nopath => f.write_str("NOPATH"),
            Self::Nosuchservice => f.write_str("NOSUCHSERVICE"),
            Self::MeasurementExpired => f.write_str("MEASUREMENT_EXPIRED"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CircuitID(String);

impl NomParse for CircuitID {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "Circuit ID",
            map(
                verify(alphanumeric1, |id: &str| id.len() < 16),
                |id: &str| Self(id.into()),
            ),
        )(input)
    }
}
impl_from_str!(CircuitID);

impl fmt::Display for CircuitID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Default, Eq, PartialEq)]
pub struct Step {
    pub fingerprint: [u8; 20],
    pub nickname: Option<String>,
}

impl fmt::Debug for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fingerprint = String::with_capacity(self.fingerprint.len() * 2);
        for byte in self.fingerprint.iter() {
            fingerprint.push_str(format!("{:02x}", *byte).as_str());
        }
        f.debug_struct("Step")
            .field("fingerprint", &fingerprint)
            .field("nickname", &self.nickname)
            .finish()
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.fingerprint.iter() {
            f.write_fmt(format_args!("{:02X}", *byte))?;
        }
        if let Some(ref nickname) = self.nickname {
            f.write_char('~')?;
            f.write_str(nickname)?;
        }
        Ok(())
    }
}

impl NomParse for Step {
    fn parse<'a, E>(s: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, (_dollar, fingerprint)) =
            context("Step fingerprint", tuple((tag("$"), count(parse_hex, 20))))(s)?;

        let mut me = Self::default();
        me.fingerprint.copy_from_slice(&fingerprint[..]);

        let (rest, nickname) = opt(nom::sequence::preceded(
            nom::branch::alt((tag("~"), tag("="))),
            context(
                "Fingerprint nickname",
                verify(alphanumeric1, |name: &str| name.len() < 20),
            ),
        ))(rest)?;

        if let Some(nickname) = nickname {
            me.nickname = Some(nickname.to_owned());
        }

        Ok((rest, me))
    }
}
impl_from_str!(Step);

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Path(Vec<Step>);

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('[')?;
        for (i, path) in self.0.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", path)?;
            } else {
                write!(f, ", {}", path)?;
            }
        }
        f.write_char(']')
    }
}

impl std::ops::Deref for Path {
    type Target = Vec<Step>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl NomParse for Path {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, paths) = nom::multi::separated_list0(tag(","), Step::parse)(input)?;
        Ok((rest, Self(paths)))
    }
}
impl_from_str!(Path);

#[derive(Debug, Eq, PartialEq)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub mseconds: u32,
}

impl NomParse for Time {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, (year, _, month, _, day, _, hour, _, minute, _, second)) = context(
            "Time created",
            tuple((
                map(verify(digit1, |s: &str| s.len() == 4), |s: &str| {
                    s.parse::<u16>().unwrap()
                }),
                tag("-"),
                map(verify(digit1, |s: &str| s.len() <= 2), |s: &str| {
                    s.parse::<u8>().unwrap()
                }),
                tag("-"),
                map(verify(digit1, |s: &str| s.len() <= 2), |s: &str| {
                    s.parse::<u8>().unwrap()
                }),
                alt((tag("T"), tag(" "))),
                map(verify(digit1, |s: &str| s.len() <= 2), |s: &str| {
                    s.parse::<u8>().unwrap()
                }),
                tag(":"),
                map(verify(digit1, |s: &str| s.len() <= 2), |s: &str| {
                    s.parse::<u8>().unwrap()
                }),
                tag(":"),
                map(verify(digit1, |s: &str| s.len() <= 2), |s: &str| {
                    s.parse::<u8>().unwrap()
                }),
            )),
        )(input)?;
        let (rest, opt_mseconds) = opt(tuple((
            tag("."),
            map(verify(digit1, |s: &str| s.len() == 6), |s: &str| {
                s.parse::<u32>().unwrap()
            }),
        )))(rest)?;
        let mseconds = opt_mseconds.map(|x| x.1).unwrap_or_default();

        Ok((
            rest,
            Self {
                year,
                month,
                day,
                hour,
                minute,
                second,
                mseconds,
            },
        ))
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}",
            self.year, self.month, self.day, self.hour, self.minute, self.second, self.mseconds
        )
    }
}

#[derive(PartialEq, Eq)]
pub enum HsAddress {
    V2([u8; 10]),
    V3([u8; 35]),
}

impl fmt::Debug for HsAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V2(addr) => {
                let mut s = String::with_capacity(addr.len() * 2);
                for byte in addr.iter() {
                    s.push_str(&format!("{:02x}", *byte));
                }
                f.debug_tuple("HsAddress::V2").field(&s).finish()
            }
            Self::V3(addr) => {
                let mut s = String::with_capacity(addr.len() * 2);
                for byte in addr.iter() {
                    s.push_str(&format!("{:02x}", *byte));
                }
                f.debug_tuple("HsAddress::V3").field(&s).finish()
            }
        }
    }
}

impl fmt::Display for HsAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let alphabet = base32::Alphabet::RFC4648 { padding: true };
        match self {
            Self::V2(addr) => f.write_str(&base32::encode(alphabet, &addr[..])),
            Self::V3(addr) => f.write_str(&base32::encode(alphabet, &addr[..])),
        }
    }
}

impl NomParse for HsAddress {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, addr) = context(
            "HS address",
            verify(base32_word, |s: &str| s.len() == 16 || s.len() == 56),
        )(input)?;
        let alphabet = base32::Alphabet::RFC4648 { padding: false };
        let ip = match addr.len() {
            16 => {
                let bytes = base32::decode(alphabet, addr).expect("Invalid base32_word func?!");
                let mut data = [0u8; 10];
                data.copy_from_slice(&bytes[..]);
                Self::V2(data)
            }
            56 => {
                let bytes = base32::decode(alphabet, addr).expect("Invalid base32_word func?!");
                let mut data = [0u8; 35];
                data.copy_from_slice(&bytes[..]);
                Self::V3(data)
            }
            _ => unreachable!(),
        };

        Ok((rest, ip))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Circuit {
    pub id: CircuitID,
    pub status: CircuitStatus,
    pub path: Path,
    pub build_flags: CircuitBuildFlags,
    pub purpose: Option<CircuitPurpose>,
    pub hs_state: Option<HsState>,
    pub rend_query: Option<HsAddress>,
    pub time_created: Option<Time>,
    pub reason: Option<CircuitReason>,
    pub socks_username: Option<String>,
    pub socks_password: Option<String>,
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id={} status={}, path={} build_flags={}",
            self.id, self.status, self.path, self.build_flags
        )?;

        if let Some(ref purpose) = self.purpose {
            write!(f, " purpose={}", purpose)?;
        }
        if let Some(ref hs_state) = self.hs_state {
            write!(f, " hs_state={}", hs_state)?;
        }
        if let Some(ref rend_query) = self.rend_query {
            write!(f, " rend_query={}", rend_query)?;
        }
        if let Some(ref time_created) = self.time_created {
            write!(f, " time_created={}", time_created)?;
        }
        if let Some(ref reason) = self.reason {
            write!(f, " reason={}", reason)?;
        }
        if let Some(ref socks_username) = self.socks_username {
            write!(f, " socks_username={}", socks_username)?;
        }
        if let Some(ref socks_password) = self.socks_password {
            write!(f, " socks_password={}", socks_password)?;
        }

        Ok(())
    }
}

impl NomParse for Circuit {
    fn parse<'a, E>(s: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, (_, circuit_id, _, status)) = tuple((
            opt(tag("\r\n")),
            CircuitID::parse,
            space1,
            CircuitStatus::parse,
        ))(s)?;

        let (rest, opt_path) = context("Path", opt(tuple((space1, Path::parse))))(rest)?;
        let path = opt_path.map(|x| x.1).unwrap_or_default();

        let (rest, opt_build_flags) = context(
            "Build flags",
            opt(tuple((
                space1,
                tag("BUILD_FLAGS="),
                CircuitBuildFlags::parse,
            ))),
        )(rest)?;
        let build_flags = opt_build_flags.map(|x| x.2).unwrap_or_default();

        let (rest, opt_purpose) = context(
            "PURPOSE",
            opt(tuple((
                space1,
                tag("PURPOSE="),
                map_opt(word, |p: &str| p.parse::<CircuitPurpose>().ok()),
            ))),
        )(rest)?;
        let purpose = opt_purpose.map(|x| x.2);

        let (rest, opt_hs_state) = opt(tuple((space1, tag("HS_STATE="), HsState::parse)))(rest)?;
        let hs_state = opt_hs_state.map(|x| x.2);

        let (rest, opt_rend_query) =
            opt(tuple((space1, tag("REND_QUERY="), HsAddress::parse)))(rest)?;
        let rend_query = opt_rend_query.map(|x| x.2);

        let (rest, opt_time_created) = context(
            "Time created",
            opt(tuple((space1, tag("TIME_CREATED="), Time::parse))),
        )(rest)?;
        let time_created = opt_time_created.map(|x| x.2);

        let (rest, opt_reason) = context(
            "reason",
            opt(tuple((space1, tag("REASON="), CircuitReason::parse))),
        )(rest)?;
        let reason = opt_reason.map(|x| x.2);

        let (rest, opt_socks_username) = context(
            "socks username",
            opt(tuple((
                space1,
                tag("SOCKS_USERNAME=\""),
                escaped(none_of("\\"), '\\', one_of("\\\"")),
                tag("\""),
            ))),
        )(rest)?;
        let socks_username = opt_socks_username.map(|x| x.2.to_owned());

        let (rest, opt_socks_password) = context(
            "socks password",
            opt(tuple((
                space1,
                tag("SOCKS_PASSWORD=\""),
                escaped(none_of("\\"), '\\', one_of("\\\"")),
                tag("\""),
            ))),
        )(rest)?;
        let socks_password = opt_socks_password.map(|x| x.2.to_owned());

        Ok((
            rest,
            Self {
                id: circuit_id,
                status,
                path,
                build_flags,
                purpose,
                hs_state,
                rend_query,
                time_created,
                reason,
                socks_username,
                socks_password,
            },
        ))
    }
}
impl_from_str!(Circuit);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_circuit() {
        let input = "\r\n50 BUILT \
        $8737307DE84C2621E6399E99123967A9590297F2~Tor0x800,\
        $243996E46218666C1CADDE17B430EA7F95124F96~GoofyRooster,\
        $3A9443710224E5182895C342D1D36C1D460A1206~CanterSecure04 \
        BUILD_FLAGS=IS_INTERNAL,NEED_CAPACITY,NEED_UPTIME \
        PURPOSE=HS_CLIENT_REND HS_STATE=HSCR_JOINED \
        REND_QUERY=cflareub6dtu7nvs3kqmoigcjdwap2azrkx5zohb2yk7gqjkwoyotwqd \
        TIME_CREATED=2021-04-30T13:28:42.004916";

        let circuit = Circuit {
            id: CircuitID("50".into()),
            status: CircuitStatus::Built,
            path: Path(vec![
                Step {
                    fingerprint: [
                        0x87, 0x37, 0x30, 0x7d, 0xe8, 0x4c, 0x26, 0x21, 0xe6, 0x39, 0x9e, 0x99,
                        0x12, 0x39, 0x67, 0xa9, 0x59, 0x02, 0x97, 0xf2,
                    ],
                    nickname: Some("Tor0x800".into()),
                },
                Step {
                    fingerprint: [
                        0x24, 0x39, 0x96, 0xe4, 0x62, 0x18, 0x66, 0x6c, 0x1c, 0xad, 0xde, 0x17,
                        0xb4, 0x30, 0xea, 0x7f, 0x95, 0x12, 0x4f, 0x96,
                    ],
                    nickname: Some("GoofyRooster".into()),
                },
                Step {
                    fingerprint: [
                        0x3a, 0x94, 0x43, 0x71, 0x02, 0x24, 0xe5, 0x18, 0x28, 0x95, 0xc3, 0x42,
                        0xd1, 0xd3, 0x6c, 0x1d, 0x46, 0x0a, 0x12, 0x06,
                    ],
                    nickname: Some("CanterSecure04".into()),
                },
            ]),
            build_flags: CircuitBuildFlags(vec![
                CircuitBuildFlag::IsInternal,
                CircuitBuildFlag::NeedCapacity,
                CircuitBuildFlag::NeedUptime,
            ]),
            purpose: Some(CircuitPurpose::HsClientRend),
            hs_state: Some(HsState::HSCRJoined),
            rend_query: Some(HsAddress::V3([
                0x11, 0x56, 0x08, 0x92, 0x81, 0xf0, 0xe7, 0x4f, 0xb6, 0xb2, 0xda, 0xa0, 0xc7, 0x20,
                0xc2, 0x48, 0xec, 0x07, 0xe8, 0x19, 0x8a, 0xaf, 0xdc, 0xb8, 0xe1, 0xd6, 0x15, 0xf3,
                0x41, 0x2a, 0xb3, 0xb0, 0xe9, 0xda, 0x03,
            ])),
            time_created: Some(Time {
                year: 2021,
                month: 4,
                day: 30,
                hour: 13,
                minute: 28,
                second: 42,
                mseconds: 4916,
            }),
            reason: None,
            socks_username: None,
            socks_password: None,
        };
        assert_eq!(
            Circuit::parse::<nom::error::VerboseError<&str>>(input),
            Ok(("", circuit))
        );
    }
}
