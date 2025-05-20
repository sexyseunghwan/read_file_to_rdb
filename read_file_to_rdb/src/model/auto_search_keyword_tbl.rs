use crate::common::*;

#[derive(Debug, FromQueryResult, Serialize, new)]
pub struct AutoSearchKeywordTbl {
    pub keyword: String,
    pub keyword_weight: i32,
    pub reg_dt: NaiveDateTime,
    pub chg_dt: Option<NaiveDateTime>,
    pub reg_id: String,
    pub chg_id: Option<String>,
}