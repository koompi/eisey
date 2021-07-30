use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{collections::HashMap, env, fs::File, io::Result, path::PathBuf};
use subprocess::Exec;
use url::Url;

use protocol::bip32_sec;
use protocol::rsa_sec;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DB {
    pub issues: HashMap<String, PathBuf>,
}

fn main() -> Result<()> {
    // rsa_sec::rsa_keygen();
    // bip32_sec::genkey().unwrap();
    // bip32_sec::sign("brilliant_phal", "brilliant.cert");
    // bip32_sec::verify("brilliant.cert", "brilliant_phal").unwrap();

    let arg = env::args().nth(1);
    let url_with_protocol = arg.unwrap_or(String::new());
    let url_parsed = Url::parse(&url_with_protocol).unwrap();

    let hash_query: HashMap<_, _> = url_parsed.query_pairs().into_owned().collect();

    let cmd: Option<_> = hash_query.get("cmd");
    let cert: Option<_> = hash_query.get("cert");

    #[cfg(debug_assertions)]
    {
        println!("");
        println!("cmd: {:?}", cmd);
        println!("");
        println!("cert: {:?}", cert);
    }

    if let Some(cert) = cert {
        // convert cert from base64
        let cert_byte = base64::decode_config(cert, base64::URL_SAFE_NO_PAD);
        if let Some(cmd) = cmd {
            let cmd_byte = base64::decode_config(cmd, base64::URL_SAFE_NO_PAD);
            match bip32_sec::verify_bytes(
                cert_byte.unwrap().as_slice(),
                &cmd_byte.clone().unwrap().as_slice(),
            ) {
                Ok(_) => {
                    let cmd = rsa_sec::rsa_decrypt(cmd_byte.unwrap());
                    match cmd {
                        Ok(cmd) => {
                            match std::str::from_utf8(cmd.as_slice()) {
                                Ok(v) => {
                                    // println!("command: {}", v);
                                    let url = Url::parse(v).unwrap();
                                    runner(url).unwrap();
                                }
                                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                            };
                        }
                        Err(e) => {
                            println!("Unable to decrypt command.");
                            println!("{}", e.to_string());
                        }
                    }
                }
                Err(e) => {
                    println!("Invalid certificate");
                    println!("{}", e.to_string())
                }
            }
        } else {
            println!("Encrypted command required.")
        }
    } else {
        println!("Certificate signed by KOOMPI is required.")
    }

    Ok(())
}

pub fn runner(url_parsed: Url) -> Result<()> {
    let hash_query: HashMap<_, _> = url_parsed.query_pairs().into_owned().collect();
    let apps: Option<_> = hash_query.get("apps");
    let ops: Option<_> = hash_query.get("ops");
    if url_parsed.scheme() == "sel" {
        if let Some(domain) = url_parsed.host_str() {
            match domain {
                "downgrade" => {}
                "maintenance" => {
                    let file_path = if cfg!(debug_assertions) {
                        "operation.yml"
                    } else {
                        "/usr/share/org.koompi.sel/operation.yml"
                    };

                    let file = File::open(file_path)?;
                    let data: DB = from_reader(file).unwrap();

                    if let Some(ops) = ops {
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
                        if let Some(apps) = apps {
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
                        if let Some(apps) = apps {
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
                        if let Some(apps) = apps {
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
                        if let Some(apps) = apps {
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
                        if let Some(apps) = apps {
                            // let apps = query.strip_prefix("apps=").unwrap();
                            let app_list = apps.replace(",", " ");
                            println!("Installing: {}", app_list);
                            Exec::shell(format!("konsole -e pkexec pix -i {}", app_list))
                                .join()
                                .unwrap();
                        }
                    }
                    "/remove" | "/r" => {
                        if let Some(apps) = apps {
                            // let apps = query.strip_prefix("apps=").unwrap();
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
