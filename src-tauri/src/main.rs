// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![feature(generators, generator_trait)] 
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::string::String;
use std::fmt::Write;
use xmlparser;
#[path = "./types.rs"] mod Types;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn process_styling(html: &str, begin: usize, end: usize, transformation: &str) -> String {
    // println!("Args: {:?}, {:?}, {:?}", html, begin, end);

    // *****************  Begin parse to vec<struct>  ******************* //
    let mut struct_collection: Vec<Types::StyledCharacter> = Vec::new(); 

    // Contextual parsing flags
    let mut basic_format = Types::BasicFormat {
        is_bold: false,
        is_emphasised: false,
        is_underline: false,
        is_strikethrough: false,
        // ::New()
    };
    let mut is_special_style: Option<Types::SpecialStyle> = None;
    let mut is_special_font: Option<Types::SpecialFont> = None;
    let mut list_type: Option<Types::ListTypes> = None;
    let mut list_item: Option<i32> = None;

    // Cide smell
    let mut rolling_char_counter = 0;
    let mut whole_area_is_bold_formatted = true;
    let mut whole_area_is_emphasised_formatted = true;
    let mut whole_area_is_underline_formatted = true;
    let mut whole_area_is_strikethrough_formatted = true;
    let mut whole_area_is_code_formatted = true;
    let mut whole_area_is_heading_formatted = true;
    let mut whole_area_is_superscript_formatted = true;
    let mut whole_area_is_subscript_formatted = true;

    let tokenizer = xmlparser::Tokenizer::from(html);
    for token in tokenizer {
        let token_val = token.expect("Token to be valid");
        match token_val {

            // Matching text
            xmlparser::Token::Text {text: val} => {
                for (index, character) in val.chars().enumerate() {
                    rolling_char_counter += 1;
                    if rolling_char_counter >= begin && rolling_char_counter <= end {
                        if !basic_format.is_bold && character != ' ' && character != '\u{200B}' {
                            whole_area_is_bold_formatted = false;
                        };
                        // does it work if it's the other way around 

                        if !basic_format.is_emphasised && character != ' ' && character != '\u{200B}' {
                            whole_area_is_emphasised_formatted = false;
                        };
                        if !basic_format.is_strikethrough && character != ' ' && character != '\u{200B}' {
                            whole_area_is_strikethrough_formatted = false;
                        };
                        if !basic_format.is_underline && character != ' ' && character != '\u{200B}' {
                            whole_area_is_underline_formatted = false;
                        };
                        // missing list items?? Is this intentional
                        if character != ' ' && character != '\u{200B}' {
                            match is_special_font {
                                Some(Types::SpecialFont::Code) => {
                                    whole_area_is_code_formatted = false;
                                },
                                Some(Types::SpecialFont::Subscript) => {
                                    whole_area_is_subscript_formatted = false;
                                },
                                Some(Types::SpecialFont::Superscript) => {
                                    whole_area_is_superscript_formatted = false;
                                },
                                None => {},
                            };
                            match &is_special_style {
                                Some(Types::SpecialStyle::Header) => {
                                    whole_area_is_heading_formatted = false;
                                },
                                None => {},
                            };
                        };

                    };
                    struct_collection.push(Types::StyledCharacter {
                        basic_formatting: basic_format.clone(),
                        is_special_style: match is_special_style {
                            Some(Types::SpecialStyle::Header) => Some(Types::SpecialStyle::Header),
                            None => None,
                        },
                        is_special_font: match is_special_font {
                            Some(Types::SpecialFont::Superscript) => Some(Types::SpecialFont::Superscript),
                            Some(Types::SpecialFont::Subscript) => Some(Types::SpecialFont::Subscript),
                            Some(Types::SpecialFont::Code) => Some(Types::SpecialFont::Code),
                            None => None,
                        },
                        character: String::from(character),
                        list_type: if list_type.is_none() {
                            None
                        } else {
                            match list_type {
                                Some(Types::ListTypes::Ordered) => Some(Types::ListTypes::Ordered),
                                Some(Types::ListTypes::Unordered) => Some(Types::ListTypes::Unordered),
                                Some(Types::ListTypes::Check) => Some(Types::ListTypes::Check),
                                None => None,
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
                        basic_format.is_bold = true;
                        is_special_style = None;
                        is_special_font = None;
                    },
                    "i" => {
                        basic_format.is_emphasised = true;
                        is_special_style = None;
                        is_special_font = None;
                    },
                    "s" => {
                        basic_format.is_strikethrough = true;
                        is_special_style = None;
                        is_special_font = None;
                    },
                    "u" => {
                        basic_format.is_underline = true;
                        is_special_style = None;
                        is_special_font = None;
                    },
                    "input" => {
                        list_type = Some(Types::ListTypes::Check);
                        list_item = Some(0);
                    },
                    "code" => {
                        basic_format.make_all_false();
                        is_special_font = Some(Types::SpecialFont::Code);
                        is_special_style = None;
                    },
                    "sup" => {
                        basic_format.make_all_false();
                        is_special_font = Some(Types::SpecialFont::Superscript);
                        is_special_style = None;
                    },
                    "sub" => {
                        basic_format.make_all_false();
                        is_special_font = Some(Types::SpecialFont::Subscript);
                        is_special_style = None;
                    },
                    "h1" => {
                        basic_format.make_all_false();
                        is_special_font = None;
                        is_special_style = Some(Types::SpecialStyle::Header);
                    },
                    "ol" => {
                        is_special_style = None;
                        list_type = Some(Types::ListTypes::Ordered);
                        list_item = Some(0);
                    },
                    "ul" => {
                        is_special_style = None;
                        list_type = Some(Types::ListTypes::Unordered);
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
                        basic_format.is_bold = false;
                    },
                    "</i>" => {
                        basic_format.is_emphasised = false;
                    },
                    "</s>" => {
                        basic_format.is_strikethrough = false;
                    },
                    "</u>" => {
                        basic_format.is_underline = false;
                    },
                    "</input>" => {
                        list_type = None;
                        list_item = None;
                    },
                        // TODO: union these matches
                    "</code>" | "</sup>" | "</sub>" => {
                        is_special_font = None;
                    },
                    "</h1>" => {
                        is_special_font = None;
                    },
                    "</ol>" | "</ul>" => {
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
                struct_collection[i].is_special_style = None;
                struct_collection[i].is_special_font = None;
                if i >= begin && i <= end {
                    struct_collection[i].basic_formatting.is_bold = !whole_area_is_bold_formatted;
                } else {
                    struct_collection[i].basic_formatting.is_bold = true;
                }
            },
            "emphasise" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].is_special_font = None;
                if i >= begin && i <= end {
                    struct_collection[i].basic_formatting.is_emphasised = !whole_area_is_emphasised_formatted;
                } else {
                    struct_collection[i].basic_formatting.is_emphasised = true;
                }
            },
            "underline" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].is_special_font = None;
                if i >= begin && i <= end {
                    struct_collection[i].basic_formatting.is_underline = !whole_area_is_underline_formatted;
                } else {
                    struct_collection[i].basic_formatting.is_underline = true;
                }
            },
            "strikethrough" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].is_special_font = None;
                if i >= begin && i <= end {
                    struct_collection[i].basic_formatting.is_strikethrough = !whole_area_is_strikethrough_formatted;
                } else {
                    struct_collection[i].basic_formatting.is_strikethrough = true;
                }
            },
            "code" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].basic_formatting.make_all_false();

                if whole_area_is_code_formatted {
                    struct_collection[i].is_special_font = Some(Types::SpecialFont::Code);
                } else {
                    struct_collection[i].is_special_font = None;
                };
            },
            "superscript" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].basic_formatting.make_all_false();

                if whole_area_is_superscript_formatted {
                    struct_collection[i].is_special_font = Some(Types::SpecialFont::Superscript);
                } else {
                    struct_collection[i].is_special_font = None;
                };
            },
            "subscript" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].basic_formatting.make_all_false();

                if whole_area_is_subscript_formatted {
                    struct_collection[i].is_special_font = Some(Types::SpecialFont::Subscript);
                } else {
                    struct_collection[i].is_special_font = None;
                };
            },
            "heading" => {
                struct_collection[i].basic_formatting.make_all_false();
                struct_collection[i].is_special_font = None;

                if whole_area_is_heading_formatted {
                    struct_collection[i].is_special_style = Some(Types::SpecialStyle::Header);
                } else {
                    struct_collection[i].is_special_style = None;
                };
            },
            "check" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].list_item = if struct_collection[i].list_item == None {
                    Some(0)
                } else {
                    None
                };
                struct_collection[i].list_type = Some(Types::ListTypes::Check);
            },
            "ordered-list" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].list_item = if struct_collection[i].list_item == None {
                    Some(0)
                } else {
                    None
                };
                struct_collection[i].list_type = Some(Types::ListTypes::Ordered);
            },
            "unordered-list" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].list_item = if struct_collection[i].list_item == None {
                    Some(0)
                } else {
                    None
                };
                struct_collection[i].list_type = Some(Types::ListTypes::Unordered);
            },
            _ => {},
        }
    }

    // println!("{:?}", struct_collection);
    // *****************  Begin final string build  ******************* //
    basic_format.make_all_false();
    let mut is_special_style: Option<Types::SpecialStyle> = None;
    let mut is_special_font: Option<Types::SpecialFont> = None;
    list_type = None;
    list_item = None;

    let property = "contenteditable=\"true\" onKeyDown=\"keyPresshandler\"";
    let mut builder = String::new();
    write!(&mut builder, "<p {}>", property).unwrap();
    if struct_collection.len() == 0 {
        write!(&mut builder, "\u{200B}").unwrap();
    }

    for char in struct_collection {
        if !basic_format.is_bold && char.basic_formatting.is_bold {
            write!(&mut builder, "<b>").unwrap();
            basic_format.is_bold = true;
        }
        if basic_format.is_bold && !char.basic_formatting.is_bold {
            write!(&mut builder, "</b>").unwrap();
            basic_format.is_bold = false;
        }
        if !basic_format.is_emphasised && char.basic_formatting.is_emphasised {
            write!(&mut builder, "<i>").unwrap();
            basic_format.is_emphasised = true;
        }
        if basic_format.is_emphasised && !char.basic_formatting.is_emphasised {
            write!(&mut builder, "</i>").unwrap();
            basic_format.is_emphasised = false;
        }
        if !basic_format.is_underline && char.basic_formatting.is_underline {
            write!(&mut builder, "<u>").unwrap();
            basic_format.is_underline = true;
        }
        if basic_format.is_underline && !char.basic_formatting.is_underline {
            write!(&mut builder, "</u>").unwrap();
            basic_format.is_underline = false;
        }
        if !basic_format.is_strikethrough && char.basic_formatting.is_strikethrough {
            write!(&mut builder, "<s>").unwrap();
            basic_format.is_strikethrough = true;
        }
        if basic_format.is_strikethrough && !char.basic_formatting.is_strikethrough {
            write!(&mut builder, "</s>").unwrap();
            basic_format.is_strikethrough = false;
        }

        match is_special_style {
            None => {
                match char.is_special_style {
                    Some(Types::SpecialStyle::Header) => {
                        write!(&mut builder, "</p><h1>").unwrap();
                        is_special_style = Some(Types::SpecialStyle::Header);
                    },
                    None => {},
                }
            },
            Some(Types::SpecialStyle::Header) => {
                match char.is_special_style {
                    None => {
                        write!(&mut builder, "</h1><p {}>", property).unwrap();
                        is_special_style = None;
                    },
                    Some(_) => {},
                }
            },
        };

        match is_special_font {
            None => {
                match char.is_special_font {
                    Some(Types::SpecialFont::Superscript) => {
                            write!(&mut builder, "<sup>").unwrap();
                            is_special_font = Some(Types::SpecialFont::Superscript);
                    },
                    Some(Types::SpecialFont::Subscript) => {
                            write!(&mut builder, "<sub>").unwrap();
                            is_special_font = Some(Types::SpecialFont::Subscript);
                    },
                    Some(Types::SpecialFont::Code) => {
                            write!(&mut builder, "<code>").unwrap();
                            is_special_font = Some(Types::SpecialFont::Code);
                    },
                    None => {}
                };
            },
            Some(Types::SpecialFont::Superscript) => {
                match char.is_special_font {
                    None => {
                        write!(&mut builder, "</sup>").unwrap();
                        is_special_font = None;
                    }
                    Some(_) => {}
                };
            },
            Some(Types::SpecialFont::Subscript) => {
                match char.is_special_font {
                    None => {
                        write!(&mut builder, "</sub>").unwrap();
                        is_special_font = None;
                    }
                    Some(_) => {}
                };
            },
            Some(Types::SpecialFont::Code) => {
                match char.is_special_font {
                    None => {
                        write!(&mut builder, "</Code>").unwrap();
                        is_special_font = None;
                    }
                    Some(_) => {}
                };
            },
        };

        match list_type {
            None => {
                match char.list_type {
                    Some(Types::ListTypes::Ordered) => {
                        if char.list_item == Some(0) {
                            write!(&mut builder, "</p><ol {}><li>", property).unwrap();
                            list_type = Some(Types::ListTypes::Ordered);
                            list_item = char.list_item;
                        }
                    },
                    Some(Types::ListTypes::Unordered) => {
                        if char.list_item == Some(0) {
                            write!(&mut builder, "</p><ul {}><li>", property).unwrap();
                            list_type = Some(Types::ListTypes::Unordered);
                            list_item = char.list_item;
                        }
                    },
                    Some(Types::ListTypes::Check) => {
                        if char.list_item == Some(0) {
                            write!(&mut builder, "<input type=\"checkbox\">").unwrap();
                            list_type = Some(Types::ListTypes::Check);
                            list_item = char.list_item;
                        }
                    },
                    None => {}
                }
            },
            Some(Types::ListTypes::Unordered) => {
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
            },
            Some(Types::ListTypes::Ordered) => {
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
            },
            Some(Types::ListTypes::Check) => {
                match char.list_item {
                    None => {
                        write!(&mut builder, "</input>").unwrap();
                        list_type = None;
                        list_item = None;
                    },
                    Some(val) => {
                        if list_item == Some(val + 1) {
                            write!(&mut builder, "</input><input type=\"checkbox\">").unwrap();
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
    if basic_format.is_bold {
        write!(&mut builder, "</b>").unwrap();
    }
    if basic_format.is_emphasised {
        write!(&mut builder, "</i>").unwrap();
    }
    if basic_format.is_strikethrough {
        write!(&mut builder, "</s>").unwrap();
    }
    if basic_format.is_underline {
        write!(&mut builder, "</u>").unwrap();
    }
    match is_special_font {
        Some(Types::SpecialFont::Code) => {
            write!(&mut builder, "</code>").unwrap();
        },
        Some(Types::SpecialFont::Subscript) => {
            write!(&mut builder, "</sub>").unwrap();
        },
        Some(Types::SpecialFont::Superscript) => {
            write!(&mut builder, "</sup>").unwrap();
        },
        None => (),
    };
    if let Some(Types::SpecialStyle::Header) = is_special_style {
        write!(&mut builder, "</h1>").unwrap();
    };
    match list_type {
        Some(Types::ListTypes::Ordered) => {
            write!(&mut builder, "</li></ol>").unwrap();
        },
        Some(Types::ListTypes::Unordered) => {
            write!(&mut builder, "</li></ull>").unwrap();
        },
        Some(Types::ListTypes::Check) => {
            write!(&mut builder, "</input>").unwrap();
        },
        None => (),
    };

    // *****************  Return  ******************* //
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
