from icedpygui.icedpygui import IPG, IpgProgressBarParam, IpgTextParam
from icedpygui.icedpygui import IpgAlignment, IpgSliderParam, IpgColor

ipg = IPG()

show = True


# Couple of callbacks for change and release
# The slider_id is not used since we are updating the bar and the text
def slider_on_change(_slider_id, data):
    ipg.update_item(on_change_id, IpgTextParam.Content, f"On Change value is {data}")
    ipg.update_item(bar_id, IpgProgressBarParam.Value, data)


def slider_on_release(_slider_id, data):
    ipg.update_item(on_release_id, IpgTextParam.Content, f"On Release value is {data}")


# The callbacks below allow you to change all of the parameters for a widget.
# They may or may not have frequent usage but it makes the gui very flexible
# when the data that may be loaded effects the placement, sizes, etc. used.
# These callbacks also demonstrate the usage of the widget parameters and
# are used in the testing of the code to make sure it behaves as expected.
def change_width(btn_id):
    ipg.update_item(sl_id, IpgSliderParam.Width, 200.0)
    # change bar too
    ipg.update_item(bar_id, IpgProgressBarParam.Width, 200.0)


def change_height(btn_id):
    ipg.update_item(sl_id, IpgSliderParam.Height, 30.0)


def change_min(btn_id):
    ipg.update_item(sl_id, IpgSliderParam.Min, 50.0)
    ipg.update_item(min_txt_id, IpgTextParam.Content, "50")


def change_max(btn_id):
    ipg.update_item(sl_id, IpgSliderParam.Max, 150.0)
    ipg.update_item(max_txt_id, IpgTextParam.Content, "150")


def change_step(btn_id):
    ipg.update_item(sl_id, IpgSliderParam.Step, 5.0)


def change_value(btn_id):
    ipg.update_item(sl_id, IpgSliderParam.Value, 100.0)


def add_styling(btn_id):
    ipg.update_item(sl_id, IpgSliderParam.Style, "color")


def toggle_show(btn_id):
    global show
    show = not show
    ipg.update_item(sl_id, IpgSliderParam.Show, show)



# Add a slider style for the colors
ipg.add_slider_style(style_id="color",
                     rail_color=IpgColor.GREEN, 
                     rail_color_hovered=IpgColor.GREEN_YELLOW,
                     handle_color=IpgColor.DARK_GREEN,
                     rail_width=10.0,
                     rail_border_radius=[8.0],
                     handle_rectangle_width=20,
                     handle_rectangle_border_radius=[5.0],
                     handle_border_width=2.0,
                     handle_border_color=IpgColor.DARK_GREEN,)

# Add the window
ipg.add_window(window_id="main", title="Slider Demo", width=600, height=600,
                pos_x=100, pos_y=25)

# Add the column and center the widgets in it.
ipg.add_column("main", container_id="col",
               align_items=IpgAlignment.Center,
               width_fill=True, height_fill=True, spacing=5)

# Add some instructions, adding spacing since the columns spacing was decreased to 5.
# Optionally I could have added another column with different spacing or used the padding.
ipg.add_space(parent_id="col", height=10)
ipg.add_text(parent_id="col", content="Use the Slider to Change the Values")
ipg.add_space(parent_id="col", height=10.0)

# Equate the bar to get an id for the callback use.
bar_id = ipg.add_progress_bar(parent_id="col", min=0.0, max=100.0, value=50.0, width=300.0)

# Add a slide to change the value with two callbacks
sl_id = ipg.add_slider(parent_id="col", 
                       min=0.0, max=100.0, 
                       step=0.5, value=50.0,
                       width=300.0, 
                       on_change=slider_on_change, 
                       on_release=slider_on_release,
                       )

# Add some value at beginning and end
ipg.add_row(window_id="main", container_id="row_0", parent_id="col",
            width=320.0, padding=[5.0])
min_txt_id = ipg.add_text(parent_id="row_0", content="0")
ipg.add_space(parent_id="row_0", width=220.0)
max_txt_id = ipg.add_text(parent_id="row_0", content="100")

# Add some space for readability
ipg.add_space("col", height=20.0)

# Add a couple of text widget to display some data
on_change_id = ipg.add_text(parent_id="col", content=f"On Change value is 0")
on_release_id = ipg.add_text(parent_id="col", content=f"On Release value is 0")

# Add a little extra spacing
ipg.add_space(parent_id="col", height=20)

# add_rows for buttons
ipg.add_row(window_id="main", container_id="row_1", parent_id="col")
ipg.add_button(parent_id="row_1", label="Press Me to Change Width", on_press=change_width)
ipg.add_button(parent_id="row_1", label="Press Me to Change Height", on_press=change_height)

ipg.add_row(window_id="main", container_id="row_2", parent_id="col")
ipg.add_button(parent_id="row_2", label="Press Me to Change Min", on_press=change_min)
ipg.add_button(parent_id="row_2", label="Press Me to Change Max", on_press=change_max)

ipg.add_row(window_id="main", container_id="row_3", parent_id="col")
ipg.add_button(parent_id="row_3", label="Press Me to Change Step", on_press=change_step)
ipg.add_button(parent_id="row_3", label="Press Me to Change Value", on_press=change_value)

ipg.add_row(window_id="main", container_id="row_4", parent_id="col")
ipg.add_button(parent_id="row_4", label="Press Me to Add Styling", on_press=add_styling)



ipg.add_row(window_id="main", container_id="row_10", parent_id="col")
ipg.add_button(parent_id="row_10", label="Press Me to Toggle Show", on_press=toggle_show)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
