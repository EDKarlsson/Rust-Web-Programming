mod processes;
mod state;
mod to_do;
mod views;

use actix_web::{App, HttpServer};

#[allow(unused, unused_mut)]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let command: &String = &args[1];
//     let title: &String = &args[2];
//     let mut state: Map<String, Value> = read_file(String::from("./state.json").as_str());
//     println!("{:?}", state);
//     let status: String;
//     match &state.get(*&title) {
//         Some(result) => {
//             status = result.to_string().replace('\"', "");
//         }
//         None => {
//             status = "pending".to_string();
//         }
//     }
//     let item = to_do_factory(&status, title)
//         .expect(&status);
//     process_input(item, command.to_string(), &state)
//     // state.insert(title.to_string(), json!(status));
//     // write_to_file("./state.json", &mut state);
// }
