// SCALABLE
// use actix_web::{App, HttpServer};
// mod views;

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         let app = App::new().configure(views::views_factory);
//         app
//     })
//     .bind("localhost:3030")?
//     .run()
//     .await
// }

// Not so scalable

use actix_web::{web, App, HttpServer};

pub async fn logout() -> String {
    format!("Logout cancel_logout")
}

pub async fn login() -> String {
    format!("Login cancel_login")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        let app = App::new()
            .route("/auth/login", web::get().to(login))
            .route("/auth/logout", web::get().to(logout));
        app
    })
    .bind("localhost:3030")?
    .run()
    .await
}
