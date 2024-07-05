from icedpygui import IPG, IpgColor


"""
Styling color notes:

    Of the three containers, Container, Column, and Row, the only one that has styling is the container.
    If you want styling for the other two, place them within a Container.

    If no styling is supplied, then the default theme styling will be used.  However, you can used the
    style_standard option for each widget to select a limited number of styles such as primary, success,
    and danger.  The theme colors are limited at this time but during the next revision, a custom
    theme will be implemented.  You can however use any othe the IpgColors to style your widgets.  There
    are 155 colors (https://www.w3schools.com/cssref/css_colors.php) to choose from or make up you 
    own using the rgba option. 

    There are 4 special color parameters used in add_styling_colors,  base, strong, weak, and text.
    These colors are used to color multiple parts of the widget and show mouse hove and drag effects. 
    There are also additional color parameters where more colors for a widget may be neededs, for
    example border color.
    
    If you supply only the base, IPG will generate the strong, weak, and text colors for you.  
    This demo demostrates that for you.  There is a special widget, get_palette colors that
    will get the additional colors for you so that you can determine if those are suitable.
    However, if you just supply any base color, IPG will automatically use them in the widget.

    If you don't like the strong and weak colors, just supply them yourself.
    
    Important:  Each widget has a styling note associated with the hint which indicates which
    color is used for each part of the widget.  In some cases, an additional color is needed
    beyond the 3 basic ones.  These are supplied by you.  For example a border color for a
    container.  The styling_border only changes the width and corner radius, you'll need to 
    add the styling_color with the border option to add color to the border. 
    
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

# global to define the 3 column widths
widths = [260, 150, 150]

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
    # Add the 3 containers for the header text
    ipg.add_container(window_id="main", 
                    container_id=f"info{i}",
                    parent_id="info_row",
                    center_xy=True,
                    width=widths[i],
                    )
    
    ipg.add_text(parent_id=f"info{i}", content=headers[i])

# Add a scrollable container for all of the colors
ipg.add_scrollable(window_id="main", container_id="scroll", height=550.0, width_fill=True)

# Add a column container to hold everything
ipg.add_column(window_id="main", parent_id="scroll", container_id="col", 
               width_fill=True, spacing=0.0)

for (i, color) in enumerate(colors):
    # Add a row for each color set
    ipg.add_row(window_id="main", parent_id="col", container_id=f"row{i}", padding=[0.0])

    # get the name by cutting off the first 9 characters
    color_name = str(color)[9:]

    # Get the 3 colors based on the given IpgColor
    # These return colors are rgba
    (strong, weak, text) = ipg.get_color_palette(color)

    # create styling for the 3 containers
    ipg.add_styling_color(style_id=f"base{i}", 
                          base_color=color)
    ipg.add_styling_color(style_id=f"strong{i}", 
                          base_rgba=strong)
    ipg.add_styling_color(style_id=f"weak{i}", 
                          base_rgba=weak)

    # add the 3 containers
    ipg.add_container("main", container_id=f"cont1{i}",
                        parent_id=f"row{i}",
                        style_color= f"base{i}",
                        width=widths[0], height=30.0,
                        center_xy=True, padding=[0.0])
    ipg.add_container("main", container_id=f"cont2{i}",
                        parent_id=f"row{i}",
                        style_color= f"strong{i}",
                        width=widths[1], height=30.0,
                        center_xy=True, padding=[0.0])
    ipg.add_container("main", container_id=f"cont3{i}",
                        parent_id=f"row{i}",
                        style_color= f"weak{i}",
                        width=widths[2], height=30.0,
                        center_xy=True, padding=[0.0])

    ipg.add_text(f"cont1{i}", f"{color_name}")



# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
