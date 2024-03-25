use crate::Types;

pub fn applyTransformation(struct_collection: &mut Vec<Types::StyledCharacter>, token_context: &mut Types::TokenContext, i: usize, transformation: &str, begin: usize, end: usize) {
    match transformation {
        "bold" => {
            struct_collection[i].is_special_style = None;
            struct_collection[i].is_special_font = None;
            if i >= begin && i <= end {
                struct_collection[i].basic_formatting.is_bold = !token_context.whole_area_is_bold_formatted;
            } else {
                struct_collection[i].basic_formatting.is_bold = true;
            }
        },
        "emphasise" => {
            struct_collection[i].is_special_style = None;
            struct_collection[i].is_special_font = None;
            if i >= begin && i <= end {
                struct_collection[i].basic_formatting.is_emphasised = !token_context.whole_area_is_emphasised_formatted;
            } else {
                struct_collection[i].basic_formatting.is_emphasised = true;
            }
        },
        "underline" => {
            struct_collection[i].is_special_style = None;
            struct_collection[i].is_special_font = None;
            if i >= begin && i <= end {
                struct_collection[i].basic_formatting.is_underline = !token_context.whole_area_is_underline_formatted;
            } else {
                struct_collection[i].basic_formatting.is_underline = true;
            }
        },
        "strikethrough" => {
            struct_collection[i].is_special_style = None;
            struct_collection[i].is_special_font = None;
            if i >= begin && i <= end {
                struct_collection[i].basic_formatting.is_strikethrough = !token_context.whole_area_is_strikethrough_formatted;
            } else {
                struct_collection[i].basic_formatting.is_strikethrough = true;
            }
        },
        "code" => {
            struct_collection[i].is_special_style = None;
            struct_collection[i].basic_formatting.make_all_false();

            if token_context.whole_area_is_code_formatted {
                struct_collection[i].is_special_font = Some(Types::SpecialFont::Code);
            } else {
                struct_collection[i].is_special_font = None;
            };
        },
        "superscript" => {
            struct_collection[i].is_special_style = None;
            struct_collection[i].basic_formatting.make_all_false();

            if token_context.whole_area_is_superscript_formatted {
                struct_collection[i].is_special_font = Some(Types::SpecialFont::Superscript);
            } else {
                struct_collection[i].is_special_font = None;
            };
        },
        "subscript" => {
            struct_collection[i].is_special_style = None;
            struct_collection[i].basic_formatting.make_all_false();

            if token_context.whole_area_is_subscript_formatted {
                struct_collection[i].is_special_font = Some(Types::SpecialFont::Subscript);
            } else {
                struct_collection[i].is_special_font = None;
            };
        },
        "heading" => {
            struct_collection[i].basic_formatting.make_all_false();
            struct_collection[i].is_special_font = None;

            if token_context.whole_area_is_heading_formatted {
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