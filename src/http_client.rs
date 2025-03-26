use reqwest::{Client,ClientBuilder,Error,Method,header::{HeaderMap,HeaderValue,CONTENT_TYPE}};
use serde_json::{json,value};
use std::time::Duration;

///Coding Summary 
///Create four functions about get/put/post/delete, and create a send_request function to call these four functions uniformly. 

//Commonly used HTTP request
pub async fn send_request(
    method: &str,
    url: &str,
    body: Option<Value>,
    headers: Option<HeaderMap>,
    timeout: Option<Duration>
) -> Result<reqwest::Response,reqwest::Error> {

    // setup headers
    let mut headers = HeaderMap::new();
    //TODO headers.insert("","");

    //TODO setup request body 
    let body = json!({
        "key":"value"
    })

    //TODO setup timeout 
    let timeout = Some(Duration::from_secs(10));//replace 10 secs with  time varible

    //TODO setup redirect policy 
    let redirect_policy = Policy::limited(5);//replace 5 with varible
           
    //send request 
    let client_builder = Client::builder()
        .timeout(timeout.unwrap_or(Duration::from_secs(30)))
        .redirect(redirect_policy);

    let client_builder = if cookie_store {
        client_builder.cookie_store(true)
    } else {
        client_builder
    };

    let client = client_builder.build?;

    let mut request_builder = client.request(method,url);
    
    if let Some(h) = headers {
        request_builder = request_builder.headers(h);
    }

    if let Some(b) = body {
        request_builder = request_builder.json(&b);
    }

    request_builder.send().await

}

#[cfg(test)]
mod tests {
    use super::*;
    
    async fn test_get_request() {
        
        let method = "GET";
        let url = "www.baidu.com";
        send_request(method,url)


        assert_eq!(resp.status(), reqwest::StatusCode::OK);


    }

}
