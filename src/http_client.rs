use reqwest::{Client,ClientBuilder,Error,Method,header::{HeaderMap,HeaderValue,CONTENT_TYPE},redirect::Policy,Url};
use serde_json::{json,value};
use std::time::{Duration,Instant};
use std::mem;

///Coding Summary 
///Create a function processes  get/put/post/delete method request.

pub struct HttpRequestConfig {
    pub method: Method,
    pub url: String,
    pub headers: HeaderMap,
    pub json_body: Option<Value>,
    pub form_body: Option<HashMap<String, String>>,
    pub timeout: Duration,
    pub cookie_store: bool,
    pub redirect_policy: Policy,
    pub response_time: Option<Duration>,
    pub client: Option<Client>,
}

impl HttpRequestConfig {
    pub fn new(
        method: Method,
        url: &str,
        headers: Option<HeaderMap>,
        json_body: Option<Value>,
        form_body: Option<HashMap<String, String>>,
        timeout: Option<Duration>,
        cookie_store: Option<bool>,
        redirect_policy: Option<Policy>
    ) -> Result<Self,String>{
        //Check URL
        if Url::parse(url).is_err() {
            return Err("Invalid URL".to_string());
        }

        //check `GET` and `DELETE` request should not have `body`
        if (method == Method::GET || method == Method::DELETE) && body.is_some(){
            return Err(format!("Method {} should not have a body", method));
        }

        //processes Headers ,default nil 
        let mut final_headers = headers.unwrap_or_default();

        //if have `body`,check `Content-Type`
        if json_body.is_some() && !final_headers.contains_key(CONTENT_TYPE) {
            final_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        } else if form_body.is_some() {
            final_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        }

        //return HttpRequestConfig
        Ok(Self {
            method,
            url: url.to_string(),
            headers: final_headers,
            json_body,
            form_body,
            timeout: timeout.unwrap_or(Duration::from_secs(30)), // default  30s
            cookie_store: cookie_store.unwrap_or(true), // default turn on Cookie
            redirect_policy: redirect_policy.unwrap_or(Policy::default()), // use default redirect 
            response_time: None,
            client: None,
        })

    }

    
    /// init Client
    pub async fn init_client(mut self) -> Result<Self, reqwest::Error> {
        
        let policy = mem::take(&mut self.redirect_policy);

        // 1. Create `Client`
        let client_builder = Client::builder()
            .timeout(self.timeout)
            .redirect(policy);

        let client = if self.cookie_store {
            client_builder.cookie_store(true).build()?
        } else {
            client_builder.build()?
        };

        self.client = Some(client);
        Ok(self)

    }

    pub async fn send(&mut self) -> Result<Response, reqwest::Error> {
        
        //1. ensure client is initialized.
        let client = self.client.as_ref().expect("Client not initialized");

        // 2. Create  `RequestBuilder`
        let mut request = client.request(self.method.clone(), &self.url);

        // 3. Setup Headers
        request = request.headers(self.headers.clone());

        // 4. Setup Body
        if let Some(ref json) = self.json_body {
            request = request.json(json);
        } else if let Some(form) = self.form_body {
            request = request.form(form);
        }

        //  send request and record response time
        let start_time = Instant::now();//start time
        let response = request.send().await;
        self.response_time = Some(start_time.elapsed());


        Ok(response)


    }

}


#[cfg(test)]
mod tests {
    use super::*;
    
    async fn test_get_request() {
        

        //assert_eq!(resp.status(), reqwest::StatusCode::OK);


    }

}
