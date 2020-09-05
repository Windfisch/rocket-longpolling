#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rocket;

use rocket::State;
use std::sync::Mutex;
use std::time::{Duration,Instant};
use uuid::Uuid;
use async_notify::Notify;
use std::sync::Arc;
use async_std;
use rocket::http::Method;


use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType};
use std::io::Cursor;

pub struct CORS();


#[rocket::async_trait]
impl Fairing for CORS {
	fn info(&self) -> Info {
		Info {
			name: "Add CORS headers to requests",
			kind: Kind::Response
		}
	}

	async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
		//if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
			response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:8080"));
			response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
			response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
			response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
		//}

		if request.method() == Method::Options {
			response.set_header(ContentType::Plain);
			response.set_sized_body(0, Cursor::new(""));
		}
	}
}



struct User {
	name: String,
	uuid: String,
	last_active: Instant,
	notification: Arc<Notify>
}

use std::collections::HashMap;

struct ActiveUsers {
	users: HashMap<String, User>
}

#[get("/login?<name>")]
fn login(users: State<Mutex<ActiveUsers>>, name: String) -> String {
	let mut data = users.lock().unwrap();
	let uuid = Uuid::new_v4().to_hyphenated().to_string();
	data.users.insert( uuid.clone(), User {
		name: name,
		uuid: uuid.clone(),
		last_active: Instant::now(),
		notification: Arc::new(Notify::new())
	} );
	return uuid;
}

#[get("/stuff?<uuid>&<num>")]
fn stuff(users: State<Mutex<ActiveUsers>>, uuid: String, num: u32) -> String {
	let mut data = users.lock().unwrap();
	match data.users.get_mut(&uuid) {
		Some(user) => {
			user.last_active = Instant::now();
			return format!("Hello, {}. Your number is {}", user.name, num);
		}
		None => {
			return "Who are you?".to_string();
		}
	}
}

#[get("/notify?<uuid>")]
fn notify(users: State<Mutex<ActiveUsers>>, uuid: String) -> String {
	let mut data = users.lock().unwrap();
	match data.users.get_mut(&uuid) {
		Some(user) => {
			user.notification.notify();
			return "Ok".to_string();
		}
		None => {
			return "Who is that?".to_string();
		}
	}
}

#[get("/poll?<uuid>&<seconds>")]
async fn poll(users: State<'_, Mutex<ActiveUsers>>, uuid: String, seconds: u64) -> String {
	let notif_opt = {
		users.lock().unwrap().users.get_mut(&uuid).map(|u| u.notification.clone())
	};
	match notif_opt {
		Some(notification) => {
			match async_std::future::timeout(Duration::from_secs(seconds), notification.notified()).await {
				Ok(_) => "Done".to_string(),
				Err(_) => "Timeout".to_string()
			}
		}
		None => {
			return "Who are you?".to_string()
		}
	}
}

#[get("/")]
async fn hello() -> &'static str {
	println!("hello");
	async_std::task::sleep(Duration::from_secs(3)).await;
	println!("world");
    "Hello, world!"
}

#[rocket::launch]
fn rocket() -> rocket::Rocket {
	let users = Mutex::new( ActiveUsers{ users: HashMap::new() } );

	rocket::ignite()
		.manage(users)
		.mount("/", routes![hello,login,stuff,poll,notify])
		.attach(CORS())
}
