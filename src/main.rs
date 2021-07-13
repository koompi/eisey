use std::{env, io::Result};
use subprocess::Exec;
use url::Url;

fn main() -> Result<()> {
    let arg = env::args().nth(1);
    let url_with_protocol = arg.unwrap_or(String::new());
    let url_parsed = Url::parse(&url_with_protocol).unwrap();

    if url_parsed.scheme() == "koompi" {
        if let Some(domain) = url_parsed.host_str() {
            match domain {
                "downgrade" => {}
                "maintainance" => {
                    println!("Running maintenance")
                }
                "os-upgrade" => {}
                "pacman" => match url_parsed.path() {
                    "/install" | "/i" => {
                        if let Some(query) = url_parsed.query() {
                            let apps = query.strip_prefix("apps=").unwrap();
                            let app_list = apps.replace(",", " ");
                            println!("Installing: {}", app_list);
                            Exec::shell(format!(
                                "konsole -e pkexec pacman -S --noconfirm {}",
                                app_list
                            ))
                            .join()
                            .unwrap();
                        }
                    }
                    "/remove" | "/r" => {
                        if let Some(query) = url_parsed.query() {
                            let apps = query.strip_prefix("apps=").unwrap();
                            let app_list = apps.replace(",", " ");
                            println!("Installing: {}", app_list);
                            Exec::shell(format!(
                                "konsole -e pkexec pacman -R --noconfirm {}",
                                app_list
                            ))
                            .join()
                            .unwrap();
                        }
                    }
                    "/update" | "/u" => {
                        Exec::shell(format!("konsole -e pkexec pacman -Syu --noconfirm"))
                            .join()
                            .unwrap();
                    }
                    _ => {}
                },
                "pi" => match url_parsed.path() {
                    "/install" | "/i" => {
                        if let Some(query) = url_parsed.query() {
                            let apps = query.strip_prefix("apps=").unwrap();
                            let app_list = apps.replace(",", " ");
                            println!("Installing: {}", app_list);
                            Exec::shell(format!(
                                "konsole -e pkexec pi -S --noconfirm {}",
                                app_list
                            ))
                            .join()
                            .unwrap();
                        }
                    }
                    "/remove" | "/r" => {
                        if let Some(query) = url_parsed.query() {
                            let apps = query.strip_prefix("apps=").unwrap();
                            let app_list = apps.replace(",", " ");
                            println!("Installing: {}", app_list);
                            Exec::shell(format!(
                                "konsole -e pkexec pi -R --noconfirm {}",
                                app_list
                            ))
                            .join()
                            .unwrap();
                        }
                    }
                    "/update" | "/u" => {
                        Exec::shell(format!("konsole -e pi -Syu --noconfirm"))
                            .join()
                            .unwrap();
                    }
                    _ => {}
                },
                "pix" => match url_parsed.path() {
                    "/install" | "/i" => {
                        if let Some(query) = url_parsed.query() {
                            let apps = query.strip_prefix("apps=").unwrap();
                            let app_list = apps.replace(",", " ");
                            println!("Installing: {}", app_list);
                            Exec::shell(format!("konsole -e pkexec pix -i {}", app_list))
                                .join()
                                .unwrap();
                        }
                    }
                    "/remove" | "/r" => {
                        if let Some(query) = url_parsed.query() {
                            let apps = query.strip_prefix("apps=").unwrap();
                            let app_list = apps.replace(",", " ");
                            println!("Installing: {}", app_list);
                            Exec::shell(format!("konsole -e pkexec pix -r {}", app_list))
                                .join()
                                .unwrap();
                        }
                    }
                    "/update" | "/u" => {
                        Exec::shell(format!("konsole -e pix -u")).join().unwrap();
                    }
                    _ => {}
                },
                _ => println!("INVALID OPERATION: {}", domain),
            }
        }
    }

    Ok(())
    // let url_without_protocol = url_with_protocol.strip_prefix("koompi://").unwrap();

    // let url_fragments: Vec<String> = url_without_protocol
    //     .split("/")
    //     .map(|f| f.to_string())
    //     .collect();

    // let domain: String = url_fragments[0].clone();
    // let paths: Vec<String> = url_fragments
    //     .iter()
    //     .skip(1)
    //     .map(|p| p.to_string())
    //     .collect();

    // println!("DOMAIN: {}", domain);
    // println!("PATHS: {:?}", paths);
}
