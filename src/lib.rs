use std::fmt;
use std::fmt::{Display, Formatter};
use std::net::{AddrParseError, IpAddr};
use std::str::FromStr;
use headers::{Header, HeaderName, HeaderValue};
use lazy_static::lazy_static;

lazy_static! {
    static ref X_REAL_IP: HeaderName = HeaderName::from_lowercase(b"x-real-ip").unwrap();
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct XRealIP(pub IpAddr);

impl XRealIP {

    fn new(ip: IpAddr) -> Self {
        XRealIP{ 0: ip }
    }

}

impl From<IpAddr> for XRealIP {

    #[inline]
    fn from(ip: IpAddr) -> XRealIP {
        XRealIP::new(ip)
    }

}

impl Display for XRealIP {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }

}

impl FromStr for XRealIP {

    type Err = AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let addr = s.parse::<IpAddr>()?;
        Ok(XRealIP::new(addr))
    }

}

impl Header for XRealIP {

    fn name() -> &'static HeaderName {
        &X_REAL_IP
    }

    fn decode<'i, I: Iterator<Item = &'i HeaderValue>>(values: &mut I) -> Result<Self, headers::Error> {
        values
            .next()
            .cloned()
            .and_then(|value| {
                if value.is_empty() {
                    return None
                }

                let ip = value.to_str().ok();

                if ip.is_some() {
                    if let Ok(address) = ip.unwrap().to_string().parse::<IpAddr>() {
                        return Some(XRealIP { 0: address });
                    }
                }

                None
            })
            .ok_or_else(headers::Error::invalid)
    }

    fn encode<E: Extend<HeaderValue>>(&self, _values: &mut E) {
        // values.extend(::std::iter::once(HeaderValue::from_static("true")));
    }
}

#[cfg(test)]
mod tests {
    use crate::XRealIP;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    #[test]
    fn string_formatter_works() {
        let ip = XRealIP::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 4)));
        assert_eq!(ip.to_string(), "127.0.0.4");
    }

    #[test]
    fn xrealip_from_ipaddr_works() {
        let ipv6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 4);
        let ip = XRealIP::from(IpAddr::V6(ipv6));
        assert_eq!(ip.to_string(), ipv6.to_string());
    }

    #[test]
    fn parse_from_string_works() {
        let xrealip = "192.168.0.4".parse::<XRealIP>().unwrap();
        assert_eq!(xrealip, XRealIP::from(IpAddr::from([192, 168, 0, 4])))
    }
}
