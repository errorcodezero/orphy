use anyhow::Error;
use chrono::prelude::*;
use reqwest::Client;
use serde_json::Value;

pub struct MailClient {
    auth_token: String,
    base_url: String,
    client: Client,
}

// All of these are options cus letters don't follow a strict schema and sometimes are missing half
// the details
#[derive(Default, Debug)]
pub struct Letter {
    pub id: Option<String>,
    pub title: Option<String>,
    pub letter_type: Option<String>,
    pub public_url: Option<String>,
    pub status: Option<String>,
    pub tags: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub events: Option<Vec<Event>>,
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
        let status = if let Value::String(str) = &letter["status"] {
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
            })
        } else {
            None
        }
    }

    pub async fn get_mail(&self) -> Result<Option<Vec<Letter>>, Error> {
        let body = self
            .client
            .get(format!("{}/mail", self.base_url))
            .bearer_auth(&self.auth_token)
            .send()
            .await?
            .text()
            .await?;

        let data: Result<Value, serde_json::Error> = serde_json::from_str(&body);

        if let Ok(data) = data {
            let mut mail: Vec<Letter> = Vec::new();

            for letter in data.get("mail").unwrap().as_array().unwrap().iter() {
                // unwrapping since this will not break on this
                mail.push(self.letter_from_data(letter).unwrap());
            }
            Ok(Some(mail))
        } else {
            Ok(None)
        }
    }

    pub async fn get_mail_by_id(&self, id: String) -> Result<Option<Letter>, Error> {
        let body = self
            .client
            .get(format!("{}/letters/{}", self.base_url, id))
            .bearer_auth(&self.auth_token)
            .send()
            .await?
            .text()
            .await?;

        let data: Result<Value, serde_json::Error> = serde_json::from_str(&body);

        if let Ok(data) = data {
            Ok(self.letter_from_data(&data["letter"]))
        } else {
            Ok(None)
        }
    }
}

impl Default for MailClient {
    fn default() -> Self {
        Self {
            auth_token: String::new(),
            base_url: String::from("https://mail.hackclub.com/api/public/v1"),
            client: Client::new(),
        }
    }
}
