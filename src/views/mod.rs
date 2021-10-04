use actix_web::web;
use std::env;
mod auth;
mod path;

pub fn views_factory(app: &mut web::ServiceConfig) {
  let args: Vec<String> = env::args().collect();
  let param: &String = &args[args.len() - 1];
  if param.as_str() == "cancel_logut" {
    println!("logout view isn't being configured");
    auth::auth_factory(app, false);
  } else {
    println!("logout view is being configured");
    auth::auth_factory(app, true);
  }
}
