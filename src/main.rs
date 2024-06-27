include!("Users/users.rs");
include!("Standard/mcapi.rs");
use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let (tx, rx) = mpsc::channel::<>(); 

    let handle1 = thread::spawn( move || {
        fn m() -> std::io::Result<()>{
            println!("{}",1);
            let mut shell = Command::new("F:\\bedrock-server-1.21.1.03\\bedrock_server.exe")
                .stdin(Stdio::piped()) // 标准输入被管道取代
                .stdout(Stdio::piped()) // 标准输出被管道取代
                .spawn()?;
            println!("{}",2);
            // let mut stdin = shell.stdin.take().expect("Failed to take stdin"); 
            let mut stdin = shell.stdin.as_mut();
            while(true){ 
                if let Some(ref mut stdin) = stdin {
                    writeln!(stdin, "kick hehehe3274 testkick \n")?;
                    drop(stdin);
                }
                thread::sleep(Duration::from_secs(30));
                println!("tick");
            }
            Ok(())
        }

        println!("{}",0 );
        m();           
        println!("{}",4 ); 
    });
    

    HttpServer::new( move || {
        App::new()
            .service(login)
            .service(register)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}