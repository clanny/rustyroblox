//use std::error::Error;
use crate::util::status_codes::status_code_to_error;
use crate::util::Error;

pub struct RequestJar {
    pub roblosecurity: Option<String>,
    pub xsrf_token: Option<String>,

    pub proxy: Option<String>,
}

impl RequestJar {
    pub async fn new() -> RequestJar {
        RequestJar {
            roblosecurity: None,
            xsrf_token: None,

            proxy: None,
        }
    }

    pub async fn set_roblosecurity(&mut self, roblosecurity: String) -> Result<(), Box<Error>> {
        self.roblosecurity = Some(roblosecurity);
        self.get_xsrf_token().await?;
        Ok(())
    }

    pub fn set_proxy(&mut self, proxy: String) {
        self.proxy = Some(proxy);
    }

    pub fn get_reqwest_client(&self) -> reqwest::Client {
        let mut client = reqwest::Client::new();

        if self.proxy.is_some() {
            client = reqwest::Client::builder()
                .proxy(reqwest::Proxy::all(self.proxy.as_ref().unwrap()).unwrap())
                .build()
                .unwrap();
        }

        client
    }

    pub async fn get(
        &mut self,
        url: &str,
        soft_fail: bool, // Determines if it should error or not if the status code is not 200.
    ) -> Result<reqwest::Response, Box<Error>> {
        let client = self.get_reqwest_client();

        let response = client
            .get(url)
            .header(
                "Cookie",
                if self.roblosecurity.is_some() {
                    format!(".ROBLOSECURITY={};", self.roblosecurity.as_ref().unwrap())
                } else {
                    "".to_string()
                },
            )
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status() != 200 && !soft_fail {
                    let error = status_code_to_error(res.status());
                    if error.is_some() {
                        return Err(Box::new(error.unwrap_or(Error::Network)));
                    };
                }
                Ok(res)
            }
            Err(_) => Err(Box::new(Error::Network)),
        }
    }

    pub async fn get_json<T: for<'de> serde::Deserialize<'de>>(
        &mut self,
        url: &str,
    ) -> Result<T, Box<Error>> {
        let response = self.get(url, false).await?;
        let json = response.json::<T>().await;

        match json {
            Ok(json) => Ok(json),
            Err(_) => Err(Box::new(Error::JSON)),
        }
    }

    pub async fn post(
        &mut self,
        url: &str,
        soft_fail: bool, // Determines if it should error or not if the status code is not 200.
        data: String,
    ) -> Result<reqwest::Response, Box<Error>> {
        let client = self.get_reqwest_client();

        let response = client
            .post(url)
            .body(data)
            .header(
                "Cookie",
                if self.roblosecurity.is_some() {
                    format!(".ROBLOSECURITY={};", self.roblosecurity.as_ref().unwrap())
                } else {
                    "".to_string()
                },
            )
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status() != 200 && !soft_fail {
                    let error = status_code_to_error(res.status());
                    if error.is_some() {
                        return Err(Box::new(error.unwrap_or(Error::Network)));
                    };
                }
                Ok(res)
            }
            Err(_) => Err(Box::new(Error::Network)),
        }
    }

    pub async fn post_json<T: for<'de> serde::Deserialize<'de>, PD: serde::Serialize>(
        &mut self,
        url: &str,
        data: PD,
    ) -> Result<T, Box<Error>> {
        let data = serde_json::to_string(&data).unwrap();
        let response = self.post(url, false, data).await?;
        let json = response.json::<T>().await;

        match json {
            Ok(json) => Ok(json),
            Err(_) => Err(Box::new(Error::JSON)),
        }
    }

    pub async fn patch(
        &mut self,
        url: &str,
        soft_fail: bool, // Determines if it should error or not if the status code is not 200.
        data: String,
    ) -> Result<reqwest::Response, Box<Error>> {
        let client = self.get_reqwest_client();

        let response = client
            .patch(url)
            .body(data)
            .header(
                "Cookie",
                if self.roblosecurity.is_some() {
                    format!(".ROBLOSECURITY={};", self.roblosecurity.as_ref().unwrap())
                } else {
                    "".to_string()
                },
            )
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status() != 200 && !soft_fail {
                    let error = status_code_to_error(res.status());
                    if error.is_some() {
                        return Err(Box::new(error.unwrap_or(Error::Network)));
                    };
                }
                Ok(res)
            }
            Err(_) => Err(Box::new(Error::Network)),
        }
    }

    pub async fn patch_json<T: for<'de> serde::Deserialize<'de>, PD: serde::Serialize>(
        &mut self,
        url: &str,
        data: PD,
    ) -> Result<T, Box<Error>> {
        let data = serde_json::to_string(&data).unwrap();
        let response = self.patch(url, false, data).await?;
        let json = response.json::<T>().await;

        match json {
            Ok(json) => Ok(json),
            Err(_) => Err(Box::new(Error::JSON)),
        }
    }

    pub async fn get_xsrf_token(&mut self) -> Result<(), Box<Error>> {
        return Ok(()); // TODO: Implement this? Might not be needed, its in noblox.js but from my very limited research it doesnt appear to be used anymore
        if self.roblosecurity.is_none() {
            return Err(Box::new(Error::Authentication));
        }

        let client = reqwest::Client::new();
        let response = client
            .post("https://auth.roblox.com/v2/logout")
            .header(
                "Cookie",
                format!(".ROBLOSECURITY={};", self.roblosecurity.as_ref().unwrap()),
            )
            .header("Content-Length", "0")
            .header("Origin", "https://www.roblox.com")
            .header("Referer", "https://www.roblox.com/")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.5304.107 Safari/537.36")
            .send()
            .await
            .unwrap(); // TODO: Handle error

        println!(
            " {}, {}",
            //response.headers(),
            response.status(),
            response.text().await.unwrap()
        );

        //let text = res.text().await?;
        //let doc = Html::parse_document(&text);
        //let selector = Selector::parse("meta[name='X-CSRF-TOKEN']").unwrap();
        //let meta = doc.select(&selector).next().unwrap();
        //let token = meta.value().attr("content").unwrap();
        //self.xsrf_token = Some(token.to_string());
        Ok(())
    }
}
