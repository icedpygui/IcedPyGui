from icedpygui import IPG, IpgMenuType, IpgWindowTheme, IpgTextParam, IpgMenuParam
from icedpygui import IpgColor, IpgStyleStandard, IpgMenuSeparatorType
from collections import OrderedDict


ipg = IPG()


def bar_spacing(sld_id: int, value):
    ipg.update_item(slider_1_text, IpgTextParam.Content, str(value))
    ipg.update_item(menu_id, IpgMenuParam.BarSpacing, value)

def bar_height(sld_id: int, value):
    ipg.update_item(slider_2_text, IpgTextParam.Content, str(value))
    ipg.update_item(menu_id, IpgMenuParam.BarHeight, value)

def bar_padding(sld_id: int, value):
    ipg.update_item(slider_3_text, IpgTextParam.Content, str(value))
    ipg.update_item(menu_id, IpgMenuParam.BarPadding, [value])

# The menu callback requires 4 parameters and an optional user_data
# if a checkbox or toggler is used, the status will be the 4th parameter
# or None if not used or selected. Use the indexes to determine which
# widget was selected.
def menu_pressed(menu_id, bar_index, menu_index, toggled, user_data):
    print(f"menu id={menu_id} bar_index={bar_index} menu_index={menu_index} toggled={toggled} user_data={user_data}")

# Adding two windows to show color contrasts 
# Add the 1st window, the default theme is Dark
ipg.add_window("main-dark", "Menu", 
               400, 500,  
               pos_x=100, pos_y=25,
               )

# Add the 2nd window with a lighter theme
# Debug is set to true here to show the effects of
# changing the menu parameters better
ipg.add_window("main-light", "Menu", 
               500, 600,  
               pos_x=600, pos_y=25,
               theme=IpgWindowTheme.GruvboxLight,
               debug=True)

# Add a column container to to each window
# If border is used in the menu and the menu 
# is aligned next to the window border, a little 
# bit of padding will neeed to be added to the 
# container so that the menu border is not cut off.
ipg.add_column("main-dark", 
               container_id="col-dark",
               padding=[5.0],
               spacing=125.0,)

# Add a contatiner and text at top for info
ipg.add_container("main-light", "cont")
ipg.add_text(parent_id="cont", content="The window debug flag is set to better show the changes that occur when the sliders are changed and the offsets for the menu items.")

ipg.add_column("main-light", 
               container_id="col-light",
               spacing=125.0,)

# A menu needs an ordered dictionary since
# the order needs to be maintained.  The key values become the 
# menu bar labels.  The values are a list of tuples which are
# the item labels, if needed.  Not all types of items need a label.
# Use the IpgMenuType to select the different types of items.
# The Text is the standard type.
items = OrderedDict({"Menu0": [(None, IpgMenuType.Dot),
                               ("item0-1", IpgMenuType.Text), 
                               ("item0-2", IpgMenuType.Text), 
                               ("item0-3", IpgMenuType.Button),
                               ],
                     "Menu1": [("item1-0", IpgMenuType.Text), 
                               (None, IpgMenuType.Line), 
                               ("item1-2", IpgMenuType.Text), 
                               ("item1-3", IpgMenuType.Button),
                               ],
                     "Menu2": [("Label-0", IpgMenuType.Label),
                               ("item2-1", IpgMenuType.Text), 
                               ("item2-2", IpgMenuType.Checkbox), 
                               ("item2-3", IpgMenuType.Toggler),
                               (None, IpgMenuType.Circle),
                               ]})

# Add styling to all the buttons
# To add styling to any widget, use that widgets stle command.
ipg.add_button_style("btn", border_radius=[10.0])
ipg.add_button_style("bar-btn-dark", border_radius=[10.0], 
                     border_color=IpgColor.LIGHT_BLUE,
                     border_width=2.0,)

# To modify the separators in the menu, use the separator style
ipg.add_menu_separator_style(style_id="line-dark", 
                            separator_type=IpgMenuSeparatorType.Line,
                             separator_color=IpgColor.BLUE)


ipg.add_button_style("bar-btn-light", border_radius=[10.0])



# A list of the widths which must equal the number of menu bar labels, the dict keys,
# or they must be a list of 1 number, i.e. [90.0] to indicate all widths are 90.0.
bar_widths = [120.0]
item_widths = [100.0, 120.0, 140.0]

# A list of the spacings in each column of items.  The size of the list  must 
# equal the number of menu bar labels, the dict keys,
# or they must be a list of 1 number, i.e. [5.0] to indicate all spacings of 5.0
item_spacings = [5.0]
item_offsets = [10.0, 40.0, 60.0]
# Add the menus to each window.
# The all styling takes a tuple (IpgStandardStyle, style_id),
# both items are optional.  Use None to default them.
# If None is the IpgStandardStyle, then the style will default
# to the primary style standard and any styling using the
# style method will replace the default style parameters.
# For example, the default primary style for the button has a 
# Dodger blue color and a square shape.  Using the styling for
# the button, you can change all of the parameters.  If you want
# to use a standard color, use the IpgStandardStyle to select the color
# and then the button style to change any other parameters.
# NOTE: The separators circle, dot, label, and line parameter 
# are not tuples but a str because they have no standard style.
ipg.add_menu("col-dark", items,
             bar_widths=bar_widths,
            item_widths=item_widths,
            on_select=menu_pressed,
            button_bar_style_all=(None, "bar-btn-dark"),
            button_item_style_all=(None, "btn"),
            line_item_style_all="line-dark", # not a tuple
            user_data="I'm in the dark window")


# Create the bar styling
# When adding a border, if the left appears to be cut off,
# then add some left padding to the container that the menu
# is in.
ipg.add_menu_bar_style(style_id="mb_style",
                       base_color=IpgColor.DIM_GREY,
                       border_width=2.0, 
                       border_color=IpgColor.WHITE,
                       border_radius=[20.0], # the default style is [8.0]
                       shadow_color=IpgColor.LIGHT_GREY,
                       shadow_offset_x=5.0,
                       shadow_offset_y=5.0,
                       shadow_blur_radius=10.0,
                       )

# The path style changes the style of the bar button
# when the menu is displayed.  The path should be
# a contrast to the bar styling.
ipg.add_menu_style(style_id="mn_style",
                       base_color=IpgColor.DIM_GREY,
                       border_width=2.0, 
                       border_color=IpgColor.WHITE,
                       border_radius=[20.0], # the default style is [8.0]
                       shadow_color=IpgColor.LIGHT_GREY,
                       shadow_offset_x=5.0,
                       shadow_offset_y=5.0,
                       shadow_blur_radius=10.0,
                       path_base_color=IpgColor.DIM_GREY,
                       path_border_color=IpgColor.ANTIQUE_WHITE,
                       path_border_width=2.0,
                       path_border_radius=[8.0],
                       )

# Add some styling to the menu bar and menu
ipg.add_menu("col-dark", items,
            bar_widths=bar_widths,
            item_widths=item_widths,
            menu_bar_style="mb_style",
            menu_style="mn_style",
            )


# Adding to second window

menu_id = ipg.add_menu("col-light", items,
                       bar_widths=bar_widths, 
                        item_widths=item_widths, 
                        item_spacings=item_spacings,
                        item_offsets=item_offsets,
                        on_select=menu_pressed,
                        button_bar_style_all=(IpgStyleStandard.Danger, "bar-btn-light"),
                        button_item_style_all=(None, "btn"), 
                        user_data="I'm in the light window")

# adding some controls to see the effect of bar parameters
ipg.add_column("main-light", "controls",
               parent_id="col-light", spacing=20.0)

ipg.add_row(window_id="main-light", container_id="slider-1", 
            parent_id="controls")

ipg.add_text(parent_id="slider-1", content="Bar Spacing")

ipg.add_slider(parent_id="slider-1", 
               min=0.0, max=30.0, step=1.0,
               value=0.0, width=200.0,
               on_change=bar_spacing)
slider_1_text = ipg.add_text(parent_id="slider-1", content=f"{0.0}")


# adding some controls to see the effect of bar parameters
ipg.add_text(parent_id="controls", content="Until height > size of bar items, the height will not increase.")
ipg.add_row(window_id="main-light", container_id="slider-2", 
            parent_id="controls")

ipg.add_text(parent_id="slider-2", content="Bar Height")

ipg.add_slider(parent_id="slider-2", 
               min=0.0, max=100.0, step=1.0,
               value=0.0, width=200.0,
               on_change=bar_height)

slider_2_text = ipg.add_text(parent_id="slider-2", content=f"{32.0}")

# adding some controls to see the effect of bar parameters
ipg.add_text(parent_id="controls", content="Reset the height to zero then see padding effect. \n Otherwise, the surrounding padding will quish to bar items when big enough")
ipg.add_row(window_id="main-light", container_id="slider-3", 
            parent_id="controls")

ipg.add_text(parent_id="slider-3", content="Bar Padding")

ipg.add_slider(parent_id="slider-3", 
               min=0.0, max=50.0, step=1.0,
               value=0.0, width=200.0,
               on_change=bar_padding)

slider_3_text = ipg.add_text(parent_id="slider-3", content=f"{0.0}")



# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
