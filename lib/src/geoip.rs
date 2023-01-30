use crate::bindings;

use std::ffi;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::ptr::NonNull;

fn get_c_string<S: AsRef<str>>(s: S) -> io::Result<ffi::CString> {
    ffi::CString::new(s.as_ref())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "nul byte in string"))
}

fn get_ipv4_mapped(ip6: &Ipv6Addr) -> Option<Ipv4Addr> {
    let segments = ip6.segments();
    let is_ipv4_mapped = segments
        .iter()
        .enumerate()
        .take(6)
        .all(|(i, &b)| (i < 6 && b == 0) || b == 0xffff);
    if is_ipv4_mapped {
        let bytes = &ip6.octets()[12..];
        assert_eq!(bytes.len(), 4);
        Some(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]))
    } else {
        None
    }
}

pub struct GeoIP {
    ip4: NonNull<bindings::GeoIP>,
    ip6: NonNull<bindings::GeoIP>,
}

impl Default for GeoIP {
    fn default() -> Self {
        Self::new()
    }
}

impl GeoIP {
    pub fn new() -> Self {
        Self::open("/usr/share/GeoIP/GeoIP.dat", "/usr/share/GeoIP/GeoIPv6.dat")
            .expect("No GeoIP dat file?!")
    }

    fn geoip_open<P: AsRef<str>>(path: P) -> io::Result<NonNull<bindings::GeoIP>> {
        let c_path = get_c_string(path.as_ref())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Nul byte in filename"))?;
        let ptr = unsafe {
            bindings::GeoIP_open(
                c_path.as_ptr(),
                bindings::GeoIPOptions_GEOIP_MEMORY_CACHE as i32,
            )
        };
        NonNull::new(ptr).ok_or(io::Error::last_os_error())
    }

    pub fn open<P: AsRef<str>, Q: AsRef<str>>(ip4_path: P, ip6_path: Q) -> io::Result<Self> {
        let ip4 = Self::geoip_open(ip4_path)?;
        let ip6 = Self::geoip_open(ip6_path)?;

        Ok(Self { ip4, ip6 })
    }

    pub fn lookup_str<S: AsRef<str>>(&self, ip: S) -> Option<&'static str> {
        let ip = get_c_string(ip).ok()?;
        let country_code_ptr =
            unsafe { bindings::GeoIP_country_code_by_addr(self.ip4.as_ptr(), ip.as_ptr()) };
        Self::country_ptr_to_option(country_code_ptr).or_else(|| {
            Self::country_ptr_to_option(unsafe {
                bindings::GeoIP_country_code_by_addr(self.ip6.as_ptr(), ip.as_ptr())
            })
        })
    }

    fn country_ptr_to_option(ptr: *const i8) -> Option<&'static str> {
        if ptr.is_null() {
            return None;
        }

        // SAFETY: country is not null
        let country = unsafe { ffi::CStr::from_ptr(ptr) };

        // SAFETY: we assumes GeoIP strings are valid UTF-8
        let str_country = unsafe { std::str::from_utf8_unchecked(country.to_bytes()) };
        Some(str_country)
    }

    pub fn lookup_ip<I: Into<IpAddr>>(&self, ip: I) -> Option<&'static str> {
        let res = match ip.into() {
            IpAddr::V4(ip4) => {
                let num = u32::from_be_bytes(ip4.octets());
                unsafe { bindings::GeoIP_country_code_by_ipnum(self.ip4.as_ptr(), num as u64) }
            }
            IpAddr::V6(ip6) => {
                if let Some(ip4) = get_ipv4_mapped(&ip6) {
                    return self.lookup_ip(IpAddr::V4(ip4));
                }
                let bytes = ip6.octets();
                let addr = bindings::in6_addr {
                    __in6_u: bindings::in6_addr__bindgen_ty_1 { __u6_addr8: bytes },
                };
                unsafe { bindings::GeoIP_country_code_by_ipnum_v6(self.ip6.as_ptr(), addr) }
            }
        };
        Self::country_ptr_to_option(res)
    }
}

impl Drop for GeoIP {
    fn drop(&mut self) {
        unsafe { bindings::GeoIP_delete(self.ip4.as_ptr()) }
        unsafe { bindings::GeoIP_delete(self.ip6.as_ptr()) }
    }
}
