use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Row {
    #[serde(rename = "No")]
    pub no: String,
    #[serde(rename = "公表_年月日")]
    pub kohyo_date: String,
    #[serde(rename = "発症_年月日")]
    pub hasyo_date: String,
    #[serde(rename = "確定_年月日")]
    pub kakutei_date: String,
    #[serde(rename = "全国地方公共団体コード")]
    pub code: String,
    #[serde(rename = "都道府県名")]
    pub prefecture: String,
    #[serde(rename = "市区町村名")]
    pub city: String,
    #[serde(rename = "患者_年代")]
    pub age: String,
    #[serde(rename = "患者_性別")]
    pub gender: String,
    #[serde(rename = "患者_状態")]
    pub status: String,
    #[serde(rename = "患者_症状")]
    pub symptoms: String,
    #[serde(rename = "患者_職業")]
    pub occupation: String,
    #[serde(rename = "患者_居住地")]
    pub residence: String,
    #[serde(rename = "患者_接触歴の有無フラグ")]
    pub contact_history_flag: String,
    #[serde(rename = "患者_渡航歴の有無フラグ")]
    pub travel_history_flag: String,
    #[serde(rename = "退院済フラグ")]
    pub discharge_flag: String,
    #[serde(rename = "備考")]
    pub note: String,
}
