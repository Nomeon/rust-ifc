// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use polars::prelude::*;
use std::io::Cursor;

mod bb;
use crate::bb::bb_csv;

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
fn parse_json(json: &str, path: &str) -> String {
    let cursor = Cursor::new(json);
    let df = JsonReader::new(cursor)
        .infer_schema_len(Some(3))
        .finish()
        .unwrap();
    

    println!("{:?}", df);

    bb_csv(
        df,
        path,
    );

    format!("Success!")
}