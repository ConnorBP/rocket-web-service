//A very simple hello world api web service in rust using Rocket

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> &'static str {
    "👋 🌍! 🚀"
}

#[get("/<username>")]
fn api(username: String) -> String {
    format!("👋, {}! 🦀🚀", username)
}

//another method of serving up files to allow more control:
/* #[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
} */

fn main() {
    println!("Starting the Hello World AsS (As a Service)! c:");
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/api", routes![api])
    .mount("/public", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
    .launch();
}
