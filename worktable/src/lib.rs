use serde::{Deserialize,Serialize};
use std::collections::HashMap;
#[derive(Serialize,Deserialize,Debug,Clone,PartialEq,Eq)]
pub struct WorkerInfo{
   pub  time:i32,                         //任务执行时间 多少秒执行一次
   pub  surplus_time:i32,               //任务执行剩余时间 还剩多少秒执行
   pub  command:String,                  //任务执行命令
   pub  name:String,                     //任务名称
   pub  is_forever:bool                  //是否永久执行   
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn map_add(){
        let mut wl:HashMap<i32,WorkerInfo> = HashMap::new();
        assert_eq!(wl.len(), 0);
        let wi=WorkerInfo{
            time:10,surplus_time:10,
            command:"curl www.baidu.com".to_string(),name:String::from("测试任务"), is_forever:false};
        wl.insert(0, wi);
        assert_eq!(wl.len(), 1);
        let wi1=WorkerInfo{
            time:10,surplus_time:10,
            command:"curl www.baidu.com".to_string(),name:String::from("测试任务"), is_forever:false};
        assert_eq!(wl[&0],wi1);
    }
}
