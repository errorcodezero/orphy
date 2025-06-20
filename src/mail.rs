use crate::cli::MailType;
use anyhow::Error;
use chrono::prelude::*;
use reqwest::Client;
use serde_json::Value;

pub struct MailClient {
    auth_token: String,
    pub base: String,
    pub api_path: String,
    client: Client,
}

// All of these are options cus letters don't follow a strict schema and sometimes are missing half
// the details
#[derive(Default, Debug)]
pub struct Letter {
    pub id: Option<String>,
    pub title: Option<String>,
    pub letter_type: Option<String>,
    pub letter_subtype: Option<String>,
    pub tracking_number: Option<String>,
    pub tracking_link: Option<String>,
    pub public_url: Option<String>,
    pub status: Option<String>,
    pub tags: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub events: Option<Vec<Event>>,
    pub path: Option<String>,
}

// These don't seem to follow as much of an optional schema but i'm putting it here just in case
#[derive(Default, Debug)]
pub struct Event {
    pub happened_at: Option<DateTime<Utc>>,
    pub source: Option<String>,
    pub facility: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
}

// /api/public/v1/me
// /api/public/v1/mail (the important one)
// /api/public/v1/letters
// /api/public/v1/letters/:id
// /api/public/v1/packages
// /api/public/v1/packages/:id
// /api/public/v1/lsv
// /api/public/v1/lsv/:type/:id

impl MailClient {
    pub fn new(auth_token: String) -> Self {
        Self {
            auth_token,
            ..Default::default()
        }
    }

    fn event_from_data(&self, event: &Value) -> Event {
        let happened_at: Option<DateTime<Utc>> = if let Value::String(str) = &event["happened_at"] {
            Some(str.parse().unwrap())
        } else {
            None
        };

        let description = if let Value::String(str) = &event["description"] {
            Some(str.parse().unwrap())
        } else {
            None
        };

        let location = if let Value::String(str) = &event["location"] {
            Some(str.parse().unwrap())
        } else {
            None
        };

        let facility = if let Value::String(str) = &event["facility"] {
            Some(str.parse().unwrap())
        } else {
            None
        };

        let source = if let Value::String(str) = &event["source"] {
            Some(str.parse().unwrap())
        } else {
            None
        };

        Event {
            happened_at,
            description,
            location,
            facility,
            source,
        }
    }

    fn letter_from_data(&self, letter: &Value) -> Option<Letter> {
        let mut letter_exists = false;
        let id = if let Value::String(str) = &letter["id"] {
            letter_exists = true;
            Some(str.parse().unwrap())
        } else {
            None
        };
        let title = if let Value::String(str) = &letter["title"] {
            letter_exists = true;
            Some(str.parse().unwrap())
        } else {
            None
        };
        let created_at: Option<DateTime<Utc>> = if let Value::String(str) = &letter["created_at"] {
            letter_exists = true;
            Some(str.parse().unwrap())
        } else {
            None
        };
        let updated_at: Option<DateTime<Utc>> = if let Value::String(str) = &letter["updated_at"] {
            letter_exists = true;
            Some(str.parse().unwrap())
        } else {
            None
        };
        let public_url = if let Value::String(str) = &letter["public_url"] {
            letter_exists = true;
            Some(str.parse().unwrap())
        } else {
            None
        };
        let letter_type = if let Value::String(str) = &letter["type"] {
            letter_exists = true;
            Some(str.parse().unwrap())
        } else {
            None
        };
        let letter_subtype = if let Value::String(str) = &letter["subtype"] {
            letter_exists = true;
            Some(str.parse().unwrap())
        } else {
            None
        };
        let status = if let Value::String(str) = &letter["status"] {
            letter_exists = true;
            Some(str.parse().unwrap())
        } else {
            None
        };
        let path = if let Value::String(str) = &letter["path"] {
            letter_exists = true;
            Some(str.parse().unwrap())
        } else {
            None
        };
        let tags = if let Value::Array(tags) = &letter["tags"] {
            letter_exists = true;
            let mut tags_final = Vec::<String>::new();
            for tag in tags.iter() {
                tags_final.push(tag.to_string());
            }
            Some(tags_final)
        } else {
            None
        };
        let tracking_number = if let Value::String(str) = &letter["tracking_number"] {
            letter_exists = true;
            Some(str.parse().unwrap())
        } else {
            None
        };
        let tracking_link = if let Value::String(str) = &letter["tracking_link"] {
            letter_exists = true;
            Some(str.parse().unwrap())
        } else {
            None
        };
        let events = if let Value::Array(events) = &letter["events"] {
            letter_exists = true;
            let mut events_final: Vec<Event> = Vec::new();
            for event in events {
                events_final.push(self.event_from_data(event));
            }

            Some(events_final)
        } else {
            None
        };

        if letter_exists {
            Some(Letter {
                id,
                title,
                created_at,
                updated_at,
                public_url,
                status,
                tags,
                letter_type,
                events,
                letter_subtype,
                path,
                tracking_link,
                tracking_number,
            })
        } else {
            None
        }
    }

    pub async fn get_id(&self) -> Result<Option<String>, Error> {
        let body = self
            .client
            .get(format!("{}/{}/me", self.base, self.api_path))
            .bearer_auth(&self.auth_token)
            .send()
            .await?
            .text()
            .await?;

        let data: Result<Value, serde_json::Error> = serde_json::from_str(&body);

        if let Ok(data) = data {
            Ok(Some(data["user"]["id"].to_string()))
        } else {
            Ok(None)
        }
    }

    pub async fn get_mail(
        &self,
        mail_type: Option<MailType>,
    ) -> Result<Option<Vec<Letter>>, Error> {
        let body = match mail_type {
            Some(MailType::Legacy) => {
                self.client
                    .get(format!("{}/{}/lsv", self.base, self.api_path))
                    .bearer_auth(&self.auth_token)
                    .send()
                    .await?
                    .text()
                    .await?
            }
            Some(MailType::Package) => {
                self.client
                    .get(format!("{}/{}/packages", self.base, self.api_path))
                    .bearer_auth(&self.auth_token)
                    .send()
                    .await?
                    .text()
                    .await?
            }
            Some(MailType::Letter) => {
                self.client
                    .get(format!("{}/{}/letters", self.base, self.api_path))
                    .bearer_auth(&self.auth_token)
                    .send()
                    .await?
                    .text()
                    .await?
            }
            _ => {
                self.client
                    .get(format!("{}/{}/mail", self.base, self.api_path))
                    .bearer_auth(&self.auth_token)
                    .send()
                    .await?
                    .text()
                    .await?
            }
        };

        let data: Result<Value, serde_json::Error> = serde_json::from_str(&body);

        if let Ok(data) = data {
            let mut mail: Vec<Letter> = Vec::new();

            let name = match mail_type {
                Some(MailType::Letter) => "letters",
                Some(MailType::Package) => "packages",
                Some(MailType::Legacy) => "legacy_shipment_viewer_records",
                None => "mail",
            };

            if let Some(Value::Array(arr)) = data.get(name) {
                for letter in arr.iter() {
                    // unwrapping since this will not break on this
                    mail.push(self.letter_from_data(letter).unwrap());
                }
                Ok(Some(mail))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn get_mail_by_path(&self, path: String) -> Result<Option<Letter>, Error> {
        let body = self
            .client
            .get(format!("{}/{path}", self.base))
            .bearer_auth(&self.auth_token)
            .send()
            .await?
            .text()
            .await?;

        let data: Result<Value, serde_json::Error> = serde_json::from_str(&body);

        if let Ok(data) = data {
            if let Value::Object(_) = &data["letter"] {
                Ok(self.letter_from_data(&data["letter"]))
            } else if let Value::Object(_) = &data["package"] {
                Ok(self.letter_from_data(&data["package"]))
            } else if let Value::Object(_) = &data["legacy_shipment_viewer_record"] {
                Ok(self.letter_from_data(&data["legacy_shipment_viewer_record"]))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl Default for MailClient {
    fn default() -> Self {
        Self {
            auth_token: String::new(),
            base: String::from("https://mail.hackclub.com"),
            api_path: String::from("/api/public/v1/"),
            client: Client::new(),
        }
    }
}
