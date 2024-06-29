include!("../Mapper/users.rs");
include!("../Standard/http.rs");
include!("../Standard/jwt.rs");

use actix_web::{get, post, delete, put,  web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};


#[post("/users/login")]
async fn login(req_body: String) -> impl Responder {
    create_user_table();
    let str_ref: &str = req_body.as_str();
    let json: LoginUser = serde_json::from_str(str_ref).unwrap();

    let user_waitting_login: User;
    match select_user_by_username(json.username.clone()){
        Ok(_v)=>user_waitting_login=_v, 
        Err(_e)=>{println!("err:{}",_e);return HttpResponse::Ok().body(error("用户名错误或不存在！"));},
    }
    if(user_waitting_login.password==json.password.clone()){
        println!("密码正确");
        //jwt下发
        let token:String;
        match create_token(json.username.clone(),user_waitting_login.role.clone()){
            Ok(_v)=>token=_v,
            Err(_e)=>{println!("err:{}",_e);return HttpResponse::Ok().body(error("token分发错误，请联系系统管理员！"));},
        }
        return HttpResponse::Ok().body(success(token.as_str()));

    }
    else{
        println!("密码错误");
        return HttpResponse::Ok().body(error("用户名或密码错误!"));
    }
}

#[post("/users/register")]
async fn register(req_body: String) -> impl Responder {
    create_user_table();
    println!("{}",req_body);
    let str_ref: &str = req_body.as_str();
    let json: _User = serde_json::from_str(str_ref).unwrap();

    println!("deserialized = {:?}", json);

    //code验证

    //创建用户
    match create_user(json.username.clone(),json.password,json.code){
        Ok(_) =>  { 
            //mc白名单添加
            if let Some(mutex) = TX.get(){
                if let Ok(tx) = mutex.lock()
                {
                    let tx = tx.clone();
                    let _ = tx.send("allowlist add ".to_string()+&json.username+" \n");
                    println!("服务器添加用户：{}",json.username);
                }
            }
            return HttpResponse::Ok().body(success("用户创建成功！"));
        },
        Err(e) => {
                    println!("err:{}",e);
                    return HttpResponse::Ok().body(error("用户名重复，请更换。"));
                }
    }
    //code关闭  

    

    HttpResponse::Ok().body(req_body) ;
}

#[get("/users")]
async fn list(req_body: String) -> impl Responder {
    //code验证

    //创建用户
    match select_users(){
        Ok(temp) => return HttpResponse::Ok().body(success_json(&temp.to_string())),
        Err(e) => {
                    println!("err:{}",e);
                    return HttpResponse::Ok().body(error("用户名重复，请更换。"));
                }
    }
    //code关闭  

    HttpResponse::Ok().body(req_body) 
}