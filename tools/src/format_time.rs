use chrono::format::StrftimeItems;
use chrono::format::DelayedFormat;
use chrono::prelude::*;
use std::time::Duration;
//打印格式化时间
pub fn print_format_time(){
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now: DateTime<Local> = Local::now();
    let dft: DelayedFormat<StrftimeItems> = now.format(fmt);
    let str_date: String = dft.to_string(); // 2021-01-04 20:02:09
    println!("now: {}", str_date);
    sleep_ints(1);
}
pub fn print_format_time_str()->String{
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now: DateTime<Local> = Local::now();
    let dft: DelayedFormat<StrftimeItems> = now.format(fmt);
    let str_date: String = dft.to_string(); // 2021-01-04 20:02:09
    return str_date;
}
pub fn sleep_ints(i:i32){
    std::thread::sleep(Duration::from_secs(i.try_into().unwrap()));
}


