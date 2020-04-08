extern crate serde_json;
use reqwest::Error;
use reqwest::{Client};
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use http::header::HeaderMap;
//use http::Uri;
use url::{form_urlencoded,UrlQuery,Url};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;



type MyResult= Result<String,Box<dyn std::error::Error + Send + Sync + 'static>>;
//type GET=RefCell<dyn  FnOnce(HashMap<String,String>)->MyResult>;
//type GET=Rc<dyn FnOnce(HashMap<String,String>)->MyResult>;
type GET=Box<dyn  FnOnce(HashMap<String,String>)->MyResult>;
type SDK=HashMap<String,GET>;
//type SDK=HashMap<String,Box<dyn Fn(i32)->i32>>;

enum Methods{
    post,
    get,
    put,
    delete,
}

pub struct Api{
    url:String,
    method:Methods,
    query:HashMap<String,String>,
    data:HashMap<String,String>,
}
pub struct Api1<T1,T2>{
    url:String,
    method:Methods,
    query:T1,
    data:T2,
}

/// { "format":"json", "aggr":1, "flag_qc":1, "p":1, "n":30, "w":"简单爱" }
#[derive(Serialize, Deserialize)]
pub struct SearchSong {
    #[serde(rename = "format")]
    format: String,

    #[serde(rename = "aggr")]
    aggr: i64,

    #[serde(rename = "flag_qc")]
    flag_qc: i64,

    #[serde(rename = "p")]
    p: i64,

    #[serde(rename = "n")]
    n: i64,

    #[serde(rename = "w")]
    w: String,
}





/// https://www.jianshu.com/p/67e4bd47d981
/// https://docs.rs/crate/reqwest/0.10.1

fn add_qs(u:&str,h:&Vec<(String,String)>)->String{
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

macro_rules!hash{
    ($($k:expr=>$v:expr),*)=> {{
            let mut m=HashMap::new();
            $(m.insert($k.to_string(),$v.to_string());)*
            m
    }}
}

fn mix_hash<K,V>(a:HashMap<K,V>,b:HashMap<K,V>)->HashMap<K,V>
 where 
 K:Hash+Eq+Debug,
 V:Hash+Eq+Debug,
{
     let mut q2 :HashMap<K,V>= HashMap::new();
     for (k,v) in a{
         q2.insert(k,v);
     }
     for (k,v) in b{
         q2.insert(k,v);
     }
     //println!("{:?}",q2);
     q2
}

fn hash2vec<K,V>(h:HashMap<K,V>)->Vec<(K,V)>
 where 
 K:Hash+Eq+Debug,
 V:Hash+Eq+Debug,
{
   let mut a:Vec<(K,V)>=vec![];
   for (k,v) in h{
       a.push((k,v));
   }
   a
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
async fn get(u:&str,q:&Vec<(String,String)>) ->MyResult
{
    let u1=add_qs(u,q);
    println!("{}",u1);
    Ok(u1)
    // let client=create_client()?;
    // let body = client.get(&u1)
    //     .send()
    //     .await?
    //     .text()
    //     .await?;
    // Ok(body)
}

fn get1(u:String,q:HashMap<String,String>)->GET{
    //let mix1=|x:i32|->i32{x*2};
    let mix=move |q1:HashMap<String,String>| {
        let q2=mix_hash(q,q1);
        let v=hash2vec(q2);
        //println!("{:?}",v);
        get(&u,&v)
        //Ok("123".to_string())
    };
    Box::new(mix)
    //Rc::new(mix)
    //RefCell::new(mix)
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



fn test_get() {
    //let u="https://c.y.qq.com/soso/fcgi-bin/client_search_cp?format=json&aggr=1&flag_qc=1&p=1&n=30&w=简单爱";
    let u="http://httpbin.org/get";
    let a:Vec<(String,String)>=vec![("x","1"),("y","2")]
        .iter()
        .map(|(k,v)|(k.to_string(),v.to_string()))
        .collect();
    let body=get(u,&a).unwrap();
    println!("{:?}", body);
}

fn test_get1(){
    let q1=hash![
                "format"=>"json",
                "aggr"=>"1",
                "flag_qc"=>"1",
                "p"=>"1",
                "n"=>"30",
                "w"=>"简单爱"
            ];
    let q2=hash!["w"=>"从开始到现在","p"=>"1"];
    let u="https://c.y.qq.com/soso/fcgi-bin/client_search_cp".to_string();
    let f=get1(u,q1);
    let r=f(q2).unwrap();
    println!("{:?}",r);
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



fn create_sdk()->SDK{
    let mut apis:HashMap<String,Api>=HashMap::new();
    apis.insert("search".to_string(),Api{
        url:"https://c.y.qq.com/soso/fcgi-bin/client_search_cp".to_string(),
        method:Methods::get,
        query:hash![
            "format"=>"json",
            "aggr"=>"1",
            "flag_qc"=>"1",
            "p"=>"1",
            "n"=>"30",
            "w"=>"简单爱"
        ],
        data:hash![],
    });

   //...

    let mut sdk:SDK =HashMap::new();
    for (k,Api{url,method,query,data}) in apis{
        let g:GET=match method{
            Methods::get => {get1(url,query)}
            _ => {get1(url,query)}
            //_ =>{Box::new(move |x|x*2)}
            // Methods::post => {}
            // Methods::put => {}
            // Methods::delete => {}
        };
        sdk.insert(k.to_string(),g);
    }
    sdk
}




fn test_sdk()->MyResult{
    let s=create_sdk();
    println!("{:?}",s.keys());

    let q2=hash!["w"=>"从开始到现在","p"=>"1"];
    let r=s.get("search").unwrap()(q2)?;
    //let r=(&s["search"])(q2)?;
    println!("{}",r);
    Ok("123".to_string())
}


fn main(){
   //test_sdk();
   //test_get();
   //test_get1();
   //test_post_json();
   //test_post_form();
}

