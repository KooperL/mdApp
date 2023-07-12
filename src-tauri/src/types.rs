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

