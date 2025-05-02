from icedpygui import IPG, IpgTextParam
from icedpygui import IpgAlignment, IpgTextInputParam, IpgColor

ipg = IPG()


secure = False

# Currently, Ipg only has the text_input widget.
# Additional types of inputs will follow soon.
# Therefore, the return data will be a string
# that you will need to convert to whatever type you want

# Add the callback for the text_input, 2 parameters are
# returned, the id of the text_input, not used here, and the
# data, string in this case.
# Since we want to update a text widget, the id of the text_input
# widget is not used.
# When you type in the text, this fires each time
def on_input(_txt_input_id, data, _user_data: any):
    ipg.update_item(text_on_input_id, IpgTextParam.Content, value=data)


# This only fires when you press enter to submit, it passes na value like
# as does the on_input.
def on_submit(_txt_input_id, value: str, user_data: any):
    ipg.update_item(text_on_submit_id, IpgTextParam.Content, value=value)
    ipg.update_item(text_user_data_id, IpgTextParam.Content, value=user_data)


# This fired when you paste something into the field
# To submit it, press enter.
def on_paste(_txt_input_id, data, _user_data: any):
    ipg.update_item(text_on_paste_id, IpgTextParam.Content, value=data)


# The callbacks below allow you to change all of the parameters for a widget.
# They may or may not have frequent usage but it makes the gui very flexible
# when the data that may be loaded effects the placement, sizes, etc. used.
# These callbacks also demonstrate the usage of the widget parameters and
# are used in the testing of the code to make sure it behaves as expected.

# Based on some action, you may want to change the placeholder instructions.
def change_placeholder(_btn_id):
    ipg.update_item(ti_id, IpgTextInputParam.Placeholder, "Replaced Placeholder")


# Changing the value or creating an entry for the user
def change_value(_btn_id):
    ipg.update_item(ti_id, IpgTextInputParam.Value, "New Value")


# Lets make the value secure, you could have a button to show or hide the entry.
def toggle_secure(_btn_id):
    global secure
    secure = not secure
    # Let's make sure a value is there
    ipg.update_item(ti_id, IpgTextInputParam.Value, "New Value")
    # Let's secure it.
    ipg.update_item(ti_id, IpgTextInputParam.IsSecure, secure)


def change_width(_btn_id):
    ipg.update_item(ti_id, IpgTextInputParam.Width, 300.0)


# Note padding is a list, effects the space around the text
# 1 element [all sides]
# 2 elements effect [Top & Bottom, Left & Right]
# 4 elements effect [Top, Right, Bottom, Left]
def change_padding(_btn_id):
    # Let's make sure a value is there
    ipg.update_item(ti_id, IpgTextInputParam.Value, "A Value for the Padding")
    # let's make it bigger to hold the text
    ipg.update_item(ti_id, IpgTextInputParam.Width, 300.0)
    # Use only two entries for the list
    ipg.update_item(ti_id, IpgTextInputParam.Padding, [0.0, 30.0])


# Change the size of the text
def change_size(_btn_id):
    # Let's make sure a value is there
    ipg.update_item(ti_id, IpgTextInputParam.Value, "A Value for the Sizing")
    # let's make it bigger to hold the text
    ipg.update_item(ti_id, IpgTextInputParam.Width, 400.0)
    ipg.update_item(ti_id, IpgTextInputParam.Size, 30.0)


# Change the line height
def change_line_height(_btn_id):
    # Let's make sure a value is there
    ipg.update_item(ti_id, IpgTextInputParam.Value, "A Value for the LineHeight")
    # let's make it bigger to hold the text
    ipg.update_item(ti_id, IpgTextInputParam.Width, 450.0)
    ipg.update_item(ti_id, IpgTextInputParam.LineHeightRelative, 3.0)


# Add some styling
def add_style(_btn_id):
    ipg.update_item(ti_id, IpgTextInputParam.StyleId, ti_style)


# add the window
ipg.add_window(
        window_id="main", 
        title="Text Input Demo", 
        width=600, 
        height=600,
        pos_x=100, 
        pos_y=25)

# add the column for the widgets, centered
ipg.add_column(
        window_id="main", 
        container_id="col",
        align=IpgAlignment.Center,
        height_fill=True, 
        width_fill=True, 
        spacing=10)

# Add some instructions
ipg.add_text(
        parent_id="col", 
        content="It's best to see the effects by pressing \nthe buttons left to right and top to bottom")

ipg.add_space(
        parent_id="col", 
        height=25.0)

# Add the text_input widget
ti_id = ipg.add_text_input(
                parent_id="col", 
                placeholder="Input Some Text",
                width=200.0,
                on_input=on_input,
                on_submit=on_submit,
                on_paste=on_paste,
                user_data="User data = Some user data")

# Add the text widget to display the info
text_on_input_id = ipg.add_text(
                            parent_id="col", 
                            content="Text here will be added when typed")

text_on_submit_id = ipg.add_text(
                            parent_id="col", 
                            content="Text here will be added when submitted")

text_on_paste_id = ipg.add_text(
                            parent_id="col", 
                            content="Text here will be added when pasted")

text_user_data_id = ipg.add_text(
                            parent_id="col", 
                            content="User data will e here when submitted")

# Add row for buttons
ipg.add_row(
        window_id="main", 
        container_id="row_1", 
        parent_id="col")

ipg.add_button(
        parent_id="row_1", 
        label="Press Me to Change Placeholder", 
        on_press=change_placeholder)

ipg.add_button(
        parent_id="row_1", 
        label="Press Me to Change Value", 
        on_press=change_value)

ipg.add_row(
        window_id="main", 
        container_id="row_2", 
        parent_id="col")

ipg.add_button(
        parent_id="row_2", 
        label="Toggle the Secure Setting", 
        on_press=toggle_secure)

ipg.add_button(
        parent_id="row_2", 
        label="Press Me to Change the Width", 
        on_press=change_width)

ipg.add_row(
        window_id="main", 
        container_id="row_3", 
        parent_id="col")

ipg.add_button(
        parent_id="row_3", 
        label="Press Me to Change the Padding", 
        on_press=change_padding)

ipg.add_button(
        parent_id="row_3", 
        label="Press Me to Change the Size", 
        on_press=change_size)

ipg.add_row(
        window_id="main", 
        container_id="row_4", 
        parent_id="col")

ipg.add_button(
        parent_id="row_4", 
        label="Press Me to Change the LineHeight", 
        on_press=change_line_height)

ipg.add_row(
        window_id="main", 
        container_id="row_5", 
        parent_id="col")

ipg.add_button(
        parent_id="row_5", 
        label="Press Me to Add Some Styling", 
        on_press=add_style)

# the add style functions can be place anywhere as long as they are before the start_session
ti_style = ipg.add_text_input_style( 
                    background_color=IpgColor.CADET_BLUE,
                    border_color=IpgColor.YELLOW,
                    border_color_focused=IpgColor.PALE_GOLDEN_ROD,
                    border_color_hovered=IpgColor.CHARTREUSE,
                    border_width=5.0,
                    border_radius=[8.0],
                    placeholder_color=IpgColor.BLACK,
                    value_color=IpgColor.LIGHT_STEEL_BLUE,
                    selection_color=IpgColor.DARK_SLATE_GRAY)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
