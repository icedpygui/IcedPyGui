//! ipg_enums
#![allow(clippy::enum_variant_names)]
use pyo3::pyclass;

use super::ipg_button::{IpgButton, IpgButtonStyle};
use super::ipg_canvas::IpgCanvas;
use super::ipg_card::IpgCard;
use super::ipg_checkbox::{IpgCheckBox, IpgCheckboxStyle};
use super::ipg_color_picker::{IpgColorPicker, IpgColorPickerStyle};
// use super::ipg_color_picker::IpgColorPicker;
use super::ipg_container::IpgContainer;
use super::ipg_column::IpgColumn;
use super::ipg_date_picker::IpgDatePicker;
use super::ipg_image::IpgImage;
use super::ipg_menu::IpgMenu;
use super::ipg_modal::IpgModal;
use super::ipg_mousearea::IpgMouseArea;
use super::ipg_opaque::IpgOpaque;
// use super::ipg_pane_grid::{IpgPaneGrid, IpgPane};
use super::ipg_pick_list::IpgPickList;
use super::ipg_progress_bar::IpgProgressBar;
use super::ipg_radio::IpgRadio;
use super::ipg_row::IpgRow;
use super::ipg_rule::IpgRule;
use super::ipg_scrollable::IpgScrollable;
use super::ipg_selectable_text::IpgSelectableText;
use super::ipg_slider::IpgSlider;
use super::ipg_space::IpgSpace;
use super::ipg_stack::IpgStack;
use super::ipg_svg::IpgSvg;
use super::ipg_table::IpgTable;
use super::ipg_text::IpgText;
// use super::ipg_text_editor::IpgTextEditor;
use super::ipg_text_input::IpgTextInput;
use super::ipg_timer::IpgTimer;
use super::ipg_timer_canvas::IpgCanvasTimer;
use super::ipg_toggle::IpgToggler;
use super::ipg_tool_tip::IpgToolTip;
use super::ipg_window::IpgWindow;


#[derive(Debug, Clone)]
pub enum IpgContainers {
    IpgCanvas(IpgCanvas),
    IpgColumn(IpgColumn),
    IpgContainer(IpgContainer),
    IpgModal(IpgModal),
    IpgMouseArea(IpgMouseArea),
    IpgOpaque(IpgOpaque),
    IpgStack(IpgStack),
    IpgTable(IpgTable),
    // IpgPaneGrid(IpgPaneGrid),
    // IpgPane(IpgPane),
    IpgRow(IpgRow),
    IpgScrollable(IpgScrollable),
    IpgToolTip(IpgToolTip),
    IpgWindow(IpgWindow),
}

#[derive(Debug, Clone)]
pub enum IpgWidgets {
    IpgButton(IpgButton),
    IpgButtonStyle(IpgButtonStyle),
    IpgCard(IpgCard),
    IpgCheckBox(IpgCheckBox),
    IpgCheckboxStyle(IpgCheckboxStyle),
    IpgColorPicker(IpgColorPicker),
    IpgColorPickerStyle(IpgColorPickerStyle),
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
    IpgSvg(IpgSvg),
    IpgText(IpgText),
    // IpgTextEditor(IpgTextEditor),
    IpgTextInput(IpgTextInput),
    IpgTimer(IpgTimer),
    IpgCanvasTimer(IpgCanvasTimer),
    IpgToggler(IpgToggler),
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgAlignment {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgHorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgVerticalAlignment {
    Top,
    Center,
    Bottom,
}
