use worktable::WorkerInfo;
use tools::{
    format_time::print_format_time,
    coder::my_serialize,
    shell::curl,
    write_log::write_log
};
use std::{sync::mpsc,io,thread,net::UdpSocket,collections::HashMap};

fn wait_for_fd() { print_format_time();print!("wait_for_fd");}
fn main(){
    let socket = UdpSocket::bind("127.0.0.1:8082").unwrap();
    socket.set_nonblocking(true).unwrap();
   
    let mut buf = [0u8;512];
    let mut wl:HashMap<i32,WorkerInfo> = HashMap::new();
    let mut thread_vec:Vec<thread::JoinHandle<()>> = Vec::new();
    let (tx, rx) = mpsc::channel(); // 创建通道
    loop{ 
        let (num_bytes_read, src) = 'a:loop {
            let mut delete_hashmap_id:Vec<i32> = Vec::new();
            tx.send("1".to_string()).unwrap(); // 发送val  
            let received = rx.recv().unwrap();
            for (k,v) in wl.iter_mut(){
                if v.surplus_time == 0 {
                    let worker_command_clone = v.command.clone();
                    let  handle_rep = thread::spawn(move ||{
                        //切割命令 |curl_www.baidu.com|
                        let command = worker_command_clone.split("_").collect::<Vec<&str>>();
                        write_log(curl(command[0],command[1]),"log.txt");
                    });
                    //把线程放到 vec
                  //  thread_vec.push(handle_rep);
                    handle_rep.join();
                    v.surplus_time = v.time;
                    if !v.is_forever{  //不是永久执行 只执行一次 踢出任务队列
                        let id = *k;delete_hashmap_id.push(id);
                    }
                }  else {
                    v.surplus_time -=1;
                }
            }
            print_format_time();
            dbg!(&wl);
            if delete_hashmap_id.len() > 0{        //删除不在执行的任务
                for index in &delete_hashmap_id{
                    wl.remove(index);
                }
            }
            if received !="1".to_owned(){
                let command = received.split("|").collect::<Vec<&str>>();
                let id:i32 = command[1].trim().parse().expect("字符串转整型失败");
               // let wi:WorkerInfo = WorkerInfo::default();
                let i32time = match command[2].trim().parse::<i32>() { Ok(t)=> t, Err(E)=>panic!("字符串转整型失败"),};
                let is_forever = command[3].trim().parse::<bool>().expect("字符串类型转布尔类型错误");
                let  wi = WorkerInfo{time:i32time,surplus_time:i32time,name:command[4].to_string(),command:command[5].to_string(),is_forever:is_forever};     
                //添加和更新都用insert        
                wl.insert(id,wi);
            }   
            match socket.recv_from(&mut buf) {
                Ok(n) => break n,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {continue 'a;}
                Err(e) => panic!("encountered IO error: {}",e),
            }
        };
        let str = String::from_utf8_lossy(&buf[..num_bytes_read]);
        let input = str.split("|").collect::<Vec<&str>>();
        let handle = input[0]; //add   update  delete  list
        if handle == "delete".to_owned(){ //删除任务直接删除
            let id:i32 = input[1].trim().parse().expect("字符串转整型失败");
            wl.remove(&id);
        } else if handle == "list".to_owned() {
            //序列化格式返回  let ser = my_serialize(&wl);
        } else {
            tx.send(str.to_string()).unwrap(); // 发送val 
        }
        socket.send_to(b"OK", src);
    }
}
