#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize, Debug)]
pub enum ArticleStatus { Subcribe, Private, Draft, Public, Delete }

#[derive(CandidType, Deserialize, Debug)]
pub enum ArticleType { Photos, Article, Shortle, Audio, Video }

#[derive(CandidType, Deserialize, Debug)]
pub struct ArticleArgs {
  pub id: String,
  pub status: ArticleStatus,
  pub thumb: String,
  pub title: String,
  pub content: String,
  pub subcate: candid::Nat,
  pub atype: ArticleType,
  pub cate: candid::Nat,
  pub tags: Vec<String>,
  pub fromurl: String,
  pub r#abstract: String,
  pub allowComment: bool,
  pub original: bool,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum OpResult { Ok{ data: String }, Err(String) }