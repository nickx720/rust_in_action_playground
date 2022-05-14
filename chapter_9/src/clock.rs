use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, Duration as ChronoDuration, TimeZone, Timelike};
use chrono::{Local, Utc};
use clap::{App, Arg};
use libc;
use std::mem::zeroed;
use std::net::UdpSocket;
use std::time::Duration;

const NTP_MESSAGE_LENGTH: usize = 48;
const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;
const LOCAL_ADDR: &'static str = "0.0.0.0:12300";

#[derive(Default, Debug, Clone, Copy)]
struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

struct NTPMessage {
    data: [u8; NTP_MESSAGE_LENGTH],
}

#[derive(Debug)]
struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

impl NTPResult {
    fn offset(&self) -> i64 {
        let duration = (self.t2 - self.t1) + (self.t4 - self.t3);
        duration.num_milliseconds() / 2
    }
    fn delay(&self) -> i64 {
        let duration = (self.t4 - self.t1) - (self.t3 - self.t2);
        duration.num_milliseconds()
    }
}

impl From<NTPTimestamp> for DateTime<Utc> {
    fn from(ntp: NTPTimestamp) -> Self {
        let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
        let mut nanos = ntp.fraction as f64;
        nanos *= 1e9;
        nanos /= 2_f64.powi(32);

        Utc.timestamp(secs, nanos as u32)
    }
}
// start here
struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::{settimeofday, timezone};
        use libc::{suseconds_t, time_t, timeval};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };
        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;
        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}

pub fn clock_main() {
    let app = App::new("clock")
        .version("0.1.2")
        .about("Gets and (aspirationally) sets the time.")
        .after_help(
            "Note: UNIX timestamps are parsed as whole \
        seconds since 1st January 1970 0:00:00 UTC. \
        For more accuracy. use another format.",
        )
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("use-standard")
                .takes_value(true)
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(Arg::with_name("datetime").help(
            "When <action> is 'set',apply , <datetime>. \
        Otherwise, ignore.",
        ));
    let args = app.get_matches();
    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

    if action == "set" {
        let t_ = args.value_of("datetime").unwrap();

        let parser = match std {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };

        let err_msg = format!("Unable to parse {t_} according to {std}");
        let t = parser(t_).expect(&err_msg);
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
