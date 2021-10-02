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

use std::thread::JoinHandle;
use std::{thread, time};

fn do_something(num: i8) -> i8 {
    println!("number {} is running", num);

    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}

fn main() {
    let now = time::Instant::now();
    let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
    let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
    let thread_three: JoinHandle<i8> = thread::spawn(|| do_something(3));

    let result_one = thread_one.join();
    let result_two = thread_two.join();
    let result_three = thread_three.join();

    println!("time elapsed {:?}", now.elapsed());
    println!(
        "result {}",
        result_one.unwrap() + result_two.unwrap() + result_three.unwrap()
    );
}
