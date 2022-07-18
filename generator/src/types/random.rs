//! Utility functions for generating random values.

use std::str::FromStr;

use anyhow::Result;
use rand::Rng;

/// A triat that implements generating a random value of a given type.
pub trait Random {
    /// Generates a random value of the given type.
    fn random() -> Result<Self>
    where
        Self: Sized;
}

impl Random for crate::types::phone_number::PhoneNumber {
    fn random() -> Result<Self> {
        let mut rng = rand::thread_rng();
        let mut number = String::new();
        for _ in 0..10 {
            number.push(rng.gen_range('0'..'9') as char);
        }
        Self::from_str(&number)
    }
}

impl Random for uuid::Uuid {
    fn random() -> Result<Self> {
        Ok(uuid::Uuid::new_v4())
    }
}

impl Random for i32 {
    fn random() -> Result<Self> {
        Ok(rand::thread_rng().gen_range(std::i32::MIN..std::i32::MAX))
    }
}

impl Random for i64 {
    fn random() -> Result<Self> {
        Ok(rand::thread_rng().gen_range(std::i64::MIN..std::i64::MAX))
    }
}

impl Random for f32 {
    fn random() -> Result<Self> {
        Ok(rand::thread_rng().gen_range(0.0..std::f32::MAX))
    }
}

impl Random for f64 {
    fn random() -> Result<Self> {
        Ok(rand::thread_rng().gen_range(0.0..std::f64::MAX))
    }
}

impl Random for u64 {
    fn random() -> Result<Self> {
        Ok(rand::thread_rng().gen_range(std::u64::MIN..std::u64::MAX))
    }
}

impl Random for std::net::Ipv4Addr {
    fn random() -> Result<Self> {
        let mut rng = rand::thread_rng();
        // Return a random IPv4 address.
        let mut ip = String::new();
        for _ in 0..4 {
            ip.push_str(&format!("{}.", rng.gen_range(0..255)));
        }
        ip.pop();
        Ok(ip.parse()?)
    }
}

impl Random for std::net::Ipv6Addr {
    fn random() -> Result<Self> {
        let mut rng = rand::thread_rng();
        // Return a random IPv6 address.
        let mut ip = String::new();
        for _ in 0..8 {
            ip.push_str(&format!("{:x}:", rng.gen_range(0..16)));
        }
        ip.pop();
        Ok(ip.parse()?)
    }
}

impl Random for std::net::IpAddr {
    fn random() -> Result<Self> {
        // Generate a random IPv4 or IPv6 address.
        let mut rng = rand::thread_rng();
        let ip_version = rng.gen_range(0..2);
        match ip_version {
            0 => Ok(std::net::IpAddr::V4(std::net::Ipv4Addr::random()?)),
            1 => Ok(std::net::IpAddr::V6(std::net::Ipv6Addr::random()?)),
            _ => unreachable!(),
        }
    }
}

impl Random for url::Url {
    fn random() -> Result<Self> {
        // Generate a random url.
        let mut rng = rand::thread_rng();
        let scheme = rng.gen_range(0..2);
        let mut url = String::new();
        match scheme {
            0 => url.push_str("http://"),
            1 => url.push_str("https://"),
            _ => unreachable!(),
        }
        let mut host = String::new();
        for _ in 0..rng.gen_range(1..10) {
            host.push_str(&format!("{}.", rng.gen_range(0..255)));
        }
        host.pop();
        url.push_str(&host);
        url.push_str(&format!("/{}", rng.gen_range(0..10)));
        Ok(url::Url::parse(&url)?)
    }
}

#[cfg(test)]
mod test {
    use super::Random;

    #[test]
    fn test_random_phone_number() {
        crate::types::phone_number::PhoneNumber::random().unwrap();
    }

    #[test]
    fn test_random_uuid() {
        uuid::Uuid::random().unwrap();
    }

    #[test]
    fn test_random_i32() {
        i32::random().unwrap();
    }

    #[test]
    fn test_random_i64() {
        i64::random().unwrap();
    }

    #[test]
    fn test_random_f32() {
        f32::random().unwrap();
    }

    #[test]
    fn test_random_f64() {
        f64::random().unwrap();
    }

    #[test]
    fn test_random_u64() {
        u64::random().unwrap();
    }

    #[test]
    fn test_random_ipv4() {
        std::net::Ipv4Addr::random().unwrap();
    }

    #[test]
    fn test_random_ipv6() {
        std::net::Ipv6Addr::random().unwrap();
    }

    #[test]
    fn test_random_ip() {
        std::net::IpAddr::random().unwrap();
    }

    #[test]
    fn test_random_url() {
        url::Url::random().unwrap();
    }
}
