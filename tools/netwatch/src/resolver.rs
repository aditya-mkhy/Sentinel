use std::net::IpAddr;
use std::str::FromStr;

pub fn reverse_dns(ip: &str) -> Option<String> {
    let addr = IpAddr::from_str(ip).ok()?;
    dns_lookup::lookup_addr(&addr).ok()
}
