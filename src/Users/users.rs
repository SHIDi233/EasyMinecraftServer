include!("../Mapper/users.rs");
include!("../Standard/http.rs");
include!("../Standard/jwt.rs");

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]
struct User {
    username: String,
    password: String,
    code: i32,
}

#[derive(Serialize, Deserialize,Debug)]
struct LoginUser {
    username: String,
    password: String,
}

#[post("/users/login")]
async fn login(req_body: String) -> impl Responder {

    // testt();


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
        match create_token(json.username.clone()){
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
    let json: User = serde_json::from_str(str_ref).unwrap();

    println!("deserialized = {:?}", json);

    //code验证

    //创建用户
    match create_user(json.username,json.password,json.code){
        Ok(_) => return HttpResponse::Ok().body(success("用户创建成功！")),
        Err(e) => {
                    println!("err:{}",e);
                    return HttpResponse::Ok().body(error("用户名重复，请更换。"));
                }
    }
    //code关闭  

    HttpResponse::Ok().body(req_body) 
}