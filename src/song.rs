// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct TopLevel {
    #[serde(rename = "code")]
    code: i64,

    #[serde(rename = "data")]
    data: Data,

    #[serde(rename = "message")]
    message: String,

    #[serde(rename = "notice")]
    notice: String,

    #[serde(rename = "subcode")]
    subcode: i64,

    #[serde(rename = "time")]
    time: i64,

    #[serde(rename = "tips")]
    tips: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "keyword")]
    keyword: String,

    #[serde(rename = "priority")]
    priority: i64,

    #[serde(rename = "qc")]
    qc: Vec<Option<serde_json::Value>>,

    #[serde(rename = "semantic")]
    semantic: Semantic,

    #[serde(rename = "song")]
    song: Semantic,

    #[serde(rename = "tab")]
    tab: i64,

    #[serde(rename = "taglist")]
    taglist: Vec<Option<serde_json::Value>>,

    #[serde(rename = "totaltime")]
    totaltime: i64,

    #[serde(rename = "zhida")]
    zhida: Zhida,
}

#[derive(Serialize, Deserialize)]
pub struct Semantic {
    #[serde(rename = "curnum")]
    curnum: i64,

    #[serde(rename = "curpage")]
    curpage: i64,

    #[serde(rename = "list")]
    list: Vec<List>,

    #[serde(rename = "totalnum")]
    totalnum: i64,
}

#[derive(Serialize, Deserialize)]
pub struct List {
    #[serde(rename = "albumid")]
    albumid: i64,

    #[serde(rename = "albummid")]
    albummid: String,

    #[serde(rename = "albumname")]
    albumname: String,

    #[serde(rename = "albumname_hilight")]
    albumname_hilight: String,

    #[serde(rename = "alertid")]
    alertid: i64,

    #[serde(rename = "belongCD")]
    belong_cd: Option<i64>,

    #[serde(rename = "cdIdx")]
    cd_idx: Option<i64>,

    #[serde(rename = "chinesesinger")]
    chinesesinger: i64,

    #[serde(rename = "docid")]
    docid: String,

    #[serde(rename = "grp")]
    grp: Option<Vec<List>>,

    #[serde(rename = "interval")]
    interval: i64,

    #[serde(rename = "isonly")]
    isonly: i64,

    #[serde(rename = "lyric")]
    lyric: Lyric,

    #[serde(rename = "lyric_hilight")]
    lyric_hilight: Lyric,

    #[serde(rename = "media_mid")]
    media_mid: Option<String>,

    #[serde(rename = "msgid")]
    msgid: i64,

    #[serde(rename = "newStatus")]
    new_status: i64,

    #[serde(rename = "nt")]
    nt: i64,

    #[serde(rename = "pay")]
    pay: Pay,

    #[serde(rename = "preview")]
    preview: Preview,

    #[serde(rename = "pubtime")]
    pubtime: i64,

    #[serde(rename = "pure")]
    list_pure: i64,

    #[serde(rename = "singer")]
    singer: Vec<Singer>,

    #[serde(rename = "size128")]
    size128: i64,

    #[serde(rename = "size320")]
    size320: i64,

    #[serde(rename = "sizeape")]
    sizeape: i64,

    #[serde(rename = "sizeflac")]
    sizeflac: i64,

    #[serde(rename = "sizeogg")]
    sizeogg: i64,

    #[serde(rename = "songid")]
    songid: i64,

    #[serde(rename = "songmid")]
    songmid: String,

    #[serde(rename = "songname")]
    songname: String,

    #[serde(rename = "songname_hilight")]
    songname_hilight: String,

    #[serde(rename = "strMediaMid")]
    str_media_mid: Option<String>,

    #[serde(rename = "stream")]
    stream: i64,

    #[serde(rename = "switch")]
    switch: i64,

    #[serde(rename = "t")]
    t: i64,

    #[serde(rename = "tag")]
    tag: i64,

    #[serde(rename = "type")]
    list_type: i64,

    #[serde(rename = "ver")]
    ver: i64,

    #[serde(rename = "vid")]
    vid: String,

    #[serde(rename = "format")]
    format: Option<Format>,

    #[serde(rename = "songurl")]
    songurl: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Pay {
    #[serde(rename = "payalbum")]
    payalbum: i64,

    #[serde(rename = "payalbumprice")]
    payalbumprice: i64,

    #[serde(rename = "paydownload")]
    paydownload: i64,

    #[serde(rename = "payinfo")]
    payinfo: i64,

    #[serde(rename = "payplay")]
    payplay: i64,

    #[serde(rename = "paytrackmouth")]
    paytrackmouth: i64,

    #[serde(rename = "paytrackprice")]
    paytrackprice: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Preview {
    #[serde(rename = "trybegin")]
    trybegin: i64,

    #[serde(rename = "tryend")]
    tryend: i64,

    #[serde(rename = "trysize")]
    trysize: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Singer {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "mid")]
    mid: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "name_hilight")]
    name_hilight: String,
}

#[derive(Serialize, Deserialize)]
pub struct Zhida {
    #[serde(rename = "chinesesinger")]
    chinesesinger: i64,

    #[serde(rename = "type")]
    zhida_type: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "mp3;common;crcommon;mp3common")]
    Mp3CommonCrcommonMp3Common,

    #[serde(rename = "qqhq;common;mp3common;wmacommon")]
    QqhqCommonMp3CommonWmacommon,

    #[serde(rename = "qqhq;common;mp3common;wmacommon;crcommon")]
    QqhqCommonMp3CommonWmacommonCrcommon,
}

#[derive(Serialize, Deserialize)]
pub enum Lyric {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "翻唱")]
    Fluffy,

    #[serde(rename = "越简单的爱越难得，越难得的爱越要珍惜。")]
    Lyric,

    #[serde(rename = "谢谢大家来听橘子的翻唱~")]
    Purple,

    #[serde(rename = "小头菜")]
    Tentacled,
}
