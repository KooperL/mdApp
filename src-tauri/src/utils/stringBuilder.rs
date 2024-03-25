use std::fmt::Write;

use crate::Types::{self, BasicFormat, TokenContext};

pub fn stringBuilder(basic_format: &mut BasicFormat, token_context: &mut TokenContext, struct_collection: &mut Vec<Types::StyledCharacter>) -> String {
    basic_format.make_all_false();
    token_context.is_special_style = None;
    token_context.is_special_font = None;
    token_context.list_type = None;
    token_context.list_item = None;

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

        match token_context.is_special_style {
            None => {
                match char.is_special_style {
                    Some(Types::SpecialStyle::Header) => {
                        write!(&mut builder, "</p><h1>").unwrap();
                        token_context.is_special_style = Some(Types::SpecialStyle::Header);
                    },
                    None => {},
                }
            },
            Some(Types::SpecialStyle::Header) => {
                match char.is_special_style {
                    None => {
                        write!(&mut builder, "</h1><p {}>", property).unwrap();
                        token_context.is_special_style = None;
                    },
                    Some(_) => {},
                }
            },
        };

        match token_context.is_special_font {
            None => {
                match char.is_special_font {
                    Some(Types::SpecialFont::Superscript) => {
                            write!(&mut builder, "<sup>").unwrap();
                            token_context.is_special_font = Some(Types::SpecialFont::Superscript);
                    },
                    Some(Types::SpecialFont::Subscript) => {
                            write!(&mut builder, "<sub>").unwrap();
                            token_context.is_special_font = Some(Types::SpecialFont::Subscript);
                    },
                    Some(Types::SpecialFont::Code) => {
                            write!(&mut builder, "<code>").unwrap();
                            token_context.is_special_font = Some(Types::SpecialFont::Code);
                    },
                    None => {}
                };
            },
            Some(Types::SpecialFont::Superscript) => {
                match char.is_special_font {
                    None => {
                        write!(&mut builder, "</sup>").unwrap();
                        token_context.is_special_font = None;
                    }
                    Some(_) => {}
                };
            },
            Some(Types::SpecialFont::Subscript) => {
                match char.is_special_font {
                    None => {
                        write!(&mut builder, "</sub>").unwrap();
                        token_context.is_special_font = None;
                    }
                    Some(_) => {}
                };
            },
            Some(Types::SpecialFont::Code) => {
                match char.is_special_font {
                    None => {
                        write!(&mut builder, "</Code>").unwrap();
                        token_context.is_special_font = None;
                    }
                    Some(_) => {}
                };
            },
        };

        match token_context.list_type {
            None => {
                match char.list_type {
                    Some(Types::ListTypes::Ordered) => {
                        if char.list_item == Some(0) {
                            write!(&mut builder, "</p><ol {}><li>", property).unwrap();
                           token_context.list_type = Some(Types::ListTypes::Ordered);
                           token_context.list_item = char.list_item;
                        }
                    },
                    Some(Types::ListTypes::Unordered) => {
                        if char.list_item == Some(0) {
                            write!(&mut builder, "</p><ul {}><li>", property).unwrap();
                           token_context.list_type = Some(Types::ListTypes::Unordered);
                           token_context.list_item = char.list_item;
                        }
                    },
                    Some(Types::ListTypes::Check) => {
                        if char.list_item == Some(0) {
                            write!(&mut builder, "<input type=\"checkbox\">").unwrap();
                           token_context.list_type = Some(Types::ListTypes::Check);
                           token_context.list_item = char.list_item;
                        }
                    },
                    None => {}
                }
            },
            Some(Types::ListTypes::Unordered) => {
                match char.list_item {
                    None => {
                        write!(&mut builder, "</li></ul><p {}>", property).unwrap();
                       token_context.list_type = None;
                       token_context.list_item = None;
                    },
                    Some(val) => {
                        if token_context.list_item == Some(val + 1) {
                            write!(&mut builder, "</li><li>").unwrap();
                            if let Some(mut value) = token_context.list_item {
                                value += 1;
                                token_context.list_item = Some(value);
                            }
                        }
                    }
                }
            },
            Some(Types::ListTypes::Ordered) => {
                match char.list_item {
                    None => {
                        write!(&mut builder, "</li></ol><p {}>", property).unwrap();
                       token_context.list_type = None;
                       token_context.list_item = None;
                    },
                    Some(val) => {
                        if token_context.list_item == Some(val + 1) {
                            write!(&mut builder, "</li><li>").unwrap();
                            if let Some(mut value) = token_context.list_item {
                                value += 1;
                                token_context.list_item = Some(value);
                            }
                        }
                    }
                }
            },
            Some(Types::ListTypes::Check) => {
                match char.list_item {
                    None => {
                        write!(&mut builder, "</input>").unwrap();
                        token_context.list_type = None;
                        token_context.list_item = None;
                    },
                    Some(val) => {
                        if token_context.list_item == Some(val + 1) {
                            write!(&mut builder, "</input><input type=\"checkbox\">").unwrap();
                            if let Some(mut value) = token_context.list_item {
                                value += 1;
                                token_context.list_item = Some(value);
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
    match token_context.is_special_font {
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
    if let Some(Types::SpecialStyle::Header) = token_context.is_special_style {
        write!(&mut builder, "</h1>").unwrap();
    };
    match token_context.list_type {
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