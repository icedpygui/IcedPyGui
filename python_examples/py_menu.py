from icedpygui import IPG, IpgColor
from icedpygui import IpgStyleStandard, IpgSeparatorType
from icedpygui import IpgMenuStyleParam, IpgMenuBarStyleParam


def bar_button_pressed(btn_id: int, data: int):
    print("bar button pressed", data)
    
    
def menu_button_pressed(btn_id: int, data: list):
    print("menu button pressed", data)


def checkbox_toppgled(chk_id: int, checked: bool):
    print("checkbox checked ", checked)


# Not pretty changes, just demoing/testing the technique.
def change_menu_colors(item_id: int, data: list):
    ipg.update_item(item_style, IpgMenuStyleParam.BaseIpgColor, IpgColor.BLUE)
    ipg.update_item(item_style, IpgMenuStyleParam.BorderRadius, [5.0])
    ipg.update_item(item_style, IpgMenuStyleParam.BorderWidth, 6.0)
    ipg.update_item(item_style, IpgMenuStyleParam.BorderIpgColor, IpgColor.LIGHT_BLUE)
    ipg.update_item(item_style, IpgMenuStyleParam.ShadowBlurRadius, 10.0)
    ipg.update_item(item_style, IpgMenuStyleParam.ShadowOffsetXY, [10.0, 10.0])


ipg = IPG()


btn_style = ipg.add_button_style(
                    border_radius=[10.0]
                    )

mb_style = ipg.add_menu_bar_style(
                    base_color=IpgColor.LIGHT_BLUE,
                    border_color=IpgColor.DARK_BLUE,
                    border_radius=[10.0],
                    border_width=4.0,
                    shadow_color=IpgColor.LIGHT_BLUE,
                    shadow_blur_radius=5.0,
                    shadow_offset_xy=[5.0, 5.0],
                    )

item_style = ipg.add_menu_style(
                    base_color=IpgColor.DARK_BLUE,
                    border_color=IpgColor.LIGHT_BLUE,
                    border_radius=[10.0],
                    border_width=4.0,
                    shadow_color=IpgColor.BLUE,
                    shadow_blur_radius=5.0,
                    shadow_offset_xy=[5.0, 5.0],
                    )


# Adding two windows to show color contrasts 
# Add the 1st window, the default theme is Dark
ipg.add_window(
        window_id="main", 
        title="Menu", 
        width=400, 
        height=400,  
        pos_centered=True)


# The menu is a container which allows one to add any widget to
# the menu bar or menu items.  The key note here is that you must
# set the number of bar_items count, followed by a list of the menu items.
# This number of int in the list must equal the number of bar items.
ipg.add_menu(
        window_id="main",
        container_id="menu",
        bar_items=2,
        bar_spacing=10.0,
        menu_items=[4, 4], 
        bar_padding=[5.0],
        item_widths=[100.0], # A list of the widths which must equal the number of menu bar items or list of 1 for all
        item_spacing=[15.0], # A list of the spacings which must equal the number of menu items or list of 1 for all
        item_offset=[15.0],
        menu_bar_style=mb_style,
        menu_style=item_style,
        ) 


# To make a bar item, create your widget and
# have to parent_id equal to the menu container_id.
# NOTE: The order of widget adding is important.
# The bar item comes first, followed by it's menu
# items.  The pattern is repeated until the menu is complete.
# The user_data parameter is used to denote which
# bar or item is pressed and/or any other info you need. 
ipg.add_button(parent_id="menu",
                label="Bar Button1",
                on_press=bar_button_pressed,
                style_id=btn_style,
                user_data=1)

for i in range(0, 4):
    label = f"Text{i}"
    cb=menu_button_pressed
    if i == 3:
        label="base color"
        cb=change_menu_colors
        
    ipg.add_button(parent_id="menu",
                    label=label,
                    style_standard=IpgStyleStandard.Text,
                    user_data=[1, i],
                    on_press=cb)
    
    
#  Second menu bar and items follow
ipg.add_button(parent_id="menu",
                        label="Bar Button2",
                        on_press=bar_button_pressed,
                        style_id=btn_style,
                        user_data=2)
   
for i in range(0, 4):
    if i == 2:
        ipg.add_separator(parent_id="menu",
                          separator_type=IpgSeparatorType.Line,
                          width=100.0,
                          height=20.0)
    elif i == 3:
        ipg.add_checkbox(parent_id="menu",
                         label="Check Me",
                         on_toggle=checkbox_toppgled)
    else:
        ipg.add_button(parent_id="menu",
                        label=f"Text{i}",
                        style_standard=IpgStyleStandard.Text,
                        user_data=[2, i],
                        on_press=menu_button_pressed)



# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
