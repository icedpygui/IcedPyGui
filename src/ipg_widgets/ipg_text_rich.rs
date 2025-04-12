//! ipg_text_rich
use iced::advanced::text::Highlight;
use iced::{Background, Border, Color, Element, Font, Padding, Pixels, Renderer, Theme};
use iced::widget::text::{LineHeight, Rich, Span};
use iced::widget::rich_text;
use crate::app::Message;

use pyo3::{pyclass, PyObject, Python};

use super::helpers::{get_height, 
    get_width, try_extract_boolean, 
    try_extract_f64, try_extract_ipg_color, try_extract_string, 
    try_extract_vec_f32};
use super::ipg_enums::{IpgHorizontalAlignment, IpgVerticalAlignment};

#[derive(Debug, Clone)]
pub struct IpgRichText {
    pub id: usize,
    pub parent_id: String,
    pub size: Option<f32>,
    pub line_height: Option<LineHeight>,
    pub color: Option<Color>,
    pub highlight: Option<IpgHighlight>,
    pub padding: Padding,
    pub underline: bool,
    pub strikethrough: bool,
    pub show: bool,
    pub style_id: Option<usize>,
}

impl IpgRichText {
    pub fn new( 
        id: usize,
        parent_id: String,
        size: Option<f32>,
        line_height: Option<LineHeight>,
        color: Option<Color>,
        highlight: Option<IpgHighlight>,
        padding: Padding,
        underline: bool,
        strikethrough: bool,
        show: bool,
        style_id: Option<usize>,
        ) -> Self {
        Self {
            id,
            parent_id,
            size,
            line_height,
            color,
            highlight,
            padding,
            underline,
            strikethrough,
            show,
            style_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgSpan {
    pub id: usize,
    pub parent_id: String,
    pub text: String,
    pub size: Option<f32>,
    pub line_height: Option<LineHeight>,
    pub color: Option<Color>,
    pub font: Option<Font>,
    pub highlight: Option<IpgHighlight>,
    pub padding: Option<Padding>,
    pub underline: bool,
    pub strikethrough: bool, 
    pub show: bool,
    pub style_id: Option<usize>,
}

impl IpgSpan {
    pub fn new( 
        id: usize,
        parent_id: String,
        text: String,
        size: Option<f32>,
        line_height: Option<LineHeight>,
        color: Option<Color>,
        font: Option<Font>,
        highlight: Option<IpgHighlight>,
        padding: Option<Padding>,
        underline: bool,
        strikethrough: bool, 
        show: bool,
        style_id: Option<usize>,
        ) -> Self {
        Self {
            id,
            parent_id,
            text,
            size,
            line_height,
            color,
            font,
            highlight,
            padding,
            underline,
            strikethrough, 
            show,
            style_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpgHighlight {
    pub background: Background,
    pub border: Border,
}

pub fn construct_rich_text<'a>(text: &IpgRichText) 
                                -> Option<Element<'a, Message>> {

    if !text.show {
        return None
    }

    let mut spans = vec![];
    for sp in text.spans.iter() {
        let mut span = Span::new(sp.text.clone()); 
        if  sp.size.is_some() {
            span.size = Some(Pixels(sp.size.unwrap()));
        }
        if sp.line_height.is_some() {
            span.line_height = sp.line_height; 
        }
        if sp.font.is_some() {
            span.font = sp.font;
        }
        if sp.color.is_some() {
            span.color = sp.color;
        }     
        if sp.highlight.is_some() {
            span.highlight = get_highlight(sp.highlight.clone().unwrap());
        }
        if sp.padding.is_some() {
            span.padding = sp.padding.unwrap();
        }
        span.underline = sp.underline;
        span.strikethrough = sp.underline; 
        
        spans.push(spans);
    }
    let rt: Rich<'a, Message, Theme, Renderer> = rich_text::<Message, Theme, Renderer>(spans);
    Some(rt.into())

}

fn get_highlight(hl: IpgHighlight) -> Option<Highlight> {
    Some(Highlight{ 
        background: hl.background, 
        border: hl.border })
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTextParam {
    Content,
    Height,
    HeightFill,
    AlignX,
    AlignY,
    LineHeight,
    Show,
    Size,
    TextColor, 
    TextRgba,
    Width,
    WidthFill,
}

pub fn text_item_update(txt: &mut IpgRichText, 
                        item: &PyObject, 
                        value: &PyObject) {

    let update = try_extract_text_update(item);
    let name = "Text".to_string();
    match update {
        IpgTextParam::Content => todo!(),
        IpgTextParam::Height => todo!(),
        IpgTextParam::HeightFill => todo!(),
        IpgTextParam::AlignX => todo!(),
        IpgTextParam::AlignY => todo!(),
        IpgTextParam::LineHeight => todo!(),
        IpgTextParam::Show => todo!(),
        IpgTextParam::Size => todo!(),
        IpgTextParam::TextColor => todo!(),
        IpgTextParam::TextRgba => todo!(),
        IpgTextParam::Width => todo!(),
        IpgTextParam::WidthFill => todo!(),
            }
}


fn try_extract_text_update(update_obj: &PyObject) -> IpgTextParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgTextParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Text update extraction failed"),
        }
    })
}

fn try_extract_hor_alignment(update_obj: &PyObject) -> IpgHorizontalAlignment {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgHorizontalAlignment>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Text HorizontalAlignment extraction failed"),
        }
    })
}

fn try_extract_vert_alignment(update_obj: &PyObject) -> IpgVerticalAlignment {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgVerticalAlignment>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Text VerticalAlignment extraction failed"),
        }
    })
}
