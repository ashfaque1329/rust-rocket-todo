/*

use rocket::response::Redirect;

Redirect::to(uri!(login))

 */
#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
use rocket::serde::{Serialize,Deserialize, json::Json};

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[derive(Database)]
#[database("sharks")]
struct Sharks(sqlx::SqlitePool);

#[get("/hello/<name>")]
async fn hello(name:&str)-> String{
    format!("Hello, {}!",name)
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/dummy?<name>&<color>")]
async fn dummy(name: &str,color:&str) -> String {
    format!("Name:{}, Color:{}", name,color)
}

#[get("/real_dummy")]
async fn real_dummy(mut db: Connection<Sharks>) -> Option<String> {
    sqlx::query("SELECT name FROM sharks")
        .fetch_one(&mut *db).await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    description: &'r str,
    complete: bool
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Body<'r> {
    name: &'r str,
    is_admin: bool
}


#[post("/todo", data = "<task>")]
async fn todo(task: Json<Task<'_>>) -> Json<Body> {
    if task.description=="testing"{
        Json(Body{ name:"Ashfaque", is_admin: true })
    } else {
        Json(Body{ name:"John", is_admin: false })
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(Sharks::init())
        .mount("/", routes![hello,delay,dummy,real_dummy,todo])
        .launch()
        .await?;

    Ok(())
}
