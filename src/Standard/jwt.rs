use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Validation, Header};
 
// 载荷数据结构
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    exp: usize,
}
 
fn test() -> Result<(), jsonwebtoken::errors::Error> {
    let secret_key = b"secret"; // 密钥，应当保密
    let token_claimes = Claims {
        username: "username".to_owned(),   
        exp:10000000000000000.to_owned(),
    };
 
    // 创建JWT
    let token = encode(&Header::default(), &token_claimes, &EncodingKey::from_secret("secret".as_ref()))?;
    println!("JWT: {}", token);
    
    // 解码JWT
    let decoded_token = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
    println!("Decoded token: {:?}", decoded_token.claims);
 
    Ok(())
}

fn create_token(username:String) -> Result<String, jsonwebtoken::errors::Error>{
    let secret_key = b"secret"; // 密钥，应当保密
    let token_claimes = Claims {
        username: username.to_owned(),   
        exp:10000000000000000.to_owned(),
    };
    let token = encode(&Header::default(), &token_claimes, &EncodingKey::from_secret("secret".as_ref()))?;
    return Ok(token);
}

fn analysis_token(token:String) -> Result<String, jsonwebtoken::errors::Error>{
    // 解码JWT
    let decoded_token = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
    println!("Decoded token: {:?}", decoded_token.claims);  
    return Ok(decoded_token.claims.username);            
}