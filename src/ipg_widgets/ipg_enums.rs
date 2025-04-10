//! ipg_enums
#![allow(clippy::enum_variant_names)]
use pyo3::pyclass;

use super::ipg_button::{IpgButton, IpgButtonStyle};
use super::ipg_canvas::IpgCanvas;
use super::ipg_card::{IpgCard, IpgCardStyle};
use super::ipg_checkbox::{IpgCheckBox, IpgCheckboxStyle};
use super::ipg_color_picker::{IpgColorPicker, IpgColorPickerStyle};
// use super::ipg_color_picker::IpgColorPicker;
use super::ipg_container::{IpgContainer, IpgContainerStyle};
use super::ipg_column::IpgColumn;
use super::ipg_date_picker::IpgDatePicker;
use super::ipg_divider::{IpgDividerHorizontal, IpgDividerVertical, IpgDividerStyle};
use super::ipg_image::IpgImage;
use super::ipg_menu::{IpgMenu, IpgMenuBarStyle, IpgMenuStyle};
// use super::ipg_modal::IpgModal;
use super::ipg_mousearea::IpgMouseArea;
use super::ipg_opaque::{IpgOpaque, IpgOpaqueStyle};
// use super::ipg_pane_grid::{IpgPaneGrid, IpgPane};
use super::ipg_pick_list::{IpgPickList, IpgPickListStyle};
use super::ipg_progress_bar::{IpgProgressBar, IpgProgressBarStyle};
use super::ipg_radio::{IpgRadio, IpgRadioStyle};
use super::ipg_row::IpgRow;
use super::ipg_rule::{IpgRule, IpgRuleStyle};
use super::ipg_scrollable::{IpgScrollable, IpgScrollableStyle};
use super::ipg_selectable_text::IpgSelectableText;
use super::ipg_separator::{IpgSeparator, IpgSeparatorStyle};
use super::ipg_slider::{IpgSlider, IpgSliderStyle};
use super::ipg_space::IpgSpace;
use super::ipg_stack::IpgStack;
use super::ipg_svg::IpgSvg;
use super::ipg_table::{IpgTable, IpgTableStyle};
use super::ipg_text::IpgText;
// use super::ipg_text_editor::IpgTextEditor;
use super::ipg_text_input::{IpgTextInput, IpgTextInputStyle};
use super::ipg_timer::{IpgTimer, IpgTimerStyle};
use super::ipg_timer_canvas::{IpgCanvasTimer, IpgCanvasTimerStyle};
use super::ipg_toggle::{IpgToggler, IpgTogglerStyle};
use super::ipg_tool_tip::IpgToolTip;
use super::ipg_window::IpgWindow;


#[derive(Debug, Clone)]
pub enum IpgContainers {
    IpgCanvas(IpgCanvas),
    IpgColumn(IpgColumn),
    IpgContainer(IpgContainer),
    IpgMenu(IpgMenu),
    // IpgModal(IpgModal),
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
    IpgCardStyle(IpgCardStyle),
    IpgCheckBox(IpgCheckBox),
    IpgCheckboxStyle(IpgCheckboxStyle),
    IpgColorPicker(IpgColorPicker),
    IpgColorPickerStyle(IpgColorPickerStyle),
    IpgContainerStyle(IpgContainerStyle),
    IpgDividerHorizontal(IpgDividerHorizontal),
    IpgDividerVertical(IpgDividerVertical),
    IpgDividerStyle(IpgDividerStyle),
    IpgDatePicker(IpgDatePicker),
    IpgImage(IpgImage),
    IpgMenuStyle(IpgMenuStyle),
    IpgMenuBarStyle(IpgMenuBarStyle),
    IpgOpaqueStyle(IpgOpaqueStyle),
    IpgPickList(IpgPickList),
    IpgPickListStyle(IpgPickListStyle),
    IpgProgressBar(IpgProgressBar),
    IpgProgressBarStyle(IpgProgressBarStyle),
    IpgRadio(IpgRadio),
    IpgRadioStyle(IpgRadioStyle),
    IpgRule(IpgRule),
    IpgRuleStyle(IpgRuleStyle),
    IpgScrollableStyle(IpgScrollableStyle),
    IpgSelectableText(IpgSelectableText),
    IpgSeparator(IpgSeparator),
    IpgSeparatorStyle(IpgSeparatorStyle),
    IpgSlider(IpgSlider),
    IpgSliderStyle(IpgSliderStyle),
    IpgSpace(IpgSpace),
    IpgSvg(IpgSvg),
    IpgTableStyle(IpgTableStyle),
    IpgText(IpgText),
    // IpgTextEditor(IpgTextEditor),
    IpgTextInput(IpgTextInput),
    IpgTextInputStyle(IpgTextInputStyle),
    IpgTimer(IpgTimer),
    IpgTimerStyle(IpgTimerStyle),
    IpgCanvasTimer(IpgCanvasTimer),
    IpgCanvasTimerStyle(IpgCanvasTimerStyle),
    IpgToggler(IpgToggler),
    IpgTogglerStyle(IpgTogglerStyle),
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgAlignment {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgHorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgVerticalAlignment {
    Top,
    Center,
    Bottom,
}
