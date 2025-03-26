use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let body = reqwest::get("http://www.baidu.com")
        .await?
        .text()
        .await?;
    
    println!("body= {body:?}");
    
    let params = [("foo","bar"),("baz","quux")];
    let client = reqwest::Client::new();
    let res = client.post("http://www.baidu.com")
        .form(&params)
        .send()
        .await?;
    println!("res={res:?}");
    Ok(())
}
