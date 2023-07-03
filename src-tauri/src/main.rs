// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Cursor;
use polars::{prelude::*, lazy::dsl::Expr};
// use polars::export::regex::*;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .invoke_handler(tauri::generate_handler![parse_json])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn parse_json(json: &str) -> () {
  let cursor = Cursor::new(json);
  let df = JsonReader::new(cursor)
    .infer_schema_len(Some(3))
    .finish()
    .unwrap();

  let expr: Expr = col("name").str().starts_with("LVLQ 90".into());

  let out: DataFrame = df.clone()
            .lazy()
            .filter(expr)
            .select(&[col("name")])
            .collect()
            .unwrap();


  // let out = df.clone().filter(re).select(!vec[col("name"), col("productcode")]).unwrap();

  println!("{:?}", out)
}