use rocket_cors;
use rocket_cors::{ AllowedOrigins, AllowedHeaders };
use rocket::http::Method;

pub fn options() -> rocket_cors::Cors {
    rocket_cors::Cors {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
}
