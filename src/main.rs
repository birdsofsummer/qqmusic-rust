extern crate serde_json;

//use reqwest::Error;
use reqwest::{Client};


use http::header::HeaderMap;
//use http::Uri;
use url::{form_urlencoded,UrlQuery,Url};
use serde::{Deserialize, Serialize};

use bytes::Bytes;

use std::rc::Rc;
use std::cell::RefCell;

use std::process;
use std::env;
use std::fs;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::{thread, time};
use std::str::FromStr;
use std::hash::Hash;
use std::collections::HashMap;
use std::fmt::Debug;

extern crate chrono;
use chrono::prelude::*;


#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref JSON_H:CorsHeaders=create_h(mime::json);
    static ref BASE64_H:CorsHeaders=create_h(mime::base64);
 //   static ref CONFIG: Config= {
 //       let client_id=env::var("baidu_ocr_APIKey").unwrap();
 //       let client_secret=env::var("baidu_ocr_SecretKey").unwrap();
 //       let query= Config { 
 //           grant_type:"client_credentials".into(),
 //           client_id: client_id.into(),
 //           client_secret:client_secret.into(),
 //       };
 //       query
 //   };
}

enum mime{
    json,
    img,
    base64,
}
fn create_h(m:mime)->CorsHeaders{
    let ACCESS_CONTROL_ALLOW_HEADERS:String="accept,accept_encoding,cf_connecting_ip,cf_ipcountry,cf_ray,cf_visitor,connection,content_length,content_type,host,user_agent,x_forwarded_proto,x_real_ip,accept_charset,accept_language,accept_datetime,authorization,cache_control,date,if_match,if_modified_since,if_none_match,if_range,if_unmodified_since,max_forwards,pragma,range,te,upgrade,upgrade_insecure_requests,x_requested_with,chrome_proxy,purpose,accept,accept_language,content_language,content_type,dpr,downlink,save_data,viewport_width,width,token".to_owned();
    let ACCESS_CONTROL_ALLOW_ORIGIN:String="*".to_owned();
    let ACCESS_CONTROL_ALLOW_METHODS: String="GET,POST,PUT,PATCH,TRACE,DELETE,HEAD,OPTIONS".to_owned();
    let ACCESS_CONTROL_MAX_AGE: String= "1728000".to_owned();
    let t=match m{
        mime::json=>{"application/json; charset=utf-8".to_owned()}
        mime::img=>{"image/jpeg;".to_owned()}
        mime::base64=>{"text/html; charset=utf-8".to_owned()}
        _=>{"text/html; charset=utf-8".to_owned()}
    };
    CorsHeaders{
        content_type:t,
        access_control_allow_origin: ACCESS_CONTROL_ALLOW_ORIGIN.clone(),
        access_control_allow_methods: ACCESS_CONTROL_ALLOW_METHODS.clone(),
        access_control_allow_headers: ACCESS_CONTROL_ALLOW_HEADERS.clone(),
        access_control_max_age: ACCESS_CONTROL_MAX_AGE.clone(),
    }
}



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



/// {"code":0,"cid":205361747,"userip":"101.233.48.172","data":{"expiration":80400,"items":[{"subcode":104003,"songmid":"003lghpv0jfFXG","filename":"C400003lghpv0jfFXG.m4a","vkey":""}]}}

#[derive(Serialize, Deserialize,Debug)]
pub struct Vkey {
    #[serde(rename = "code")]
    code: i64,

    #[serde(rename = "cid")]
    cid: i64,

    #[serde(rename = "userip")]
    userip: String,

    #[serde(rename = "data")]
    data: Data,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Data {
    #[serde(rename = "expiration")]
    expiration: i64,

    #[serde(rename = "items")]
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Item {
    #[serde(rename = "subcode")]
    subcode: i64,

    #[serde(rename = "songmid")]
    songmid: String,

    #[serde(rename = "filename")]
    filename: String,

    #[serde(rename = "vkey")]
    vkey: String,
}





/// https://www.jianshu.com/p/67e4bd47d981
/// https://docs.rs/crate/reqwest/0.10.1


/// struct - > str
fn to_s<T: serde::Serialize>(d:T)->String{
    serde_json::json!(d).to_string()
}

/// str -> json
fn to_json<'a,T : Deserialize<'a>>(s:&'a str) -> T{
    serde_json::from_str::<T>(s).unwrap()
}


fn base64encode(s:String)->String{
   let c=Bytes::from(s);
   let d=base64::encode(&c);
   d
}


fn base64decode(s:&str)->String{
   let b=base64::decode(s).unwrap();
   let s3=String::from_utf8(b).unwrap();
   s3
}
fn test_base64(){
   let s1="123";
   let s2=base64encode(s1.to_owned()); //MTIz
   let s3=base64decode(&s2);
   println!("{}|{}|{}",s1,s2,s3);     // 123
}


fn timestamp()-> String{
    let t=Utc::now();
    let ms=t.timestamp_millis();
    let s=ms/1000;
    let s1=format!("{}",s);
    println!("{}",s1);
    s1
}



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

macro_rules!to_qs{
    ($($k:expr=>$v:expr),*)=> {{
            let mut qs=form_urlencoded::Serializer::new(String::new());
            $(
                qs.append_pair($k,$v);
             )*
            qs.finish()
    }}
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


fn create_client()-> Result<Client, Box<dyn std::error::Error + Send + Sync + 'static>>{
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
async fn get(u:&str,q:&Vec<(String,String)>) ->MyResult {
    let u1=add_qs(u,q);
    //println!("{}",u1);
    //Ok(u1)
    let client=create_client()?;
    let body = client.get(&u1)
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

fn url2name(u:&str)->Result<String,url::ParseError>{
    let u1= Url::parse(u)?;   
    let p=u1.path().split("/").last().unwrap();
    Ok(p.to_string())
}


#[tokio::main]
async fn download(u:&str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>  {
    let client=create_client()?;
    let body = client.get(u)
        .send()
        .await?
        .bytes()
        .await?;
    let path=url2name(u)?;
    println!("download to {}",path);
    let mut o: File = File::create(path)?;
    o.write(&Bytes::from(body))?;
    Ok(())
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



fn get_song(h:HashMap<String,String>)->String{
        let q1=hash![
                    "format"=>"json",
                    "aggr"=>"1",
                    "flag_qc"=>"1",
                    "p"=>"1",
                    "n"=>"30",
                    "w"=>"简单爱"
                ];
        let u="https://c.y.qq.com/soso/fcgi-bin/client_search_cp".to_string();
        let f=get1(u,q1);
        let r=f(h).unwrap();
        r
}

trait QQmusic {
    fn get_song(&self,h:HashMap<String,String>)->String;
    fn test(&self);
    //...
    //fn echo(&self);
    //fn double(&mut self);
}

impl QQmusic for i32 {
    fn get_song(&self,h:HashMap<String,String>)->String{
            let q1=hash![
                        "format"=>"json",
                        "aggr"=>"1",
                        "flag_qc"=>"1",
                        "p"=>"1",
                        "n"=>"30",
                        "w"=>"简单爱"
                    ];
            let u="https://c.y.qq.com/soso/fcgi-bin/client_search_cp".to_string();
            let f=get1(u,q1);
            let r=f(h).unwrap();
            r
    }

    fn test(&self){
       let a=1_i32;
       let q2=hash!["w"=>"从开始到现在".to_owned(),"p"=>"1"];
       let r=a.get_song(q2);
       println!("{}",r);
    }
    //fn echo(&self){println!("{}",self);}
    //fn double(&mut self){*self*=2;}
}


fn test_get1(){
    let q2=hash!["w"=>"从开始到现在","p"=>"1"];
    let r=get_song(q2);
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

fn id2album (id:&str)->String{
    format!("http://imgcache.qq.com/music/photo/album_300/17/300_albumpic_{}_0.jpg",id)
}
fn mid2file(mid:&str)->String{
    // 文件类型
    let SONG_TYPES=hash![
           "A000"=>"ape",
           "F000"=>"flac",
           "M800"=>"mp3",
           "M500"=>"mp3",
           "C400"=>"m4a"
    ];
    format!("C400{}.m4a",mid)
}

//"http://ws.stream.qqmusic.qq.com/C400003lghpv0jfFXG.m4a?fromtag=0&guid=126548448&vkey=D661E5DF19B8FEB2FBFC554276746AC608AE98B0F30595B3B3BAD5C1C89ECCDD7BE599E306F786621856D22D6BD6B96F5DD344CF3814DB71"
fn vkey2song(mid:&str,vkey:&str)->String{
    let q=to_qs![
      "fromtag" => "0",
      "guid" => "126548448",
      "vkey" => vkey 
    ];
    let file=format!("C400{}.m4a",mid);
    format!("http://ws.stream.qqmusic.qq.com/{}?{}",file,q)
}


/// {"code":0,"cid":205361747,"userip":"101.233.48.172","data":{"expiration":80400,"items":[{"subcode":104003,"songmid":"003lghpv0jfFXG","filename":"C400003lghpv0jfFXG.m4a","vkey":"123"}]}}
fn vkey2song1(s:String)->Vec<String>{
    let d: Vkey = serde_json::from_str(&s).unwrap();
    let f=move |i:&Item|->String{format!("http://ws.stream.qqmusic.qq.com/{}?fromtag=0&guid=126548448&vkey={}",i.filename,i.vkey)} ;
    let r:Vec<String>=d.data.items
        .iter()
        .filter(|i|(i.vkey.len()>0))
        .map(|i|f(i))
        .collect();
    r
}


fn create_sdk()->SDK{
    let mut apis:HashMap<String,Api>=HashMap::new();

    let top_lists=hash![
	"27"=> "巅峰榜·新歌",
	"28"=> "巅峰榜·网络歌曲",
	"29"=> "巅峰榜·影视金曲",
	"30"=> "巅峰榜·梦想的声音",
	"31"=> "巅峰榜·微信分享",
	"32"=> "巅峰榜·音乐人",
	"33"=> "全军出击·巅峰榜·歌手2018",
	"34"=> "巅峰榜·人气",
	"35"=> "QQ音乐巅峰分享榜",
	"36"=> "巅峰榜·K歌金曲",
	"50"=> "巅峰榜·中国有嘻哈",
	"51"=> "巅峰榜·明日之子",
	"52"=> "巅峰榜·腾讯音乐人原创榜",
	"53"=> "机车",
	"54"=> "勇闯天涯·巅峰榜·明日之子",
	"55"=> "江小白YOLO·巅峰榜·中国新说唱",
	"56"=> "巅峰榜·2018中国好声音",
	"57"=> "电音榜",
	"58"=> "说唱榜",
	"59"=> "香港地区榜",
	"60"=> "抖音排行榜",
	"61"=> "台湾地区榜"
    ];

    apis.insert("search".to_string(),Api{
        url:"https://c.y.qq.com/soso/fcgi-bin/client_search_cp".to_string(),
        method:Methods::get,
        query:hash![
            //"cr": "1",
            //"t":"8",
            "format"=>"json", //每页有限制
            "aggr"=>"1",
            "flag_qc"=>"1",
            "p"=>"1", ////p = x.data.song.totalnum / 60 , 16
            "n"=>"30",
            "w"=>"简单爱"
        ],
        data:hash![],
    });

    apis.insert("toplist1".to_string(),Api{
        url:"https://c.y.qq.com/v8/fcg-bin/fcg_v8_toplist_cp.fcg".to_string(),
        method:Methods::get,
        query:hash![
              "g_tk" => "5381",
              "uin" => "0",
              "format" => "json",
              "inCharset" => "utf-8",
              "outCharset" => "utf-8¬ice=0",
              "platform" => "h5",
              "needNewCode" => "1",
              "tpl" => "3",
              "page" => "detail",
              "type" => "top",
              "topid" => "27", //27..100 top_lists
              "_" => "1519963122923" 
        ],
        data:hash![],
    });

    // 有些songmid不行
    apis.insert("vkey".to_string(),Api{
        url:"https://c.y.qq.com/base/fcgi-bin/fcg_music_express_mobile3.fcg".to_string(),
        method:Methods::get,
        query:hash![
          "format" => "json205361747",
          "platform" => "yqq",
          "guid" => "126548448",
          "cid" => "205361747",
          "songmid" => "003lghpv0jfFXG",            //  查询多个songmid=id1,id2,id3,id4  get url长度有限制,不可无限多个,批量获取需分页
          "filename" => "C400003lghpv0jfFXG.m4a"    //  c400 + songmid + .m4a   多个同上逗号分隔 k=v1,v2,v3,v4...
        ],
        data:hash![],
    });

    apis.insert("album".to_string(),Api{
        url:"https://c.y.qq.com/v8/fcg-bin/fcg_v8_album_info_cp.fcg".to_string(),
        method:Methods::get,
        query:hash![
            "albummid"=>"000QXjVc1r7NQO" 
        ],
        data:hash![],
    });

    apis.insert("lyric".to_string(),Api{
        url:"https://c.y.qq.com/lyric/fcgi-bin/fcg_query_lyric_new.fcg".to_string(),
        method:Methods::get,
        query:hash![
            "songmid"=>"xxx",
            "format"=>"json",
            "nobase64"=>"1"
        ],
        data:hash![],
    });


/*
{
  "req_0": {
    "module": "vkey.GetVkeyServer",
    "method": "CgiGetVkey",
    "param": {
      "guid": "358840384",
      "songmid": "ccc",
      "songtype": [
        0
      ],
      "uin": "1443481947",
      "loginflag": 1,
      "platform": "20"
    }
  },
  "comm": {
    "uin": "18585073516",
    "format": "json",
    "ct": 24,
    "cv": 0
  }
}
*/
    apis.insert("song".to_string(),Api{
        url:"https://u.y.qq.com/cgi-bin/musicu.fcg".to_string(),
        method:Methods::get,
        query:hash![
            "format"=>"json",
            "data"=>""  //上面这个复杂的json...
        ],
        data:hash![],
    });

    //https://www.jianshu.com/p/ce1180eac37b
    apis.insert("playlist".to_string(),Api{
        url:"https://c.y.qq.com/qzone/fcg-bin/fcg_ucc_getcdinfo_byids_cp.fcg".to_string(),
        method:Methods::get,
        query:hash![
              "uin"=> "0",
              "format"=> "json",
              "inCharset"=> "utf-8",
              "outCharset"=> "utf-8",
              "notice"=> "0",
              "platform"=> "h5",
              "needNewCode"=> "1",
              "new_format"=> "1",
              "pic"=> "500",
              "disstid"=> "3719969047", //歌单id
              "type"=> "1",
              "json"=> "1",
              "utf8"=> "1",
              "onlysong"=> "0",
              "picmid"=> "1",
              "nosign"=> "1",
              "song_begin"=> "0",
              "song_num"=> "1000", //歌曲数量
              "_"=> "1537276176570" //unix ts
        ],
        data:hash![],
    });

    // ...


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
    let s =create_sdk();
    println!("{:?}",s.keys());

    let q2=hash!["w"=>"从开始到现在","p"=>"1"];
//    let r=s.get("search").unwrap()(q2)?;
//    //let r=(&s["search"])(q2)?;
//    println!("{}",r);
    Ok("123".to_string())
}

fn test_download(){
    let u="https://img-blog.csdnimg.cn/20190918135101160.png?x=1&y=2";
    download(u);
}


fn test_vkey2song(){
    let s=r#"
{"code":0,"cid":205361747,"userip":"101.233.48.172","data":{"expiration":80400,"items":[{"subcode":104003,"songmid":"003lghpv0jfFXG","filename":"C400003lghpv0jfFXG.m4a","vkey":""},{"subcode":0,"songmid":"000B4ijs4Ufwql","filename":"C400000B4ijs4Ufwql.m4a","vkey":"A8331F2CF22B4BE50873EF46BC10ECBA76F842FD61D8BB5BB4956A863585A796D9151373EC2D0597B19985CB3B1ADA7D195B8DE605DA8397"}]}}
        "#;
    let r=vkey2song1(s.to_string());
    println!("{:?}",r);
    for i in &r{
        download(i);
    }
}



#[derive(Serialize, Deserialize,Clone)]
pub struct CorsHeaders {
    #[serde(rename = "Content-Type")]
    content_type: String,

    #[serde(rename = "access-control-allow-origin")]
    access_control_allow_origin: String,

    #[serde(rename = "access-control-allow-methods")]
    access_control_allow_methods: String,

    #[serde(rename = "access-control-allow-headers")]
    access_control_allow_headers: String,

    #[serde(rename = "access-control-max-age")]
    access_control_max_age: String,
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Res {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,

    #[serde(rename = "statusCode")]
    status_code: i64,

    #[serde(rename = "headers")]
    headers: CorsHeaders,

    #[serde(rename = "body")]
    body: String,
}


#[derive(Serialize, Deserialize,Clone)]
pub struct Body <T>{
    #[serde(rename = "errorCode")]
    error_code: i64,

    #[serde(rename = "errorMessage")]
    error_message: String,

    #[serde(rename = "ok")]
    ok: bool,

    #[serde(rename = "data")]
    data:T,
}

fn res_json<T: serde::Serialize>(code:i64,body:T)->String{
   let b=Body{
        error_code:code,
        error_message:"".to_owned(),
        ok:true,
        data:body,
   };
   let body1=serde_json::json!(b).to_string();
   let r=Res{
        is_base64_encoded: false,
        status_code: code,
        headers: create_h(mime::json),
        body: body1,
   };
   serde_json::json!(r).to_string()
}

fn res_json1<T: serde::Serialize>(code:i64,body:T)->String{
   let body1=serde_json::json!(body).to_string();
   let r=Res{
        is_base64_encoded: false,
        status_code: code,
        headers: create_h(mime::json),
        body: body1,
   };
   serde_json::json!(r).to_string()
}


fn res_base64<T: serde::Serialize>(code:i64,body:T)->String{
   let body1=serde_json::json!(body).to_string();
   let r=Res{
        is_base64_encoded: true,
        status_code: code,
        headers: create_h(mime::base64),
        body: base64encode(body1),
   };
   serde_json::json!(r).to_string()
}



#[derive(Serialize, Deserialize,Clone)]
pub struct Test {
    #[serde(rename = "x")]
    x: i64,
}
fn test_res_json(){
   let code=200_i64;
   let body1=Test{x:123};
   let r1=res_json::<Test>(code,body1.clone());
   let r2=res_json(code,body1.clone());
   let r3=res_json(code,"sss".to_owned());
   let r4=res_json1(code,body1.clone());
   println!("{}",r1);
   println!("{}",r2);
   println!("{}",r3);
   println!("{}",r4);
}


fn search_song(){
    let mut arg=Vec::new();
    for i in env::args() {
        arg.push(i);
    }
    let song=arg.last().unwrap();
   // println!("{}",song);
    let q2=hash!["w"=>song.to_owned(),"p"=>"1"];
    let r=get_song(q2);
    println!("{}",r);


}


fn main(){
   search_song();

   //let a=10_i32;
   //a.test();

   //test_base64();
   //test_res_json();
   //test_vkey2song();
   //test_download();
   //test_sdk();
   //test_get();
   //test_get1();
   //test_post_json();
   //test_post_form();
}

