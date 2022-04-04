extern crate mysql;
// ...

use mysql as my;
use serde::{ Serialize};
use serde_json;

// #[derive(Debug, PartialEq, Eq)]
#[derive(Serialize)]
struct Users {
    user_id: i32,
    user_name: String,
}

pub fn get_users() -> String {

    let pool = my::Pool::new("mysql://root:1234@127.0.0.1:3306/test").unwrap();
    
    //方式2：将数据集取出存储在Vec中
    let selected_payments: Vec<Users> =
    pool.prep_exec("select user_id, user_name from users", ())
    .map(|result| { 
        result.map(|x| x.unwrap()).map(|row| {
            let (user_id, user_name) = my::from_row(row);
            Users {
                user_id: user_id,
                user_name: user_name
            }
        }).collect() 
    }).unwrap();
    let result = serde_json::to_string(&selected_payments).unwrap();
    // println!("{:?}",result);
    let final_result = String::from(result);
    // println!("{:?}",final_result);
    return final_result;
    // Ok(())
    // println!("{:?}",serde_json::to_string(&selected_payments));
    // println!("{:?}",selected_payments);
}
