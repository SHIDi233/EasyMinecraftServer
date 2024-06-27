use serde_json::json;

fn success(body: &str) -> String{
    let json_value = json!({
        "code": 0,
        "msg": "success",
        "data": body
    });
    return json_value.to_string();
}

fn noauth(body: &str) -> String{
    let json_value = json!({
        "code": 0,
        "msg": "noauth",
        "data": "没有访问权限"
    });
    return json_value.to_string();
}

fn error(body: &str) -> String{
    let json_value = json!({
        "code": 0,
        "msg": "error",
        "data": body
    });
    return json_value.to_string();
}