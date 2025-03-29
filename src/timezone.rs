use chrono::{self, Datelike, TimeZone, Timelike};
use chrono::prelude::*;
use chrono_tz::Tz;
use crate::fun;

// OLD WORK VERSION -------------------------------------------------------------------------------------------------------------------
// pub fn get_time() -> String {
//     let dt = Local::now();
//     let start_work_time = Utc.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 8, 59, 00).unwrap();
//     let end_work_time = Utc.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 17, 59, 00).unwrap();

//     if dt.hour() >= start_work_time.hour() && dt.hour() <= end_work_time.hour() {
//         let h = if end_work_time.hour() - dt.hour() < 10 {
//             format!("0{}", end_work_time.hour() - dt.hour())
//         } else {
//             format!("{}", end_work_time.hour() - dt.hour())
//         };

//         let m = if end_work_time.minute() - dt.minute() < 10 {
//             format!("0{}", end_work_time.minute() - dt.minute())
//         } else {
//             format!("{}", end_work_time.minute() - dt.minute())
//         };

//         let s = if 60 - dt.second() < 10 {
//             format!("0{}", 60 - dt.second())
//         } else {
//             format!("{}", 60 - dt.second())
//         };

//         // return format!("native time now: {}\ntime now: {}\nОсталось работать: {}:{}:{}", dt, time_format, h, m, s);
//         return format!("Время: {}:{}:{}\nДо конца рабочего дня осталось: {}:{}:{}",  dt.hour(), dt.minute(), dt.second(), h, m, s)
//     } else {
//         return format!("{}", fun::get_time_random_phrases());
//     }
// }
// ------------------------------------------------------------------------------------------------------------------- OLD WORK VERSION

pub fn get_time(choice_city: String) -> String {
    let utc_now = Utc::now();
    let week_today = utc_now.weekday().to_string();

    // MSK -----------------------------------------------------------------------------------
    let moscow_tz: Tz = "Europe/Moscow".parse().unwrap();
    let moscow_time = moscow_tz.from_utc_datetime(&utc_now.naive_utc());
    let moscow_work_time = [
        [8, 59, 59], // start work time (h, m, s)
        [16, 59, 59], // end work time (h, m, s)
    ];
    
    // CHLB -----------------------------------------------------------------------------------
    let chelyabinsk_tz: Tz = "Asia/Yekaterinburg".parse().unwrap();
    let chelyabinsk_time = chelyabinsk_tz.from_utc_datetime(&utc_now.naive_utc());
    let chelyabinsk_work_time = [
        [8, 59, 59], // start work time (h, m, s)
        [17, 59, 59], // end work time (h, m, s)
    ];

    match week_today.as_str() {
        "Mon" | "Tue" | "Wed" | "Thu" | "Fri" => {
            match choice_city.as_str() {
                "msk" => {
                    if moscow_time.hour() >= moscow_work_time[0][0] && moscow_time.hour() <= moscow_work_time[1][0] {
                        format!("Время мск: {}:{}:{}\nОсталось работать: {}:{}:{}", 
                            moscow_time.hour(), moscow_time.minute(), moscow_time.second(),
                            // Hours
                            if (moscow_work_time[1][0] - moscow_time.hour()) <= 10 {
                                format!("0{}", moscow_work_time[1][0] - moscow_time.hour())
                            } else {
                                format!("{}", moscow_work_time[1][0] - moscow_time.hour())
                            },
    
                            // Minutes
                            if (moscow_work_time[1][1] - moscow_time.minute()) <= 10 {
                                format!("0{}", moscow_work_time[1][1] - moscow_time.minute())
                            } else {
                                format!("{}", moscow_work_time[1][1] - moscow_time.minute())
                            },
    
                            // Seconds
                            if (moscow_work_time[1][2] - moscow_time.second()) < 10 {
                                format!("0{}", moscow_work_time[1][2] - moscow_time.second())
                            } else {
                                format!("{}", moscow_work_time[1][2] - moscow_time.second())
                            }
                        )
                    } else {
                        format!("{}", fun::get_time_random_phrases())
                    }
                },
                "chlb" => {
                    if chelyabinsk_time.hour() >= chelyabinsk_work_time[0][0] && chelyabinsk_time.hour() <= chelyabinsk_work_time[1][0] {
                        format!("Время члб: {}:{}:{}\nОсталось работать: {}:{}:{}",
                            chelyabinsk_time.hour(), chelyabinsk_time.minute(), chelyabinsk_time.second(),
                            // Hours
                            if (chelyabinsk_work_time[1][0] - chelyabinsk_time.hour()) <= 10 {
                                format!("0{}", chelyabinsk_work_time[1][0] - chelyabinsk_time.hour())
                            } else {
                                format!("{}", chelyabinsk_work_time[1][0] - chelyabinsk_time.hour())
                            },
    
                            // Minutes
                            if (chelyabinsk_work_time[1][1] - chelyabinsk_time.minute()) <= 10 {
                                format!("0{}", chelyabinsk_work_time[1][1] - chelyabinsk_time.minute())
                            } else {
                                format!("{}", chelyabinsk_work_time[1][1] - chelyabinsk_time.minute())
                            },
    
                            // Seconds
                            if (chelyabinsk_work_time[1][2] - chelyabinsk_time.second()) < 10 {
                                format!("0{}", chelyabinsk_work_time[1][2] - chelyabinsk_time.second())
                            } else {
                                format!("{}", chelyabinsk_work_time[1][2] - chelyabinsk_time.second())
                            }
                        )
                    } else {
                        format!("{}", fun::get_time_random_phrases())
                    }
                },
                _ => fun::err_command_random_phrases(),
            }
        }
        _ => fun::rest_time_phrases(),
    }

}

// TEST VERSION -------------------------------------------------------------------------------------------------------------------

pub fn testing() -> String {
    let dt = Utc::now();
    let week = dt.weekday().to_string();

    match week.as_str() {
        "Mon" | "Tue" | "Wed" | "Thu" | "Fri" => {
            format!("ok")
        },
        _ => {
            format!("ne ok")
        },
    }
}

// pub fn get_time() -> String {
//     let dt = Local::now();
//     let start_work_time = Utc.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 00, 00, 59).unwrap();
//     let end_work_time = Utc.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 23, 59, 59).unwrap();

//     if dt.hour() >= start_work_time.hour() && dt.hour() <= end_work_time.hour() {
//         let h = if end_work_time.hour() - dt.hour() < 10 {
//             format!("0{}", end_work_time.hour() - dt.hour())
//         } else {
//             format!("{}", end_work_time.hour() - dt.hour())
//         };

//         let m = if end_work_time.minute() - dt.minute() < 10 {
//             format!("0{}", end_work_time.minute() - dt.minute())
//         } else {
//             format!("{}", end_work_time.minute() - dt.minute())
//         };

//         let s = if 60 - dt.second() < 10 {
//             format!("0{}", 60 - dt.second())
//         } else {
//             format!("{}", 60 - dt.second())
//         };

//         // return format!("native time now: {}\ntime now: {}\nОсталось работать: {}:{}:{}", dt, time_format, h, m, s);
//         return format!("Время: {}:{}:{}\nДо конца рабочего дня осталось: {}:{}:{}",  dt.hour(), dt.minute(), dt.second(), h, m, s)
//     } else {
//         return format!("{}", fun::get_time_random_phrases());
//     }
// }

// ------------------------------------------------------------------------------------------------------------------- TEST VERSION

// let dt_test = chrono::offset::Utc::now();
// let start_test_time = Utc.with_ymd_and_hms(dt.year(), dt.month(), dt.day(),0, 0, 1);
// let end_test_time = Utc.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 23, 59, 59);