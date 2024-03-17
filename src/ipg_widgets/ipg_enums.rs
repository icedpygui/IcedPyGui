#![allow(unused)]

use core::panic;

use super::ipg_button::IpgButton;
use super::ipg_card::IpgCard;
use super::ipg_checkbox::IpgCheckBox;
use super::ipg_color_picker::IpgColorPicker;
use super::ipg_container::IpgContainer;
use super::ipg_column::IpgColumn;
use super::ipg_date_picker::IpgDatePicker;
use super::ipg_image::IpgImage;
use super::ipg_menu::{IpgMenuBar, IpgMenuItem};
use super::ipg_pane_grid::{IpgPaneGrid, IpgPane};
use super::ipg_pick_list::IpgPickList;
use super::ipg_progress_bar::IpgProgressBar;
use super::ipg_radio::{IpgRadio, Choice};
use super::ipg_row::IpgRow;
use super::ipg_scrollable::IpgScrollable;
use super::ipg_selectable_text::IpgSelectableText;
use super::ipg_slider::IpgSlider;
use super::ipg_space::IpgSpace;
use super::ipg_table::IpgTable;
use super::ipg_text::IpgText;
use super::ipg_text_editor::IpgTextEditor;
use super::ipg_text_input::IpgTextInput;
use super::ipg_tool_tip::IpgToolTip;
use super::ipg_window::IpgWindow;

use crate::access_state;

use iced::Color;

use pyo3::PyObject;

#[derive(Debug)]
pub enum IpgContainers {
    IpgColumn(IpgColumn),
    IpgContainer(IpgContainer),
    IpgPaneGrid(IpgPaneGrid),
    IpgPane(IpgPane),
    IpgRow(IpgRow),
    IpgScrollable(IpgScrollable),
    IpgToolTip(IpgToolTip),
    IpgWindow(IpgWindow),
}

#[derive(Debug)]
pub enum IpgWidgets {
    IpgButton(IpgButton),
    IpgCard(IpgCard),
    IpgCheckBox(IpgCheckBox),
    IpgColorPicker(IpgColorPicker),
    IpgDatePicker(IpgDatePicker),
    IpgImage(IpgImage),
    IpgMenuBar(IpgMenuBar),
    IpgMenuItem(IpgMenuItem),
    IpgPickList(IpgPickList),
    IpgProgressBar(IpgProgressBar),
    IpgRadio(IpgRadio),
    IpgSelectableText(IpgSelectableText),
    IpgSlider(IpgSlider),
    IpgSpace(IpgSpace),
    IpgTable(IpgTable),
    IpgText(IpgText),
    IpgTextEditor(IpgTextEditor),
    IpgTextInput(IpgTextInput),
}

pub fn get_set_widget_data(id: usize, 
                            data_bool:Option<bool>,
                            data_str: Option<String>,
                            data_vec64: Option<Vec<f64>>,
                            data_choice: Option<Choice>,
                            ) -> (
                                    Option<String>, 
                                    Option<PyObject>, 
                                    Option<String>,
                                    Option<f64>,
                                    Option<bool>,
                                    )
{

    let mut state = access_state();

    let widget_opt = state.widgets.get_mut(&id);
    
    match widget_opt 
    {
        Some(widget) => 
            match widget {
                IpgWidgets::IpgButton(btn) => {
                    let user_data = btn.user_data.clone();
                    let cb_name = btn.cb_name.clone();
                    drop(state);
                    return (cb_name, user_data, None, None, None)
                },
                IpgWidgets::IpgCard(crd) => {
                    let user_data = crd.user_data.clone();
                    let cb_name = crd.cb_name.clone();
                    drop(state);
                    return (cb_name, user_data, None, None, None)
                },
                IpgWidgets::IpgCheckBox(cbox) => {
                    cbox.is_checked = match data_bool {
                        Some(data) => data,
                        None => panic!("Checkbox is_checked not found")
                    };
                     
                    let user_data = cbox.user_data.clone();
                    let cb_name = cbox.cb_name.clone();
                    drop(state);
                    return (cb_name, user_data, None, None, None)
                },
                IpgWidgets::IpgColorPicker(cp) => {
                    cp.show = match data_bool {
                        Some(s) => s,
                        None => panic!("The show value for color_picker could not be found"),
                    };

                    if data_vec64.is_some() {
                        let color = match data_vec64 {
                            Some(c) => c,
                            None => panic!("The color value for color_picker could not be found"),
                        };

                        cp.color = Color::from_rgba(color[0] as f32, color[1] as f32, 
                                                color[2] as f32, color[3] as f32);
                    }
                    
                    let user_data = cp.user_data.clone();
                    let cb_name = cp.cb_name.clone();
                    drop(state);
                    return (cb_name, user_data, None, None, None)
                },
                IpgWidgets::IpgDatePicker(_) => {
                    return (None, None, None, None, None)
                },
                IpgWidgets::IpgImage(img) => {
                    let user_data = img.user_data.clone();
                    drop(state);
                    return (None, user_data, None, None, None)
                },
                IpgWidgets::IpgMenuBar(_) => {
                    return (None, None, None, None, None)
                },
                IpgWidgets::IpgMenuItem(_) => {
                    return (None, None, None, None, None)
                },
                IpgWidgets::IpgPickList(pl) => {
                    pl.selected = data_str;
                    let user_data = pl.user_data.clone();
                    let cb_name = pl.cb_name.clone();
                    drop(state);
                    return (cb_name, user_data, None, None, None)
                },
                IpgWidgets::IpgProgressBar(_) => {
                    return (None, None, None, None, None)
                },
                IpgWidgets::IpgRadio(radio) => {
                    let mut selected_index = 0;

                    for (i, choice) in  Choice::into_iter().enumerate() {

                        if Some(choice) == data_choice {
                            selected_index = i;
                            break;
                        }
                    }
                    radio.selected = data_choice;
                    let user_data = radio.user_data.clone();
                    let cb_name = radio.cb_name.clone();
                    let selected_label = Some(radio.labels[selected_index].clone());

                    drop(state);

                    return (cb_name, user_data, selected_label, None, None)
                },
                IpgWidgets::IpgSelectableText(st) => {
                    let user_data = st.user_data.clone();
        
                    drop(state);
                    return (None, user_data, None, None, None)
                },
                IpgWidgets::IpgSlider(slider) => {
                    let mut cb_name: Option<String> = None;
                    // Do on_chnage if something
                    if data_vec64.is_some() {
                        slider.value = match data_vec64 {
                            Some(v) => v[0] as f32,
                            None => panic!("Slider submit value could not be found"),
                        };
                        cb_name = slider.cb_name_change.clone(); 
                    } else {
                        cb_name = slider.cb_name_release.clone();
                    }
                    let value = slider.value.clone() as f64;
                    let user_data = slider.user_data.clone();
                    
                    drop(state);
                    return (cb_name, user_data, None, Some(value), None)
                },
                IpgWidgets::IpgSpace(_) => {
                    return (None, None, None, None, None)
                },
                IpgWidgets::IpgTable(_) => {
                    return (None, None, None, None, None)
                },
                IpgWidgets::IpgText(_) => {
                    return (None, None, None, None, None)
                },
                IpgWidgets::IpgTextEditor(_) => {
                    return (None, None, None, None, None)
                },
                IpgWidgets::IpgTextInput(input) => {
                    let value = match data_str {
                        Some(v) => v,
                        None => panic!("Could not find the text_input value")
                    };
                    input.value = value;
                    let user_data = input.user_data.clone();
                    let cb_name = input.cb_name_submit.clone();

                    drop(state);

                    return (cb_name, user_data, None, None, None)
                },
            },
    
    None => panic!("Widget with id {} not found", id)
    }
}