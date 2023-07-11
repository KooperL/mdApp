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
    Check
}

#[derive(Debug)]
enum SpecialFont {
    Code,
    Subscript,
    Superscript,
}

#[derive(Debug)]
enum SpecialStyle {
    Header
}

#[derive(Debug)]
struct StyledCharacter {
    is_bold: bool,
    is_emphasised: bool,
    is_underline: bool,
    is_strikethrough: bool,
    is_special_font: Option<SpecialFont>,
    is_special_style: Option<SpecialStyle>,
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
    let mut is_special_style: Option<SpecialStyle> = None;
    let mut is_special_font: Option<SpecialFont> = None;
    let mut is_underline = false;
    let mut is_strikethrough = false;
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
                        // does it work if it's the other way around 

                        if !is_emphasised && character != ' ' && character != '\u{200B}' {
                            whole_area_is_emphasised_formatted = false;
                        };
                        if character != ' ' && character != '\u{200B}' {
                            match is_special_font {
                                Some(SpecialFont::Code) => {
                                    whole_area_is_code_formatted = false;
                                },
                                Some(SpecialFont::Subscript) => {
                                    whole_area_is_subscript_formatted = false;
                                },
                                Some(SpecialFont::Superscript) => {
                                    whole_area_is_superscript_formatted = false;
                                },
                                _ => {},
                            };
                            match &is_special_style {
                                Some(SpecialStyle::Header) => {
                                    whole_area_is_heading_formatted = false;
                                },
                                _ => {},
                            };
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
                        is_special_style: match is_special_style {
                            Some(SpecialStyle::Header) => Some(SpecialStyle::Header),
                            None => None,
                        },
                        is_special_font: match is_special_font {
                            Some(SpecialFont::Superscript) => Some(SpecialFont::Superscript),
                            Some(SpecialFont::Subscript) => Some(SpecialFont::Subscript),
                            Some(SpecialFont::Code) => Some(SpecialFont::Code),
                            _ => None
                        },
                        character: String::from(character),
                        list_type: if list_type.is_none() {
                            None
                        } else {
                            match list_type {
                                Some(ListTypes::Ordered) => Some(ListTypes::Ordered),
                                Some(ListTypes::Unordered) => Some(ListTypes::Unordered),
                                Some(ListTypes::Check) => Some(ListTypes::Check),
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
                        is_special_style = None;
                        is_special_font = None;
                    },
                    "i" => {
                        is_emphasised = true;
                        is_special_style = None;
                        is_special_font = None;
                    },
                    "s" => {
                        is_strikethrough = true;
                        is_special_style = None;
                        is_special_font = None;
                    },
                    "input" => {
                        list_type = Some(ListTypes::Check);
                        list_item = Some(0);
                    },
                    "u" => {
                        is_underline = true;
                        is_special_style = None;
                        is_special_font = None;
                    },
                    "code" => {
                        is_emphasised = false;
                        is_bold = false;
                        is_strikethrough = false;
                        is_underline =false;
                        is_special_font = Some(SpecialFont::Code);
                        is_special_style = None;
                    },
                    "sup" => {
                        is_emphasised = false;
                        is_bold = false;
                        is_strikethrough = false;
                        is_underline =false;
                        is_special_font = Some(SpecialFont::Superscript);
                        is_special_style = None;
                    },
                    "sub" => {
                        is_emphasised = false;
                        is_bold = false;
                        is_strikethrough = false;
                        is_underline =false;
                        is_special_font = Some(SpecialFont::Subscript);
                        is_special_style = None;
                    },
                    "h1" => {
                        is_emphasised = false;
                        is_bold = false;
                        is_strikethrough = false;
                        is_underline =false;
                        is_special_font = None;
                        is_special_style = Some(SpecialStyle::Header);
                    },
                    "ol" => {
                        is_special_style = None;
                        list_type = Some(ListTypes::Ordered);
                        list_item = Some(0);
                    },
                    "ul" => {
                        is_special_style = None;
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
                        list_type = None;
                        list_item = None;
                    },
                    "</code>" => {
                        is_special_font = None;
                    },
                    "</sup>" => {
                        is_special_font = None;
                    },
                    "</sub>" => {
                        is_special_font = None;
                    },
                    "</h1>" => {
                        is_special_font = None;
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
                struct_collection[i].is_special_style = None;
                struct_collection[i].is_special_font = None;
                if i >= begin && i <= end {
                    struct_collection[i].is_bold = !whole_area_is_bold_formatted;
                } else {
                    struct_collection[i].is_bold = true;
                }
            },
            "emphasise" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].is_special_font = None;
                if i >= begin && i <= end {
                    struct_collection[i].is_emphasised = !whole_area_is_emphasised_formatted;
                } else {
                    struct_collection[i].is_emphasised = true;
                }
            },
            "underline" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].is_special_font = None;
                if i >= begin && i <= end {
                    struct_collection[i].is_underline = !whole_area_is_underline_formatted;
                } else {
                    struct_collection[i].is_underline = true;
                }
            },
            "strikethrough" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].is_special_font = None;
                if i >= begin && i <= end {
                    struct_collection[i].is_strikethrough = !whole_area_is_strikethrough_formatted;
                } else {
                    struct_collection[i].is_strikethrough = true;
                }
            },
            "code" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].is_bold = false;
                struct_collection[i].is_emphasised = false;
                struct_collection[i].is_strikethrough = false;
                struct_collection[i].is_underline = false;

                if whole_area_is_code_formatted {
                    struct_collection[i].is_special_font = Some(SpecialFont::Code);
                } else {
                    struct_collection[i].is_special_font = None;
                };
            },
            "superscript" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].is_bold = false;
                struct_collection[i].is_emphasised = false;
                struct_collection[i].is_strikethrough = false;
                struct_collection[i].is_underline = false;

                if whole_area_is_superscript_formatted {
                    struct_collection[i].is_special_font = Some(SpecialFont::Superscript);
                } else {
                    struct_collection[i].is_special_font = None;
                };
            },
            "subscript" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].is_bold = false;
                struct_collection[i].is_emphasised = false;
                struct_collection[i].is_strikethrough = false;
                struct_collection[i].is_underline = false;

                if whole_area_is_subscript_formatted {
                    struct_collection[i].is_special_font = Some(SpecialFont::Subscript);
                } else {
                    struct_collection[i].is_special_font = None;
                };
            },
            "heading" => {
                struct_collection[i].is_bold = false;
                struct_collection[i].is_emphasised = false;
                struct_collection[i].is_strikethrough = false;
                struct_collection[i].is_underline = false;
                struct_collection[i].is_special_font = None;

                if whole_area_is_heading_formatted {
                    struct_collection[i].is_special_style = Some(SpecialStyle::Header);
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
                struct_collection[i].list_type = Some(ListTypes::Check);
            },
            "ordered-list" => {
                struct_collection[i].is_special_style = None;
                struct_collection[i].list_item = if struct_collection[i].list_item == None {
                    Some(0)
                } else {
                    None
                };
                struct_collection[i].list_type = Some(ListTypes::Ordered);
            },
            "unordered-list" => {
                struct_collection[i].is_special_style = None;
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
    let mut is_special_style: Option<SpecialStyle> = None;
    let mut is_special_font: Option<SpecialFont> = None;
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
       // if !is_check && char.is_check {
       //     write!(&mut builder, "<input type=\"checkbox\">").unwrap();
       //     is_check = true;
       // }
       // if is_check && !char.is_check {
       //     write!(&mut builder, "</input>").unwrap();
       //     is_check = false;
       // }
        if !is_strikethrough && char.is_strikethrough {
            write!(&mut builder, "<s>").unwrap();
            is_strikethrough = true;
        }
        if is_strikethrough && !char.is_strikethrough {
            write!(&mut builder, "</s>").unwrap();
            is_strikethrough = false;
        }

        match is_special_style {
            None => {
                match char.is_special_style {
                    Some(SpecialStyle::Header) => {
                        write!(&mut builder, "<h1>").unwrap();
                        is_special_style = Some(SpecialStyle::Header);
                    },
                    None => {},
                }
            },
            Some(SpecialStyle::Header) => {
                match char.is_special_style {
                    None => {
                        write!(&mut builder, "</h1>").unwrap();
                        is_special_style = None;
                    },
                    Some(_) => {},
                }
            },
        };

        match is_special_font {
            None => {
                match char.is_special_font {
                    Some(SpecialFont::Superscript) => {
                            write!(&mut builder, "<sup>").unwrap();
                            is_special_font = Some(SpecialFont::Superscript);
                    },
                    Some(SpecialFont::Subscript) => {
                            write!(&mut builder, "<sub>").unwrap();
                            is_special_font = Some(SpecialFont::Subscript);
                    },
                    Some(SpecialFont::Code) => {
                            write!(&mut builder, "<code>").unwrap();
                            is_special_font = Some(SpecialFont::Code);
                    },
                    None => {}
                };
            },
            Some(SpecialFont::Superscript) => {
                match char.is_special_font {
                    None => {
                        write!(&mut builder, "</sup>").unwrap();
                        is_special_font = None;
                    }
                    Some(_) => {}
                };
            },
            Some(SpecialFont::Subscript) => {
                match char.is_special_font {
                    None => {
                        write!(&mut builder, "</sub>").unwrap();
                        is_special_font = None;
                    }
                    Some(_) => {}
                };
            },
            Some(SpecialFont::Code) => {
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
                    Some(ListTypes::Check) => {
                        if char.list_item == Some(0) {
                            write!(&mut builder, "<input type=\"checkbox\">").unwrap();
                            list_type = Some(ListTypes::Check);
                            list_item = char.list_item;
                        }
                    },
                    None => {}
                }
            },
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
            },
            Some(ListTypes::Check) => {
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
    match is_special_font {
        Some(SpecialFont::Code) => {
            write!(&mut builder, "</code>").unwrap();
        },
        Some(SpecialFont::Subscript) => {
            write!(&mut builder, "</sub>").unwrap();
        },
        Some(SpecialFont::Superscript) => {
            write!(&mut builder, "</sup>").unwrap();
        },
        _ => {}
    };
    if let Some(val) = is_special_style {
        write!(&mut builder, "</h1>").unwrap();
    };
    match list_type {
        Some(ListTypes::Ordered) => {
            write!(&mut builder, "</li></ol>").unwrap();
        },
        Some(ListTypes::Unordered) => {
            write!(&mut builder, "</li></ull>").unwrap();
        },
            _ => {}
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
