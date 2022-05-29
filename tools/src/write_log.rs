use std::io::prelude::*;
use std::fs::OpenOptions;
use crate::format_time::print_format_time_str;
/*
 写入日志
 */
pub fn write_log(data:String,file_name:&str) ->std::io::Result<()>{
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // 新建，若文件存在则打开这个文件
        .append(true) // 追加内容
        .open(file_name)?;
    f.write("记录日志时间:".to_string().as_bytes())?;
    f.write(print_format_time_str().as_bytes())?;
    f.write("\n".to_string().as_bytes())?;
    f.write(data.as_bytes())?;
    Ok(())
}
