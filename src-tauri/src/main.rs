// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![feature(generators, generator_trait)] 
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::string::String;
use std::fmt::Write;
use xmlparser;


struct StyledCharacter {
    is_bold: bool,
    is_emphasized: bool,
    is_heading: bool,
    character: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn process_styling(html: &str, begin: usize, end: usize, transformation: &str) -> String {
    let mut struct_collection: Vec<StyledCharacter> = Vec::new(); 

    let mut is_bold = false;
    let mut is_emphasized = false;
    let mut is_heading = false;

    let tokenizer = xmlparser::Tokenizer::from(html);
    for token in tokenizer {
        let token_val = token.expect("Token to be valid");
            match token_val {
                xmlparser::Token::Text {text: val} => {
                    struct_collection.push(StyledCharacter {
                        is_bold,
                        is_emphasized,
                        is_heading,
                        character: String::from(val.as_str())
                    });
                },
                xmlparser::Token::ElementStart {prefix: _, local, span} => {
                println!("token: {}", local.as_str());
                match local.as_str() {
                    "b" => {
                        is_bold = true;
                    },
                    "i" => {
                        is_bold = true;
                        is_emphasized = true;
                    },
                    "h1" => {
                        is_bold = false;
                        is_emphasized = false;
                        is_heading = true;
                    },
                    _ => {
                        // others tags here
                    }
                }
            },
                _ => {},
            };
        
        println!("{:?}", token);
    }

    // apply transformations with start/end
    for i in begin..end {
        match transformation {
            "b" => {
                struct_collection[i].is_bold = true;
            },
            "i" => {
                struct_collection[i].is_emphasized = true;
            },
            "h1" => {
                struct_collection[i].is_bold = false;
                struct_collection[i].is_emphasized = false;
                struct_collection[i].is_heading = true;
            },
            _ => {},
        }
    }

    is_bold = false;
    is_emphasized = false;
    is_heading = false;
    
    let mut builder = String::new();
    for char in struct_collection {
        if !is_bold && char.is_bold {
            write!(&mut builder, "<b>").unwrap();
            is_bold = true;
        }
        if is_bold && !char.is_bold {
            write!(&mut builder, "</b>").unwrap();
            is_bold = false;
        }
        if !is_emphasized && char.is_emphasized {
            write!(&mut builder, "<em>").unwrap();
            is_emphasized = true;
        }
        if is_emphasized && !char.is_emphasized {
            write!(&mut builder, "</em>").unwrap();
            is_emphasized = false;
        }
        if !is_heading && char.is_heading {
            write!(&mut builder, "<h1>").unwrap();
            is_heading = true;
        }
        if is_heading && !char.is_heading {
            write!(&mut builder, "</h1>").unwrap();
            is_heading = false;
        }
        write!(&mut builder, "{}", char.character).unwrap();
    }


    builder
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_styling])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}