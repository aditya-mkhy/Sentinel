use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;
use trust_dns_resolver::{
    Resolver,
    config::{ResolverConfig, ResolverOpts, LookupIpStrategy},
};


// pub fn reverse_dns(ip: &str) -> Option<String> {
//     let addr = IpAddr::from_str(ip).ok()?;
//     dns_lookup::lookup_addr(&addr).ok()
// }


pub fn reverse_dns(
    resolver: &Resolver,
    ip: &str,
) -> Option<String> {
    let addr = IpAddr::from_str(ip).ok()?;

    let response = resolver.reverse_lookup(addr).ok()?;

    response.iter().next().map(|name| {
        name.to_utf8().trim_end_matches('.').to_string()
    })
}

pub fn create_fast_resolver() -> Resolver {
    let mut opts = ResolverOpts::default();

    opts.timeout = Duration::from_millis(300); // short timeout
    opts.attempts = 1;                         // NO retries
    opts.cache_size = 0;                       // disable internal cache
    opts.use_hosts_file = false;               // skip hosts file
    opts.ip_strategy = LookupIpStrategy::Ipv4AndIpv6;

    Resolver::new(ResolverConfig::default(), opts)
        .expect("Failed to create DNS resolver")
}