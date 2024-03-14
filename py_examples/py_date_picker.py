from icedpygui.icedpygui import IPG


ipg = IPG()

selected_date = "No selection"


# callback for the date picker.  The id is the date_picker id so you have to get
# the id of whatever widget you want to update using a class or for small projects, a global var.
# dataclass is not supported at this time.
def date_selected(id, name, date):
    ipg.update_item(selected_date_id, "content", f"You submitted {date}")


# Another callback with some user data
def date_selected_with_ud(id, name, date, user_data):
    ipg.update_item(selected_with_ud_id, "content", 
                    f"You submitted {date} with user_data = {user_data}")
    ipg.update_item(btn_id, "show", True)

# callback for the date picker.  The id is the date_picker id so you have to get
# the id of whatever widget you want to update using a class or for small projects, a global var.
# dataclass is not supported at this time.
def date_resize(id, name):
    ipg.update_item(dp2_id, "size_factor", 1.5)

ipg.add_window("main", "Date Picker Demo", 800, 800, 
                                    500, 100)

ipg.add_container("main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col", parent_id="cont", align_items="center")

ipg.add_text(parent_id="col", content="Press the calendar buttons to access the calendar selection")

# The date picker size can be scaled from > 1.0.  Anything less than 1 will
# give an error and is not readable anyway.
ipg.add_date_picker(parent_id="col", size_factor=1.2, on_select=date_selected)

# text widget id needed for callback.
selected_date_id = ipg.add_text(parent_id="col", content=selected_date)

# Another date picker to test the user_data
dp2_id = ipg.add_date_picker(parent_id="col", size_factor=1.2, 
                             on_select=date_selected_with_ud, 
                             user_data="Some user data")

# text widget id needed for callback.
selected_with_ud_id = ipg.add_text(parent_id="col", content=selected_date)

btn_id = ipg.add_button(parent_id="col", label="Click to resize the calendar", 
                               on_press=date_resize, show=False)

ipg.start_session()
