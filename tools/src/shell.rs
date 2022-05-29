use std::process::{self, Command, Stdio};
//执行shell
pub fn curl(command:&str,arg:&str)->String{
  //  let output = Command::new("curl").arg("www.baidu.com").output().expect("执行异常，提示");
    let output = Command::new(command).arg(arg).output().expect("执行异常，提示");
    let out = String::from_utf8(output.stdout).unwrap();
    return out;
}
