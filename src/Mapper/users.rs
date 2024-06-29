use rusqlite::{params, Connection, Result, NO_PARAMS};
use rusqlite::Error;
use serde_json::to_string;
use serde_json::Value;

//Init Table
fn create_user_table() -> Result<()> {
    let conn = Connection::open("easymc.db")?;
    conn.execute(
        "CREATE TABLE if not exists users (
            username TEXT NOT NULL PRIMARY KEY,
            password TEXT NOT NULL,
            code INTEGER,
            role TEXT NOT NULL
        )",
        params![],
    )?;
    Ok(())
}

//For Register
fn create_user(username:String, password:String, code:i32) -> Result<(),Error> {
    let res:Result<(),Error>;
    let conn = Connection::open("easymc.db")?;
    let sql = "INSERT INTO users (username,password,code,role) VALUES (?1, ?2, ?3, 'user')";
    let mut stmt = conn.prepare(sql)?;
    match stmt.execute(params![username, password, code]){
        Ok(_) => res=Ok(()),
        Err(e) => res=Err(e)    
    }
    return res;
}


fn select_users() -> Result<String,Error> {
    println!("into");
    let conn = Connection::open("easymc.db")?;
    let mut stmt = conn.prepare("SELECT username,password,code,role FROM users")?;
    println!("1");
    let persons = stmt.query_map(params![],|row| {
        Ok(User{
            username: row.get(0)?,
            password: row.get(1)?,
            code: row.get(2)?,
            role: row.get(3)?,
        })
    })?;
    println!("2");


    // println!("{}",std::any::type_name_of_val(&persons));
    // let mut stmt = conn.prepare("SELECT * FROM users;")?;
    // let rows = stmt.mapped_rows::<serde_json::Value, _>(|row| {
    //     let mut json_row = serde_json::map::Map::new();
    //     for column in row.columns() {
    //         let name = column.name();
    //         let value = row.get_unwrap(name);
    //         json_row.insert(name.to_string(), serde_json::to_value(value).unwrap());
    //     }
    //     json_row.into()
    // })?;
 
    // for row in rows {
    //     let row = row?;
    //     println!("{:?}", row);
    // }

    // let mut res = "[".to_string();
    let mut res: Value = json!([]);
    for person in persons {
        // println!("{}",std::any::type_name_of_val(&person));
        let a:User;
        match person{
            Ok(value)=>{
                let json_value = json!({
                    "username": value.username,
                    "role": value.role,
                });
                res.as_array_mut().unwrap().push(json_value);
                // res=res+&json_value.to_string()+",";
            },
            Err(e) => {
                println!("err:{}",e);
            }
        }
    }
    // res=res+"]";
    println!("res:{:?}",res); 
    Ok(res.to_string())
}

//For Login
fn select_user_by_username(username:String) -> Result<User,Error> {
    let res:Result<User,Error>;
    let conn = Connection::open("easymc.db")?;
    let mut stmt = conn.prepare("SELECT username,password,code,role FROM users WHERE username = ?")?;
    let person_iter = stmt.query_map(params![username], |row| {
        Ok(User{
            username: row.get(0)?,
            password: row.get(1)?,
            code: row.get(2)?,
            role: row.get(3)?,
        })
    })?;
    let (min_len, max_len) = person_iter.size_hint();
    
    for person in person_iter{
        res=person;
        return res;
        break;
    }
    if max_len==Some(0){
        res=Ok(User{
            username: "null".to_string(),
            password: "null".to_string(),
            code: 0 ,
            role: "vistor".to_string()
        });
        return res; 
    }
    else{
        res=Ok(User{
            username: "null".to_string(),
            password: "null".to_string(),
            code: 0 ,
            role: "vistor".to_string()
        });
        return res;
    }
    
}