#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash};





/* TODO: Add Connection timeout */

/* A Low-Level Http Wrapper Used for Handling http requests to geometry dash */
pub struct GDClient {
    client: reqwest::Client,
    host:String,
}

impl GDClient {
    pub (crate) fn new(gdps_host: String, connection_timeout: u64, proxy:Option<String>) -> GDClient{
        let client = reqwest::Client::builder();

        if proxy.is_some() {
            return GDClient{client: client
                /* Cloudflare firewall will throw a fit unless user-agent is removed */
                .user_agent("")
                .proxy(reqwest::Proxy::all(proxy.unwrap()).expect("Invalid Proxy Url"))
                .connect_timeout(std::time::Duration::from_secs(connection_timeout))
                .build()
                .expect("GDClient Failed to build"), 
                host:gdps_host}
        }
        
        /* Enabling a User-Agent will bring us unwanted trouble */
        
        return GDClient{client: client
                .user_agent("")
                .connect_timeout(std::time::Duration::from_secs(connection_timeout))
                .build()
                .expect("GDClient Failed to build"), 
                host:gdps_host
        };
    }


    // TODO Generate this code using python to speed things up and make this easier to maintain 
    pub (crate) fn url(&self, endpoint:String) -> reqwest::Url {
        return reqwest::Url::parse(&self.host).unwrap().join(&endpoint).expect("url failed to parse");
    }

    pub (crate) async fn post(&self, endpoint:String, fields:HashMap<&str, String>) -> Result<reqwest::Response, reqwest::Error> {
        return self.client.post(self.url(endpoint)).form(&fields).send().await
    }

    pub (crate) async fn get_gjdaily_level(&self, weekly:String) -> Result<reqwest::Response, reqwest::Error> {
        let mut fields = HashMap::new();
        fields.insert("weekly", weekly);
        fields.insert("secret", "Wmfd2893gb7".to_string());
        return self.post("/database/GetGJDailyLevel.php".to_string(), fields).await
    }

    pub (crate) async fn download_level(&self, levelid:String) -> Result<reqwest::Response, reqwest::Error> {
        let mut fields = HashMap::new();
        fields.insert("secret", "Wmfd2893gb7".to_string());
        fields.insert("levelID", levelid);
        return self.post("/database/downloadGJLevel22.php".to_string(), fields).await
    }

    pub (crate) async fn get_comments(&self, levelid:String, count:String, page:String, total:String, mode:String)  -> Result<reqwest::Response, reqwest::Error> {
        let mut fields = HashMap::new();
        fields.insert("secret", "Wfd2893gb7".to_string());
        fields.insert("levelID", levelid);
        fields.insert("count", count);
        fields.insert("page", page);
        fields.insert("total", total);
        fields.insert("mode", mode);
        return self.post("/database/getGJComments21.php".to_string(), fields).await
    }

    pub (crate) async fn get_user_info(&self, target_account_id:String) -> Result<reqwest::Response, reqwest::Error> {
        let mut fields: HashMap<&str, String> = HashMap::new();
        fields.insert("secret", "Wmfd2893gb7".to_string());
        fields.insert("targetAccountID", target_account_id);
        return self.post("/database/getGJUserInfo20.php".to_string(), fields).await
    }

    pub (crate) async fn fetch_user(&self, name_or_id:String) -> Result<reqwest::Response, reqwest::Error> {
        let mut fields: HashMap<&str, String> = HashMap::new();
        fields.insert("secret", "".to_string());
        fields.insert("str", name_or_id);
        return self.post("/database/getGJUsers20.php".to_string(), fields).await
    }

    /// Uploads a level Comment to geometry dash
    /// Important Paremeters
    /// - chk: the checksum value of the comment this must be encoded 
    ///     with the following order of username, comment, levelID , percent 
    /// - gjp2 : sha1 hash of user's password with the following salt inplace TODO: the Salt ""
    pub (crate) async fn upload_level_comment(
        &self, 
        accountid:String, 
        username:String, 
        gjp2:String,
        comment:String,
        levelid:String,
        chk:String,
        percent:String
    ) -> Result<reqwest::Response, reqwest::Error> {
        let mut fields: HashMap<&str, String> = HashMap::new();
        fields.insert("secret", "Wmfd2893gb7".to_string());
        fields.insert("accountID", accountid);
        fields.insert("userName", username);
        fields.insert("gjp2", gjp2);
        fields.insert("comment", comment);
        fields.insert("levelID", levelid);
        fields.insert("chk", chk);
        fields.insert("percent", percent);
        return self.post("/database/uploadGJComment21.php".to_string(), fields).await;
    }

    pub (crate) async fn upload_profile_comment(
        &self,
        accountid:String,
        username:String,
        gjp2:String,
    ){

    }

}



pub struct GDClientBuilder {
    host: String,
    proxy:Option<String>,
    connection_timeout: u64
}

impl GDClientBuilder {
    pub fn new() -> (){
        GDClientBuilder{host:"https://www.boomlings.com".to_string(), proxy:None, connection_timeout:30};   
    }

    /** This Can be used to set a GDPS Server as the host to request from */ 
    pub fn set_host(&mut self, host: String){
        self.host = host;
    }
    /** Sets a proxy to request from NOTE: The Majority of Proxies are banned from use, Use at your own risk!! */
    pub fn set_proxy(&mut self, proxy: String){
        self.proxy = Some(proxy);
    }

    /** Sets a connection timeout to the server DEFAULT: 30 seconds */
    pub fn set_connection_timeout(&mut self, timeout: u64){
        self.connection_timeout = timeout;
    }



    pub fn finish(self) -> GDClient {
        return GDClient::new(self.host, self.connection_timeout, self.proxy);
    }

}


