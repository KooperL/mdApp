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
        self.is_underline = false;
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
pub enum Tags {
    Bold,
    Emphasised,
    Underline,
    Stikethrough,
    Code,
    Superscript,
    Subscript,
    ListItem,
    UnorderedList,
    OrderedList,
    Check,
    Heading,
    Paragraph,
}

impl Tags {
    const BOLD_OPEN: &'static str = "<b>";
    const EMPHASISED_OPEN: &'static str = "<i>";
    const UNDERLINE_OPEN: &'static str = "<u>";
    const STRIKETHROUGH_OPEN: &'static str = "<s>";
    const CODE_OPEN: &'static str = "<code>";
    const SUPERSCRIPT_OPEN: &'static str = "<sup>";
    const SUBSCRIPT_OPEN: &'static str = "<sub>";
    const LIST_ITEM: &'static str = "<li>";
    const UNORDERED_LIST: &'static str = "<ul>";
    const ORDERED_LIST: &'static str = "<ol>";
    const CHECK_OPEN: &'static str = "<input type=\"checkbox\">";
    const HEADING_OPEN: &'static str = "<h1>";
    const PARAGRAPH: &'static str = "<p contenteditable=\"true\" onKeyDown=\"keyPresshandler\">";
    
    pub fn get_open_tag(&self) -> &'static str {
        match self {
            Tags::Bold => Self::BOLD_OPEN,
            Tags::Emphasised => Self::BOLD_OPEN,
            Tags::Stikethrough => Self::BOLD_OPEN,
            Tags::Underline => Self::BOLD_OPEN,
            Tags::Code => Self::BOLD_OPEN,
            Tags::Subscript => Self::BOLD_OPEN,
            Tags::Superscript => Self::BOLD_OPEN,
            Tags::ListItem => Self::BOLD_OPEN,
            Tags::UnorderedList => Self::BOLD_OPEN,
            Tags::OrderedList => Self::BOLD_OPEN,
            Tags::Check => Self::BOLD_OPEN,
            Tags::Heading => Self::BOLD_OPEN,
            Tags::Paragraph => Self::BOLD_OPEN,
        }
    }

    pub fn get_close_tag(&self) -> &'static str {
        match *self {
            Tags::Bold => "</b>",
            Tags::Emphasised => "</i>",
            Tags::Underline => "</u>",
            Tags::Stikethrough => "</s>",
            Tags::Code => "</code>",
            Tags::Subscript => "</sub>",
            Tags::Superscript => "</sup>",
            Tags::UnorderedList => "</ul>",
            Tags::OrderedList => "</ol>",
            Tags::ListItem => "</li>",
            Tags::Check=> "</input>",
            Tags::Paragraph=> "</p>",
            Tags::Heading=> "</h1>",
        }
    }
}

fn test() {
    let tag = Tags::Bold.get_open_tag();
}
