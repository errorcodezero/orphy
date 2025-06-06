use anyhow::Error;
use chrono::prelude::*;
use reqwest::Client;
use serde_json::Value;

pub struct MailClient {
    auth_token: String,
    base_url: String,
    client: Client,
}

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

    pub async fn get_lsv(&self) -> Result<String, Error> {
        let body = self
            .client
            .get(format!("{}/lsv", self.base_url))
            .header("Authorization", format!("Bearer {}", &self.auth_token))
            .send()
            .await?
            .text()
            .await?;

        Ok(body)
    }

    pub async fn get_lsv_with_type_and_id(
        &self,
        id: String,
        lsv_type: String,
    ) -> Result<String, Error> {
        let body = self
            .client
            .get(format!("{}/lsv/{}/{}", self.base_url, lsv_type, id))
            .header("Authorization", format!("Bearer {}", &self.auth_token))
            .send()
            .await?
            .text()
            .await?;

        Ok(body)
    }

    pub async fn get_package_with_id(&self, id: String) -> Result<String, Error> {
        let body = self
            .client
            .get(format!("{}/packages/{}", self.base_url, id))
            .header("Authorization", format!("Bearer {}", &self.auth_token))
            .send()
            .await?
            .text()
            .await?;

        Ok(body)
    }

    pub async fn get_packages(&self) -> Result<String, Error> {
        let body = self
            .client
            .get(format!("{}/packages", self.base_url))
            .header("Authorization", format!("Bearer {}", &self.auth_token))
            .send()
            .await?
            .text()
            .await?;

        Ok(body)
    }

    pub async fn get_letter_with_id(&self, id: String) -> Result<String, Error> {
        let body = self
            .client
            .get(format!("{}/letters/{}", self.base_url, id))
            .header("Authorization", format!("Bearer {}", &self.auth_token))
            .send()
            .await?
            .text()
            .await?;

        Ok(body)
    }

    pub async fn get_letters(&self) -> Result<String, Error> {
        let body = self
            .client
            .get(format!("{}/letters", self.base_url))
            .header("Authorization", format!("Bearer {}", &self.auth_token))
            .send()
            .await?
            .text()
            .await?;

        Ok(body)
    }

    pub async fn get_me(&self) -> Result<String, Error> {
        let body = self
            .client
            .get(format!("{}/me", self.base_url))
            .header("Authorization", format!("Bearer {}", &self.auth_token))
            .send()
            .await?
            .text()
            .await?;

        Ok(body)
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
            println!("{:#?}", data);
            let mut mail = Vec::new();

            for letter in data.get("mail").unwrap().as_array().unwrap().iter() {
                let id = if let Value::String(str) = &letter["id"] {
                    Some(str.parse().unwrap())
                } else {
                    None
                };
                let title = if let Value::String(str) = &letter["title"] {
                    Some(str.parse().unwrap())
                } else {
                    None
                };
                let created_at: Option<DateTime<Utc>> =
                    if let Value::String(str) = &letter["created_at"] {
                        Some(str.parse().unwrap())
                    } else {
                        None
                    };
                let updated_at: Option<DateTime<Utc>> =
                    if let Value::String(str) = &letter["updated_at"] {
                        Some(str.parse().unwrap())
                    } else {
                        None
                    };
                let public_url = if let Value::String(str) = &letter["public_url"] {
                    Some(str.parse().unwrap())
                } else {
                    None
                };
                let letter_type = if let Value::String(str) = &letter["type"] {
                    Some(str.parse().unwrap())
                } else {
                    None
                };
                let status = if let Value::String(str) = &letter["status"] {
                    Some(str.parse().unwrap())
                } else {
                    None
                };
                let tags = if let Value::Array(tags) = &letter["tags"] {
                    let mut tags_final = Vec::<String>::new();
                    for tag in tags.iter() {
                        tags_final.push(tag.to_string());
                    }
                    Some(tags_final)
                } else {
                    None
                };

                mail.push(Letter {
                    id,
                    title,
                    created_at,
                    updated_at,
                    public_url,
                    status,
                    tags,
                    letter_type,
                });
            }
            Ok(Some(mail))
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
