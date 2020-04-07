use reqwest::Error;
use reqwest::{Client};
use std::collections::HashMap;
use http::header::HeaderMap;
//use http::Uri;
use url::{form_urlencoded,UrlQuery,Url};

/// https://www.jianshu.com/p/67e4bd47d981
/// https://docs.rs/crate/reqwest/0.10.1

fn add_qs(u:&str,h:&Vec<(&str,&str)>)->String{
    let mut uri = Url::parse(u).unwrap();
    for (k,v) in h{
        uri.query_pairs_mut().append_pair(k,v);
    }
    uri.to_string()
}

fn to_qs(h:&Vec<(&str,&str)>)->String{
    let mut qs=form_urlencoded::Serializer::new(String::new());
    for (k,v) in h{
        qs.append_pair(k,v);
    }
    qs.finish()
}

fn create_client()-> Result<(Client), Box<dyn std::error::Error + Send + Sync + 'static>>{
        let h=vec![
            ("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0"),
            ("Accept-Language", "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2"),
            ("Accept", "*/*"),
            ("Pragma", "no-cache"),
            ("Cache-Control", "no-cache"),
            //("X-Forwarded-For", ip.as_str().parse().unwrap()),
            //("x-real-ip", ip.as_str().parse().unwrap()),
        ];
        let mut headers = HeaderMap::new();
        for (k,v) in h{
            headers.insert(k, v.to_string().as_str().parse().unwrap());
        }
        let client = reqwest::Client::builder()
                .cookie_store(true)
                .default_headers(headers)
                .build()?;
        Ok(client)
}


#[tokio::main]
async fn get(u:&str,q:&Vec<(&str,&str)>) ->Result<String,Box<dyn std::error::Error + Send + Sync + 'static>>{
    let u1=add_qs(u,q);
    let client=create_client()?;
    let body = client.get(&u1)
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

fn test_get() {
    //let u="https://c.y.qq.com/soso/fcgi-bin/client_search_cp?format=json&aggr=1&flag_qc=1&p=1&n=30&w=简单爱";
    let u="http://httpbin.org/get";
    let a=vec![("x","1"),("y","2")];
    let body=get(u,&a).unwrap();
    println!("{:?}", body);
}

#[tokio::main]
async fn post_form(u:&str,d:&Vec<(&str,&str)>)->Result<String,Box<dyn std::error::Error + Send + Sync + 'static>>{
     let client=create_client()?;
     let res = client.post(u)
         .form(d)
         .send()
         .await?
         .text()
         .await?;
     Ok(res)
}

#[tokio::main]
async fn post_json(u:&str,d:&HashMap<&str,&str>) ->Result<String,Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client=create_client()?;
    let res = client.post(u)
        .json(d)
        .send()
        .await?
        .text()
        .await?;
     Ok(res)
}

fn test_post_json(){
    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map :HashMap<&str,&str>= HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");
    let u="http://httpbin.org/post";
    let res=post_json(u,&map);
    println!("{:?}", res);
}

fn test_post_form(){
    let u="http://httpbin.org/post";
    let params =vec![("foo", "bar"), ("baz", "quux")];
    let res=post_form(u,&params);
    println!("{:?}", res);
}

fn main(){
   //test_get();
   //test_post_json();
   //test_post_form();
}

