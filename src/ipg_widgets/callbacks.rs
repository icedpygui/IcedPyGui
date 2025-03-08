//! callbacks
#![allow(dead_code)]
use crate::IpgState;
use super::ipg_enums::IpgContainers;
use super::ipg_table::IpgTableMouse;
use super::{helpers::{format_date, MONTH_NAMES}, ipg_enums::IpgWidgets, ipg_radio::Choice};

use iced::{Color, Point};

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
    pub table_mouse: IpgTableMouse,
    pub date_format: Option<String>,
    pub show: Option<bool>,
    pub submit_str: Option<String>,
    pub value_float_64: Option<f64>,
    pub value_float_32: Option<f32>,
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
    pub on_modal_open: Option<bool>,
    pub points: Option<Vec<(String, f32)>>,
    pub scroll_pos: Vec<(String, f32)>,
    pub selected_index: Option<usize>,
    pub selected_label: Option<String>,
    pub selected_date: Option<String>,
    pub user_data: Option<PyObject>,
    pub button_user_data: Option<PyObject>,
    pub checkbox_user_data: Option<PyObject>,
    pub toggler_user_data: Option<PyObject>,
    pub scroller_user_data: Option<PyObject>,
    pub value_bool: Option<bool>,
    pub value_float: Option<f64>,
    pub value_str: Option<String>,
}

impl WidgetCallbackOut{}

pub fn set_or_get_widget_callback_data(state: &mut IpgState, wci: WidgetCallbackIn) -> WidgetCallbackOut                                     
{
    let widget_opt = state.widgets.get_mut(&wci.id);

    if widget_opt.is_some() {
        match widget_opt.unwrap() {
            IpgWidgets::IpgButton(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgButtonStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgCard(crd) => {
                let is_open = match wci.value_bool {
                    Some(open) => open,
                    None => panic!("Card is_open value not found"),
                };
                crd.is_open = is_open;
                return WidgetCallbackOut{
                     ..Default::default()
                    }
            },
            IpgWidgets::IpgCardStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgCheckBox(cbox) => {
                cbox.is_checked = match wci.on_toggle {
                    Some(data) => data,
                    None => panic!("Checkbox is_checked not found")
                };
                return WidgetCallbackOut{
                    ..Default::default()
                } 
            },
            IpgWidgets::IpgCheckboxStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgColorPicker(cp) => {
                cp.show = match wci.value_bool {
                    Some(s) => s,
                    None => panic!("The open value for color_picker could not be found"),
                };

                if wci.color.is_some() {
                    let color = match wci.color {
                        Some(c) => c,
                        None => panic!("The color value for color_picker could not be found"),
                    };
                    cp.color = Color::from_rgba(color[0] as f32, color[1] as f32, 
                                            color[2] as f32, color[3] as f32);
                }
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgColorPickerStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgContainerStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
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
                    dp.selected_year += yr;             
                }

                if wci.date_format.is_some() {
                    dp.selected_format = wci.date_format.unwrap();
                }
                dp.selected_date = 
                    format_date(
                        dp.selected_format.clone(), 
                        dp.selected_year, 
                        dp.selected_month_index, 
                        dp.selected_day
                        );
                
                if wci.is_submitted.is_some() {
                    dp.is_submitted = wci.is_submitted.unwrap();
                }
                if wci.show.is_some() {
                    dp.show_calendar = wci.show.unwrap();
                };
                return WidgetCallbackOut{
                    selected_date: Some(dp.selected_date.clone()),
                    ..Default::default()
                }
            },
            IpgWidgets::IpgImage(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            // IpgWidgets::IpgMenu(menu) => {
            //     let mut wco = WidgetCallbackOut::default();
            //     if wci.is_checked.is_some() {
            //         menu.is_checked = wci.is_checked.unwrap();
            //         wco.is_checked = wci.is_checked;
            //     }
            //     if wci.on_toggle.is_some() {
            //         menu.is_toggled = wci.on_toggle.unwrap();
            //         wco.on_toggle = wci.on_toggle;
            //     }
            //     wco.user_data = menu.user_data.clone();
            //     return wco
            // },
            // IpgWidgets::IpgMenuStyle(_) => {
            //     return WidgetCallbackOut{
            //         ..Default::default()
            //     }
            // },
            // IpgWidgets::IpgMenuBarStyle(_) => {
            //     return WidgetCallbackOut{
            //         ..Default::default()
            //     }
            // },
            // IpgWidgets::IpgMenuSeparatorStyle(_) => {
            //     return WidgetCallbackOut{
            //         ..Default::default()
            //     }
            // },
            IpgWidgets::IpgOpaqueStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgPickList(pl) => {
                pl.selected = wci.value_str;
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgPickListStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgProgressBar(_) => {
                return WidgetCallbackOut::default()
            },
            IpgWidgets::IpgProgressBarStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgRadio(_) => {
                return WidgetCallbackOut::default()
            },
            IpgWidgets::IpgRadioStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgRule(_) => {
                return WidgetCallbackOut::default()
            },
            IpgWidgets::IpgRuleStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgScrollableStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgSelectableText(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgSeparator(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgSeparatorStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgSlider(slider) => {
                // Do on_change if something
                if wci.value_float_64.is_some() {
                    slider.value = match wci.value_float_64 {
                        Some(v) => v as f32,
                        None => panic!("Slider submit value could not be found"),
                    };
                }
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgSliderStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgSpace(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgSvg(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgText(_) => {
                return WidgetCallbackOut::default()
            },
            // IpgWidgets::IpgTextEditor(_) => {
            //     let wco = WidgetCallbackOut::default();
            //     wco
            // },
            IpgWidgets::IpgTextInput(ti) => {
                ti.value = wci.value_str.unwrap();
                return WidgetCallbackOut{
                    value_str: Some(ti.value.clone()),
                    ..Default::default()
                }
            },
            IpgWidgets::IpgTextInputStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgTimer(tim) => {
                tim.counter += 1;
                // value_str is set when a tick occurs
                // so no value_bool present
                if wci.value_str.is_none() {
                    tim.started = wci.value_bool.unwrap();
                }
                return WidgetCallbackOut{
                    counter: Some(tim.counter),
                    duration: Some(tim.duration_ms),
                    value_bool: Some(tim.started),
                    ..Default::default()
                }
            },
            IpgWidgets::IpgTimerStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgCanvasTimer(ctim) => {
                ctim.counter += 1;
                // value_str is set when a tick occurs
                // so no value_bool present
                if wci.value_str.is_none() {
                    ctim.started = wci.value_bool.unwrap();
                }
                return WidgetCallbackOut{
                    counter: Some(ctim.counter),
                    duration: Some(ctim.duration_ms),
                    value_bool: Some(ctim.started),
                    ..Default::default()
                }
            },
            IpgWidgets::IpgCanvasTimerStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgToggler(tog) => {
                if let Some(tg) = wci.on_toggle { tog.is_toggled = tg }
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgTogglerStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgMenuStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
            IpgWidgets::IpgMenuBarStyle(_) => {
                return WidgetCallbackOut{
                    ..Default::default()
                }
            },
        }
    } else {

        let container_opt = state.containers.get_mut(&wci.id);
        if container_opt.is_some() {
            match container_opt.unwrap() {
                // IpgContainers::IpgModal(modal) => {
                //     modal.show = true;
                //     return WidgetCallbackOut{
                //         user_data: modal.user_data.clone(),
                //         ..Default::default()
                //     }
                // },
                IpgContainers::IpgTable(tbl) => {
                    match wci.table_mouse {
                        IpgTableMouse::None => (),
                        IpgTableMouse::Resizing => {
                            tbl.resize_offset[wci.index.unwrap()] = wci.value_float_32;
                        },
                        IpgTableMouse::Resized => {
                            let mut not_none_index = 0;
                            tbl.resize_offset
                                .iter_mut()
                                .enumerate()
                                .for_each(|(index, offset)| {
                                    if let Some(offset) = offset.take() {
                                        tbl.column_widths[index] += offset;
                                        not_none_index = index;
                                    }
                                });
                            tbl.resize_offset[not_none_index] = None;
                            
                        },
                    }
                    
                    return WidgetCallbackOut{
                        ..Default::default()
                    }
                },
                _ => panic!("container not found")
            }
        }
    }
    panic!("get_set_wci: id {} not found", wci.id)
    
}


pub fn container_callback_data(state: &mut IpgState, wci: WidgetCallbackIn) -> WidgetCallbackOut {

    let container_type_opt = state.containers.get_mut(&wci.id);

    let container_type = match container_type_opt {
        Some(cont) => cont,
        None => panic!("Container with id {} could not be found", wci.id),
    };
    
    match container_type {
        IpgContainers::IpgCanvas(_) => {
            WidgetCallbackOut::default()
        },
        IpgContainers::IpgMouseArea(_) => {
            WidgetCallbackOut{..Default::default()}
        },
        IpgContainers::IpgTable(_) => {
            WidgetCallbackOut{
                ..Default::default()
            }
        }
        IpgContainers::IpgScrollable(_) => {
            WidgetCallbackOut{..Default::default()}
        }
        _ => {
            WidgetCallbackOut::default()
        }
    }
        
}
