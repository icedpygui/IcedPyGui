

use std::collections::HashMap;

use crate::access_state;

use super::{helpers::{format_date, MONTH_NAMES}, ipg_enums::IpgWidgets, ipg_radio::Choice};

use iced::{Color, Point};

use pyo3::PyObject;


#[derive(Default, Debug)]
pub struct WidgetCallbackIn {
    pub id: usize,
    pub choice: Option<Choice>,
    pub color: Option<Vec<f64>>,
    pub index: Option<usize>,
    pub is_submitted: Option<bool>,
    pub on_toggle: Option<bool>,
    pub point: Option<Point>,
    pub selected: Option<String>,
    pub selected_day: Option<usize>,
    pub selected_date: Option<String>,
    pub selected_month: Option<String>,
    pub selected_year: Option<i32>,
    pub show: Option<bool>,
    pub submit_str: Option<String>,
    pub value_float: Option<f64>,
    pub value_str: Option<String>,
}

impl WidgetCallbackIn{}

#[derive(Default, Debug)]
pub struct WidgetCallbackOut {
    pub id: usize,
    pub color: Option<Vec<f64>>,
    pub is_checked: Option<bool>,
    pub event_name: Option<String>,
    pub points: Option<HashMap<String, f32>>,
    pub user_data: Option<PyObject>, 
    pub selected_index: Option<usize>,
    pub selected_label: Option<String>,
    pub selected_date: Option<String>,
    pub value_float: Option<f64>,
    pub value_str: Option<String>,
}

impl WidgetCallbackOut{}

pub fn get_set_widget_callback_data(wci: WidgetCallbackIn) -> WidgetCallbackOut
                                            
{
    let mut state = access_state();

    let widget_opt = state.widgets.get_mut(&wci.id);
    
    match widget_opt 
    {
        Some(widget) => 
            match widget {
                IpgWidgets::IpgButton(btn) => {
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = btn.user_data.clone();
                    drop(state);
                    return wco
                },
                IpgWidgets::IpgCard(crd) => {
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = crd.user_data.clone();
                    drop(state);
                    return wco
                },
                IpgWidgets::IpgCheckBox(cbox) => {
                    cbox.is_checked = match wci.on_toggle {
                        Some(data) => data,
                        None => panic!("Checkbox is_checked not found")
                    };
                    let mut wco = WidgetCallbackOut::default(); 
                    wco.is_checked = Some(cbox.is_checked);
                    wco.user_data = cbox.user_data.clone();
                    drop(state);
                    return wco
                },
                IpgWidgets::IpgColorPicker(cp) => {
                    cp.show = match wci.show {
                        Some(s) => s,
                        None => panic!("The show value for color_picker could not be found"),
                    };

                    let mut wco = WidgetCallbackOut::default();
                    if wci.color.is_some() {
                        let color = match wci.color {
                            Some(c) => c,
                            None => panic!("The color value for color_picker could not be found"),
                        };
                        wco.color = Some(color.clone());
                        cp.color = Color::from_rgba(color[0] as f32, color[1] as f32, 
                                                color[2] as f32, color[3] as f32);
                    }
                    wco.user_data = cp.user_data.clone();
                    drop(state);
                    return wco
                },
                IpgWidgets::IpgDatePicker(dp) => {
                    if wci.index.is_some() {
                        let index = match wci.index {
                                            Some(idx) => idx,
                                            None => panic!("Index not found")
                                        };
                        if index == 12 {
                            dp.selected_month_index = 1
                        } else {
                            dp.selected_month_index += 1; 
                        }
                        dp.selected_month = MONTH_NAMES[index].to_string();
                    }
                    
                    if wci.selected_year.is_some() {
                        dp.selected_year = match wci.selected_year {
                                            Some(yr) => yr + dp.selected_year,
                                            None => panic!("Selected year not found")
                                        };
                    }
                    if dp.selected_date != "".to_string() {
                        dp.selected_date = format_date(dp.selected_format.clone(), 
                                                dp.selected_year, 
                                                dp.selected_month_index, 
                                                dp.selected_day);
                    }
                    
                    if wci.is_submitted.is_some() {
                        dp.is_submitted = match wci.is_submitted {
                            Some(is_sub) => is_sub,
                            None => panic!("is_submitted not found")
                        }
                    }
                    let mut wco = WidgetCallbackOut::default();
                    wco.selected_date = Some(dp.selected_date.clone());
                    wco.user_data = dp.user_data.clone();
                    drop(state);
                    return wco
                },
                IpgWidgets::IpgImage(img) => {
                    let mut points: HashMap<String, f32> = HashMap::new();
                    if wci.point.is_some() {
                        match wci.point {
                            Some(pt) => {
                            points.insert("x".to_string(), pt.x);
                            points.insert("y".to_string(), pt.y);
                        },
                            None => panic!("Point could not be found")
                        }
                    }
                    
                    let mut wco = WidgetCallbackOut::default();
                    wco.points = Some(points);
                    wco.user_data = img.user_data.clone();
                    drop(state);
                    return wco
                },
                IpgWidgets::IpgMenuBar(_) => {
                    let wco = WidgetCallbackOut::default();
                    return wco
                },
                IpgWidgets::IpgMenuItem(_) => {
                    let wco = WidgetCallbackOut::default();
                    return wco
                },
                IpgWidgets::IpgPickList(pl) => {
                    pl.selected = wci.selected;
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = pl.user_data.clone();
                    drop(state);
                    return wco
                },
                IpgWidgets::IpgProgressBar(_) => {
                    let wco = WidgetCallbackOut::default();
                    return wco
                },
                IpgWidgets::IpgRadio(radio) => {
                    let mut selected_index = 0;
                    for (i, choice) in  Choice::into_iter().enumerate() {
                        if Some(choice) == wci.choice {
                            selected_index = i;
                            break;
                        }
                    }
                    radio.selected = wci.choice;
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = radio.user_data.clone();
                    wco.selected_label = Some(radio.labels[selected_index].clone());
                    wco.selected_index = Some(selected_index);
                    drop(state);
                    return wco
                },
                IpgWidgets::IpgSelectableText(st) => {
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = st.user_data.clone();
                    drop(state);
                    return wco
                },
                IpgWidgets::IpgSlider(slider) => {
                    // Do on_change if something
                    if wci.value_float.is_some() {
                        slider.value = match wci.value_float {
                            Some(v) => v as f32,
                            None => panic!("Slider submit value could not be found"),
                        };
                    }
                    let mut wco = WidgetCallbackOut::default();
                    wco.value_float = Some(slider.value.clone() as f64);
                    wco.user_data = slider.user_data.clone();
                    drop(state);
                    return wco
                },
                IpgWidgets::IpgSpace(_) => {
                    let wco = WidgetCallbackOut::default();
                    return wco
                },
                IpgWidgets::IpgTable(_) => {
                    let wco = WidgetCallbackOut::default();
                    return wco
                },
                IpgWidgets::IpgText(_) => {
                    let wco = WidgetCallbackOut::default();
                    return wco
                },
                IpgWidgets::IpgTextEditor(_) => {
                    let wco = WidgetCallbackOut::default();
                    return wco
                },
                IpgWidgets::IpgTextInput(input) => {
                    // During the input, the widget is assigned the value so that it shows
                    // during typing.  On submit, the text box is cleared, so no value.
                    // However, in both cases the value is passed to the callback.
                    match wci.value_str {
                        Some(v) => input.value = v,
                        None => input.value = "".to_string()
                    };
                    let mut wco = WidgetCallbackOut::default();
                    if wci.submit_str.is_some() {
                        wco.value_str = wci.submit_str;
                    }
                    wco.user_data = input.user_data.clone();
                    drop(state);
                    return wco
                },
            },
    
    None => panic!("Widget with id {} not found", wci.id)
    }
}