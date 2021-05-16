extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct N {
  pub r#type: String,
  pub value: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct O {
  pub r#type: String,
  #[serde(default)]
  pub datatype: String,
  #[serde(default, rename = "xml:lang")]
  pub xml_lang: String,
  pub value: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Bindings {
  pub c: O,
}
#[derive(Debug, Deserialize)]
pub struct Results {
  pub bindings: Vec<Bindings>,
}
#[derive(Debug, Deserialize)]
pub struct Head {
  pub vars: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct Response {
  pub head: Head,
  pub results: Results,
}
#[derive(Debug, Deserialize, Serialize)]
#[warn(non_camel_case_types)]
pub struct BindingsCallTable {
  pub callee: N,
  pub called: O,
}
#[derive(Debug, Deserialize)]
#[warn(non_camel_case_types)]
pub struct ResultsCallTable {
  pub bindings: Vec<BindingsCallTable>,
}
#[derive(Debug, Deserialize)]
#[warn(non_camel_case_types)]
pub struct ResponseCallTable {
  pub head: Head,
  pub results: ResultsCallTable,
}
#[derive(Serialize)]
pub struct MessageContent {
  pub title: String,
  pub num: usize,
  pub json: Vec<Bindings>,
  pub calltable: Vec<BindingsCallTable>,
}
