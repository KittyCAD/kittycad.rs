//! Utility functions for generating random values.

use std::{fmt::Write as _, str::FromStr};

use anyhow::Result;
use chrono::TimeZone;
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

impl Random for i8 {
    fn random() -> Result<Self> {
        Ok(rand::thread_rng().gen_range(std::i8::MIN..std::i8::MAX))
    }
}

impl Random for i16 {
    fn random() -> Result<Self> {
        Ok(rand::thread_rng().gen_range(std::i16::MIN..std::i16::MAX))
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

impl Random for u8 {
    fn random() -> Result<Self> {
        Ok(rand::thread_rng().gen_range(std::u8::MIN..std::u8::MAX))
    }
}

impl Random for u16 {
    fn random() -> Result<Self> {
        Ok(rand::thread_rng().gen_range(std::u16::MIN..std::u16::MAX))
    }
}

impl Random for u32 {
    fn random() -> Result<Self> {
        Ok(rand::thread_rng().gen_range(std::u32::MIN..std::u32::MAX))
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
            write!(ip, "{}.", rng.gen_range(0..255))?;
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
            write!(ip, "{:x}:", rng.gen_range(0..16))?;
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
            write!(host, "{}.", rng.gen_range(0..255))?;
        }
        host.pop();
        url.push_str(&host);
        write!(url, "/{}", rng.gen_range(0..10))?;
        Ok(url::Url::parse(&url)?)
    }
}

impl Random for chrono::NaiveTime {
    fn random() -> Result<Self> {
        // Generate a random time.
        let mut rng = rand::thread_rng();
        let hour = rng.gen_range(0..24);
        let minute = rng.gen_range(0..60);
        let second = rng.gen_range(0..60);
        Ok(chrono::NaiveTime::from_hms(hour, minute, second))
    }
}

impl Random for chrono::NaiveDate {
    fn random() -> Result<Self> {
        // Generate a random date.
        let mut rng = rand::thread_rng();
        let year = rng.gen_range(1900..2100);
        let month = rng.gen_range(1..13);
        let day = rng.gen_range(1..28);
        Ok(chrono::NaiveDate::from_ymd(year, month, day))
    }
}

impl Random for chrono::NaiveDateTime {
    fn random() -> Result<Self> {
        // Generate a random date and time.
        let date = chrono::NaiveDate::random()?;
        let time = chrono::NaiveTime::random()?;
        Ok(chrono::NaiveDateTime::new(date, time))
    }
}

impl Random for chrono::DateTime<chrono::Utc> {
    fn random() -> Result<Self> {
        // Generate a random date and time.
        let mut rng = rand::thread_rng();
        Ok(chrono::Utc
            .ymd(
                rng.gen_range(1900..2100),
                rng.gen_range(1..13),
                rng.gen_range(1..28),
            )
            .and_hms_milli(
                rng.gen_range(0..24),
                rng.gen_range(0..60),
                rng.gen_range(0..60),
                rng.gen_range(0..1_000),
            ))
    }
}

impl Random for bool {
    fn random() -> Result<Self> {
        Ok(rand::thread_rng().gen())
    }
}

impl Random for crate::types::base64::Base64Data {
    fn random() -> Result<Self> {
        let mut rng = rand::thread_rng();
        let mut bytes = Vec::new();
        for _ in 0..rng.gen_range(8..16) {
            bytes.push(rng.gen_range(0..256) as u8);
        }
        Ok(crate::types::base64::Base64Data(bytes))
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
    fn test_random_i8() {
        i8::random().unwrap();
    }

    #[test]
    fn test_random_i16() {
        i16::random().unwrap();
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
    fn test_random_u8() {
        u8::random().unwrap();
    }

    #[test]
    fn test_random_u16() {
        u16::random().unwrap();
    }

    #[test]
    fn test_random_u32() {
        u32::random().unwrap();
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

    #[test]
    fn test_random_naive_time() {
        chrono::NaiveTime::random().unwrap();
    }

    #[test]
    fn test_random_naive_date() {
        chrono::NaiveDate::random().unwrap();
    }

    #[test]
    fn test_random_naive_datetime() {
        chrono::NaiveDateTime::random().unwrap();
    }

    #[test]
    fn test_random_datetime() {
        chrono::DateTime::<chrono::Utc>::random().unwrap();
    }

    #[test]
    fn test_random_bool() {
        bool::random().unwrap();
    }

    #[test]
    fn test_random_base64() {
        crate::types::base64::Base64Data::random().unwrap();
    }
}
