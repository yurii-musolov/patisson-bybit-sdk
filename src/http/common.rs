use serde::Deserialize;

use crate::{Timestamp, enums::Category, serde::empty_string_as_none};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp<T> {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: T,
    pub time: Option<Timestamp>,
    pub ret_ext_info: Option<RetExtInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct APIErrorResponse {
    pub ret_code: i64,
    pub ret_msg: String,
}

#[derive(Debug, PartialEq)]
pub struct Response<T> {
    pub result: T,
    pub time: Option<Timestamp>,
    pub headers: Headers,
    /// Per-item results for batch endpoints; `None` for all non-batch calls.
    pub ret_ext_info: Option<RetExtInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CursorPagination<T> {
    pub category: Option<Category>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
    pub list: Vec<T>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    pub list: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub struct Headers {
    pub ret_code: Option<i32>,
    pub trace_id: Option<String>,
    pub time_now: Option<Timestamp>,
    pub api_limit: Option<u64>,
    pub api_limit_status: Option<u64>,
    pub api_limit_reset_timestamp: Option<Timestamp>,
}

impl Headers {
    pub fn is_ret_code_ok(&self) -> bool {
        match self.ret_code {
            Some(code) => code == 0,
            None => false,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct BatchItemResult {
    pub code: i64,
    pub msg: String,
}

/// Per-item status for batch trade endpoints.
/// For all other endpoints `retExtInfo` is `{}` — the `list` defaults to empty.
#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct RetExtInfo {
    #[serde(default)]
    pub list: Vec<BatchItemResult>,
}

/// Returned by position-management and other void-result endpoints whose
/// `result` field is the empty object `{}`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct EmptyResult {}
