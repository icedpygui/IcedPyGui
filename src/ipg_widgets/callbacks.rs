
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
    pub increment_value: Option<i8>,
    pub is_submitted: Option<bool>,
    pub on_toggle: Option<bool>,
    pub is_checked: Option<bool>,
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
    pub bar_index: Option<usize>,
    pub menu_index: Option<usize>,
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
    
    if widget_opt.is_some() {
        match widget_opt.unwrap() {
            IpgWidgets::IpgButton(btn) => {
                let mut wco = WidgetCallbackOut::default();
                wco.user_data = btn.user_data.clone();
                drop(state);
                return wco
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
                    dp.selected_day = wci.selected_day.unwrap();
                }
                // month index
                if wci.index.is_some() {
                    let increment = wci.increment_value.unwrap();
                    let index = wci.index.unwrap();
                    if index == 12 && increment == 1 {
                        dp.selected_month_index = 1
                    } else if index == 1 && increment == -1 {
                        dp.selected_month_index = 12;
                    } else if increment == -1 {
                        dp.selected_month_index -= 1;
                    } else {
                        dp.selected_month_index += 1;
                    }
                    
                    dp.selected_month = MONTH_NAMES[dp.selected_month_index].to_string();
                }
                
                if wci.selected_year.is_some() {
                    let yr = wci.selected_year.unwrap();
                    dp.selected_year = yr + dp.selected_year;             
                }

                if wci.date_format.is_some() {
                    dp.selected_format = wci.date_format.unwrap();
                }
                dp.selected_date = format_date(
                                                dp.selected_format.clone(), 
                                                dp.selected_year, 
                                                dp.selected_month_index, 
                                                dp.selected_day
                                                );
                
                if wci.is_submitted.is_some() {
                    dp.is_submitted = wci.is_submitted.unwrap();
                }
                if wci.show.is_some() {
                    dp.show = wci.show.unwrap();
                };
                let mut wco = WidgetCallbackOut::default();
                wco.selected_date = Some(dp.selected_date.clone());
                wco.user_data = dp.user_data.clone();
                drop(state);
                return wco
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
                return wco
            },
            IpgWidgets::IpgMenu(menu) => {
                let mut wco = WidgetCallbackOut::default();
                if wci.is_checked.is_some() {
                    menu.is_checked = wci.is_checked.unwrap();
                    wco.is_checked = wci.is_checked;
                }
                if wci.on_toggle.is_some() {
                    menu.is_toggled = wci.on_toggle.unwrap();
                    wco.on_toggle = wci.on_toggle;
                }
                wco.user_data = menu.user_data.clone();
                return wco
            },
            IpgWidgets::IpgPickList(pl) => {
                pl.selected = wci.value_str;
                let mut wco = WidgetCallbackOut::default();
                wco.user_data = pl.user_data.clone();
                drop(state);
                return wco
            },
            IpgWidgets::IpgProgressBar(_) => {
                return WidgetCallbackOut::default()
            },
            IpgWidgets::IpgRadio(_) => {
                return WidgetCallbackOut::default()
            },
            IpgWidgets::IpgRule(_) => {
                return WidgetCallbackOut::default()
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
                return WidgetCallbackOut::default();
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
                return wco
            },
            IpgWidgets::IpgTable(tbl) => {
                let mut wco = WidgetCallbackOut::default();
                wco.user_data = tbl.user_data.clone();
                let (row_index, col_index) = if wci.index_table.is_some() {
                        wci.index_table.unwrap()
                } else {
                    return wco;
                };

                if wci.value_str == Some("checkbox".to_string()) {
                    let mut found_idx: Option<usize> = None;
                    for (idx, (_id, row, col, _bl)) in tbl.check_ids.iter().enumerate() {
                        // dbg!(row, col, _bl, &wci.on_toggle);
                        if col_index != *col {
                            break;
                        }
                        if row_index == *row {
                            found_idx = Some(idx);
                            break;
                        }
                    }
                    if found_idx.is_some() {
                        dbg!("here found");
                        tbl.check_ids[found_idx.unwrap()].3 = wci.on_toggle.unwrap();
                        return wco;
                    }
                }

                if wci.value_str == Some("toggler".to_string()) {
                    let mut found_idx: Option<usize> = None;
                    for (idx, (_id, row, col, _bl)) in tbl.toggler_ids.iter().enumerate() {
                        if col_index != *col {
                            break;
                        }
                        if row_index == *row {
                            found_idx = Some(idx);
                            break;
                        }
                    }
                    if found_idx.is_some() {
                        tbl.toggler_ids[found_idx.unwrap()].3 = wci.on_toggle.unwrap();
                        return wco;
                    }
                }
                
                return wco
            },
            IpgWidgets::IpgText(_) => {
                return WidgetCallbackOut::default()
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
                return wco
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
                return wco
            }
            IpgWidgets::IpgToggler(tog) => {
                match wci.on_toggle {
                    Some(tg) => tog.is_toggled = tg,
                    None => (),
                }
                let mut wco = WidgetCallbackOut::default();
                wco.user_data = tog.user_data.clone();
                drop(state);
                return wco
            },
        }
    } else {
        let mut state = access_state();

        let container_opt = state.containers.get_mut(&wci.id);
        if container_opt.is_some() {
            match container_opt.unwrap() {
                IpgContainers::IpgColumn(_) => todo!(),
                IpgContainers::IpgContainer(_) => todo!(),
                IpgContainers::IpgModal(modal) => {
                    let mut wco = WidgetCallbackOut::default();
                    modal.show = true;
                    wco.user_data = modal.user_data.clone();
                    drop(state);
                    return wco
                },
                IpgContainers::IpgMouseArea(_) => todo!(),
                IpgContainers::IpgRow(_) => todo!(),
                IpgContainers::IpgScrollable(_) => todo!(),
                IpgContainers::IpgToolTip(_) => todo!(),
                IpgContainers::IpgWindow(_) => todo!(),
            }
        }
    }
    panic!("get_set_wci: id {} not found", wci.id)
    
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
            drop(state);
            return wco
        },
        IpgContainers::IpgContainer(_) => {
            let wco = WidgetCallbackOut::default();
            drop(state);
            return wco
        },
        IpgContainers::IpgMouseArea(m_area) => {
            let mut wco = WidgetCallbackOut::default();
            wco.user_data = m_area.user_data.clone();
            drop(state);
            return wco
        },
        IpgContainers::IpgModal(_modal) => {
            let wco = WidgetCallbackOut::default();
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
            drop(state);
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
            drop(state);
            return wco
        },
        IpgContainers::IpgWindow(_) => {
            let wco = WidgetCallbackOut::default();
            drop(state);
            return wco
        },
    }
        
}
