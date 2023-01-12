use std::fmt;
use std::net::Ipv6Addr;

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{digit1, space1};
use nom::combinator::{map, map_opt, opt};
use nom::error::{context, ContextError, ParseError};
use nom::multi::many0;
use nom::sequence::tuple;

use crate::tor::common::{Target, Time};
use crate::tor::utils::{base64_word, word};
use crate::tor::NomParse;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum OnionRouterFlag {
    /// if the router is a directory authority.
    Authority = 0,

    /// if the router is believed to be useless as an exit node (because its ISP censors it, because it is behind a restrictive proxy, or for some similar reason).
    BadExit,

    /// if the router is more useful for building general-purpose exit circuits than for relay circuits.  The path building algorithm uses this flag; see path-spec.txt.
    Exit,

    /// if the router is suitable for high-bandwidth circuits.
    Fast,

    /// if the router is suitable for use as an entry guard.
    Guard,

    /// if the router is considered a v2 hidden service directory.
    HSDir,

    /// if any Ed25519 key in the router's descriptor or microdesriptor does not reflect authority consensus.
    NoEdConsensus,

    /// if the router is suitable for long-lived circuits.
    Stable,

    /// if the router should upload a new descriptor because the old one is too old.
    StaleDesc,

    /// if the router is currently usable over all its published ORPorts. (Authorities ignore IPv6 ORPorts unless configured to check IPv6 reachability.) Relays without this flag are omitted from the consensus, and current clients (since 0.2.9.4-alpha) assume that every listed relay has this flag.
    Running,

    /// if the router has been 'validated'. Clients before 0.2.9.4-alpha would not use routers without this flag by default. Currently, relays without this flag are omitted fromthe consensus, and current (post-0.2.9.4-alpha) clients assume that every listed relay has this flag.
    Valid,

    /// if the router implements the v2 directory protocol or higher.
    V2Dir,
}

impl NomParse for OnionRouterFlag {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "Onion Router flag",
            alt((
                map(tag("Authority"), |_| Self::Authority),
                map(tag("BadExit"), |_| Self::BadExit),
                map(tag("Exit"), |_| Self::Exit),
                map(tag("Fast"), |_| Self::Fast),
                map(tag("Guard"), |_| Self::Guard),
                map(tag("HSDir"), |_| Self::HSDir),
                map(tag("NoEdConsensus"), |_| Self::NoEdConsensus),
                map(tag("Stable"), |_| Self::Stable),
                map(tag("StaleDesc"), |_| Self::StaleDesc),
                map(tag("Running"), |_| Self::Running),
                map(tag("Valid"), |_| Self::Valid),
                map(tag("V2Dir"), |_| Self::V2Dir),
            )),
        )(input)
    }
}

impl fmt::Display for OnionRouterFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authority => write!(f, "Authority"),
            Self::BadExit => write!(f, "BadExit"),
            Self::Exit => write!(f, "Exit"),
            Self::Fast => write!(f, "Fast"),
            Self::Guard => write!(f, "Guard"),
            Self::HSDir => write!(f, "HSDir"),
            Self::NoEdConsensus => write!(f, "NoEdConsensus"),
            Self::Stable => write!(f, "Stable"),
            Self::StaleDesc => write!(f, "StaleDesc"),
            Self::Running => write!(f, "Running"),
            Self::Valid => write!(f, "Valid"),
            Self::V2Dir => write!(f, "V2Dir"),
        }
    }
}

#[derive(Default, Eq, PartialEq, Clone, Copy)]
pub struct OnionRouterFlags {
    flags: u32,
}

impl OnionRouterFlags {
    pub fn new() -> Self {
        Self { flags: 0 }
    }

    pub fn set(&mut self, flag: OnionRouterFlag) -> &mut Self {
        let flag = 1 << (flag as u32);
        self.flags |= flag;
        self
    }

    pub fn or(&mut self, other: OnionRouterFlags) -> &mut Self {
        self.flags |= other.flags;
        self
    }

    pub fn and(&mut self, other: OnionRouterFlags) -> &mut Self {
        self.flags &= other.flags;
        self
    }

    pub fn remove(&mut self, flag: OnionRouterFlag) -> &mut Self {
        let flag = u32::MAX ^ (1u32 << (flag as u32));
        self.flags &= flag;
        self
    }

    pub fn is_set(&self, flag: OnionRouterFlag) -> bool {
        let flag = 1u32 << (flag as u32);
        self.flags & flag == flag
    }
}

impl fmt::Display for OnionRouterFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flags = [
            OnionRouterFlag::Authority,
            OnionRouterFlag::BadExit,
            OnionRouterFlag::Exit,
            OnionRouterFlag::Fast,
            OnionRouterFlag::Guard,
            OnionRouterFlag::HSDir,
            OnionRouterFlag::NoEdConsensus,
            OnionRouterFlag::Stable,
            OnionRouterFlag::StaleDesc,
            OnionRouterFlag::Running,
            OnionRouterFlag::Valid,
            OnionRouterFlag::V2Dir,
        ];
        let mut first = true;
        for flag in flags.iter() {
            if self.is_set(*flag) {
                if first {
                    first = false;
                    write!(f, "{}", flag)?;
                } else {
                    write!(f, "|{}", flag)?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct OnionRouter {
    pub nickname: String,
    pub identity: [u8; 20],
    pub digest: [u8; 20],
    pub publication: Time,
    pub target: Target,
    pub directory_port: Option<u16>,
    pub advertise_ipv6: Option<(Ipv6Addr, u16)>,
    pub flags: OnionRouterFlags,
    pub bandwidth: Option<u32>,
}

impl fmt::Display for OnionRouter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "$")?;
        for byte in self.identity.iter() {
            write!(f, "{:02x}", *byte)?;
        }
        write!(f, "~{}", self.nickname)
    }
}

impl fmt::Debug for OnionRouter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut identity = String::with_capacity(self.identity.len() * 2);
        for byte in self.identity.iter() {
            identity.push_str(format!("{:02x}", *byte).as_str());
        }
        let mut digest = String::with_capacity(self.digest.len() * 2);
        for byte in self.digest.iter() {
            digest.push_str(format!("{:02x}", *byte).as_str());
        }
        let mut dbg = f.debug_struct("OnionRouter");
        dbg.field("nickname", &self.nickname)
            .field("identity", &identity)
            .field("digest", &digest)
            .field("publication", &self.publication)
            .field("target", &self.target);
        if let Some(advertise_ipv6) = self.advertise_ipv6.as_ref() {
            dbg.field("advertise_ipv6", advertise_ipv6);
        }
        dbg.field("flags", &format!("{}", self.flags));
        if let Some(bandwidth) = self.bandwidth.as_ref() {
            dbg.field("bandwidth", bandwidth);
        }
        dbg.finish()
    }
}

impl NomParse for OnionRouter {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        fn copy_slice(dest: &mut [u8], src: &[u8]) {
            if dest.len() < src.len() {
                dest.copy_from_slice(&src[..dest.len()]);
            } else {
                (&mut dest[..src.len()]).copy_from_slice(src);
            }
        }
        let mut identity = [0u8; 20];
        let mut digest = [0u8; 20];
        let mut buf = Vec::new();

        let (rest, _newline) = opt(tag("\r\n"))(input)?;
        let (rest, _tag) = context("tag 'r'", tag("r"))(rest)?;

        let (rest, (_, nickname)) =
            context("nickname", tuple((space1, map(word, String::from))))(rest)?;

        buf.clear();
        let (rest, (_, identity64)) = context("identity", tuple((space1, base64_word)))(rest)?;
        STANDARD_NO_PAD
            .decode_vec(identity64, &mut buf)
            .expect("Invalid base64_word function?!");
        copy_slice(&mut identity[..], &buf[..]);

        buf.clear();
        let (rest, (_, digest64)) = context("digest", tuple((space1, base64_word)))(rest)?;
        STANDARD_NO_PAD
            .decode_vec(digest64, &mut buf)
            .expect("Invalid base64_word function?!");
        copy_slice(&mut digest[..], &buf[..]);
        let (rest, (_, publication)) = context("publication", tuple((space1, Time::parse)))(rest)?;

        let (rest, (_, target)) = tuple((space1, Target::parse))(rest)?;

        let (rest, (_, dir_port)) = context(
            "dir-port",
            tuple((space1, map_opt(digit1, |s: &str| s.parse::<u16>().ok()))),
        )(rest)?;
        let directory_port = if dir_port == 0 { None } else { Some(dir_port) };

        let (rest, opt_advertise_ipv6) = opt(tuple((
            tag("\r\na ["),
            map_opt(
                take_while(|c: char| c.is_ascii_hexdigit() || c == ':'),
                |s: &str| s.parse::<Ipv6Addr>().ok(),
            ),
            tag("]:"),
            map_opt(digit1, |s: &str| s.parse::<u16>().ok()),
        )))(rest)?;
        let advertise_ipv6 = opt_advertise_ipv6.map(|x| (x.1, x.3));

        let mut flags = OnionRouterFlags::new();
        let (rest, _newline_tag) = tag("\r\ns")(rest)?;
        let (rest, _) = many0(map(tuple((space1, OnionRouterFlag::parse)), |(_, f)| {
            flags.set(f);
        }))(rest)?;

        let (rest, opt_bandwidth) = opt(tuple((
            tag("\r\nw Bandwidth="),
            map_opt(digit1, |s: &str| s.parse::<u32>().ok()),
        )))(rest)?;
        let bandwidth = opt_bandwidth.map(|x| x.1);
        // let (rest, _until_double_newline) = tuple((take_until("\r\n\r\n"), tag("\r\n")))(rest)?;

        Ok((
            rest,
            Self {
                nickname,
                identity,
                digest,
                publication,
                target,
                directory_port,
                advertise_ipv6,
                flags,
                bandwidth,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn onion_router() {
        let input = "r Tor0x800 hzcwfehMJiHmOZ6ZEjlnqVkCl/I psMf4zW8kU7rScOKz7Qowqe63oc 2021-05-01 01:11:24 185.80.30.102 9001 9030\n";
        let or = OnionRouter {
            nickname: "Tor0x800".into(),
            identity: [
                0x87, 0x37, 0x30, 0x7d, 0xe8, 0x4c, 0x26, 0x21, 0xe6, 0x39, 0x9e, 0x99, 0x12, 0x39,
                0x67, 0xa9, 0x59, 0x02, 0x97, 0xf2,
            ],
            digest: [
                0xa6, 0xc3, 0x1f, 0xe3, 0x35, 0xbc, 0x91, 0x4e, 0xeb, 0x49, 0xc3, 0x8a, 0xcf, 0xb4,
                0x28, 0xc2, 0xa7, 0xba, 0xde, 0x87,
            ],
            publication: Time {
                year: 2021,
                month: 5,
                day: 1,
                hour: 1,
                minute: 11,
                second: 24,
                mseconds: 0,
            },
            target: Target {
                addr: crate::tor::common::HostOrAddr::Addr(IpAddr::V4(Ipv4Addr::new(
                    185, 80, 30, 102,
                ))),
                port: 9001,
            },
            directory_port: Some(9030),
            advertise_ipv6: None,
            flags: OnionRouterFlags::new(),
            bandwidth: None,
        };

        assert_eq!(
            OnionRouter::parse::<nom::error::VerboseError<&str>>(input),
            Ok(("", or))
        );
    }
}
