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

use futures::executor::block_on;
use futures::join;
use std::{thread, time};

async fn do_something(num: i8) -> i8 {
    println!("number {} is running", num);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}

fn main() {
    // let future_one = do_something(1);
    // let outcome = block_on(future_one);
    // A bit verbose but gives extra functionality
    // let future_one = async { do_something(2).await };
    // let future_one = block_on(future_one);
    // println!("Here is the outcome for 2 {:?},", future_one);
    // Two futures in an async block
    let future_two = async {
        join!(do_something(3),do_something(4))
    };
    let now_two = time::Instant::now();
    let future_two = block_on(future_two);
    println!("Here is the outcome for three {:?}", future_two);
    println!("Time elapsed {:?}", now_two.elapsed());

    let future_three = async {
        [do_something(5).await, do_something(6).await]
    };
    let now_three = time::Instant::now();
    let future_three = block_on(future_three);
    println!("Here is the outcome for three {:?}", future_three);
    println!("Time elapsed {:?}", now_three.elapsed());

}
