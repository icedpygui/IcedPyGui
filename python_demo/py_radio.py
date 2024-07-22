from icedpygui import IPG, IpgRadioParams, IpgRadioDirection, IpgTextParams
from icedpygui import IpgColumnAlignment, IpgColor


# The radio widget has a count limitation unlike the other widgets due to the way
# the Rust enums are connected to this widget.  Therefore a limit of 26 groups of buttons with
# a limit of 26 buttons per group was set for this widget.  In the unlikely case where you need
# more, you could used checkboxes and just uncheck the all on a callback, keeping the
# one you selected as checked.

# On some of this demo, seeing the changes that occur will be better if you set the debub=True
# for the window.

ipg = IPG()


# The callback data parameter for the radio is a list = [index, label]
# In this case, the user_data is a string but it can be anything
def selected_radio_1(rd_id: int, data: list[int, str], user_data: str):
    ipg.update_item(rd_text_id_1, IpgTextParams.Content,
                    f"Radio callback id = {rd_id}, \n index = {data[0]}, \nlabel = {data[1]} \n user_data = {user_data}")


def selected_radio_2(rd_id: int, data: list[int, str], user_data: str):
    ipg.update_item(rd_text_id_2, IpgTextParams.Content,
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
    radio1 = IpgRadioDirection.Vertical
    radio2 = IpgRadioDirection.Horizontal
    if checked:
        radio1 = IpgRadioDirection.Horizontal
        radio2 = IpgRadioDirection.Vertical
    ipg.update_item(radio_ids[0], IpgRadioParams.Direction, radio1)
    ipg.update_item(radio_ids[1], IpgRadioParams.Direction, radio2)


def change_selection(chk_id: int, checked: bool, radio_ids: list[int]):
    selected = None
    if checked:
        selected = 2
    ipg.update_item(radio_ids[0], IpgRadioParams.SelectedIndex, selected)
    ipg.update_item(radio_ids[1], IpgRadioParams.SelectedIndex, selected)


# When you change the labels you are actually replacing the radios, so the
# lengths can change too.
def change_labels(chk_id: int, checked: bool, radio_ids: list[int]):
    labels = [["one", "two", "three", "four"], ["five", "six", "seven"]]
    if checked:
        labels = [["ants", "flies", "bees", "wasps"], ["cat", "dog", "horse"]]

    ipg.update_item(radio_ids[0], IpgRadioParams.Labels, labels[0])
    ipg.update_item(radio_ids[1], IpgRadioParams.Labels, labels[1])
    # unselecting any if selected
    ipg.update_item(radio_ids[0], IpgRadioParams.SelectedIndex, None)
    ipg.update_item(radio_ids[1], IpgRadioParams.SelectedIndex, None)


def hide_left_radio(chk_id: int, checked: bool, rd_left_id: int):
    show = True
    if checked:
        show = False
    ipg.update_item(rd_left_id, IpgRadioParams.Show, show)


def change_size(chk_id: int, checked: bool, radio_ids: list[int]):
    size = 20.0
    if checked:
        size = 10.0
    ipg.update_item(radio_ids[0], IpgRadioParams.Size, size)
    ipg.update_item(radio_ids[1], IpgRadioParams.Size, size)


def change_spacing(chk_id: int, checked: bool, radio_ids: list[int]):
    spacing = 10.0
    if checked:
        spacing = 20.0
    ipg.update_item(radio_ids[0], IpgRadioParams.Spacing, spacing)
    ipg.update_item(radio_ids[1], IpgRadioParams.Spacing, spacing)


def change_text_spacing(chk_id: int, checked: bool, radio_ids: list[int]):
    ts = 15
    if checked:
        ts = 30.0
    ipg.update_item(radio_ids[0], IpgRadioParams.TextSpacing, ts)
    ipg.update_item(radio_ids[1], IpgRadioParams.TextSpacing, ts)


def change_text_size(chk_id: int, checked: bool, radio_ids: list[int]):
    ts = 16.0
    if checked:
        ts = 20.0
    ipg.update_item(radio_ids[0], IpgRadioParams.TextSize, ts)
    ipg.update_item(radio_ids[1], IpgRadioParams.TextSize, ts)


def change_text_line_heigth(chk_id: int, checked: bool, radio_ids: list[int]):
    tlh = 1.3
    if checked:
        tlh = 3.0
    ipg.update_item(radio_ids[0], IpgRadioParams.TextLineHeight, tlh)
    ipg.update_item(radio_ids[1], IpgRadioParams.TextLineHeight, tlh)


def change_width(chk_id: int, checked: bool, radio_ids: list[int]):
    wd = None  # defaults to shrink
    if checked:
        wd = 150.0
    ipg.update_item(radio_ids[0], IpgRadioParams.Width, wd)
    ipg.update_item(radio_ids[1], IpgRadioParams.Width, wd)


def change_width_fill(chk_id: int, checked: bool, radio_ids: list[int]):
    # Width overrides WidthFill, so make sure to Width is not set
    # in this case, let's set the Width to none just in case.
    wdf = False  # defaults to shrink
    if checked:
        wdf = True
    ipg.update_item(radio_ids[0], IpgRadioParams.Width, None)
    ipg.update_item(radio_ids[1], IpgRadioParams.Width, None)

    ipg.update_item(radio_ids[0], IpgRadioParams.WidthFill, wdf)
    ipg.update_item(radio_ids[1], IpgRadioParams.WidthFill, wdf)


def change_height(chk_id: int, checked: bool, radio_ids: list[int]):
    ht = None  # defaults to shrink
    if checked:
        ht = 150.0
    ipg.update_item(radio_ids[0], IpgRadioParams.Height, ht)
    ipg.update_item(radio_ids[1], IpgRadioParams.Height, ht)

    # Note:  Changing the height to HeightFill doesn't work in this case.
    # It seems to happens sometimes when there is a clash between the
    # widget and the containers.  In this case, it's easily solved by
    # using a set value for the radios or defaulting to the Shrink value.

def change_right_radio_colors(chk_id: int, checked: bool, radio_right_id: int):
    if checked:
        ipg.update_item(radio_right_id, IpgRadioParams.Style, "color")
    else:
        ipg.update_item(radio_right_id, IpgRadioParams.Style, None)



# Define the styling that's used later
# See the hint for the color discussion.
ipg.add_radio_style(style_id="color",
                    circle_inner_color=IpgColor.YELLOW,
                    border_color=IpgColor.DARK_ORANGE,
                    dot_color=IpgColor.DARK_ORANGE, 
                    text_color=IpgColor.YELLOW,
                    border_width=2.0)


# **************Window Constructions Starts Here*************************

ipg.add_window("main", "Radio Demo", 500, 600,
                pos_x=100, pos_y=25)

ipg.add_container("main", container_id="cont",
                  width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col", parent_id="cont",
               align_items=IpgColumnAlignment.Center, height=600.0, width=500.0)

ipg.add_space("col", height=100)

ipg.add_text(parent_id="col", content="Press the radio buttons to see the selection")
ipg.add_text(parent_id="col", content="Press the checkboxes in window 2 to see the effects")

# Add a row for the two groups of radio buttons
ipg.add_row("main", "row1", parent_id="col",
            spacing=30.0, width_fill=True)

rd_left_id = ipg.add_radio(parent_id="row1", labels=["one", "two", "three", "four"],
                           on_select=selected_radio_1, user_data="Some data")

rd_right_id = ipg.add_radio(parent_id="row1", labels=["five", "six", "seven"],
                            direction=IpgRadioDirection.Horizontal,
                            on_select=selected_radio_2, user_data="Some data")

# add a row for the text associated with the above radio groups
ipg.add_row("main", "row2", parent_id="col", spacing=50.0)

rd_text_id_1 = ipg.add_text("row2", content="Radio callback data:")
rd_text_id_2 = ipg.add_text("row2", content="Radio callback data:")

ipg.add_space(parent_id="col", height=90)

# ********** Adding Window 2 with Checkboxes ****************************************

ipg.add_window("main2", "Radio Demo", 400, 600,
                pos_x=650, pos_y=25)

ipg.add_container("main2", container_id="cont",
                  width_fill=True, height_fill=True)

ipg.add_column(window_id="main2", container_id="chk_col", parent_id="cont")

ipg.add_checkbox("chk_col", label="Change Direction of radios",
                 on_toggle=change_direction, user_data=[rd_left_id, rd_right_id])

ipg.add_checkbox("chk_col", label="Change the selected radios to the 3rd label on each",
                 on_toggle=change_selection, user_data=[rd_left_id, rd_right_id])

ipg.add_checkbox("chk_col", label="Change the Labels",
                 on_toggle=change_labels, user_data=[rd_left_id, rd_right_id])

ipg.add_checkbox("chk_col", label="Change the Size", user_data=[rd_left_id, rd_right_id],
                 on_toggle=change_size)

ipg.add_checkbox("chk_col", label="Change the Spacing", user_data=[rd_left_id, rd_right_id],
                 on_toggle=change_spacing)

ipg.add_checkbox("chk_col", label="Change the TextSpacing", user_data=[rd_left_id, rd_right_id],
                 on_toggle=change_text_spacing)

ipg.add_checkbox("chk_col", label="Change the TextSize", user_data=[rd_left_id, rd_right_id],
                 on_toggle=change_text_size)

ipg.add_checkbox("chk_col", label="Change the TextLineHeight", user_data=[rd_left_id, rd_right_id],
                 on_toggle=change_text_line_heigth)

ipg.add_checkbox("chk_col", label="Change the Width", user_data=[rd_left_id, rd_right_id],
                 on_toggle=change_width)

ipg.add_checkbox("chk_col", label="Change the WidthFill", user_data=[rd_left_id, rd_right_id],
                 on_toggle=change_width_fill)

ipg.add_checkbox("chk_col", label="Change the Height", user_data=[rd_left_id, rd_right_id],
                 on_toggle=change_height)

ipg.add_checkbox("chk_col", label="Hide the left Radios", user_data=rd_left_id,
                 on_toggle=hide_left_radio)

ipg.add_checkbox("chk_col", label="Change the border, dot and text color", user_data=rd_right_id,
                 on_toggle=change_right_radio_colors)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
