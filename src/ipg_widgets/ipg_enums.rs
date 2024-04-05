#![allow(unused)]

use core::panic;
use std::collections::HashMap;

use super::ipg_button::IpgButton;
use super::ipg_card::IpgCard;
use super::ipg_checkbox::IpgCheckBox;
use super::ipg_color_picker::IpgColorPicker;
use super::ipg_container::IpgContainer;
use super::ipg_column::IpgColumn;
use super::ipg_date_picker::IpgDatePicker;
use super::ipg_image::IpgImage;
use super::ipg_menu::IpgMenu;
use super::ipg_pane_grid::{IpgPaneGrid, IpgPane};
use super::ipg_pick_list::IpgPickList;
use super::ipg_progress_bar::IpgProgressBar;
use super::ipg_radio::{IpgRadio, Choice};
use super::ipg_row::IpgRow;
use super::ipg_rule::IpgRule;
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
use super::helpers::{format_date, MONTH_NAMES};
use crate::access_state;

use iced::{Color, Point};

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
    IpgMenu(IpgMenu),
    IpgPickList(IpgPickList),
    IpgProgressBar(IpgProgressBar),
    IpgRadio(IpgRadio),
    IpgRule(IpgRule),
    IpgSelectableText(IpgSelectableText),
    IpgSlider(IpgSlider),
    IpgSpace(IpgSpace),
    IpgTable(IpgTable),
    IpgText(IpgText),
    IpgTextEditor(IpgTextEditor),
    IpgTextInput(IpgTextInput),
}
