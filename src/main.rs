use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{collections::HashMap, env, fs::File, io::Result, path::PathBuf};
use subprocess::Exec;
use url::Url;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DB {
    pub issues: HashMap<String, PathBuf>,
}

fn main() -> Result<()> {
    let arg = env::args().nth(1);
    let url_with_protocol = arg.unwrap_or(String::new());
    let url_parsed = Url::parse(&url_with_protocol).unwrap();

    if url_parsed.scheme() == "koompi" {
        if let Some(domain) = url_parsed.host_str() {
            match domain {
                "downgrade" => {}
                "maintenance" => {
                    let file = File::open("operation.yml")?;
                    let data: DB = from_reader(file).unwrap();

                    if let Some(query) = url_parsed.query() {
                        let ops = query.strip_prefix("ops=").unwrap();
                        let ops_list: Vec<String> = ops.split(",").map(|f| f.to_string()).collect();
                        for op in ops_list.iter() {
                            if let Some(operation) = data.issues.get(op) {
                                Exec::shell(format!("{}", operation.to_str().unwrap()))
                                    .join()
                                    .unwrap();
                            }
                        }
                    }
                }
                "os" => {}
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
}
