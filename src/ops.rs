// let operation = fragments[0].clone();
// match operation.as_ref() {
//     "install" => {
//         println!("Installing: app");
//     }
//     "maintain" => {
//         println!("troubleshooting...")
//     }
//     "update" => {
//         println!("updating...");
//         Command::new("konsole")
//             .args(&["-e", "pkexec pacman -Syu --noconfirm"])
//             .spawn()
//             .unwrap();
//     }
//     "upgrade" => {
//         println!("upgrading...")
//     }
//     "custom" => {
//         if fragments.len() >= 2 {
//             let commands: Vec<String> =
//                 fragments.iter().skip(1).map(|f| f.to_string()).collect();
//             Exec::cmd("konsole")
//                 .arg("-e")
//                 .arg(commands.join(" "))
//                 .join()
//                 .unwrap();
//         }
//     }
//     _ => {
//         println!("Unknown operations")
//     }
// }
