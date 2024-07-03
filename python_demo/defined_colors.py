from icedpygui import IPG, IpgColor


"""
Styling color rules:

    Of the three containers, Container, Column, and Row, the only one that has styling is the container.
    If you want styling for the other two, place them within a Container.

    The styling_color has many options for styling various widgets.  You will need to use the proper 
    parameter for a given widget.  For example, to style a scrollbar, use the scrollbar_color or 
    scrollbar_rgba.  The base_color or base_rgba is used for the background and if no other colors
    are given, it will generate the other colors by generating a base, weak, and strong color for 
    items such as mouse hover or drag, text color, etc.  If these are not to your liking, supply
    whatever color you would like.

    Each widget has a styling note associated with the hint.

    Along with the theme colors used in Iced, IPG also uses colors obtained from
    https://www.w3schools.com/cssref/css_colors.php.

    This program generates the colors in a container along with the name, base, weak, and strong
    variations.  The text color is generated automatically to be readable but can also be defined
    by the user.

"""


ipg = IPG()

colors = [IpgColor.PRIMARY, IpgColor.SECONDARY, IpgColor.SUCCESS, IpgColor.DANGER, IpgColor.WARNING, 
          IpgColor.INFO, IpgColor.LIGHT, IpgColor.DARK, IpgColor.ALICE_BLUE, IpgColor.ANTIQUE_WHITE, 
          IpgColor.AQUA, IpgColor.AQUAMARINE, IpgColor.AZURE, IpgColor.BEIGE, IpgColor.BISQUE, 
          IpgColor.BLACK, IpgColor.BLANCHED_ALMOND, IpgColor.BLUE, IpgColor.BLUE_VIOLET, IpgColor.BROWN, 
          IpgColor.BURLY_WOOD, IpgColor.CADET_BLUE, IpgColor.CHARTREUSE, IpgColor.CHOCOLATE, IpgColor.CORAL, 
          IpgColor.CORNFLOWER_BLUE, IpgColor.CORNSILK, IpgColor.CRIMSON, IpgColor.CYAN, IpgColor.DARK_BLUE, 
          IpgColor.DARK_CYAN, IpgColor.DARK_GOLDEN_ROD, IpgColor.DARK_GRAY, IpgColor.DARK_GREY, 
          IpgColor.DARK_GREEN, IpgColor.DARK_KHAKI, IpgColor.DARK_MAGENTA, IpgColor.DARK_OLIVE_GREEN, 
          IpgColor.DARK_ORANGE, IpgColor.DARK_ORCHID, IpgColor.DARK_RED, IpgColor.DARK_SALMON, 
          IpgColor.DARK_SEA_GREEN, IpgColor.DARK_SLATE_BLUE, IpgColor.DARK_SLATE_GRAY, IpgColor.DARK_SLATE_GREY, 
          IpgColor.DARK_TURQUOISE, IpgColor.DARK_VIOLET, IpgColor.DEEP_PINK, IpgColor.DEEP_SKY_BLUE, 
          IpgColor.DIM_GRAY, IpgColor.DIM_GREY, IpgColor.DODGER_BLUE, IpgColor.FIRE_BRICK, IpgColor.FLORAL_WHITE, 
          IpgColor.FOREST_GREEN, IpgColor.FUCHSIA, IpgColor.GAINSBORO, IpgColor.GHOST_WHITE, IpgColor.GOLD, 
          IpgColor.GOLDEN_ROD, IpgColor.GRAY, IpgColor.GREY, IpgColor.GREEN, IpgColor.GREEN_YELLOW, IpgColor.HONEY_DEW, 
          IpgColor.HOT_PINK, IpgColor.INDIAN_RED, IpgColor.INDIGO, IpgColor.IVORY, IpgColor.KHAKI, IpgColor.LAVENDER, 
          IpgColor.LAVENDER_BLUSH, IpgColor.LAWN_GREEN, IpgColor.LEMON_CHIFFON, IpgColor.LIGHT_BLUE, IpgColor.LIGHT_CORAL, 
          IpgColor.LIGHT_CYAN, IpgColor.LIGHT_GOLDEN_ROD_YELLOW, IpgColor.LIGHT_GRAY, IpgColor.LIGHT_GREY, 
          IpgColor.LIGHT_GREEN, IpgColor.LIGHT_PINK, IpgColor.LIGHT_SALMON, IpgColor.LIGHT_SEA_GREEN, 
          IpgColor.LIGHT_SKY_BLUE, IpgColor.LIGHT_SLATE_GRAY, IpgColor.LIGHT_SLATE_GREY, IpgColor.LIGHT_STEEL_BLUE, 
          IpgColor.LIGHT_YELLOW, IpgColor.LIME, IpgColor.LIME_GREEN, IpgColor.LINEN, IpgColor.MAGENTA, IpgColor.MAROON, 
          IpgColor.MEDIUM_AQUA_MARINE, IpgColor.MEDIUM_BLUE, IpgColor.MEDIUM_ORCHID, IpgColor.MEDIUM_PURPLE, 
          IpgColor.MEDIUM_SEA_GREEN, IpgColor.MEDIUM_SLATE_BLUE, IpgColor.MEDIUM_SPRING_GREEN, IpgColor.MEDIUM_TURQUOISE, 
          IpgColor.MEDIUM_VIOLET_RED, IpgColor.MIDNIGHT_BLUE, IpgColor.MINT_CREAM, IpgColor.MISTY_ROSE, IpgColor.MOCCASIN, 
          IpgColor.NAVAJO_WHITE, IpgColor.NAVY, IpgColor.OLD_LACE, IpgColor.OLIVE, IpgColor.OLIVE_DRAB, IpgColor.ORANGE, 
          IpgColor.ORANGE_RED, IpgColor.ORCHID, IpgColor.PALE_GOLDEN_ROD, IpgColor.PALE_GREEN, IpgColor.PALE_TURQUOISE, 
          IpgColor.PALE_VIOLET_RED, IpgColor.PAPAYA_WHIP, IpgColor.PEACH_PUFF, IpgColor.PERU, IpgColor.PINK, IpgColor.PLUM,
          IpgColor.POWDER_BLUE, IpgColor.PURPLE, IpgColor.REBECCA_PURPLE, IpgColor.RED, IpgColor.ROSY_BROWN, 
          IpgColor.ROYAL_BLUE, IpgColor.SADDLE_BROWN, IpgColor.SALMON, IpgColor.SANDY_BROWN, IpgColor.SEA_GREEN, 
          IpgColor.SEA_SHELL, IpgColor.SIENNA, IpgColor.SILVER, IpgColor.SKY_BLUE, IpgColor.SLATE_BLUE, 
          IpgColor.SLATE_GRAY, IpgColor.SLATE_GREY, IpgColor.SNOW, IpgColor.SPRING_GREEN, IpgColor.STEEL_BLUE, 
          IpgColor.TAN, IpgColor.TEAL, IpgColor.THISTLE, IpgColor.TOMATO, IpgColor.TRANSPARENT, IpgColor.TURQUOISE, 
          IpgColor.VIOLET, IpgColor.WHEAT, IpgColor.WHITE, IpgColor.WHITE_SMOKE, IpgColor.YELLOW, IpgColor.YELLOW_GREEN]

widths = [300, 150, 150]
# Add the window first
ipg.add_window("main", "Menu", 
               600, 600,  
               pos_x=100, pos_y=25,
               )

headers = ["Base Color", "Weak Color", "Strong Color"]

# add row with some padding on top
ipg.add_row(window_id="main", 
            container_id="info_row",
            width_fill=True,
            padding=[20.0, 0.0, 0.0, 0.0])

for i in range(0, 3):
    ipg.add_container(window_id="main", 
                    container_id=f"info{i}",
                    parent_id="info_row",
                    center_xy=True,
                    width=widths[i],
                    )
    
    ipg.add_text(parent_id=f"info{i}", content=headers[0])


ipg.add_scrollable(window_id="main", container_id="scroll", height=550.0, width_fill=True)

# Add a column container to hold everything
ipg.add_column(window_id="main", parent_id="scroll", container_id="col", 
               width_fill=True, spacing=0.0)

for (i, color) in enumerate(colors):

    ipg.add_row(window_id="main", parent_id="col", container_id=f"row{i}", padding=[0.0])

    color_name = str(color)[9:]

    ipg.add_styling_color(style_id=f"cont{i}", base_color=color)

    ipg.add_container("main", container_id=f"cont{i}",
                        parent_id=f"row{i}",
                        style_color= f"cont{i}",
                        width=widths[0], height=30.0,
                        center_xy=True, padding=[0.0])

    ipg.add_text(f"cont{i}", f"{color_name}")

    

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
