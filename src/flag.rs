// use xlog_rs::log;
// pub struct Flags {
//     pub endpoint: transport::Endpoint,
//     // iced_futures::subscription::Recipe
//     pub recipe: fn() -> Subscription<Message>,
// }
// fn show_help() {
//     println!("Usage: qst [options]");
//     println!("Options:");
//     println!("  -uri <uri>    set uri");
//     println!("  -help         show help");
// }
// impl Flags {
//     pub fn new(args: Vec<String>, recipe: fn() -> Subscription<Message>) -> Self {
//         for (i, arg) in args.iter().enumerate() {
//             match arg.as_str() {
//                 "-help" => {
//                     show_help();
//                     std::process::exit(0);
//                 }
//                 "-uri" => {
//                     if i + 1 < args.len() {
//                         match transport::Channel::from_shared(args[i + 1].clone()) {
//                             Err(e) => {
//                                 log::error(format!("invalid uri: {}", e).as_str());
//                                 show_help();
//                                 std::process::exit(1);
//                             }
//                             Ok(c) => {
//                                 log::info(format!("addr: {:#?}", c.uri()).as_str());
//                                 return Self {
//                                     endpoint: c,
//                                     recipe: recipe,
//                                 };
//                             }
//                         }
//                     }
//                 }
//                 _ => {}
//             }
//         }
//         println!("invalid args");
//         show_help();
//         std::process::exit(1);
//     }
// }
