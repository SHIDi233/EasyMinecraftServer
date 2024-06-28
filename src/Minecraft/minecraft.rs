#[delete("/api/kickoff")]
async fn kickoff(req_body: String) -> impl Responder {

    let str_ref: &str = req_body.as_str();
    let json: Cmd = serde_json::from_str(str_ref).unwrap();

    //通行请求
    let role: String;
    match analysis_token_role(json.token.clone()){
        Ok(_v)=>role=_v,
        Err(_e)=>{println!("err:{}",_e);return HttpResponse::Ok().body(error("token错误！"));},
    }
    if(role=="owner" || role=="administrator"){

    }
    else{
        return HttpResponse::Ok().body(error("权限错误!"));
    }

    if let Some(mutex) = TX.get(){
        if let Ok(tx) = mutex.lock()
        {
            let tx = tx.clone();
            let _ = tx.send("kick ".to_string()+&json.data+" testkick \n");//发送
            println!("服务器踢出：{}",json.data);
        }
    }
    return HttpResponse::Ok().body(success("success"));
}   

#[post("/api/op")]
async fn op(req_body: String) -> impl Responder {

    let str_ref: &str = req_body.as_str();
    let json: Cmd = serde_json::from_str(str_ref).unwrap();

    //通行请求
    let role: String;
    match analysis_token_role(json.token.clone()){
        Ok(_v)=>role=_v,
        Err(_e)=>{println!("err:{}",_e);return HttpResponse::Ok().body(error("token错误！"));},
    }
    if(role=="owner" || role=="administrator"){

    }
    else{
        return HttpResponse::Ok().body(error("权限错误!"));
    }

    if let Some(mutex) = TX.get(){
        if let Ok(tx) = mutex.lock()
        {
            let tx = tx.clone();
            let _ = tx.send("op ".to_string()+&json.data+" \n");//发送
            println!("服务器授予用户管理员权限：{}",json.data);
        }
    }
    return HttpResponse::Ok().body(success("success"));
}   

#[delete("/api/deop")]
async fn deop(req_body: String) -> impl Responder {
    
    let str_ref: &str = req_body.as_str();
    let json: Cmd = serde_json::from_str(str_ref).unwrap();

    //通行请求
    let role: String;
    match analysis_token_role(json.token.clone()){
        Ok(_v)=>role=_v,
        Err(_e)=>{println!("err:{}",_e);return HttpResponse::Ok().body(error("token错误！"));},
    }
    if(role=="owner" || role=="administrator"){

    }
    else{
        return HttpResponse::Ok().body(error("权限错误!"));
    }

    if let Some(mutex) = TX.get(){
        if let Ok(tx) = mutex.lock()
        {
            let tx = tx.clone();
            let _ = tx.send("deop ".to_string()+&json.data+" \n");//发送
            println!("服务器取消授予用户管理员权限：{}",json.data);
        }
    }
    return HttpResponse::Ok().body(success("success"));
}   

#[post("/api/add")]
async fn add_user(req_body: String) -> impl Responder {
    
    let str_ref: &str = req_body.as_str();
    let json: Cmd = serde_json::from_str(str_ref).unwrap();

    //通行请求
    let role: String;
    match analysis_token_role(json.token.clone()){
        Ok(_v)=>role=_v,
        Err(_e)=>{println!("err:{}",_e);return HttpResponse::Ok().body(error("token错误！"));},
    }
    if(role=="owner" || role=="administrator"){

    }
    else{
        return HttpResponse::Ok().body(error("权限错误!"));
    }

    if let Some(mutex) = TX.get(){
        if let Ok(tx) = mutex.lock()
        {
            let tx = tx.clone();
            let _ = tx.send("allowlist add ".to_string()+&json.data+" \n");
            println!("服务器添加用户：{}",json.data);
        }
    }
    return HttpResponse::Ok().body(success("success"));
}   

#[delete("/api/del")]
async fn del_user(req_body: String) -> impl Responder {
    
    let str_ref: &str = req_body.as_str();
    let json: Cmd = serde_json::from_str(str_ref).unwrap();

    //通行请求
    let role: String;
    match analysis_token_role(json.token.clone()){
        Ok(_v)=>role=_v,
        Err(_e)=>{println!("err:{}",_e);return HttpResponse::Ok().body(error("token错误！"));},
    }
    if(role=="owner" || role=="administrator"){

    }
    else{
        return HttpResponse::Ok().body(error("权限错误!"));
    }
    if let Some(mutex) = TX.get(){
        if let Ok(tx) = mutex.lock()
        {
            let tx = tx.clone();
            let _ = tx.send("allowlist remove ".to_string()+&json.data+" \n");
            println!("服务器删除用户：{}",json.data);
        }
    }
    return HttpResponse::Ok().body(success("success"));
}   

#[post("/api/notice")]
async fn notice(req_body: String) -> impl Responder {
        
    let str_ref: &str = req_body.as_str();
    let json: Cmd = serde_json::from_str(str_ref).unwrap();

    //通行请求
    let role: String;
    match analysis_token_role(json.token.clone()){
        Ok(_v)=>role=_v,
        Err(_e)=>{println!("err:{}",_e);return HttpResponse::Ok().body(error("token错误！"));},     
    }
    println!("{}",role);
    if(role=="owner" || role=="administrator"){

    }
    else{
        return HttpResponse::Ok().body(error("权限错误!"));
    }

    if let Some(mutex) = TX.get(){
        if let Ok(tx) = mutex.lock()
        {
            let tx = tx.clone();
            let _ = tx.send("say ".to_string()+&json.data+" \n");
            println!("服务器发布广播信息：{}",json.data);
        }
    }
    return HttpResponse::Ok().body(success("success"));
}   