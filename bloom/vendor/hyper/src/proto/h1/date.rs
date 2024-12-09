use std::cell::RefCell;
use std::fmt::{self, Write};
use std::str;
use std::time::{Duration, SystemTime};

use http::header::HeaderValue;
use httpdate::HttpDate;

// "Sun, 06 Nov 1994 08:49:37 GMT".len()
pub const DATE_VALUE_LENGTH: usize = 29;

pub fn extend(dst: &mut Vec<u8>) {
    CACHED.with(|cache| {
        dst.extend_from_slice(cache.borrow().buffer());
    })
}

pub fn update() {
    CACHED.with(|cache| {
        cache.borrow_mut().check();
    })
}

pub(crate) fn update_and_header_value() -> HeaderValue {
    CACHED.with(|cache| {
        let mut cache = cache.borrow_mut();
        cache.check();
        HeaderValue::from_bytes(cache.buffer()).expect("Date format should be valid HeaderValue")
    })
}

struct CachedDate {
    bytes: [u8; DATE_VALUE_LENGTH],
    pos: usize,
    next_update: SystemTime,
}

thread_local!(static CACHED: RefCell<CachedDate> = RefCell::new(CachedDate::new()));

impl CachedDate {
    fn new() -> Self {
        let mut cache = CachedDate {
            bytes: [0; DATE_VALUE_LENGTH],
            pos: 0,
            next_update: SystemTime::now(),
        };
        cache.update(cache.next_update);
        cache
    }

    fn buffer(&self) -> &[u8] {
        &self.bytes[..]
    }

    fn check(&mut self) {
        let now = SystemTime::now();
        if now > self.next_update {
            self.update(now);
        }
    }

    fn update(&mut self, now: SystemTime) {
        self.render(now);
        self.next_update = now + Duration::new(1, 0);
    }

    fn render(&mut self, now: SystemTime) {
        self.pos = 0;
        let _ = write!(self, "{}", HttpDate::from(now));
        debug_assert!(self.pos == DATE_VALUE_LENGTH);
    }
}

impl fmt::Write for CachedDate {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let len = s.len();
        self.bytes[self.pos..self.pos + len].copy_from_slice(s.as_bytes());
        self.pos += len;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "nightly")]
    use test::Bencher;

    #[test]
    fn test_date_len() {
        assert_eq!(DATE_VALUE_LENGTH, "Sun, 06 Nov 1994 08:49:37 GMT".len());
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_date_check(b: &mut Bencher) {
        let mut date = CachedDate::new();
        // cache the first update
        date.check();

        b.iter(|| {
            date.check();
        });
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_date_render(b: &mut Bencher) {
        let mut date = CachedDate::new();
        let now = SystemTime::now();
        date.render(now);
        b.bytes = date.buffer().len() as u64;

        b.iter(|| {
            date.render(now);
            test::black_box(&date);
        });
    }
}