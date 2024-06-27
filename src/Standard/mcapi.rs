


 
// fn testt() -> io::Result<()> {
//     println!("{}",1.1);
//     // 创建一个子进程来运行shell


//     // let output_1 = shell.wait_with_output()?;
//     // if !output_1.status.success() {
//     //     println!("Command failed to execute successfully");
//     // } else { 
//     //     let output_1 = String::from_utf8_lossy(&output_1.stdout);
//     //     println!("Command output: {}", output_1);
//     // }
 
//     // 获取可写的标准输入流
//     let mut stdin = shell.stdin.take().expect("Failed to take stdin");
//     println!("{}",1.2);
//     // 写入要执行的命令
//     writeln!(stdin, "kick 111 testkick \n")?;
//     // writeln!(stdin, "echo 'Another command'")?;
//     println!("{}",1.3);
//     // 关闭输入流以发送EOF给shell
//     // drop(stdin);
//     println!("{}",1.4);
//     // 获取输出并打印
//     // let output = shell.wait_with_output()?;
//     drop(stdin);
//     // if !output.status.success() {
//     //     println!("Command failed to execute successfully");
//     // } else {
//     //     let output = String::from_utf8_lossy(&output.stdout);
//     //     println!("Command output: {}", output);
//     // }
//     println!("执行完毕！");
//     Ok(())
// }