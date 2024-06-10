from icedpygui import IPG, IpgProgressBarParams, IpgTextParams
from icedpygui import IpgColumnAlignment, IpgColor


ipg = IPG()

# global var for callback
value = 75.0


# The callbacks below allow you to change all of the parameters for a widget.
# They may or may not have frequent usage but it makes the gui very flexible
# when the data that may be loaded effects the placement, sizes, etc. used.
# These callbacks also demonstrate the usage of the widget parameters and
# are used in the testing of the code to make sure it behaves as expected.

# increment the bar, the pg_id comes in as the user_data for the button
# This could be a global var or usually placed in a class
def change_value_plus(btn_id: int, pg_id: any):
    global value
    value += 1
    ipg.update_item(pg_id, IpgProgressBarParams.Value, value)
    ipg.update_item(current_value_id, IpgTextParams.Content, f"Current Value = {value}")


# decrement the bar value
def change_value_minus(_btn_id: int, pg_id: any):
    global value
    value -= 1
    ipg.update_item(pg_id, IpgProgressBarParams.Value, value)
    ipg.update_item(current_value_id, IpgTextParams.Content, f"Current Value = {value}")


def change_min(_text_input_id: int, min_val: str, pg_id: any):
    # text_input values are str therefore they are changed to  a float
    # Int and float input soon to follow.
    min_float = float(min_val)
    ipg.update_item(pg_id, IpgProgressBarParams.Min, min_float)
    # Since the text content requires a str, the min_val can be used
    ipg.update_item(min_text, IpgTextParams.Content, min_val)


def change_max(_text_input_id: int, max_val: str, pg_id: any):
    max_float = float(max_val)
    ipg.update_item(pg_id, IpgProgressBarParams.Max, max_float)
    ipg.update_item(max_text, IpgTextParams.Content, max_val)


def change_height(_btn_id: int, pg_id: any):
    ipg.update_item(pg_id, IpgProgressBarParams.Height, 30.0)


def change_width(_btn_id: int, pg_id: any):
    ipg.update_item(pg_id, IpgProgressBarParams.Width, 300.0)


def change_width_to_fill(_btn_id: int, pg_id: any):
    ipg.update_item(pg_id, IpgProgressBarParams.WidthFill, True)


def hide_bar(_btn_id: int, pg_id: any):
    ipg.update_item(pg_id, IpgProgressBarParams.Show, False)


# Add the window
ipg.add_window("main", "CheckBox Demo",
               600, 600, 100, 25)

# Add the container to help with aligment
ipg.add_container(window_id="main", container_id="cont", width_fill=True,
                  height_fill=True)

# Add a column for the widgets
ipg.add_column("main", "col", parent_id="cont",
               align_items=IpgColumnAlignment.Center, spacing=2)

# add a row for the  for the pg bar
ipg.add_row(window_id="main", container_id="row1", parent_id="col",
            width=400.0, padding=[0])

# Add the pg bar
pg_id = ipg.add_progress_bar("row1", 50.0, 100.0, value)

# add a row to display text value at the start and end of the pg bar
ipg.add_row(window_id="main", container_id="row2", parent_id="col",
            width=400.0, padding=[0])

# The text, space and more text just below the pg bar.
min_text = ipg.add_text("row2", "50")
sp_id = ipg.add_space("row2", width=320.0)
max_text = ipg.add_text("row2", "100")

# Adding new column because current column has too small of a spacing value
# This column can go into the column above because container only holds 1 widget
# If this was more complex, you could add another container to the window then proceed.
ipg.add_column("main", "col2", parent_id="col",
               align_items=IpgColumnAlignment.Center)

# Add a text widget for current value
current_value_id = ipg.add_text(parent_id="col2", content=f"Current Value = {value}")

# Add row for increment and decrement buttons
ipg.add_row("main", "value_row", parent_id="col2")

# Increment button
ipg.add_button("value_row", "Press Me to + ",
               on_press=change_value_plus, user_data=pg_id)

# Decrement button
ipg.add_button("value_row", "Press Me to - ",
               on_press=change_value_minus, user_data=pg_id)

# add row for min and max
ipg.add_row("main", "minmax_row", parent_id="col2")

# text input widgets are used for the inputs which you convert to floats in the callback
# Numeric input widgets to come.  No error checking done.
ipg.add_text_input("minmax_row", "Enter Min",
                   on_submit=change_min, width=150.0, user_data=pg_id)
ipg.add_text_input("minmax_row", "Enter Max",
                   on_submit=change_max, width=150.0, user_data=pg_id)

# Add a button the short the bar
ipg.add_button("col2", "Press Me to shorten the bar",
               on_press=change_width, user_data=pg_id)

# Add a button the lengthen the bar
ipg.add_button("col2", "Press Me to to fill the bar width, do the above first",
               on_press=change_width_to_fill, user_data=pg_id)

# Add a button to hide the bar
ipg.add_button("col2", "Press me to hide the bar.",
               on_press=hide_bar, user_data=pg_id)


# add some styling to a new bar
ipg.add_styling_background("bkg", color=IpgColor.DANGER)
ipg.add_styling_bar_color("bar", color=IpgColor.CORAL)


ipg.add_progress_bar("col2", 0.0, 100.0, 50.0,
                     style_background="bkg",
                     style_bar_color="bar")


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()