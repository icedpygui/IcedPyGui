from icedpygui import IPG, IpgRadioParam, IpgRadioDirection, IpgTextParam
from icedpygui import IpgAlignment, IpgColor


# The radio widget has a count limitation unlike the other widgets due to the way
# the Rust enums are connected to this widget.  Therefore a limit of 26 groups of buttons with
# a limit of 26 buttons per group was set for this widget.  In the unlikely case where you need
# more, you could used checkboxes and just uncheck the all on a callback, keeping the
# one you selected as checked.

# On some of this demo, seeing the changes that occur will be better if you set the debug=True
# for the window.

ipg = IPG()


# The callback data parameter for the radio is a list = [index, label]
# In this case, the user_data is a string but it can be anything
def selected_radio_1(
        rd_id: int, 
        data: list[int, str], 
        user_data: str):
    
    ipg.update_item(
            rd_text_id_1, 
            IpgTextParam.Content,
            f"Radio callback id = {rd_id}, \n index = {data[0]}, \nlabel = {data[1]} \n user_data = {user_data}")


def selected_radio_2(rd_id: int, data: list[int, str], user_data: str):
    ipg.update_item(
            rd_text_id_2, 
            IpgTextParam.Content,
            f"Radio callback id = {rd_id}, \n index = {data[0]}, \nlabel = {data[1]} \n user_data = {user_data}")


# The callbacks below allow you to change all of the parameters for a widget.
# They may or may not have frequent usage but it makes the gui very flexible
# when the data that may be loaded effects the placement, sizes, etc. used.
# These callbacks also demonstrate the usage of the widget parameters and
# are used in the testing of the code to make sure it behaves as expected.

# The user_data is called radio_ids and is a list [ids]
# The second parameter for update_item is from the imported params class.
# This is the case for all widgets being updated.
def change_direction(chk_id: int, checked: bool, radio_ids: list[int]):
    if checked:
        radio1 = IpgRadioDirection.Horizontal
        radio2 = IpgRadioDirection.Vertical
    else:
        radio1 = IpgRadioDirection.Vertical
        radio2 = IpgRadioDirection.Horizontal
        
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.Direction, 
            radio1)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.Direction, 
            radio2)


def change_selection(chk_id: int, checked: bool, radio_ids: list[int]):
    if checked:
        selected = 2
    else:
        selected = None
        
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.SelectedIndex, 
            selected)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.SelectedIndex, 
            selected)


# When you change the labels you are actually replacing the radios, so the
# lengths can change too.
def change_labels(chk_id: int, checked: bool, radio_ids: list[int]):
    if checked:
        labels = [["ants", "flies", "bees", "wasps"], ["cat", "dog", "horse"]]
    else:
        labels = [["one", "two", "three", "four"], ["five", "six", "seven"]]

    # unselecting any if selected
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.SelectedIndex, 
            None)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.SelectedIndex, 
            None)
    # now select
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.Labels, 
            labels[0])
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.Labels, 
            labels[1])


def hide_left_radio(chk_id: int, checked: bool, rd_left_id: int):
    if checked:
        show = False
    else:
        show = True
        
    ipg.update_item(
            rd_left_id, 
            IpgRadioParam.Show, 
            show)


def change_size(chk_id: int, checked: bool, radio_ids: list[int]):
    if checked:
        size = 10.0
    else:
        size = 20.0
        
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.Size, 
            size)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.Size, 
            size)


def change_spacing(chk_id: int, checked: bool, radio_ids: list[int]):
    if checked:
        spacing = 20.0
    else:
        spacing = 10.0
        
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.Spacing, 
            spacing)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.Spacing, 
            spacing)


def change_text_spacing(chk_id: int, checked: bool, radio_ids: list[int]):
    if checked:
        ts = 30.0
    else:
        ts = 15
        
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.TextSpacing, 
            ts)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.TextSpacing, 
            ts)


def change_text_size(chk_id: int, checked: bool, radio_ids: list[int]):
    if checked:
        ts = 20.0
    else:
        ts = 16.0
        
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.TextSize, 
            ts)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.TextSize, 
            ts)


def change_line_height_relative(chk_id: int, checked: bool, radio_ids: list[int]):
    if checked:
        tlh = 3.0
    else:
        tlh = 1.3
        
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.LineHeightRelative, 
            tlh)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.LineHeightRelative, 
            tlh)


def change_width(chk_id: int, checked: bool, radio_ids: list[int]):
    if checked:
        wd = 150.0
    else:
        wd = None  # defaults to shrink
        
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.Width, 
            wd)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.Width, 
            wd)


def change_width_fill(chk_id: int, checked: bool, radio_ids: list[int]):
    # Width overrides WidthFill, so make sure to Width is not set
    # in this case, let's set the Width to none just in case.
    if checked:
        wdf = True
    else:
        wdf = False  # defaults to shrink
        
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.Width, 
            None)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.Width, 
            None)

    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.WidthFill, 
            wdf)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.WidthFill, 
            wdf)


def change_height(chk_id: int, checked: bool, radio_ids: list[int]):
    if checked:
        ht = 150.0
    else:
        ht = None  # defaults to shrink
        
    ipg.update_item(
            radio_ids[0], 
            IpgRadioParam.Height, 
            ht)
    ipg.update_item(
            radio_ids[1], 
            IpgRadioParam.Height, 
            ht)

    # Note:  Changing the height to HeightFill doesn't work in this case.
    # It seems to happens sometimes when there is a clash between the
    # widget and the containers.  In this case, it's easily solved by
    # using a set value for the radios or defaulting to the Shrink value.

def change_right_radio_colors(chk_id: int, checked: bool, radio_right_id: int):
    if checked:
        ipg.update_item(
                radio_right_id, 
                IpgRadioParam.StyleId, 
                color)
    else:
        ipg.update_item(
                radio_right_id, 
                IpgRadioParam.StyleId, 
                0)



# Define the styling that's used later
# See the hint for the color discussion.
color = ipg.add_radio_style(
                background_color=IpgColor.YELLOW,
                border_color=IpgColor.DARK_ORANGE,
                dot_color=IpgColor.DARK_ORANGE, 
                text_color=IpgColor.YELLOW,
                border_width=2.0)


# **************Window Constructions Starts Here*************************

ipg.add_window(
        window_id="main", 
        title="Radio Demo", 
        width=500, 
        height=600,
        pos_x=100, 
        pos_y=25)

ipg.add_container(
        window_id="main", 
        container_id="cont",
        width_fill=True, 
        height_fill=True)

ipg.add_column(
        window_id="main", 
        container_id="col", 
        parent_id="cont",
        align=IpgAlignment.Center, 
        height=600.0, 
        width=500.0)

ipg.add_space(
        parent_id="col", 
        height=100)

ipg.add_text(
        parent_id="col", 
        content="Press the radio buttons to see the selection")
ipg.add_text(
        parent_id="col", 
        content="Press the checkboxes in window 2 to see the effects")

# Add a row for the two groups of radio buttons
ipg.add_row(
        window_id="main", 
        container_id="row1", 
        parent_id="col",
        spacing=30.0, 
        width_fill=True)

rd_left_id = ipg.add_radio(
                    parent_id="row1", 
                    labels=["one", "two", "three", "four"],
                    on_select=selected_radio_1, 
                    user_data="Some data")

rd_right_id = ipg.add_radio(
                        parent_id="row1", 
                        labels=["five", "six", "seven"],
                        direction=IpgRadioDirection.Horizontal,
                        on_select=selected_radio_2, 
                        user_data="Some data")

# add a row for the text associated with the above radio groups
ipg.add_row(
        window_id="main", 
        container_id="row2", 
        parent_id="col", 
        spacing=50.0)

rd_text_id_1 = ipg.add_text(
                    parent_id="row2", 
                    content="Radio callback data:")
rd_text_id_2 = ipg.add_text(
                    parent_id="row2", 
                    content="Radio callback data:")

ipg.add_space(
        parent_id="col", 
        height=90)

# ********** Adding Window 2 with Checkboxes ****************************************

ipg.add_window(
        window_id="main2", 
        title="Radio Demo", 
        width=400, 
        height=600,
        pos_x=650, 
        pos_y=25)

ipg.add_container(
        window_id="main2", 
        container_id="cont",
        width_fill=True, 
        height_fill=True)

ipg.add_column(
        window_id="main2", 
        container_id="chk_col", 
        parent_id="cont")

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change Direction of radios",
        on_toggle=change_direction, 
        user_data=[rd_left_id, rd_right_id])

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change the selected radios to the 3rd label on each",
        on_toggle=change_selection, 
        user_data=[rd_left_id, rd_right_id])

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change the Labels",
        on_toggle=change_labels, 
        user_data=[rd_left_id, rd_right_id])

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change the Size", 
        user_data=[rd_left_id, rd_right_id],
        on_toggle=change_size)

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change the Spacing", 
        user_data=[rd_left_id, rd_right_id],
        on_toggle=change_spacing)

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change the TextSpacing", 
        user_data=[rd_left_id, rd_right_id],
        on_toggle=change_text_spacing)

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change the TextSize", 
        user_data=[rd_left_id, rd_right_id],
        on_toggle=change_text_size)

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change the LineHeightRelative", 
        user_data=[rd_left_id, rd_right_id],
        on_toggle=change_line_height_relative)

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change the Width", 
        user_data=[rd_left_id, rd_right_id],
        on_toggle=change_width)

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change the WidthFill", 
        user_data=[rd_left_id, rd_right_id],
        on_toggle=change_width_fill)

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change the Height", 
        user_data=[rd_left_id, rd_right_id],
        on_toggle=change_height)

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Hide the left Radios", 
        user_data=rd_left_id,
        on_toggle=hide_left_radio)

ipg.add_checkbox(
        parent_id="chk_col", 
        label="Change the border, dot and text color", 
        user_data=rd_right_id,
        on_toggle=change_right_radio_colors)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
