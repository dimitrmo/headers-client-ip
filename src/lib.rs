use std::net::IpAddr;
use headers::{Header, HeaderName, HeaderValue};
use lazy_static::lazy_static;

lazy_static! {
    static ref X_REAL_IP: HeaderName = HeaderName::from_lowercase(b"x-real-ip").unwrap();
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct XRealIP(pub IpAddr);

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
