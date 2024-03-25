use crate::Types::{self, BasicFormat, ListTypes, SpecialFont, SpecialStyle, TokenContext};

pub fn updateContextFromOpeningHtmlToken(char: &str, basic_format: &mut BasicFormat, token_context: &mut TokenContext) {
    match char {
        "b" => {
            basic_format.is_bold = true;
            token_context.is_special_style = None;
            token_context.is_special_font = None;
        },
        "i" => {
            basic_format.is_emphasised = true;
            token_context.is_special_style = None;
            token_context.is_special_font = None;
        },
        "s" => {
            basic_format.is_strikethrough = true;
            token_context.is_special_style = None;
            token_context.is_special_font = None;
        },
        "u" => {
            basic_format.is_underline = true;
            token_context.is_special_style = None;
            token_context.is_special_font = None;
        },
        "input" => {
            token_context.list_type = Some(ListTypes::Check);
            token_context.list_item = Some(0);
        },
        "code" => {
            basic_format.make_all_false();
            token_context.is_special_font = Some(SpecialFont::Code);
            token_context.is_special_style = None;
        },
        "sup" => {
            basic_format.make_all_false();
            token_context.is_special_font = Some(SpecialFont::Superscript);
            token_context.is_special_style = None;
        },
        "sub" => {
            basic_format.make_all_false();
            token_context.is_special_font = Some(SpecialFont::Subscript);
            token_context.is_special_style = None;
        },
        "h1" => {
            basic_format.make_all_false();
            token_context.is_special_font = None;
            token_context.is_special_style = Some(SpecialStyle::Header);
        },
        "ol" => {
            token_context.is_special_style = None;
            token_context.list_type = Some(ListTypes::Ordered);
            token_context.list_item = Some(0);
        },
        "ul" => {
            token_context.is_special_style = None;
            token_context.list_type = Some(ListTypes::Unordered);
            token_context.list_item = Some(0);
        },
        _ => {
            // Others tags here
        }
    }
}

pub fn updateContextFromClosingHtmlToken(char: &str, basic_format: &mut BasicFormat, token_context: &mut TokenContext) {
    match char {
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
            token_context.list_type = None;
            token_context.list_item = None;
        },
            // TODO: union these matches
        "</code>" | "</sup>" | "</sub>" => {
            token_context.is_special_font = None;
        },
        "</h1>" => {
            token_context.is_special_font = None;
        },
        "</ol>" | "</ul>" => {
           token_context.list_type = None;
           token_context.list_item = None;
        },
        "</li>" => {
            if let Some(mut value) = token_context.list_item {
                value += 1;
                token_context.list_item = Some(value);
            }
        },
        _ => {
            // Others tags here
        }
    }
}


pub fn interpretToken(token_val: xmlparser::Token<'_>, struct_collection: &mut Vec<Types::StyledCharacter>, basic_format: &mut Types::BasicFormat, token_context: &mut TokenContext, begin: usize, end: usize) {
    match token_val {

        // Matching text
        xmlparser::Token::Text {text: val} => {
            for (index, character) in val.chars().enumerate() {
                token_context.rolling_char_counter += 1;
                if token_context.rolling_char_counter >= begin && token_context.rolling_char_counter <= end {
                    if !basic_format.is_bold && character != ' ' && character != '\u{200B}' {
                        token_context.whole_area_is_bold_formatted = false;
                    };
                    // does it work if it's the other way around 

                    if !basic_format.is_emphasised && character != ' ' && character != '\u{200B}' {
                        token_context.whole_area_is_emphasised_formatted = false;
                    };
                    if !basic_format.is_strikethrough && character != ' ' && character != '\u{200B}' {
                        token_context.whole_area_is_strikethrough_formatted = false;
                    };
                    if !basic_format.is_underline && character != ' ' && character != '\u{200B}' {
                        token_context.whole_area_is_underline_formatted = false;
                    };
                    // missing list items?? Is this intentional
                    if character != ' ' && character != '\u{200B}' {
                        match token_context.is_special_font {
                            Some(Types::SpecialFont::Code) => {
                                token_context.whole_area_is_code_formatted = false;
                            },
                            Some(Types::SpecialFont::Subscript) => {
                                token_context.whole_area_is_subscript_formatted = false;
                            },
                            Some(Types::SpecialFont::Superscript) => {
                                token_context.whole_area_is_superscript_formatted = false;
                            },
                            None => {},
                        };
                        match &token_context.is_special_style {
                            Some(Types::SpecialStyle::Header) => {
                                token_context.whole_area_is_heading_formatted = false;
                            },
                            None => {},
                        };
                    };

                };
                struct_collection.push(Types::StyledCharacter {
                    basic_formatting: basic_format.clone(),
                    is_special_style: match token_context.is_special_style {
                        Some(Types::SpecialStyle::Header) => Some(Types::SpecialStyle::Header),
                        None => None,
                    },
                    is_special_font: match token_context.is_special_font {
                        Some(Types::SpecialFont::Superscript) => Some(Types::SpecialFont::Superscript),
                        Some(Types::SpecialFont::Subscript) => Some(Types::SpecialFont::Subscript),
                        Some(Types::SpecialFont::Code) => Some(Types::SpecialFont::Code),
                        None => None,
                    },
                    character: String::from(character),
                    list_type: if token_context.list_type.is_none() {
                        None
                    } else {
                        match token_context.list_type {
                            Some(Types::ListTypes::Ordered) => Some(Types::ListTypes::Ordered),
                            Some(Types::ListTypes::Unordered) => Some(Types::ListTypes::Unordered),
                            Some(Types::ListTypes::Check) => Some(Types::ListTypes::Check),
                            None => None,
                        }
                    },
                    list_item: token_context.list_item,
                });
            }
        },

        // Matching opening token
        xmlparser::Token::ElementStart {prefix: _, local, span} => {
            updateContextFromOpeningHtmlToken(local.as_str(), basic_format, token_context);
        },

        // Matchin closing token
        xmlparser::Token::ElementEnd {span, end} => {
            // println!("{}", span.as_str());
            updateContextFromClosingHtmlToken(span.as_str(),basic_format, token_context);
        },
        _ => {},
    };
}