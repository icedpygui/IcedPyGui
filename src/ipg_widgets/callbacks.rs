
use crate::access_state;
use super::ipg_enums::IpgContainers;
use super::{helpers::{format_date, MONTH_NAMES}, ipg_enums::IpgWidgets, ipg_radio::Choice};

use iced::Point;

use pyo3::PyObject;


#[derive(Default, Debug)]
pub struct WidgetCallbackIn {
    pub id: usize,
    pub choice: Option<Choice>,
    pub choice_index: Option<usize>,
    pub color: Option<Vec<f64>>,
    pub counter: Option<u64>,
    pub index: Option<usize>,
    pub index_table: Option<(usize, usize)>,
    pub is_submitted: Option<bool>,
    pub on_toggle: Option<bool>,
    pub point: Option<Point>,
    pub selected: Option<String>,
    pub selected_index: Option<usize>,
    pub selected_day: Option<usize>,
    pub selected_date: Option<String>,
    pub selected_month: Option<String>,
    pub selected_year: Option<i32>,
    pub started: Option<bool>,
    pub ticking: Option<bool>,
    pub date_format: Option<String>,
    pub show: Option<bool>,
    pub submit_str: Option<String>,
    pub value_float: Option<f64>,
    pub value_str: Option<String>,
    pub value_bool: Option<bool>,
    pub value_usize: Option<usize>,
    pub on_tick_count: f32,
}

impl WidgetCallbackIn{}

#[derive(Default, Debug)]
pub struct WidgetCallbackOut {
    pub id: usize,
    pub color: Option<Vec<f64>>,
    pub duration: Option<u64>,
    pub counter: Option<u64>,
    pub event_name: String,
    pub is_checked: Option<bool>,
    pub index: Option<usize>,
    pub index_table: Option<(usize, usize)>,
    pub on_toggle: Option<bool>,
    pub points: Option<Vec<(String, f32)>>,
    pub scroll_pos: Vec<(String, f32)>, 
    pub selected_index: Option<usize>,
    pub selected_label: Option<String>,
    pub selected_date: Option<String>,
    pub user_data: Option<PyObject>,
    pub value_bool: Option<bool>,
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
                    wco
                },
                IpgWidgets::IpgCard(crd) => {
                    let is_open = match wci.value_bool {
                        Some(open) => open,
                        None => panic!("Card is_open value not found"),
                    };
                    crd.is_open = is_open;
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = crd.user_data.clone();
                    drop(state);
                    wco
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
                    wco
                },
                // IpgWidgets::IpgColorPicker(cp) => {
                //     cp.open = match wci.value_bool {
                //         Some(s) => s,
                //         None => panic!("The open value for color_picker could not be found"),
                //     };

                //     let mut wco = WidgetCallbackOut::default();
                //     if wci.color.is_some() {
                //         let color = match wci.color {
                //             Some(c) => c,
                //             None => panic!("The color value for color_picker could not be found"),
                //         };
                //         wco.color = Some(color.clone());
                //         cp.color = Color::from_rgba(color[0] as f32, color[1] as f32, 
                //                                 color[2] as f32, color[3] as f32);
                //     }
                //     wco.user_data = cp.user_data.clone();
                //     drop(state);
                //     wco
                // },
                IpgWidgets::IpgDatePicker(dp) => {
                    
                    if wci.selected_day.is_some() {
                        dp.selected_day = match wci.selected_day {
                            Some(day) => day,
                            None => panic!("DatePicker could not find selected_day"),
                        };
                    }
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
                    if wci.date_format.is_some() {
                        dp.selected_format = match wci.date_format {
                            Some(format) => format,
                            None => panic!("DatePicker selected_format could not be found."),
                        };
                    }
                    dp.selected_date = format_date(
                                                    dp.selected_format.clone(), 
                                                    dp.selected_year, 
                                                    dp.selected_month_index, 
                                                    dp.selected_day
                                                    );
                    
                    
                    if wci.is_submitted.is_some() {
                        dp.is_submitted = match wci.is_submitted {
                            Some(is_sub) => is_sub,
                            None => panic!("DatePicker is_submitted not found")
                        }
                    }
                    if wci.show.is_some() {
                        dp.show = match wci.show {
                            Some(sh) => sh,
                            None => panic!("DatePicker show is not found"),
                        }
                    };
                    let mut wco = WidgetCallbackOut::default();
                    wco.selected_date = Some(dp.selected_date.clone());
                    wco.user_data = dp.user_data.clone();
                    drop(state);
                    wco
                },
                IpgWidgets::IpgImage(img) => {
                    let mut points: Vec<(String, f32)> = vec![];
                    if wci.point.is_some() {
                        match wci.point {
                            Some(pt) => {
                            points.push(("x".to_string(), pt.x));
                            points.push(("y".to_string(), pt.y));
                        },
                            None => panic!("Image Point could not be found")
                        }
                    }
                    
                    let mut wco = WidgetCallbackOut::default();
                    wco.points = Some(points);
                    wco.user_data = img.user_data.clone();
                    drop(state);
                    wco
                },
                IpgWidgets::IpgMenu(menu) => {
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = menu.user_data.clone();
                    wco
                },
                IpgWidgets::IpgPickList(pl) => {
                    pl.selected = wci.value_str;
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = pl.user_data.clone();
                    drop(state);
                    wco
                },
                IpgWidgets::IpgProgressBar(_) => {
                    WidgetCallbackOut::default()
                },
                IpgWidgets::IpgRadio(_) => {
                    WidgetCallbackOut::default()
                },
                IpgWidgets::IpgRule(_) => {
                    WidgetCallbackOut::default()
                },
                IpgWidgets::IpgSelectableText(st) => {
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = st.user_data.clone();
                    drop(state);
                    wco
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
                    wco
                },
                IpgWidgets::IpgSpace(_) => {
                    let wco = WidgetCallbackOut::default();
                    wco
                },
                IpgWidgets::IpgSvg(isvg) => {
                    let mut points: Vec<(String, f32)> = vec![];
                    if wci.point.is_some() {
                        match wci.point {
                            Some(pt) => {
                            points.push(("x".to_string(), pt.x));
                            points.push(("y".to_string(), pt.y));
                        },
                            None => panic!("Svg Point could not be found")
                        }
                    }
                    
                    let mut wco = WidgetCallbackOut::default();
                    wco.points = Some(points);
                    wco.user_data = isvg.user_data.clone();
                    drop(state);
                    wco
                },
                IpgWidgets::IpgTable(tbl) => {
                    if wci.value_str == Some("checkbox".to_string()) {
                        let (col_index, row_index) = wci.index_table.unwrap();
                        let on_toggles = tbl.on_toggled.as_mut().unwrap();
                        let on_togged = on_toggles.get_mut(&col_index).unwrap();
                        on_togged[row_index] = wci.on_toggle.unwrap();
                    }
                    
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = tbl.user_data.clone();
                    wco
                },
                IpgWidgets::IpgText(_) => {
                    let wco = WidgetCallbackOut::default();
                    wco
                },
                // IpgWidgets::IpgTextEditor(_) => {
                //     let wco = WidgetCallbackOut::default();
                //     wco
                // },
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
                    wco
                },
                IpgWidgets::IpgTimer(tim) => {
                    match wci.started {
                        Some(st) => tim.started = st,
                        None => (),
                    }
                    match wci.counter {
                        Some(ct) => {
                            if ct == 0 {
                                tim.counter = 0;
                            } else {
                                tim.counter += ct
                            }
                        },
                        None => (),
                    }
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = tim.user_data.clone();
                    wco.counter = Some(tim.counter);
                    wco.duration = Some(tim.duration_ms);
                    drop(state);
                    wco
                }
                IpgWidgets::IpgToggler(tog) => {
                    match wci.on_toggle {
                        Some(tg) => tog.is_toggled = tg,
                        None => (),
                    }
                    let mut wco = WidgetCallbackOut::default();
                    wco.user_data = tog.user_data.clone();
                    drop(state);
                    wco
                },
            },
    
    None => panic!("Widget with id {} not found", wci.id)
    }
}


pub fn get_set_container_callback_data(wci: WidgetCallbackIn) -> WidgetCallbackOut {

    let mut state = access_state();

    let container_type_opt = state.containers.get_mut(&wci.id);

    let container_type = match container_type_opt {
        Some(cont) => cont,
        None => panic!("Container with id {} could not be found", wci.id),
    };
    
    match container_type {
        IpgContainers::IpgColumn(_) => {
            let wco = WidgetCallbackOut::default();
            return wco
        },
        IpgContainers::IpgContainer(_) => {
            let wco = WidgetCallbackOut::default();
            return wco
        },
        IpgContainers::IpgMouseArea(m_area) => {
            let mut wco = WidgetCallbackOut::default();
            wco.user_data = m_area.user_data.clone();
            drop(state);
            return wco
        },
        // IpgContainers::IpgPaneGrid(_) => {
        //     let wco = WidgetCallbackOut::default();
        //     return wco
        // },
        // IpgContainers::IpgPane(_) => {
        //     let wco = WidgetCallbackOut::default();
        //     return wco
        // },
        IpgContainers::IpgRow(_) => {
            let wco = WidgetCallbackOut::default();
            return wco
        },
        IpgContainers::IpgScrollable(scroll) => {
            let mut wco = WidgetCallbackOut::default();
            wco.user_data = scroll.user_data.clone();
            drop(state);
            return wco
        }
        IpgContainers::IpgToolTip(_) => {
            let wco = WidgetCallbackOut::default();
            return wco
        },
        IpgContainers::IpgWindow(_) => {
            let wco = WidgetCallbackOut::default();
            return wco
        },
    }
        
}
