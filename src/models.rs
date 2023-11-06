use diesel::{Queryable, Insertable, AsChangeset}; 
use serde::{Serialize, Deserialize};
use super::schema::users; 
use super::schema::orders;
use diesel::prelude::*; 





// orders struct 
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub user_id: Option<i32>,
    pub product: String,
}








//  
#[derive( Debug, Queryable, Serialize,  Deserialize)]
pub struct User {
    pub id: i32, 
    pub name: String, 

}



// users
#[derive(Insertable, Queryable, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    
}


// update user
#[derive(Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: Option<String>,  

}




