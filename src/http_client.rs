use reqwest::{Client,Response,Method,header::{HeaderMap,HeaderValue,CONTENT_TYPE},redirect::Policy,Url};
use serde_json::{Value};
use std::time::{Duration,Instant};
use std::mem;
use std::collections::HashMap;
use thiserror::Error;
use std::sync::Mutex;
use tokio;
use futures::future::join_all;

///Coding Summary 
///Create a function processes  get/put/post/delete method request.

#[derive(Debug, Error)]
pub enum HttpRequestError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid Method: {0}")]
    InvalidMethod(String),

    //#[error("Invalid method {0}, GET and DELETE should not have a body")]
    //InvalidMethod(Method),

    #[error("Request build error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub struct HttpRequestConfig {
    //message of request 
    pub method: Method,
    pub url: String,
    pub headers: HeaderMap,
    pub json_body: Option<Value>,
    pub form_body: Option<HashMap<String, String>>,

    //collect response time
    pub response_time: Option<Duration>,

    //client for send different request 
    pub client: Option<Client>,
}

impl HttpRequestConfig {
    //init client and request message 
    pub fn new(
        method: &str,
        url: &str,
        headers: Option<HeaderMap>,
        json_body: Option<Value>,
        form_body: Option<HashMap<String, String>>,
        timeout: Option<Duration>,
        cookie_store: Option<bool>,
        redirect_policy: Option<Policy>
    ) -> Result<Self,HttpRequestError>{
        //Check URL
        if Url::parse(url).is_err(){
            return Err(HttpRequestError::InvalidUrl(url.to_string()));
        }
        
        let method =  method
            .trim()
            .to_uppercase()
            .parse::<Method>()
            .map_err(|_| HttpRequestError::InvalidMethod(method.to_string()))?;

        //check `GET` and `DELETE` request should not have `body`
        if (method == Method::GET || method == Method::DELETE) && (json_body.is_some() && form_body.is_some()) {
            return Err(HttpRequestError::InvalidMethod(format!(
                        "{}should not have a body",method
            )));
        }

        //processes Headers ,default nil 
        let mut final_headers = headers.unwrap_or_default();

        //if have `body`,check `Content-Type`
        if json_body.is_some() && !final_headers.contains_key(CONTENT_TYPE) {
            final_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        } else if form_body.is_some() {
            final_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        }
        
        //let policy = mem::take(&mut self.redirect_policy);
        let mut policy = redirect_policy.unwrap_or(Policy::default()); //sue default policy
        let default_timeout = timeout.unwrap_or(Duration::from_secs(30));
        let default_cookie_store = cookie_store.unwrap_or(true);

        // 1. Create `Client`
        let client_builder = Client::builder()
            .timeout(default_timeout)
            .redirect(mem::take(&mut policy));

        let client = if default_cookie_store {
            client_builder.cookie_store(true).build().map_err(HttpRequestError::ReqwestError)?
        } else {
            client_builder.build().map_err(HttpRequestError::ReqwestError)?
        };

        //self.client = Some(client);

        //return HttpRequestConfig
        Ok(Self {
            method,
            url: url.to_string(),
            headers: final_headers,
            json_body,
            form_body,
            response_time: None,
            client: Some(client),
        })

    }

    

    pub async fn send(&mut self) -> Result<Response, reqwest::Error> {
        
        //1. ensure client is initialized.
        let client = self.client.as_ref().expect("Client not initialized");

        // 2. Create  `RequestBuilder` with method and url 
        let mut request = client.request(self.method.clone(), &self.url);

        // 3. Setup Headers
        request = request.headers(self.headers.clone());

        // 4. Setup Body
        if let Some(ref json) = self.json_body {
            request = request.json(json);
        } else if let Some(form) = &self.form_body {
            request = request.form(&form);
        }

        //  send request and record response time
        let start_time = Instant::now();//start time
        let response = request.send().await;
        self.response_time = Some(start_time.elapsed());

        Ok(response?)


    }

    fn build_request(&self) -> reqwest::RequestBuilder {
        let client = self.client.as_ref().expect("Client not initialized");
        let mut request = client.request(self.method.clone(), &self.url);
        request = request.headers(self.headers.clone());
        if let Some(ref json) = self.json_body {
            request = request.json(json);
        } else if let Some(ref form) = self.form_body {
            request = request.form(form);
        }
        request
    }

    pub async fn single_thread_send(&mut self,duration: u64) {
        let duration =  Duration::new(duration,0);
        let start = Instant::now();
        let mut count = 0;
        
        loop {
            let elapsed = start.elapsed();

            if elapsed >= duration {
                println!("Single thread stress test over,duration:{} seconds",elapsed.as_secs());
                break;
            }

            count += 1;
            let begain = Instant::now();
            let response = self.build_request().send().await;
            match response {
                Ok(resp) => println!("Request {}: {}: response_time{}", count, resp.status(),begain.elapsed().subsec_nanos()),
                Err(e) => println!("Request {} error: {}", count, e),
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    
    async fn test_get_request() {
        
        //assert_eq!(resp.status(), reqwest::StatusCode::OK);
    }
} 