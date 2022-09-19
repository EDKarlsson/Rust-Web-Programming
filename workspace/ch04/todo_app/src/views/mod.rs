use actix_web::web;

mod path;
mod auth;
mod to_do;
mod token;

pub fn views_factory(app: &mut web::ServiceConfig) {
/*    let args: Vec<String> = env::args().collect();
    let param: &String = &args[args.len() - 1];
    if param.as_str() == "cancel_logout" {
        println!("logout view isn't configured");
        auth::auth_factory(app, false);
    } else {
        println!("logout view is being configured");
        auth::auth_factory(app, true);
    }
*/
    auth::auth_factory(app);
    to_do::item_factory(app);
}