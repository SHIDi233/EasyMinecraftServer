use rusqlite::{params, Connection, Result, NO_PARAMS};
use rusqlite::Error;


//Init Table
fn create_user_table() -> Result<()> {
    let conn = Connection::open("easymc.db")?;
    conn.execute(
        "CREATE TABLE if not exists users (
            username TEXT NOT NULL PRIMARY KEY,
            password TEXT NOT NULL,
            code INTEGER
        )",
        params![],
    )?;
    Ok(())
}

//For Register
fn create_user(username:String, password:String, code:i32) -> Result<(),Error> {
    let res:Result<(),Error>;
    let conn = Connection::open("easymc.db")?;
    let sql = "INSERT INTO users (username,password,code) VALUES (?1, ?2, ?3)";
    let mut stmt = conn.prepare(sql)?;
    match stmt.execute(params![username, password, code]){
        Ok(_) => res=Ok(()),
        Err(e) => res=Err(e)    
    }
    return res;
}


// fn select_users() -> Result<()> {
//     let conn = Connection::open("easymc.db")?;
//     let mut stmt = conn.prepare("SELECT id, name, age FROM person WHERE age > ?")?;
//     let person_iter = stmt.query_map(params![20], |row| {
//         Ok(Person {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             age: row.get(2)?,
//         })
//     })?;

//     for person in person_iter {
//         println!("Found person: {:?}", person?);
//     }
//     Ok(())
// }

//For Login
fn select_user_by_username(username:String) -> Result<User,Error> {
    let res:Result<User,Error>;
    let conn = Connection::open("easymc.db")?;
    let mut stmt = conn.prepare("SELECT username,password,code FROM users WHERE username = ?")?;
    let person_iter = stmt.query_map(params![username], |row| {
        Ok(User{
            username: row.get(0)?,
            password: row.get(1)?,
            code: row.get(2)?,
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
        });
        return res; 
    }
    else{
        res=Ok(User{
            username: "null".to_string(),
            password: "null".to_string(),
            code: 0 ,
        });
        return res;
    }
    
}