// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![feature(generators, generator_trait)] 
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::borrow::Borrow;
use std::io::{self, stdin};
use std::iter::repeat;
use std::string::String;

use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, ParseOpts};
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use xi_core_lib::styles::Style;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Writer;
use std::io::Cursor;

struct StyledCharacter {
    is_bold: bool,
    is_emphasized: bool,
    is_heading: bool,
    character: String,
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn process_arg(name: &str) -> String {
//     let opts = ParseOpts {
//         tree_builder: TreeBuilderOpts {
//             drop_doctype: true,
//             ..Default::default()
//         },
//         ..Default::default()
//     };
//     let dom = parse_document(RcDom::default(), Default::default())
//         .from_utf8()
//         .read_from(&mut name.as_bytes())
//         .unwrap();

        let characters = vec![
        StyledCharacter {
            is_bold: true,
            is_emphasized: false,
            is_heading: false,
            character: String::from("Hello"),
        },
        StyledCharacter {
            is_bold: false,
            is_emphasized: true,
            is_heading: false,
            character: String::from("world!"),
        },
    ];

    format!("Hello, {}! You've been greeted from Rust!", name)
}

// struct StyledCharacter {
//     isStrong: bool,
//     isEmphasised: bool,
//     character: String
// }
// 
// struct TreeNode {
//     tag: String,
//     children: Vec<TreeNode>,
//     text: Option<String>
// }
// 
// fn xml_parse(xml_str: String) {
//     let mut inside_tag = false;
//     let mut current_token = "";
//     let mut current_tag = "";
//     let mut inside_attribute  = false;
//     let mut current_attributes = {};
//     let mut attribute_name = "";
// 
//     let mut generator = || {
//         for char in xml_str.chars() {
//             if char == '<' {
//                 if !inside_tag {
//                     inside_tag = true;
//                     current_tag = "";
//                 } else {
//                     // Panic, invalid xml format
//                     // Shouldn't be inside another tag
//                 }
//             } else if char == '>' {
//                 if inside_tag {
//                     if current_token.starts_with("/") {
//                         if current_token[1..] != current_tag {
//                             // Panic, invalid xml format
//                             //
//                         }
//                         yield; // end tag
//                     } else if current_token.ends_with("/") {
//                         current_token = current_token[..1];
//                         yield; // start tag, tag struct
//                         yield; // end tag
//                     } else {
//                         yield; // start tag, tag struct
//                     }
//                     current_token = "";
//                     inside_tag = false;
//                 } else {
//                     if current_token != "" {
//                         yield; // text, current_token
//                     }
//                     current_token = "";
//                 }
//             } else if char == '=' {
//                 if inside_tag && !inside_attribute {
//                     inside_attribute = true;
//                     attribute_name = current_token.trim();
//                     current_token = "";
//                 }
//             } else if char == '"' {
//                 if inside_tag && inside_attribute {
//                     current_attributes[attribute_name] = current_token.trim();
//                     current_token = "";
//                     inside_attribute = false;
//                 }
//             } else if char == ' ' {
//                 if inside_tag {
//                     if current_token != "" {
//                         if !inside_attribute {
//                             current_tag = current_token.trim();
//                         } else {
//                             // panic, invalid XML format
//                             // Unexpected whitespace inside attribute value
//                         }
//                     }
//                 }
//             } else {
//                 current_token = current_token + char;
//             }
//         }
//         yield; // text, current_token
//     };
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
