/*
 * readable_time
 * Copyright (c) 2025 BayonetArch
 *
 * This software is released under the MIT License.
 * See LICENSE file for details.
 */

//! A lightweight time library for Basic use cases.
//! This library doesnot depend upon any external crates.
//!
//! Basic usage :
//!
//! ```
//! let rt:ReadableTime  = get_readable_time()?;
//! // You  can use rt.month,rt.year,rt.day,rt.hour_24,rt.hour_12,......
//! // Or specific method to get formatted_time like 'rt.get_timef()' 'rt.get_ptimef',....
//!
//! println!("{}",;rt.get_timef());             // OUTPUT:  2025-01-01 03:04:05
//! println!("{}",;rt.get_ptimef()?);           // OUTPUT:  Mon Jan 15 2024 03:45 PM
//! println!("{}",;rt.get_extended_ptimef()?);  // OUTPUT:  Sun Nov 30 07:14:00 +0545 2025
//! ```
//!
//! If you want to get the time period for 'hour_24' you can use :
//! ```
//! ReadableTime::get_time_period(hour_24)?; // "AM" or "PM"
//! ````

use std::{
    error::Error,
    ffi::{CStr, c_char, c_int, c_long},
    time::{self, UNIX_EPOCH},
};

#[allow(nonstandard_style)]
#[repr(C)]
struct tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    tm_gmtoff: c_long,
    tm_zone: *const c_char,
}

#[allow(nonstandard_style)]
/// a type to match 'time_t' in c
pub type time_t = c_long;

#[allow(unused)]
unsafe extern "C" {
    fn localtime(t: *mut time_t) -> *const tm;
}

#[allow(unused)]
pub fn time_since_epoch() -> Result<time_t, Box<dyn Error>> {
    Ok(time::SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as time_t)
}

/// NOTE: for some reason not making fields public makes then inviisible to  lsp ?
#[derive(Debug, Clone)]
pub struct ReadableTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub week_day: i32,
    pub hour_24: i32,
    pub hour_12: i32,
    pub minute: i32,
    pub second: i32,
    pub time_zone: String,
}

#[allow(unused)]
impl ReadableTime {
    /// returns Basic formatted date string
    /// 'Y-M-D H-m-S'
    /// EXAMPLE: 2025-01-01 03:04:05
    pub fn get_timef(&self) -> String {
        format!(
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour_24, self.minute, self.second
        )
    }

    /// Get prettier date string
    /// EXAMPLE: Mon Jan 15 2024 03:45 PM
    pub fn get_ptimef(&self) -> Result<String, Box<dyn Error>> {
        Ok(format!(
            "{} {} {} {} {:02}:{:02} {}",
            Self::weekstr(self.week_day)?,
            Self::monthstr(self.month)?,
            self.day,
            self.year,
            self.hour_12,
            self.minute,
            Self::get_time_period(self.hour_24)?,
        ))
    }

    /// Get pretty formatted date with extra info
    /// EXAMPLE: Sun Nov 30 07:14:00 +0545 2025
    pub fn get_extended_ptimef(&self) -> Result<String, Box<dyn Error>> {
        Ok(format!(
            "{} {} {} {:02}:{:02}:{:02} {} {}",
            Self::weekstr(self.week_day)?,
            Self::monthstr(self.month)?,
            self.day,
            self.hour_24,
            self.minute,
            self.second,
            self.time_zone,
            self.year
        ))
    }
    pub fn weekstr(weekday: i32) -> Result<String, Box<dyn Error>> {
        match weekday {
            1 => Ok("Sun".to_string()),
            2 => Ok("Mon".to_string()),
            3 => Ok("Tue".to_string()),
            4 => Ok("Wed".to_string()),
            5 => Ok("Thu".to_string()),
            6 => Ok("Fri".to_string()),
            7 => Ok("Sat".to_string()),
            _ => Err("invalid day of week.only 1-7 are valid days".into()),
        }
    }

    pub fn monthstr(month: i32) -> Result<String, Box<dyn Error>> {
        match month {
            1 => Ok("Jan".to_string()),
            2 => Ok("Feb".to_string()),
            3 => Ok("Mar".to_string()),
            4 => Ok("Apr".to_string()),
            5 => Ok("May".to_string()),
            6 => Ok("Jun".to_string()),
            7 => Ok("Jul".to_string()),
            8 => Ok("Aug".to_string()),
            9 => Ok("Sep".to_string()),
            10 => Ok("Oct".to_string()),
            11 => Ok("Nov".to_string()),
            12 => Ok("Dec".to_string()),
            _ => Err("invalid month. month should be 1-12".into()),
        }
    }
    pub fn get_time_period(hour_24: i32) -> Result<String, Box<dyn Error>> {
        match hour_24 {
            12..=23 => Ok("PM".to_string()),
            0..=11 => Ok("AM".to_string()),
            _ => Err("invalid hour. hour should be in 0-23 format.".into()),
        }
    }
}

pub fn get_readable_time() -> Result<ReadableTime, Box<dyn Error>> {
    let mut t = time_since_epoch()?;
    let lt;

    let year: i32;
    let month: i32;
    let day: i32;
    let week_day: i32;
    let hour_24: i32;
    let hour_12: i32;
    let minute: i32;
    let second: i32;
    let time_zone;

    unsafe {
        lt = localtime(&mut t);
        if lt.is_null() {
            return Err("Could not get local time.function 'localtime' failed.".into());
        }

        year = (*lt).tm_year + 1900;
        month = (*lt).tm_mon + 1;
        day = (*lt).tm_mday;
        week_day = (*lt).tm_wday + 1;
        hour_24 = (*lt).tm_hour;
        hour_12 = match hour_24 {
            0 => 12,
            1..=12 => hour_24,
            13..=23 => hour_24 - 12,
            _ => hour_24,
        };

        minute = (*lt).tm_min;
        second = (*lt).tm_sec;
        let tz = (*lt).tm_zone;
        time_zone = if !tz.is_null() {
            CStr::from_ptr(tz).to_string_lossy().to_string()
        } else {
            "unknown time_zone".to_string()
        }
    };

    Ok(ReadableTime {
        year,
        month,
        day,
        week_day,
        hour_24,
        hour_12,
        minute,
        second,
        time_zone,
    })
}
