include!("Users/users.rs");
include!("Minecraft/minecraft.rs");
include!("Standard/mcapi.rs");
use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::io::{BufRead, BufReader};  

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::sync::Mutex; 
use once_cell::sync::OnceCell;
use std::io::Read;

use actix_files as fs;

static TX: OnceCell<Mutex<Sender<String>>> = OnceCell::new();  
static RX: OnceCell<Mutex<Receiver<String>>> = OnceCell::new();  

#[derive(Serialize, Deserialize,Debug,Clone)]
struct User {
    username: String,
    password: String,
    code: i32,
    role:String,
}

#[derive(Serialize, Deserialize,Debug)]
struct _User {
    username: String,
    password: String,
    code: i32,
}

#[derive(Serialize, Deserialize,Debug)]
struct LoginUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize,Debug)]
struct Cmd {
    data: String,
    token: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    println!("服务器初始化中...");
    println!("开始自检流程---↓");

    let (tx, rx) = mpsc::channel();

    //初始化TX
    TX.get_or_init(||{
        Mutex::new(tx)   //tx所有权转移到全局变量TX
    });
    println!("|管道加载");


    let handle = thread::spawn( move || {
        fn m(rx:Receiver<String>) -> std::io::Result<()>{
            let mut shell = Command::new("F:\\bedrock-server-1.21.1.03\\bedrock_server.exe")  
                .stdin(Stdio::piped()) // 标准输入被管道取代
                // .stdout(Stdio::piped()) // 标准输出被管道取代
                .stdout(io::stdout()) // 标准输出被屏幕取代
                .spawn()?;

            let mut stdin = shell.stdin.as_mut();
            let mut stdout = shell.stdout.as_mut();

            // let output = shell   
            //     .wait_with_output()
            //     .expect("failed to wait on child");


            println!("---服务器交互已启动---"); 
            while let Ok(method) = rx.recv(){
                print!("rec:{}",method);
                // println!("{:?}", method.type_id());
                // if let Some(ref mut stdout) = stdout {
                    
                // }
                // if let Some(ref mut stdout) = stdout {
                //     // drop(stdout.get_mut());
                    
                //     // stdout.flush();
                //     let mut f = BufReader::new(stdout); 
                //     let mut buf = vec![];
                //     println!("len:{}",num_bytes = f.read_until(b'\n', &mut buf) 
                //         .expect("reading from cursor won't fail"));
                //     while(true){
                //         let mut line = String::new();
                //         let len = f.read_line(&mut line)?;
                //         print!("len:{} ",len); 
                //         println!("text:{}",line); 
                //         if len==0 {
                //             break;  
                //         }
                //     }
                // }
                

                if let Some(ref mut stdin) = stdin {
                    writeln!(stdin, "{}",method.as_str())?;
                    // drop(stdin);
                    // let output = shell.wait_with_output().unwrap();
                    // let mut buf = String::new();
                    // // f.flush();
                    // match f.read_line(&mut buf) {
                    //     Ok(_) => {
                    //         print!("callback--->");
                    //     }
                    //     Err(e) => print!("error callback--->: {:?}", e),
                    // }
                    // println!("result:{}",buf);

                    // if let Some(ref mut stdout) = stdout {

                    //     println!("result:{}",stdout.stdout.as_slice())
                    // }
                    // println!("Here is your result : {:?}", stdout);
                }
                println!("下一条--->");
            }
            Ok(())
        } 
        m(rx);
    });
    
    println!("|守护线程启动");
    println!("自检结束---");
    println!("---API服务已启动---");
    HttpServer::new( move || {
        App::new()
            .service(login)
            .service(register)
            .service(kickoff)
            .service(op)
            .service(deop)
            .service(add_user)
            .service(del_user)
            .service(notice)
            .service(stop)
            .service(list)
            .service(fs::Files::new("/res", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()  
    .await
}