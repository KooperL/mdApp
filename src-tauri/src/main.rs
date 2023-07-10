// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![feature(generators, generator_trait)] 
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::string::String;
use std::fmt::Write;
use xmlparser;

#[derive(Debug)]
enum ListTypes {
    Ordered,
    Unordered,
}

#[derive(Debug)]
struct StyledCharacter {
    is_bold: bool,
    is_emphasised: bool,
    is_superscript: bool,
    is_subscript: bool,
    is_underline: bool,
    is_strikethrough: bool,
    is_code: bool,
    is_heading: bool,
    is_check: bool,
    character: String,
    list_type: Option<ListTypes>,
    list_item: Option<i32>
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn process_styling(html: &str, begin: usize, end: usize, transformation: &str) -> String {
    // println!("Args: {:?}, {:?}, {:?}", html, begin, end);

    // *****************  Begin parse to vec<struct>  ******************* //
    let mut struct_collection: Vec<StyledCharacter> = Vec::new(); 

    // Contextual parsing flags
    let mut is_bold = false;
    let mut is_emphasised = false;
    let mut is_heading = false;
    let mut is_code = false;
    let mut is_underline = false;
    let mut is_strikethrough = false;
    let mut is_superscript = false;
    let mut is_subscript = false;
    let mut is_check = false;
    let mut list_type: Option<ListTypes> = None;
    let mut list_item: Option<i32> = None;

    let mut rolling_char_counter = 0;
    let mut whole_area_is_bold_formatted = true;
    let mut whole_area_is_emphasised_formatted = true;
    let mut whole_area_is_underline_formatted = true;
    let mut whole_area_is_strikethrough_formatted = true;
    let mut whole_area_is_code_formatted = true;
    let mut whole_area_is_heading_formatted = true;
    let mut whole_area_is_superscript_formatted = true;
    let mut whole_area_is_subscript_formatted = true;
    let mut whole_area_is_check_formatted = true;

    let tokenizer = xmlparser::Tokenizer::from(html);
    for token in tokenizer {
        let token_val = token.expect("Token to be valid");
        match token_val {

            // Matching text
            xmlparser::Token::Text {text: val} => {
                for (index, character) in val.chars().enumerate() {
                    rolling_char_counter += 1;
                    if rolling_char_counter >= begin && rolling_char_counter <= end {
                        if !is_bold && character != ' ' && character != '\u{200B}' {
                            whole_area_is_bold_formatted = false;
                        };
                        if !is_emphasised && character != ' ' && character != '\u{200B}' {
                            whole_area_is_emphasised_formatted = false;
                        };
                        if !is_code && character != ' ' && character != '\u{200B}' {
                            whole_area_is_code_formatted = false;
                        };
                        if !is_superscript && character != ' ' && character != '\u{200B}' {
                            whole_area_is_superscript_formatted = false;
                        };
                        if !is_subscript && character != ' ' && character != '\u{200B}' {
                            whole_area_is_subscript_formatted = false;
                        };
                        if !is_heading && character != ' ' && character != '\u{200B}' {
                            whole_area_is_heading_formatted = false;
                        };
                        if !is_strikethrough && character != ' ' && character != '\u{200B}' {
                            whole_area_is_strikethrough_formatted = false;
                        };
                        if !is_check && character != ' ' && character != '\u{200B}' {
                            whole_area_is_check_formatted = false;
                        };
                        if !is_underline && character != ' ' && character != '\u{200B}' {
                            whole_area_is_underline_formatted = false;
                        };
                    };
                    struct_collection.push(StyledCharacter {
                        is_bold,
                        is_emphasised,
                        is_underline,
                        is_strikethrough,
                        is_heading,
                        is_code,
                        is_check,
                        is_subscript,
                        is_superscript,
                        character: String::from(character),
                        list_type: if list_type.is_none() {
                            None
                        } else {
                            // if let Some(val) = list_type {
                            //     Some(val)
                            // } else {
                            //     None
                            // }
                            match list_type {
                                Some(ListTypes::Ordered) => Some(ListTypes::Ordered),
                                Some(ListTypes::Unordered) => Some(ListTypes::Unordered),
                                _ => None
                            }
                        },
                        list_item
                    });
                }
            },

            // Matching opening token
            xmlparser::Token::ElementStart {prefix: _, local, span} => {
                match local.as_str() {
                    "b" => {
                        is_bold = true;
                        is_code = false;
                        is_heading = false;
                    },
                    "i" => {
                        is_emphasised = true;
                        is_code = false;
                        is_heading = false;
                    },
                    "s" => {
                        is_strikethrough = true;
                        is_code = false;
                        is_heading = false;
                    },
                    "input" => {
                        // type="check"??
                        is_strikethrough = false;
                        is_code = false;
                        is_heading = false;
                        is_check = true;
                        list_type = None;
                        list_item = None;
                    },
                    "u" => {
                        is_underline = true;
                        is_code = false;
                        is_heading = false;
                    },
                    "code" => {
                        is_emphasised = false;
                        is_bold = false;
                        is_code = true;
                        is_heading = false;
                    },
                    "sup" => {
                        is_emphasised = false;
                        is_bold = false;
                        is_code = false;
                        is_heading = false;
                        is_superscript = true;
                        is_subscript = false;
                    },
                    "sub" => {
                        is_emphasised = false;
                        is_bold = false;
                        is_code = false;
                        is_heading = false;
                        is_superscript = false;
                        is_subscript = true;
                    },
                    "h1" => {
                        is_bold = false;
                        is_emphasised = false;
                        is_heading = true;
                        is_code = false
                    },
                    "ol" => {
                        list_type = Some(ListTypes::Ordered);
                        list_item = Some(0);
                    },
                    "ul" => {
                        list_type = Some(ListTypes::Unordered);
                        list_item = Some(0);
                    },
                    _ => {
                        // Others tags here
                    }
                }
            },

            // Matchin closing token
            xmlparser::Token::ElementEnd {span, end} => {
                // println!("{}", span.as_str());
                match span.as_str() {
                    "</b>" => {
                        is_bold = false;
                    },
                    "</i>" => {
                        is_emphasised = false;
                    },
                    "</s>" => {
                        is_strikethrough = false;
                    },
                    "</u>" => {
                        is_underline = false;
                    },
                    "</input>" => {
                        is_check = false;
                    },
                    "</code>" => {
                        is_code = false;
                    },
                    "</sup>" => {
                        is_superscript = false;
                    },
                    "</sub>" => {
                        is_subscript = false;
                    },
                    "</h1>" => {
                        is_heading = false;
                    },
                    "</ol>" => {
                        list_type = None;
                        list_item = None;
                    },
                    "</ul>" => {
                        list_type = None;
                        list_item = None;
                    },
                    "</li>" => {
                        if let Some(mut value) = list_item {
                            value += 1;
                            list_item = Some(value);
                        }
                    },
                    _ => {
                        // Others tags here
                    }
                }
            },
            _ => {},
        };
    }


    // println!("{:?}", struct_collection);
    // *****************  Begin transformations  *******************//
    if end >= struct_collection.len() {
        // panic!();
    }
    for i in begin..end {
        match transformation {
            "bold" => {
                struct_collection[i].is_code = false;
                struct_collection[i].is_subscript = false;
                struct_collection[i].is_superscript = false;
                struct_collection[i].is_heading = false;
                if i >= begin && i <= end {
                    struct_collection[i].is_bold = !whole_area_is_bold_formatted;
                } else {
                    struct_collection[i].is_bold = true;
                }
            },
            "emphasise" => {
                struct_collection[i].is_code = false;
                struct_collection[i].is_subscript = false;
                struct_collection[i].is_superscript = false;
                struct_collection[i].is_heading = false;
                if i >= begin && i <= end {
                    struct_collection[i].is_emphasised = !whole_area_is_emphasised_formatted;
                } else {
                    struct_collection[i].is_emphasised = true;
                }
            },
            "check" => {
                struct_collection[i].is_heading = false;
                struct_collection[i].list_type = None;
                struct_collection[i].list_item = None;
                if i >= begin && i <= end {
                    struct_collection[i].is_check = !whole_area_is_check_formatted;
                } else {
                    struct_collection[i].is_check = true;
                }
            },
            "underline" => {
                struct_collection[i].is_code = false;
                struct_collection[i].is_subscript = false;
                struct_collection[i].is_superscript = false;
                struct_collection[i].is_heading = false;
                if i >= begin && i <= end {
                    struct_collection[i].is_underline = !whole_area_is_underline_formatted;
                } else {
                    struct_collection[i].is_underline = true;
                }
            },
            "strikethrough" => {
                struct_collection[i].is_code = false;
                struct_collection[i].is_subscript = false;
                struct_collection[i].is_superscript = false;
                struct_collection[i].is_heading = false;
                if i >= begin && i <= end {
                    struct_collection[i].is_strikethrough = !whole_area_is_strikethrough_formatted;
                } else {
                    struct_collection[i].is_strikethrough = true;
                }
            },
            "code" => {
                struct_collection[i].is_bold = false;
                struct_collection[i].is_emphasised = false;
                struct_collection[i].is_subscript = false;
                struct_collection[i].is_strikethrough = false;
                struct_collection[i].is_underline = false;
                struct_collection[i].is_superscript = false;
                struct_collection[i].is_heading = false;
                if i >= begin && i <= end {
                    struct_collection[i].is_code = !whole_area_is_code_formatted;
                } else {
                    struct_collection[i].is_code = true;
                }
            },
            "superscript" => {
                struct_collection[i].is_bold = false;
                struct_collection[i].is_emphasised = false;
                struct_collection[i].is_code = false;
                struct_collection[i].is_strikethrough = false;
                struct_collection[i].is_underline = false;
                struct_collection[i].is_subscript = false;
                struct_collection[i].is_heading = false;
                if i >= begin && i <= end {
                    struct_collection[i].is_superscript = !whole_area_is_superscript_formatted;
                } else {
                    struct_collection[i].is_superscript = true;
                }
            },
            "subscript" => {
                struct_collection[i].is_bold = false;
                struct_collection[i].is_emphasised = false;
                struct_collection[i].is_strikethrough = false;
                struct_collection[i].is_underline = false;
                struct_collection[i].is_code = false;
                struct_collection[i].is_superscript = false;
                struct_collection[i].is_heading = false;
                if i >= begin && i <= end {
                    struct_collection[i].is_subscript = !whole_area_is_subscript_formatted;
                } else {
                    struct_collection[i].is_subscript = true;
                }
            },
            "heading" => {
                // TODO: Apply this logic to the other conditions
                struct_collection[i].is_bold = false;
                struct_collection[i].is_emphasised = false;
                struct_collection[i].is_strikethrough = false;
                struct_collection[i].is_underline = false;
                struct_collection[i].is_code = false;
                struct_collection[i].is_heading = true;
                struct_collection[i].is_check = false;
                if i >= begin && i <= end {
                    struct_collection[i].is_heading = !whole_area_is_heading_formatted;
                } else {
                    struct_collection[i].is_heading = true;
                }
            },
            "ordered-list" => {
                struct_collection[i].is_heading = false;
                struct_collection[i].is_check = false;
                struct_collection[i].list_item = if struct_collection[i].list_item == None {
                    Some(0)
                } else {
                    None
                };
                struct_collection[i].list_type = Some(ListTypes::Ordered);
            },
            "unordered-list" => {
                struct_collection[i].is_heading = false;
                struct_collection[i].is_check = false;
                struct_collection[i].list_item = if struct_collection[i].list_item == None {
                    Some(0)
                } else {
                    None
                };
                struct_collection[i].list_type = Some(ListTypes::Unordered);
            },
            _ => {},
        }
    }

    // println!("{:?}", struct_collection);
    // *****************  Begin final string build  ******************* //
    is_bold = false;
    is_emphasised = false;
    is_strikethrough = false;
    is_underline = false;
    is_code = false;
    is_superscript = false;
    is_subscript = false;
    is_heading = false;
    is_check = false;
    list_type = None;
    list_item = None;

    let property = "contenteditable=\"true\" onKeyDown=\"keyPresshandler\"";
    let mut builder = String::new();
    write!(&mut builder, "<p {}>", property).unwrap();
    if struct_collection.len() == 0 {
        write!(&mut builder, "\u{200B}").unwrap();
    }
    for char in struct_collection {
        if !is_bold && char.is_bold {
            write!(&mut builder, "<b>").unwrap();
            is_bold = true;
        }
        if is_bold && !char.is_bold {
            write!(&mut builder, "</b>").unwrap();
            is_bold = false;
        }
        if !is_emphasised && char.is_emphasised {
            write!(&mut builder, "<i>").unwrap();
            is_emphasised = true;
        }
        if is_emphasised && !char.is_emphasised {
            write!(&mut builder, "</i>").unwrap();
            is_emphasised = false;
        }
        if !is_underline && char.is_underline {
            write!(&mut builder, "<u>").unwrap();
            is_underline = true;
        }
        if is_underline && !char.is_underline {
            write!(&mut builder, "</u>").unwrap();
            is_underline = false;
        }
        if !is_check && char.is_check {
            write!(&mut builder, "<input type=\"checkbox\">").unwrap();
            is_check = true;
        }
        if is_check && !char.is_check {
            write!(&mut builder, "</input>").unwrap();
            is_check = false;
        }
        if !is_strikethrough && char.is_strikethrough {
            write!(&mut builder, "<s>").unwrap();
            is_strikethrough = true;
        }
        if is_strikethrough && !char.is_strikethrough {
            write!(&mut builder, "</s>").unwrap();
            is_strikethrough = false;
        }
        if !is_code && char.is_code {
            write!(&mut builder, "<code>").unwrap();
            is_code = true;
        }
        if is_code && !char.is_code {
            write!(&mut builder, "</code>").unwrap();
            is_code = false;
        }
        if !is_superscript && char.is_superscript {
            write!(&mut builder, "<sup>").unwrap();
            is_superscript = true;
        }
        if is_superscript && !char.is_superscript {
            write!(&mut builder, "</sup>").unwrap();
            is_superscript = false;
        }
        if !is_subscript && char.is_subscript {
            write!(&mut builder, "<sub>").unwrap();
            is_subscript = true;
        }
        if is_subscript && !char.is_subscript {
            write!(&mut builder, "</sub>").unwrap();
            is_subscript = false;
        }
        if !is_heading && char.is_heading {
            write!(&mut builder, "<h1>").unwrap();
            is_heading = true;
        }
        if is_heading && !char.is_heading {
            write!(&mut builder, "</h1>").unwrap();
            is_heading = false;
        }

        match list_type {
            None => {
                match char.list_type {
                    Some(ListTypes::Ordered) => {
                        if char.list_item == Some(0) {
                            write!(&mut builder, "</p><ol {}><li>", property).unwrap();
                            list_type = Some(ListTypes::Ordered);
                            list_item = char.list_item;
                        }
                    },
                    Some(ListTypes::Unordered) => {
                        if char.list_item == Some(0) {
                            write!(&mut builder, "</p><ul {}><li>", property).unwrap();
                            list_type = Some(ListTypes::Unordered);
                            list_item = char.list_item;
                        }
                    },
                    None => {}
                }
            },
            Some(ListTypes::Ordered) => {
                match char.list_item {
                    None => {
                        write!(&mut builder, "</li></ol><p {}>", property).unwrap();
                        list_type = None;
                        list_item = None;
                    },
                    Some(val) => {
                        if list_item == Some(val + 1) {
                            write!(&mut builder, "</li><li>").unwrap();
                            if let Some(mut value) = list_item {
                                value += 1;
                                list_item = Some(value);
                            }
                        }
                    }
                }
            }
            Some(ListTypes::Unordered) => {
                match char.list_item {
                    None => {
                        write!(&mut builder, "</li></ul><p {}>", property).unwrap();
                        list_type = None;
                        list_item = None;
                    },
                    Some(val) => {
                        if list_item == Some(val + 1) {
                            write!(&mut builder, "</li><li>").unwrap();
                            if let Some(mut value) = list_item {
                                value += 1;
                                list_item = Some(value);
                            }
                        }
                    }
                }
            }
        }


        // Write the character
        write!(&mut builder, "{}", char.character).unwrap();
    }

    // Close remaining open tags
    if is_bold {
        write!(&mut builder, "</b>").unwrap();
    }
    if is_check {
        write!(&mut builder, "</input>").unwrap();
    }
    if is_emphasised {
        write!(&mut builder, "</i>").unwrap();
    }
    if is_strikethrough {
        write!(&mut builder, "</s>").unwrap();
    }
    if is_underline {
        write!(&mut builder, "</u>").unwrap();
    }
    if is_code {
        write!(&mut builder, "</code>").unwrap();
    }
    if is_superscript {
        write!(&mut builder, "</sup>").unwrap();
    }
    if is_subscript {
        write!(&mut builder, "</sub>").unwrap();
    }
    if is_heading {
        write!(&mut builder, "</h1>").unwrap();
    }
    match list_type {
        Some(ListTypes::Ordered) => {
            write!(&mut builder, "</li></ol>").unwrap();
        },
        Some(ListTypes::Unordered) => {
            write!(&mut builder, "</li></ull>").unwrap();
        },
            _ => {}
    }
    write!(&mut builder, "</p>").unwrap();

    // *****************  Return  ******************* //
     builder
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_styling])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
