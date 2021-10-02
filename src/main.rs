// use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");

//     format!("Hello  {}!", name)
// }
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         println!("Closure is being called");
//         let app = App::new()
//             .route("/", web::get().to(greet))
//             .route("/{name}", web::get().to(greet));
//             app
//     })
//     .bind("localhost:3020")?
//     .run()
//     .await
// }

use std::{thread, time};
use std::thread::JoinHandle;

fn do_something(num: i8) -> i8 {
    println!("number {} is running", num);

    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}

fn main() {
    let now = time::Instant::now();
    let one: i8 = do_something(1);
    let two: i8 = do_something(2);
    let three: i8 = do_something(3);

    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", one + two + three);
}
