from icedpygui import IPG, IpgMenuType, IpgWindowTheme, IpgTextParam, IpgMenuParam
from icedpygui import IpgColor, IpgStyleStandard, IpgMenuSeparatorType
from collections import OrderedDict


ipg = IPG()


def menu_pressed(menu_id, bar_index, menu_index, toggled, user_data):
    print(f"menu id={menu_id} bar_index={bar_index} menu_index={menu_index} toggled={toggled} user_data={user_data}")

ipg.add_window(
        window_id="main", 
        title="Menu", 
        width=400, 
        height=500,  
        pos_x=100, 
        pos_y=25)


ipg.add_column(
        window_id="main", 
        container_id="col",
        padding=[5.0],
        spacing=125.0,)

bar_btn = ipg.add_button_style(border_radius=[10.0])
item_btn = ipg.add_button_style(border_radius=[10.0])
item_red_btn = ipg.add_button_style(
                        background_color=IpgColor.RED, 
                        border_radius=[10.0])

bar_btn_dark = ipg.add_button_style(border_radius=[10.0], 
                     border_color=IpgColor.LIGHT_BLUE,
                     border_width=2.0,)

# To modify the separators in the menu, use the separator style
line_blue = ipg.add_menu_separator_style( 
                    separator_type=IpgMenuSeparatorType.Line,
                    separator_color=IpgColor.BLUE)

# [label, type, style_id]
items = [ 
        [("Menu0", IpgMenuType.Button, bar_btn),
         ("Menu1", IpgMenuType.Button, bar_btn),
         ("Menu2", IpgMenuType.Button, bar_btn)
        ],
        [(None, IpgMenuType.Dot, None),
         ("item0-1", IpgMenuType.Text, None), 
         ("item0-2", IpgMenuType.Text, None), 
         ("item0-3", IpgMenuType.Button, item_btn)
        ],
        [("item1-0", IpgMenuType.Text, None), 
         (None, IpgMenuType.Line, line_blue), 
         ("item1-2", IpgMenuType.Text, None), 
         ("item1-3", IpgMenuType.Button, item_red_btn)
        ],
        [("Label-0", IpgMenuType.Label, None),
         ("item2-1", IpgMenuType.Text, None), 
         ("item2-2", IpgMenuType.Checkbox, None), 
         ("item2-3", IpgMenuType.Toggler, None),
         (None, IpgMenuType.Circle, None)
        ]
        ]

bar_widths = [120.0]
item_widths = [100.0, 120.0, 140.0]


item_spacings = [5.0]
item_offsets = [10.0, 40.0, 60.0]

mb_style = ipg.add_menu_bar_style(
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
mn_style = ipg.add_menu_style(
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
                    path_border_radius=[8.0])

ipg.add_menu(
        parent_id="col-dark",
        items=items,
        item_offsets=item_offsets,
        bar_widths=bar_widths,
        item_widths=item_widths,
        menu_bar_style=mb_style,
        menu_style=mn_style)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
