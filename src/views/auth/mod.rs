use actix_web::web;

use super::path::Path;

mod login;
mod logout;

pub fn auth_factory(app: &mut web::ServiceConfig, logout: bool) {
  let base_path: Path = Path {
    prefix: String::from("/auth"),
  };

  app.route(
    &base_path.define(String::from("/login")),
    web::get().to(login::login),
  );
  if logout {
    app.route(
      &base_path.define(String::from("/logout")),
      web::get().to(logout::logout),
    );
  }
}