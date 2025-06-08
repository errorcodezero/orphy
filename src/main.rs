use anyhow::Error;
use clap::Parser;
use cli::Cli;
use cli_table::{Cell, CellStruct, Table};
use dotenv::dotenv;
use mail::{Letter, MailClient};

mod cli;
mod mail;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().unwrap();
    let args = Cli::parse();
    let client = MailClient::new(std::env::var("API_KEY").unwrap());

    match args {
        Cli::Mail => {
            println!("Loading your mail...");
            let letters = client.get_mail().await;
            if let Ok(Some(letters)) = letters {
                if !letters.is_empty() {
                    let mut table: Vec<Vec<CellStruct>> = Vec::new();
                    for letter in letters {
                        let mut row: Vec<CellStruct> = Vec::new();
                        if let Some(title) = letter.title {
                            row.push(title.cell());
                        } else if let Some(tags) = letter.tags {
                            let mut title = String::from("letter: ");
                            tags.iter().enumerate().for_each(|x| {
                                title += x.1;
                                if x.0 < tags.len() - 1 {
                                    title += ", ";
                                }
                            });
                            row.push(title.cell());
                        }
                        if let Some(letter_type) = letter.letter_type {
                            row.push(letter_type.cell())
                        } else {
                            row.push(String::from("no type").cell())
                        }
                        if let Some(id) = letter.id {
                            row.push(id.cell())
                        } else {
                            row.push(String::from("no id").cell())
                        }
                        if let Some(status) = letter.status {
                            row.push(status.cell());
                        } else {
                            row.push(String::from("no status").cell())
                        }
                        if let Some(created_at) = letter.created_at {
                            row.push(created_at.date_naive().to_string().cell());
                        } else {
                            row.push(String::from("no date").cell())
                        }
                        table.push(row);
                    }
                    let table = table
                        .table()
                        .title(vec!["Name", "Type", "ID", "Status", "Creation Date"])
                        .display()
                        .unwrap();

                    println!("{}", table);
                    println!(
                        "View more details by using orphy view --id (put the id of your letter here!)"
                    )
                } else {
                    println!("You don't have any mail!")
                }
            } else {
                eprintln!("There was an error [1]!");
            }
        }
        Cli::View { id } => {
            println!("Loading your mail...");
            let mail_list = client.get_mail().await;
            let mut mail = Ok(Some(Letter::default()));
            if let Ok(Some(mail_list)) = mail_list {
                for current_mail in mail_list {
                    if let Some(current_id) = current_mail.id {
                        if current_id == id {
                            if let Some(path) = current_mail.path {
                                mail = client.get_mail_by_path(path).await;
                                break;
                            } else {
                                eprintln!("There was an error [2]!");
                            }
                        }
                    }
                }
            }
            if let Ok(mail) = mail {
                if let Some(letter) = mail {
                    let mut table: Vec<Vec<CellStruct>> = Vec::new();
                    if let Some(id) = letter.id {
                        table.push(vec!["ID".cell(), id.cell()]);
                    } else {
                        table.push(vec!["ID".cell(), "no id".cell()]);
                    }
                    if let Some(title) = letter.title {
                        table.push(vec!["Name".cell(), title.cell()]);
                    } else if let Some(tags) = letter.tags {
                        let mut name = String::from("letter: ");
                        for (i, e) in tags.iter().enumerate() {
                            name += e;
                            if i < tags.len() - 1 {
                                name += ", "
                            }
                        }
                        table.push(vec!["Name".cell(), name.cell()])
                    } else {
                        table.push(vec!["Name".cell(), "no name".cell()])
                    }
                    if let Some(letter_type) = letter.letter_type {
                        table.push(vec!["Type".cell(), letter_type.cell()]);
                    } else {
                        table.push(vec!["Type".cell(), "no type".cell()]);
                    }
                    if let Some(letter_subtype) = letter.letter_subtype {
                        table.push(vec!["Subtype".cell(), letter_subtype.cell()]);
                    }
                    if let Some(status) = letter.status {
                        table.push(vec!["Status".cell(), status.cell()]);
                    } else {
                        table.push(vec!["Status".cell(), "no status".cell()]);
                    }
                    if let Some(created_at) = letter.created_at {
                        table.push(vec!["Created At".cell(), created_at.cell()]);
                    } else {
                        table.push(vec!["Created At".cell(), "no creation date".cell()]);
                    }
                    if let Some(updated_at) = letter.updated_at {
                        table.push(vec!["Updated At".cell(), updated_at.cell()]);
                    }
                    if let Some(public_url) = letter.public_url {
                        table.push(vec!["Public URL".cell(), public_url.cell()]);
                    }
                    if let Some(tracking_number) = letter.tracking_number {
                        table.push(vec!["Tracking Number".cell(), tracking_number.cell()]);
                    }
                    if let Some(tracking_link) = letter.tracking_link {
                        table.push(vec!["Tracking Link".cell(), tracking_link.cell()]);
                    }

                    let table = table.table().display().unwrap();
                    println!("{}", table);

                    if let Some(events) = letter.events {
                        if !events.is_empty() {
                            println!("Events");
                            for event in events {
                                let mut table: Vec<Vec<CellStruct>> = Vec::new();
                                if let Some(source) = event.source {
                                    table.push(vec!["Source".cell(), source.cell()]);
                                } else {
                                    table.push(vec!["Source".cell(), "no source".cell()]);
                                }

                                if let Some(facility) = event.facility {
                                    table.push(vec!["Facility".cell(), facility.cell()]);
                                } else {
                                    table.push(vec!["Facility".cell(), "no facility".cell()]);
                                }

                                if let Some(description) = event.description {
                                    table.push(vec!["Description".cell(), description.cell()]);
                                } else {
                                    table.push(vec!["Description".cell(), "no description".cell()]);
                                }

                                if let Some(location) = event.location {
                                    table.push(vec!["Location".cell(), location.cell()]);
                                } else {
                                    table.push(vec!["Location".cell(), "no location".cell()]);
                                }

                                if let Some(happened_at) = event.happened_at {
                                    table.push(vec!["Happened At".cell(), happened_at.cell()]);
                                } else {
                                    table.push(vec![
                                        "Happened At".cell(),
                                        "no happening date".cell(),
                                    ]);
                                }

                                let table = table.table().display().unwrap();

                                println!("{}", table);
                            }
                        }
                    }
                } else {
                    eprintln!("Your letter does not exist!");
                }
            } else {
                eprintln!("There was an error [3]!");
            }
        }
        Cli::Fetch => {
            println!("Loading your stats...");
            let letters = client.get_mail().await;
            if let Ok(Some(letters)) = letters {
                let first = letters.first();
                println!(
                    "
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡤⠶⠒⠛⠉⠙⠛⠒⠶⢤⣀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠋⠁⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠈⠙⢦⡀⠀⠀⠀
⠀⠀⠀⠀⣀⣠⠤⠤⡴⠻⢓⣶⠦⠤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢦⡀⠀
⣀⡠⠴⠊⠁⠀⠀⠀⠀⠀⠒⠽⠀⠀⠀⠉⢙⠒⢢⡄⠀⠀⠀⠀⠀⠀⠀⠈⢷⡀
⠘⢆⠉⠑⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢆⠁⣠⠎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣇ mail: {}
⠀⠈⠓⠦⠁⢀⣀⠀⠀⠀⠀⠀⠀⣀⣀⢸⡊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻ {}
⠀⠀⠀⡸⠀⠉⠀⠙⠆⠀⠀⠀⠏⠀⠈⠀⢇⠀⠀⠀⠀⠀⠀⠀⢀⠔⠀⠀⠀⣼
⠀⠀⠀⠹⣄⠀⠠⣤⡡⠪⡭⠃⡤⢤⡄⡰⠋⠀⠀⠀⣀⡠⠴⠊⠁⠀⠀⠀⢠⠇
⠀⠀⢠⢿⣠⠟⠓⠛⠉⠛⡟⠛⠛⠛⠛⠒⠒⠚⠉⠉⠁⠀⠀⠀⠀⠀⢀⡴⠋⠀
⠀⠀⠈⠛⢯⣀⣀⣀⡤⠤⠤⠤⢤⣤⣀⣀⣀⣀⣀⣀⣀⣤⠤⠴⠒⠋⠁⠀⠀⠀
",
                    letters.len(),
                    if !letters.is_empty() && first.unwrap().created_at.is_some() {
                        format!(
                            "last mailed: {}",
                            first.unwrap().created_at.unwrap().naive_utc()
                        )
                    } else {
                        String::from("")
                    }
                )
            }
        }
    }
    Ok(())
}
