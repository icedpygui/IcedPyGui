from icedpygui import IPG, IpgTextParam, IpgButtonParam, IpgDatePickerParam
from icedpygui import IpgAlignment, IpgStyleStandard


ipg = IPG()


# Callback for the date picker.  The id is the date_picker id so you have to get
# the id of whatever widget you want to update using a class or for small projects,
# a global variable.  Dataclass is not supported at this time but you can use a class
# as seen in other demo files.
def date_selected(_dp_id: int, date: str):
    ipg.update_item(selected_date_id, 
                    IpgTextParam.Content, 
                    f"You submitted {date}")


# Another callback with some user data
def date_selected_with_ud(_dp_id: int, date: str, user_data: any):
    ipg.update_item(selected_with_ud_id, 
                    IpgTextParam.Content,
                    f"You submitted {date} with user_data = {user_data}")

    ipg.update_item(btn_id, 
                    IpgButtonParam.Show, 
                    True)


# Another callback for the date picker that changes its size.
def date_resize(_dp_id: int):
    ipg.update_item(dp2_id, 
                    IpgDatePickerParam.SizeFactor, 
                    1.5)


# Add a window first
ipg.add_window(
        window_id="main", 
        title="Date Picker Demo", 
        width=800, 
        height=800,
        pos_x=100, 
        pos_y=25)

# Add the container to center both x and y.  Holds only one widget.
ipg.add_container(
        window_id="main", 
        container_id="cont",
        width_fill=True, 
        height_fill=True)

# Add a column to hold more than one widget and put this into the container/
ipg.add_column(
        window_id="main", 
        container_id="col", 
        parent_id="cont",
        align_x=IpgAlignment.Center)
 
# Add info text
ipg.add_text(
        parent_id="col", 
        content="Press the first calendar buttons to access the calendar.  "
                "Select a date then press submit.  Do the same for the second button and you will see another button to "
                "resize the calendar")

# The date picker size can be scaled from > 1.0.  Anything less than 1 will
# give an error and is not readable anyway.
ipg.add_date_picker(
        parent_id="col", 
        size_factor=1.2, 
        on_submit=date_selected)

# text widget id needed for callback.
selected_date_id = ipg.add_text(
                        parent_id="col", 
                        content="No selection")

# Another date picker to test the user_data abd button style
dp2_id = ipg.add_date_picker(
                    parent_id="col", 
                    size_factor=1.2,
                    on_submit=date_selected_with_ud,
                    user_data="Some user data",
                    button_style_standard=IpgStyleStandard.Success)

# text widget id needed for callback.
selected_with_ud_id = ipg.add_text(
                            parent_id="col", 
                            content="No selection")

# Add the button for the resize but hide it until the second calendar is opened
btn_id = ipg.add_button(
                parent_id="col", 
                label="Click to resize the calendar",
                on_press=date_resize, 
                show=False)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
