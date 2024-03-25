#[derive(Debug)]
pub enum ListTypes {
    Ordered,
    Unordered,
    Check
}

#[derive(Debug)]
pub enum SpecialFont {
    Code,
    Subscript,
    Superscript,
}

#[derive(Debug)]
pub enum SpecialStyle {
    Header
}

#[derive(Debug)]
#[derive(Clone)]
pub struct BasicFormat {
    pub is_bold: bool,
    pub is_emphasised: bool,
    pub is_underline: bool,
    pub is_strikethrough: bool,
}

impl BasicFormat {
    pub fn new() -> Self {
        return BasicFormat {
            is_bold: false,
            is_emphasised: false,
            is_underline: false,
            is_strikethrough: false,
        };
    }
}

impl BasicFormat {
    pub fn make_all_false(&mut self) {
        self.is_bold = false;
        self.is_emphasised = false;
        self.is_emphasised = false;
        self.is_strikethrough = false;
    }
}

#[derive(Debug)]
pub struct StyledCharacter {
    pub basic_formatting: BasicFormat,
    pub is_special_font: Option<SpecialFont>,
    pub is_special_style: Option<SpecialStyle>,
    pub character: String,
    pub list_type: Option<ListTypes>,
    pub list_item: Option<i32>
}

#[derive(Debug)]
pub struct TokenContext {
    pub is_special_style: Option<SpecialStyle>,
    pub is_special_font: Option<SpecialFont>,
    pub list_type: Option<ListTypes>,
    pub list_item: Option<i32>,
    pub rolling_char_counter: usize,
    pub whole_area_is_bold_formatted: bool,
    pub whole_area_is_emphasised_formatted: bool,
    pub whole_area_is_underline_formatted: bool,
    pub whole_area_is_strikethrough_formatted: bool,
    pub whole_area_is_code_formatted: bool,
    pub whole_area_is_heading_formatted: bool,
    pub whole_area_is_superscript_formatted: bool,
    pub whole_area_is_subscript_formatted: bool,
}

impl TokenContext {
    pub fn new () -> Self {
        return TokenContext {
            is_special_style: None,
            is_special_font: None,
            list_type: None,
            list_item: None,
            rolling_char_counter: 0,
            whole_area_is_bold_formatted: true,
            whole_area_is_emphasised_formatted: true,
            whole_area_is_underline_formatted: true,
            whole_area_is_strikethrough_formatted: true,
            whole_area_is_code_formatted: true,
            whole_area_is_heading_formatted: true,
            whole_area_is_superscript_formatted: true,
            whole_area_is_subscript_formatted: true,
        }
    }
}
