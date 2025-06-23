use core::time;
use std::thread::sleep;

use anyhow::Error;
use clap::Parser;
use cli::{Cli, Config};
use cli_table::{Cell, CellStruct, Table};
use confy::{ConfyError, load, store};
use mail::{Letter, MailClient};

mod cli;
mod mail;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cfg: Result<Config, ConfyError> = load("orphy_hackclub_mail_client", None);
    let args = Cli::parse();

    match args {
        Cli::Setup { api_key } => {
            let cfg = Config { api_key };
            match store("orphy_hackclub_mail_client", None, cfg) {
                Ok(_) => println!("Saved your api key!"),
                Err(_) => eprintln!("There was an error! [4]"),
            }
        }
        Cli::Mail { r#type } => {
            if let Ok(cfg) = cfg {
                if cfg.api_key.is_empty() {
                    println!("No api key! Add one using orphy setup [api key]");
                    return Ok(());
                }
                let client = MailClient::new(cfg.api_key);
                println!("Loading your mail...");
                let letters = client.get_mail(r#type).await;
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
                    eprintln!("There was an error [1]! api key might be invalid!");
                }
            } else {
                eprintln!(
                    "You don't have an api key! Run orphy setup [your api key] with your api key."
                )
            }
        }
        Cli::View { id } => {
            if let Ok(cfg) = cfg {
                if cfg.api_key.is_empty() {
                    println!("No api key! Add one using orphy setup [api key]");
                    return Ok(());
                }
                let client = MailClient::new(cfg.api_key);
                println!("Loading your mail...");
                let mail_list = client.get_mail(None).await;
                let mut mail = Ok(Some(Letter::default()));
                let mut letter_exists = false;
                if let Ok(Some(mail_list)) = mail_list {
                    for current_mail in mail_list {
                        if let Some(current_id) = current_mail.id {
                            if current_id == id {
                                if let Some(path) = current_mail.path {
                                    mail = client.get_mail_by_path(path).await;
                                    letter_exists = true;
                                    break;
                                } else {
                                    eprintln!("There was an error [2]! api key might be invalid!");
                                }
                            }
                        }
                    }
                }
                if !letter_exists {
                    eprintln!("Letter doesn't exist or api key may be invalid!")
                } else if let Ok(mail) = mail {
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
                                        table.push(vec![
                                            "Description".cell(),
                                            "no description".cell(),
                                        ]);
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
                    eprintln!("There was an error [3]! api key might be invalid!");
                }
            } else {
                eprintln!(
                    "You don't have an api key! Run orphy setup [your api key] with your api key."
                )
            }
        }
        Cli::Fetch => {
            if let Ok(cfg) = cfg {
                if cfg.api_key.is_empty() {
                    println!("No api key! Add one using orphy setup [api key]");
                    return Ok(());
                }
                let client = MailClient::new(cfg.api_key);
                println!("Loading your stats...");
                let letters = client.get_mail(None).await;
                if let Ok(Some(letters)) = letters {
                    let first = letters.first();
                    let mut letter_count = 0;
                    let mut package_count = 0;
                    let mut legacy_count = 0;
                    letters.iter().for_each(|x| {
                        if x.letter_type == Some(String::from("letter")) {
                            letter_count += 1;
                        } else if x.letter_type == Some(String::from("package")) {
                            package_count += 1;
                        } else {
                            legacy_count += 1;
                        }
                    });
                    println!(
                        "
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡤⠶⠒⠛⠉⠙⠛⠒⠶⢤⣀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠋⠁⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠈⠙⢦⡀⠀⠀⠀ {}
⠀⠀⠀⠀⣀⣠⠤⠤⡴⠻⢓⣶⠦⠤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢦⡀⠀ instance: {}
⣀⡠⠴⠊⠁⠀⠀⠀⠀⠀⠒⠽⠀⠀⠀⠉⢙⠒⢢⡄⠀⠀⠀⠀⠀⠀⠀⠈⢷⡀ {}
⠘⢆⠉⠑⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢆⠁⣠⠎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣇ mail: {}
⠀⠈⠓⠦⠁⢀⣀⠀⠀⠀⠀⠀⠀⣀⣀⢸⡊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻ letters: {}
⠀⠀⠀⡸⠀⠉⠀⠙⠆⠀⠀⠀⠏⠀⠈⠀⢇⠀⠀⠀⠀⠀⠀⠀⢀⠔⠀⠀⠀⣼ packages: {}
⠀⠀⠀⠹⣄⠀⠠⣤⡡⠪⡭⠃⡤⢤⡄⡰⠋⠀⠀⠀⣀⡠⠴⠊⠁⠀⠀⠀⢠⠇ legacy: {}
⠀⠀⢠⢿⣠⠟⠓⠛⠉⠛⡟⠛⠛⠛⠛⠒⠒⠚⠉⠉⠁⠀⠀⠀⠀⠀⢀⡴⠋⠀
⠀⠀⠈⠛⢯⣀⣀⣀⡤⠤⠤⠤⢤⣤⣀⣀⣀⣀⣀⣀⣀⣤⠤⠴⠒⠋⠁⠀⠀⠀ 
",
                        if let Ok(Some(id)) = client.get_id().await {
                            format!("id: {id}")
                        } else {
                            String::from("")
                        },
                        client.base,
                        if !letters.is_empty() && first.unwrap().created_at.is_some() {
                            format!(
                                "last mailed: {}",
                                first.unwrap().created_at.unwrap().naive_utc()
                            )
                        } else {
                            String::from("")
                        },
                        letters.len(),
                        letter_count,
                        package_count,
                        legacy_count
                    )
                }
            } else {
                eprintln!(
                    "You don't have an api key! Run orphy setup [your api key] with your api key."
                )
            }
        }
        Cli::Credit => {
            println!(
                "
███████╗██████╗ ██████╗  ██████╗ ██████╗  ██████╗ ██████╗ ██████╗ ███████╗ ██████╗ 
██╔════╝██╔══██╗██╔══██╗██╔═══██╗██╔══██╗██╔════╝██╔═══██╗██╔══██╗██╔════╝██╔═████╗
█████╗  ██████╔╝██████╔╝██║   ██║██████╔╝██║     ██║   ██║██║  ██║█████╗  ██║██╔██║
██╔══╝  ██╔══██╗██╔══██╗██║   ██║██╔══██╗██║     ██║   ██║██║  ██║██╔══╝  ████╔╝██║
███████╗██║  ██║██║  ██║╚██████╔╝██║  ██║╚██████╗╚██████╔╝██████╔╝███████╗╚██████╔╝
╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═════╝ ╚═════╝ ╚══════╝ ╚═════╝ 
"
            );
            println!("Created with <3 by ErrorCode0");
            println!("@errorcodezero on github")
        }
        Cli::Fun => {
            let animation_frames: Vec<String> = vec![
                String::from(
                    "
________               .__           
\\_____  \\_____________ |  |__ ___.__.
 /   |   \\_  __ \\____ \\|  |  <   |  |
/    |    \\  | \\/  |_> >   Y  \\___  |
\\_______  /__|  |   __/|___|  / ____|
        \\/      |__|        \\/\\/     
",
                ),
                String::from(
                    "
 _______  _______  _______                   
(  ___  )(  ____ )(  ____ )|\\     /||\\     /|
| (   ) || (    )|| (    )|| )   ( |( \\   / )
| |   | || (____)|| (____)|| (___) | \\ (_) / 
| |   | ||     __)|  _____)|  ___  |  \\   /  
| |   | || (\\ (   | (      | (   ) |   ) (   
| (___) || ) \\ \\__| )      | )   ( |   | |   
(_______)|/   \\__/|/       |/     \\|   \\_/   

",
                ),
                String::from(
                    "The story of the existence of this command is one of intense emotion. You may cry, whine, program a little, but know that you will come out of it a better person.",
                ),
                String::from(
                    "It all started in 2025 when a young programmer going by the online pseudonym of ErrorCode0 decided that he'd like to participate in a strange online hackathon named Shipwrecked.",
                ),
                String::from(
                    "This young but enthusiastic programmer decided to get right to work and began working on four projects of which needed to comprise of 15 hours per in order to make it to this strange island-based hackathon.",
                ),
                String::from(
                    "His forth project was a 16-bit custom CPU architecture he had built as well as a virtual machine to go along with it.",
                ),
                String::from(
                    "Initially, his efforts of virality focused upon trying to win the alluring ship showcase, in which contestants would come together and battle in a showcase of projects, attempting to best one another in having the most well recieved projects, of which top 3 would be automatically considered viral."),
                String::from("As it was his turn to speak, he stuttered. He realized that the audience wasn't right. These were webdevs... not low level enthusiasts like he had been. They didn't quite see the appeal of the 16-bit cpu as much as he did.
            "),
                String::from("He walked out of that ship showcase determined thinking that even if he wouldn't win on this, he still had the option of virality at hand"),
                String::from(
                    "He realized that he needed to make one of his apps go viral on some social media site. The young entrepreneur chose Hacker news with a fallback of Github for virality in order to try to push his project outwards.",
                ),
                String::from(
                    "Despite his valient endeavors, even going up to the point of publishing an entire technical writeup of the project on his blog, he failed to have it gain much traction and reach the virality requirement.",
                ),
                String::from(
                    "Out of desperation, he begged the organizers to allow him an exception: \"B-B-But it hit the front page of the Show HN page... That's gotta count for something r-r-right?!\"",
                ),
                String::from(
                    "The organizer laughed, \"You pitiful creature, we only take the main front page for virality. I am nothing if not a merciful man. I will give you 1 week to finish every last project and if it is done, I will waive the virality. Work fast.\""
                ),
                String::from("He fell back to the ground in shock, his ears ringing with pain and that line replaying in his head over and over again like a broken record. He clutched his fist in determination realizing what his final shot would come to."),
                    String::from("He looked for anything... desperate for what could allow him to finish all his projects in time. He searched and searched and ran his eyes dry until finally he discovered a loophole. The projects didn't technically need to be finished, but rather they would need to be in a functional state."),
                String::from("The young error code dashed to submit his projects after trying to wrap up what functional state he believed they were in. That island was finally in reach."),
                String::from("After submitting everything and realizing it was all in order, he stared upwards at the 95%. \"95? Why am I behind by just 5%?\". He looked around and around until he saw his project orpheus. He realized he only had 12 out of the 15 hours required for orpheus."),
                String::from("Desperate for just three more hours added to his count, he looked for any features to add to the project."),
                String::from("He settled on writing this command..."),
                String::from("Based on a true story. Shipwrecked admins pls accept this."),
                String::from("PART 2:"),
                String::from("The confident ErrorCode0 stepped up to the mighty platform upon which the shipwrecked organizers rested upon."),
                String::from("He tapped his foot on the ground, in that same rhythmic fashion as one would knock on a door with.")
                ,
                String::from("The towering shipwrecked organizer awoke from his slumber: \"WHO DARES AWAKE ME FROM MY SLUMBER\""),
                String::from("The young programmer got onto a knee and said \"My lord, please accept my humble request of a rounding of my hours on my project of name Orphy. I have exceeded the count of hours within all my other projects by a surplus of nearing 20 per! Please my lord"),
                String::from("The shipwrecked organizer laughed a hearty and evil chuckle. \"You stupid mortal. I will not do that for that is FRAUD. You don't want to be banned from this event for fraud would you?\" He picked up the young programmer."),
                String::from("\"I don't want to see you here until 15 hours no less has been reached by your puny project. Have we come to an understanding, or do more... serious measures need to be taken into account\""),
                String::from("The bumbling giant dropped the young programmer down into sea of piracy. He came to realize that he would need more than a humble request to persuade the organizer."),
                String::from("The humble programmer peered into his hackatime stats to no avail. He had only accumulated 14 hours and 4 minutes of the required time he would need. He wouldn't make it onto this hackathon unless a miracle took place."),
                String::from("Is this the end for our young ErrorCode0's journey?"),
                String::from("He worked tirelessly to stretch his time to the 15 hour mark. Useless feature here, unecessary story there, but it was tireless work. This last hour felt more demanding than all that had come before it and it certainly required much of him."),
                String::from("Slowly the time ticked up. Hackatime began reporting his time slowly increasing as he typed on and on yapping and yapping."),
                String::from("His mind slowly began to blank. What else could he write about? There was literally nothing else he could say. He had yapped away for nearly a century at this point and yet the 1 final hour still had not been reached."),
                String::from("Perhaps if the requirements were a bit less stringent he would have a choice."),
                String::from("He isn't as lucky as he would hope unfortunately."),
                String::from("PART 3:"),
                String::from("One morning, he woke up to a ping on slack. The hacker news tracker finally updated: The project blew up."),
                String::from("He opened the github to see nearly 50+ stars compared to last night."),
                String::from("\"Wow... that's actually pretty crazy. How did that happen?\""),
                String::from("The hacker news mods realized this was a golden piece of content that slipped through the cracks. They gloriously blessed the article with a second chance to hit the front page."),
                String::from("It was a little late for it to go viral but nevertheless it was still helpful for the project."),
                String::from("He walked towards the platform for the final time. Hours in hand, virality on his chest, and a tough demeanor on the outside."),
                String::from("\"You again??\" the shipwrecked organizer grunted. \"Haven't you troubled me enough?\""),
                String::from("\"I finally fulfilled the requirements. Let me through onto the island\""),
                String::from("\"I suppose you can make it aboard\""),
                String::from("\"THE END\""),
                String::from("shipwrecked organizer is not anyone in particular but it's just an embodiment of the different setbacks I had wwhile trying to qualify for this event put into the body of a giant."),
                String::from("pls don't mark my hours for fraud ok since writing this story == coding time"),
                String::from("ty for reading this story."),
                String::from("im still typing so I can hit the five minute mark."),
                String::from("these last lines are useless but I will hit 15 hours no matter what."),
                String::from("im like only 5 minutes away from hitting it come on"),
                String::from("lemme check the time now."),
                String::from("bro its still on 5 minutes"),
                String::from("ok im still typing something here."),
                String::from("i would recommend just Control-C-ing atp cus theres no use in reading the rest of this"),
                String::from("ok bye"),
                String::from("alright fine you're still here."),
                String::from("you should look at my other stuff on my website -> errorcodezero.dev"),
                String::from("my github is @errorcodezero"),
                String::from("if you don't alr know, this is a terminal client for hackclub mail."),
                String::from("Hackclub mail is this system to track packages and whatnot that hc is sending u"),
                String::from("if you don't know what hackclub is well... hackclub.com will probably get you sorted out."),
                String::from("lemme check if i hit the mark one sec"),
                String::from("ok 2 minutes away."),
                String::from("come on come on come on come on come on come on"),
                String::from("this command never would have existed if the minimum requirement for a project wasn't 15 hours with no option to overflow."),
                String::from("ok i think its done lemme check."),
                String::from("11 seconds til self destruction"),
                String::from("thanks for reading all of this whoever is doing this thanks."),
                String::from("why can't this mark me as done already."),
                String::from("ok im done for good now bye")
            ];
            for animation in animation_frames {
                println!("{}", animation);
                sleep(time::Duration::from_secs(4));
            }
        }
    }
    Ok(())
}
