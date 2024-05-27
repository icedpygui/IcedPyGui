

use crate::app::{Message, self};
use crate::access_callbacks;
use super::ipg_container::date_picker_container;
use super::ipg_modal::IpgModal;
use super::callbacks::{WidgetCallbackIn, 
                        WidgetCallbackOut, 
                        get_set_widget_callback_data};
use crate::ICON_FONT_BOOT;
use super::helpers::{get_padding, try_extract_boolean, try_extract_f64, try_extract_string, try_extract_vec_f64, DATE_FORMATS, DAYS, MONTH_NAMES, WEEKDAYS};
use super::ipg_button::{theme_primary, theme_success};

use iced::advanced::graphics::core::Element;
use iced::{Length, Padding, Renderer, Theme};
use iced::alignment::{self, Alignment};
use iced::widget::{Button, Column, Container, PickList, Row, Space, Text};

use chrono::prelude::*;
use pyo3::{pyclass, PyObject, Python};



#[derive(Debug, Clone)]
pub struct IpgDatePicker {
    pub id: usize,
    pub label: String,
    pub size_factor: f32,
    pub padding: Padding,
    pub show: bool,
    pub user_data: Option<PyObject>,
    
    pub selected_format: String,
    pub selected_year: i32,
    pub selected_month: String,
    pub selected_month_index: usize,
    pub selected_day: usize,
    pub selected_date: String,

    show_width: f32,
    show_height: f32,
    hide_width: Length,
    hide_height: Length,
    pub is_submitted: bool,
}

impl IpgDatePicker {
    pub fn new( 
        id: usize,
        label: String,
        size_factor: f32,
        padding: Padding,
        show: bool,
        user_data: Option<PyObject>,
        ) -> Self {
        Self {
            id,
            label,
            size_factor,
            padding,
            show,
            user_data,

            selected_format: "YYYY-mm-dd".to_string(),
            selected_year: Utc::now().year(),
            selected_month_index: Utc::now().month() as usize,
            selected_month: MONTH_NAMES[Utc::now().month() as usize].to_string(),
            selected_day: Utc::now().day() as usize,
            selected_date: "".to_string(),

            show_width: 145.0,
            show_height: 180.0,
            hide_width: Length::Fixed(100.0),
            hide_height: Length::Fixed(50.0),
            is_submitted: false,
        }
    }
}


#[derive(Debug, Clone)]
pub enum DPMessage {
    ShowModal,
    HideModal,
    DayPressed(usize),
    MonthLeftPressed(usize),
    MonthRightPressed(usize),
    YearLeftPressed,
    YearRightPressed,
    DatePickerFormat(String),
    OnSubmit,
}   

pub fn construct_date_picker(dp: IpgDatePicker) -> Element<'static, Message, Theme, Renderer> {

    if !dp.show {
        return calendar_show_button(dp.clone());
    }
    
    let width = Length::Fixed(dp.show_width * dp.size_factor);
    let height = Length::Fixed(dp.show_height * dp.size_factor);
    
    let content: Element<Message, Theme, Renderer> = 
                                    Container::new(Space::new(0.0, 0.0))
                                                .padding(dp.padding)
                                                .align_x(alignment::Horizontal::Center)
                                                .align_y(alignment::Vertical::Center)
                                                .width(width)
                                                .height(height)
                                                .style(|theme| date_picker_container(theme))
                                                .into();
    
    if dp.show {
        
        let col_content: Element<Message, Theme, Renderer> =
            Column::with_children(vec![
                create_first_row_arrows(dp.id, 
                                        dp.selected_month, 
                                        dp.selected_month_index, 
                                        dp.selected_year,
                                        dp.size_factor),
                
                // Column titles S M T W T F S
                Row::with_children(vec![Space::with_width(7.0*dp.size_factor).into(), 
                                        create_day_row(dp.size_factor)])
                                    .width(Length::Fill).into(),
                
                // days of the month
                Row::with_children(vec![Space::with_width(5.0*dp.size_factor).into(), 
                                        get_calendar_days(dp.id, 
                                                            dp.selected_year.clone(),
                                                            dp.selected_month_index.clone(),
                                                            dp.selected_day,
                                                            dp.size_factor),
                                        ])
                                        .width(Length::Fill).into(),

                // close btn and format picklist
                Row::with_children(vec![Space::with_width(5.0*dp.size_factor).into(), 
                                        create_select_row(dp.id, 
                                                            dp.selected_format.clone(), 
                                                            dp.size_factor),
                                        ])
                                        .width(Length::Fill).into(),
                
                // bottom submit btn and selected date, if any
                Row::with_children(vec![Space::with_width(5.0*dp.size_factor).into(),
                                        create_submit_row(dp.id, 
                                                            dp.size_factor, 
                                                            dp.selected_date.clone())
                                    ])
                                    .width(Length::Fill).into(),
                
            ])
            .spacing(3.0*dp.size_factor)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into();
 
        let cont: Element<Message, Theme, Renderer> = Container::new(col_content)
                                                                    // .style(theme::Container::Box)
                                                                    .into();

        let modal: Element<Message, Theme, Renderer> = 
                                    IpgModal::new(content, cont)
                                        // .on_blur(Message::DatePicker(DPMessage::HideModal))
                                        .into();
        modal

    } else {
        content.into()
    }
        
}

fn icon(unicode: char, size: f32) -> Text<'static> {
    Text::new(unicode.to_string())
        .font(ICON_FONT_BOOT)
        .size(size)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center)
}

fn left_arrow_icon(size: f32) -> Text<'static> {
    icon('\u{f12c}', size)
}

fn right_arrow_icon(size: f32) -> Text<'static> {
    icon('\u{f135}', size)
}


fn get_days_of_month(year: i32, month: u32) -> i64 {

    let mut mon: u32 = month;
    let mut yr: i32 = year;

    if month == 12 {
        mon = 1;
        yr = yr + 1;
    } else {
        mon += 1;
    }

    let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(yr, mon, 1).unwrap();
    let since = NaiveDate::signed_duration_since;
   
    since(end, start).num_days()
    
}

fn calendar_show_button(dp: IpgDatePicker) -> Element<'static, Message, Theme, Renderer> {

    let show_btn: Element<DPMessage, Theme, Renderer> = 
                    Button::new(Text::new(dp.label.clone()))
                                    .on_press(DPMessage::ShowModal)
                                    .height(Length::Shrink)
                                    .width(Length::Shrink)
                                    .into();

    let s_btn: Element<Message, Theme, Renderer> = 
                            show_btn.map(move |message| Message::DatePicker(dp.id, message));

    Container::new(s_btn)
                    .padding(dp.padding)
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center)
                    .width(dp.hide_width)
                    .height(dp.hide_height)
                    .into()

}


fn create_first_row_arrows(id: usize, selected_month: String, 
                            selected_month_index: usize, 
                            selected_year: i32,
                            size_factor: f32) 
                            -> Element<'static, Message, Theme, Renderer> 
{
    let btn_arrow_width = 18.0 * size_factor;
    let btn_arrow_height = 15.0 * size_factor;
    let arrow_size = 11.0 * size_factor;

    // sets a width for all month names which prevents shifing when month names differ.
    let month_container_width = 45.0 * size_factor; 
    let text_size = 9.0 * size_factor;
    let padding = 0;

    let left_btn: Element<DPMessage, Theme, Renderer> = 
                Button::new(left_arrow_icon(arrow_size))
                        .on_press(DPMessage::MonthLeftPressed(selected_month_index))
                        .width(btn_arrow_width)
                        .height(btn_arrow_height)
                        .padding(padding)
                        .into();
    let month_left_btn: Element<'_, Message, Theme, Renderer> = 
                left_btn.map(move |message| Message::DatePicker(id, message));

    let right_btn: Element<DPMessage, Theme, Renderer> = 
                Button::new(right_arrow_icon(arrow_size))
                        .on_press(DPMessage::MonthRightPressed(selected_month_index))
                        .width(btn_arrow_width)
                        .height(btn_arrow_height)
                        .padding(padding)
                        .into();
    let month_right_btn: Element<'_, Message, Theme, Renderer> = 
                right_btn.map(move |message| Message::DatePicker(id, message));

    let left_btn: Element<DPMessage, Theme, Renderer> = 
                Button::new(left_arrow_icon(arrow_size))
                        .on_press(DPMessage::YearLeftPressed)
                        .width(btn_arrow_width)
                        .height(btn_arrow_height)
                        .padding(padding)
                        .into();
    let year_left_btn: Element<'_, Message, Theme, Renderer> = 
                left_btn.map(move |message| Message::DatePicker(id, message));

    let right_btn: Element<DPMessage, Theme, Renderer> = 
                Button::new(right_arrow_icon(arrow_size))
                        .on_press(DPMessage::YearRightPressed)
                        .width(btn_arrow_width)
                        .height(btn_arrow_height)
                        .padding(padding)
                        .into();
    let year_right_btn: Element<'_, Message, Theme, Renderer> = 
                right_btn.map(move |message| Message::DatePicker(id, message));

    let selected_month_cont: Element<Message, Theme, Renderer> = 
            Container::new(Text::new(selected_month.clone())
                        .size(text_size))
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center)
                        .width(Length::Fixed(month_container_width))
                        .into();

    Row::with_children(vec![
        Row::with_children(vec![
                            month_left_btn,
                            selected_month_cont, 
                            month_right_btn,
                            // --------------------------------------
                            year_left_btn,
                            
                            Text::new(selected_year.to_string())
                                        .size(text_size)
                                        .into(),
                            year_right_btn,
                        ])
                        .spacing(2)
                        .align_items(Alignment::Center).into(),
    ])
    .align_items(Alignment::Center)
    .width(Length::Fill)
    .into()

}


fn get_calendar_days(id: usize, selected_year: i32, 
                        selected_month_index: usize, 
                        selected_day: usize,
                        size_factor: f32) 
                        -> Element<'static, Message, Theme, Renderer> 
{

    let days = get_days_of_month(selected_year, selected_month_index as u32) as f32;

    let first_day_index = NaiveDate::from_ymd_opt(selected_year, selected_month_index as u32, 1).unwrap().num_days_from_ce();
    let first_day = NaiveDate::from_ymd_opt(selected_year, selected_month_index as u32, 1).unwrap().weekday();
    
    let mut weeks: usize = (days/7.0).ceil() as usize;
    if weeks as f32 * 7.0 < days + first_day_index as f32 {
        weeks += 1;
    } 

    let mut calendar_days: Vec<Element<'static, Message, Theme, Renderer>> = vec![];

    let mut start_weekday = false;
    let mut start_correction = 0;

    for week in 0..weeks {

        let mut row: Vec<Element<'static, Message, Theme, Renderer>> = vec![];

        for d in 1..=7 {
            let mut day = week * 7 + d - start_correction;
            if !start_weekday {
            
                if WEEKDAYS[d-1].to_string() == first_day.to_string() {
                    start_weekday = true;
                    start_correction = d-1;
                    day = day - start_correction;
                    
                } else {
                    row.push(Space::new(15.0*size_factor, 15.0*size_factor).into());
                }
            }
            if day <= days as usize && start_weekday {
                
                let btn: Element<DPMessage, Theme, Renderer> = 
                        Button::new(Text::new(day.to_string())
                                            .size(8.0*size_factor)
                                            .horizontal_alignment(alignment::Horizontal::Center)
                                            .vertical_alignment(alignment::Vertical::Center)
                                            ).on_press(DPMessage::DayPressed(day))
                                    .height(15.0*size_factor)
                                    .width(15.0*size_factor)
                                    .padding(0)
                                    .style(move|theme: &Theme, status| {
                                            if day == selected_day {
                                                theme_success(theme, status, 10.0)
                                            } else {
                                                theme_primary(theme, status, 10.0)
                                            }}
                                        )
                                    .into();
                row.push(btn.map(move |message| Message::DatePicker(id, message)));
                
            }
        }
        calendar_days.push(Row::with_children(row).spacing(5.0*size_factor).into());
    }

    let col: Element<'static, Message, Theme, Renderer> = 
                    Column::with_children(calendar_days)
                                .align_items(Alignment::Start)
                                .width(Length::Fill)
                                .padding(0)
                                .into();
    col

}


fn create_day_row(size_factor: f32) -> Element<'static, Message, Theme, Renderer> {
    
    let days = DAYS.into_iter().map(|x| 
                                            Text::new(x.to_string())
                                            .size(8.0*size_factor)
                                            .into())
                                            .collect::<Vec<Element<'static, Message, Theme, Renderer>>>();

    Row::with_children(days).spacing(15.0*size_factor).width(Length::Fill).into()
}

fn create_select_row(id: usize, 
                    selected_format: String,
                    size_factor: f32) -> Element<'static, Message, Theme, Renderer> 
{

    let date_formats = DATE_FORMATS.into_iter().map(|x| x.to_string()).collect::<Vec<String>>();

    let close_text: Element<DPMessage, Theme, Renderer> = Text::new("Close").size(10.0*size_factor).into();
    
    let cl_button: Element<DPMessage, Theme, Renderer> = 
                                Button::new(close_text)
                                    .on_press(DPMessage::HideModal)
                                    .into();
                                
    let close_button: Element<Message, Theme, Renderer> = 
                        cl_button.map(move |message| Message::DatePicker(id, message));

    let picklist: Element<DPMessage, Theme, Renderer> = PickList::new(
                                                date_formats,
                                                Some(selected_format),
                                                DPMessage::DatePickerFormat)
                                            .text_size(8.0*size_factor)
                                            .placeholder("Choose format...")
                                            .into();
    
    let mapped_pl: Element<Message, Theme, Renderer> = 
                    picklist.map(move |message| app::Message::DatePicker(id, message));

    Row::with_children(vec![
        Row::with_children(vec![
            close_button,
            mapped_pl,    
        ]).width(Length::Fill).spacing(10.0*size_factor).into(),

    ]).into()  
}


fn create_submit_row(id: usize, size_factor: f32, selected_date: String) -> Element<'static, Message, Theme, Renderer> 
{
    let submit_text: Element<DPMessage, Theme, Renderer> = Text::new("Submit").size(10.0*size_factor).into();

    let submit_btn: Element<DPMessage, Theme, Renderer> = 
                                            Button::new(submit_text)
                                                    .on_press(DPMessage::OnSubmit)
                                                    .into();
    let submit_btn_mapped: Element<Message, Theme, Renderer> = 
                                submit_btn.map(move |message| app::Message::DatePicker(id, message));

    Row::new()
        .push(submit_btn_mapped)
        .push(Text::new(selected_date).size(10.0*size_factor))
        .width(Length::Fill)
        .spacing(10.0*size_factor)
        .into()
}


pub fn date_picker_update(id: usize, message: DPMessage) {

    match message {
        DPMessage::ShowModal => {
            // Non callback just sending the values.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.show = Some(true);
            wci.is_submitted = Some(false);
            let _ = get_set_widget_callback_data(wci);
        }
        DPMessage::HideModal => {
            // Non callback just sending the values.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.show = Some(false);
            let _ = get_set_widget_callback_data(wci);
        }
        DPMessage::DayPressed(day) => {
            // Non callback just sending the values.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.selected_day = Some(day);
            let _ = get_set_widget_callback_data(wci);
        }
        DPMessage::DatePickerFormat(date_format) => {
            // Non callback just sending the values.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.date_format = Some(date_format);
            let _ = get_set_widget_callback_data(wci);
        }
        DPMessage::MonthRightPressed(index) => {
            // Non callback just sending the values.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.index = Some(index);
            wci.is_submitted = Some(false);
            let _ = get_set_widget_callback_data(wci);
        }
        DPMessage::MonthLeftPressed(index) => {
            // Non callback just sending the values.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.index = Some(index);
            wci.is_submitted = Some(false);
            let _ = get_set_widget_callback_data(wci);
        }
        DPMessage::YearRightPressed => {
            // Non callback just sending the values.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.selected_year = Some(1);
            wci.is_submitted = Some(false);
            let _ = get_set_widget_callback_data(wci);
        }
        DPMessage::YearLeftPressed => {
            // Non callback just sending the values.
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.selected_year = Some(-1);
            wci.is_submitted = Some(false);
            let _ = get_set_widget_callback_data(wci);
        }
        DPMessage::OnSubmit => {
            let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
            wci.id = id;
            wci.is_submitted = Some(true);
            let mut wco = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_submit".to_string();
            process_callback(wco);
        }
    }
    

}


fn process_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("DatePicker callback could not be found with id {}", wco.id),
    };
                  
    Python::with_gil(|py| {
        if wco.user_data.is_some() {
            let user_data = match wco.user_data {
                Some(ud) => ud,
                None => panic!("DatePicker user_data in callback not found"),
            };
            let res = callback.call1(py, (
                                                                wco.id.clone(), 
                                                                wco.selected_date, 
                                                                user_data
                                                                ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("DatePicker: 3 parameters (id, value, user_data) are required or a python error in this function. {er}"),
            }
        } else {
            let res = callback.call1(py, (
                                                                wco.id.clone(),
                                                                wco.selected_date, 
                                                                ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("InputText: 2 parameters (id, value) are required or a python error in this function. {er}"),
            }
        } 
    });

    drop(app_cbs); 

}      


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgDatePickerParams {
    Label,
    Padding,
    SizeFactor,
    Show,
}

pub fn date_picker_item_update(dp: &mut IpgDatePicker,
                                item: PyObject,
                                value: PyObject,
                                )
{
    let update = try_extract_date_picker_update(item);

    match update {
        IpgDatePickerParams::Label => {
            dp.label = try_extract_string(value);
        },
        IpgDatePickerParams::Padding => {
            let pd = try_extract_vec_f64(value);
            dp.padding = get_padding(pd);
        },
        IpgDatePickerParams::SizeFactor => {
                dp.size_factor = try_extract_f64(value) as f32;
        },
        IpgDatePickerParams::Show => {
            dp.show = try_extract_boolean(value);
        },
    }
}

pub fn try_extract_date_picker_update(update_obj: PyObject) -> IpgDatePickerParams {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgDatePickerParams>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("DatePicker update extraction failed"),
        }
    })
}
