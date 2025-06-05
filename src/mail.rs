use anyhow::Error;
use reqwest::Client;

pub struct MailClient {
    auth_token: String,
    base_url: String,
    client: Client,
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

    pub async fn get_mail(&self) -> Result<String, Error> {
        let body = self
            .client
            .get(format!("{}/mail", self.base_url))
            .header("Authorization", format!("Bearer {}", &self.auth_token))
            .send()
            .await?
            .text()
            .await?;

        Ok(body)
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
