from icedpygui import IPG, IpgProgressBarParams, IpgTextParams


ipg = IPG()

value = 75.0

def change_value_plus(btn_id, pg_id):
    global value
    value += 1
    ipg.update_item(pg_id, IpgProgressBarParams.Value, value)


def change_value_minus(btn_id, pg_id):
    global value
    value -= 1
    ipg.update_item(pg_id, IpgProgressBarParams.Value, value)


def change_min(btn_id, min, pg_id):
    min_float = float(min)
    ipg.update_item(pg_id, IpgProgressBarParams.Min, min_float)
    ipg.update_item(min_text, IpgTextParams.Content, min)


def change_max(btn_id, max, pg_id):
    max_float = float(max)
    ipg.update_item(pg_id, IpgProgressBarParams.Max, max_float)
    ipg.update_item(max_text, IpgTextParams.Content, max)


def change_height(btn_id, pg_id):
    ipg.update_item(pg_id, IpgProgressBarParams.Height, 30.0)

def change_width(btn_id, pg_id):
    ipg.update_item(pg_id, IpgProgressBarParams.Width, 300.0)

def change_width_to_fill(btn_id, pg_id):
    ipg.update_item(pg_id, IpgProgressBarParams.WidthFill, True)

def hide_bar(btn_id, pg_id):
    ipg.update_item(pg_id, IpgProgressBarParams.Show, False)




ipg.add_window("main", "CheckBox Demo",
                800, 800, 500, 100)
        
ipg.add_container(window_id="main", container_id="cont", width_fill=True,
                        height_fill=True, align_x="center", align_y="center")

ipg.add_column("main", "col", parent_id="cont", 
               align_items="center", spacing=2)

ipg.add_row(window_id="main", container_id="row1", parent_id="col",
                    width=400.0, padding=[0])

pg_id = ipg.add_progress_bar("row1", 50.0, 100.0, value)

ipg.add_row(window_id="main", container_id="row2", parent_id="col",
                    width=400.0, padding=[0])

min_text = ipg.add_text("row2", "50")
sp_id = ipg.add_space("row2", width=320.0)
max_text = ipg.add_text("row2", "100")

# adding new column because current column has too small spacing
# This column can go into the column above because container only holds 1 widget
# If this was more complex, you could add another container tothe window then proceed.
ipg.add_column("main", "col2", parent_id="col", align_items="center")

# Add row for increment and decrement
ipg.add_row("main", "value_row", "col2")
ipg.add_button("value_row", "Press Me to + ",
               on_press=change_value_plus, user_data=pg_id)
ipg.add_button("value_row", "Press Me to - ",
               on_press=change_value_minus, user_data=pg_id)

# add row for min and max
ipg.add_row("main", "minmax_row", "col2")
# These use text input that you convert in the callback
# Numeric input widgets to come.  No error checking done.
ipg.add_text_input("minmax_row", "Enter Min",
               on_submit=change_min, width=150.0, user_data=pg_id)
ipg.add_text_input("minmax_row", "Enter Max",
               on_submit=change_max, width=150.0, user_data=pg_id)


ipg.add_button("col2", "Press Me to shorten the bar",
               on_press=change_width, user_data=pg_id)

ipg.add_button("col2", "Press Me to to fill the bar width, do the above first",
               on_press=change_width_to_fill, user_data=pg_id)

ipg.add_button("col2", "Press me to hide the bar.",
               on_press=hide_bar, user_data=pg_id)


ipg.start_session()