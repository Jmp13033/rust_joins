#![feature(proc_macro_hygiene, decl_macro)]
mod models; 
mod schema;
#[macro_use]
extern crate diesel;
use diesel::prelude::*; 
use diesel::pg::PgConnection; 

use rocket::{get, post, put, delete, routes}; 
use rocket_contrib::json::{Json, JsonValue};

use std::env; 
use dotenvy::dotenv; 

use crate::schema::users::dsl::users;
use crate::schema::orders::dsl::orders;



use serde_json::json;
use models::{User, NewUser, UpdateUser, Order}; 

// connection to postgres 
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[get("/users")]
pub fn get_users() -> Json<JsonValue> {
    let connection = establish_connection();

    let get_users = users.load::<User>(&connection)
    .expect("Error loading user");

    Json(JsonValue::from(json!({
        "users": get_users,
    })))
}


#[post("/create_user", format="json", data="<new_user>")]
pub fn create_user(new_user:Json<NewUser>) -> Json<JsonValue> {
     // create connection 
    let connection = establish_connection(); 

    // create users
    let new_user = NewUser {
        name: new_user.name, 

    }; 

     // added
     diesel::insert_into(crate::schema::users::dsl::users)
     .values(&new_user)
     .execute(&connection)
     .expect("Error saving new users");


 // put json here  
    Json(JsonValue::from(json!({
     "status": "success",
     "message": "user has been created",

 })))




} 


// update users
#[put("/users/<id>", data = "<update_data>")]
pub fn update_users(id: i32, update_data: Json<UpdateUser>) ->Json<JsonValue> 
{
    let connection = establish_connection();

    // Use the `update` method of the Diesel ORM to update 
    // the student's record
    let _updated_users = diesel::update(users.find(id))
        .set(&update_data.into_inner())
        .execute(&connection)
        .expect("Failed to update users");

    // Return a JSON response indicating success
    Json(JsonValue::from(json!({
        "status": "success",
        "message": format!("users {} has been updated", id ),
    })))
}



// delete user
#[delete("/user/<id>")]
pub fn delete_user(id: i32) -> Json<JsonValue> {
    
    let connection = establish_connection();

    diesel::delete(users.find(id)).execute(&connection).
    expect(&format!("Unable to find users {}", id));

    Json(JsonValue::from(json!({
        "status": "success",
        "message": format!("users with ID {} has been deleted", id),
    })))
}




// orders 

#[get("/orders")]
pub fn get_order() -> Json<JsonValue> {
    let connection = establish_connection();

    let get_orders = orders.load::<Order>(&connection)
    .expect("Error loading user");

    Json(JsonValue::from(json!({
        "orders": get_orders,
    })))
}


// combined routes to get both the infornation from the tables 
#[get("/combined")]
pub fn get_combined_data() -> Json<JsonValue> {
    let connection = establish_connection();

    use crate::schema::users::dsl::*;
    use crate::schema::orders::dsl::*;

    let results = users
        .inner_join(orders)
        // get the data from both Tables 
        .load::<(User, Order)>(&connection)
        // expect for error handling; 
        .expect("Error to be handled");

    let combined_data: Vec<JsonValue> = results
        .iter()
        .map(|(user, order)| {
            json!({
                "user_id": user.id,
                "user_name": user.name,
                "order_id": order.id,
                "product": order.product,
            }).into()
        })
        .collect(); 

    Json(JsonValue::from(json!({
        "combined_data": combined_data,
    })))
}


fn main() {
    rocket::ignite().mount("/", routes![
        create_user, get_users, update_users, delete_user, get_order, get_combined_data
]).launch();
}
