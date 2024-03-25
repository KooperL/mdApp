// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![feature(generators, generator_trait)] 
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::string::String;
use std::fmt::Write;
use xmlparser;
use TransformationUtils::applyTransformation;
#[path = "./types.rs"] mod Types;
#[path = "./utils/tokenizerUtils.rs"] mod TokenizerUtils;
#[path = "./utils/transformations.rs"] mod TransformationUtils;
#[path = "./utils/stringBuilder.rs"] mod StringBuilderUtils;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn process_styling(html: &str, begin: usize, end: usize, transformation: &str) -> String {
    // println!("Args: {:?}, {:?}, {:?}", html, begin, end);

    // *****************  Begin parse to vec<struct>  ******************* //
    let mut struct_collection: Vec<Types::StyledCharacter> = Vec::new(); 

    // Contextual parsing flags
    let mut basic_format = Types::BasicFormat::new();
    let mut token_context = Types::TokenContext::new();

    let tokenizer = xmlparser::Tokenizer::from(html);
    for token in tokenizer {
        let token_val = token.expect("Token to be valid");
        TokenizerUtils::interpretToken(token_val, &mut struct_collection, &mut basic_format, &mut token_context, begin, end);
    }


    // println!("{:?}", struct_collection);
    // *****************  Begin transformations  *******************//
    if end >= struct_collection.len() {
        // panic!();
    }

    // moving this out to a new files
    for i in begin..end {
        applyTransformation(&mut struct_collection, &mut token_context, i, transformation, begin, end);
    }

    // println!("{:?}", struct_collection);
    // *****************  Begin final string build  ******************* //
    let builtString = StringBuilderUtils::stringBuilder(&mut basic_format, &mut token_context, &mut struct_collection);
    return  builtString;

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_styling])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
