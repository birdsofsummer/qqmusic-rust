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

    #[serde(rename = "color")]
    color: i64,

    #[serde(rename = "comment_num")]
    comment_num: i64,

    #[serde(rename = "cur_song_num")]
    cur_song_num: i64,

    #[serde(rename = "date")]
    date: String,

    #[serde(rename = "day_of_year")]
    day_of_year: String,

    #[serde(rename = "song_begin")]
    song_begin: i64,

    #[serde(rename = "songlist")]
    songlist: Vec<Songlist>,

    #[serde(rename = "topinfo")]
    topinfo: Topinfo,

    #[serde(rename = "total_song_num")]
    total_song_num: i64,

    #[serde(rename = "update_time")]
    update_time: String,
}

#[derive(Serialize, Deserialize)]
pub struct Songlist {
    #[serde(rename = "Franking_value")]
    franking_value: String,

    #[serde(rename = "cur_count")]
    cur_count: String,

    #[serde(rename = "data")]
    data: Data,

    #[serde(rename = "in_count")]
    in_count: String,

    #[serde(rename = "old_count")]
    old_count: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "albumdesc")]
    albumdesc: String,

    #[serde(rename = "albumid")]
    albumid: i64,

    #[serde(rename = "albummid")]
    albummid: String,

    #[serde(rename = "albumname")]
    albumname: String,

    #[serde(rename = "alertid")]
    alertid: i64,

    #[serde(rename = "belongCD")]
    belong_cd: i64,

    #[serde(rename = "cdIdx")]
    cd_idx: i64,

    #[serde(rename = "interval")]
    interval: i64,

    #[serde(rename = "isonly")]
    isonly: i64,

    #[serde(rename = "label")]
    label: String,

    #[serde(rename = "msgid")]
    msgid: i64,

    #[serde(rename = "pay")]
    pay: Pay,

    #[serde(rename = "preview")]
    preview: Preview,

    #[serde(rename = "rate")]
    rate: i64,

    #[serde(rename = "singer")]
    singer: Vec<Singer>,

    #[serde(rename = "size128")]
    size128: i64,

    #[serde(rename = "size320")]
    size320: i64,

    #[serde(rename = "size5_1")]
    size5_1: i64,

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

    #[serde(rename = "songorig")]
    songorig: String,

    #[serde(rename = "songtype")]
    songtype: i64,

    #[serde(rename = "strMediaMid")]
    str_media_mid: String,

    #[serde(rename = "stream")]
    stream: i64,

    #[serde(rename = "switch")]
    switch: i64,

    #[serde(rename = "type")]
    data_type: i64,

    #[serde(rename = "vid")]
    vid: String,
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

    #[serde(rename = "timefree")]
    timefree: i64,
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
}

#[derive(Serialize, Deserialize)]
pub struct Topinfo {
    #[serde(rename = "ListName")]
    list_name: String,

    #[serde(rename = "MacDetailPicUrl")]
    mac_detail_pic_url: String,

    #[serde(rename = "MacListPicUrl")]
    mac_list_pic_url: String,

    #[serde(rename = "UpdateType")]
    update_type: String,

    #[serde(rename = "albuminfo")]
    albuminfo: String,

    #[serde(rename = "headPic_v12")]
    head_pic_v12: String,

    #[serde(rename = "info")]
    info: String,

    #[serde(rename = "listennum")]
    listennum: i64,

    #[serde(rename = "pic")]
    pic: String,

    #[serde(rename = "picDetail")]
    pic_detail: String,

    #[serde(rename = "pic_album")]
    pic_album: String,

    #[serde(rename = "pic_h5")]
    pic_h5: String,

    #[serde(rename = "pic_v11")]
    pic_v11: String,

    #[serde(rename = "pic_v12")]
    pic_v12: String,

    #[serde(rename = "topID")]
    top_id: String,

    #[serde(rename = "type")]
    topinfo_type: String,
}
