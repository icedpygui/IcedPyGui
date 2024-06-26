from turtle import color
from icedpygui import IPG, IpgColor
from icedpygui import IpgWindowThemes


ipg = IPG()

main_1 = "main_1"
main_2 = "main_2"

ipg.add_window(main_1, "Scollable - Styling",
                            400, 500,
                            pos_x=100.0, pos_y=100.0)

ipg.add_window(main_2, "Scollable - Styling",
                            400, 500,
                            pos_x=600.0, pos_y=100.0,
                            theme=IpgWindowThemes.SolarizedLight)

# Add a container to center widgets
ipg.add_container(window_id=main_1, container_id="cont0",
                  width_fill=True, height_fill=True,
                  center_xy=True)

# Add column to hold containers
ipg.add_column(window_id=main_1, container_id="col",
               parent_id="cont0")

ipg.add_styling_border(style_id="border", radius=[12.0], width=5.0)

ipg.add_styling_shadow(style_id="shadow", 
                       offset_x=10.0, offset_y=10.0, 
                       blur_radius=20.0,
                       )

# add a default looking container
ipg.add_container(window_id=main_1, container_id="cont1",
                  parent_id="col",
                  width=200.0, height=100.0,
                  center_xy=True)

# Add some text to sow the text_color styling
ipg.add_text(parent_id="cont1", content="Default Styling")

# add another color styling 
ipg.add_styling_color("theme_color",
                      IpgColor.BACKGROUND_THEME, 
                      border_color=IpgColor.ORANGE)

# Add the container with just a border
ipg.add_container(window_id=main_1, container_id="cont2",
                    parent_id="col",
                    width=200.0, height=100.0,
                    center_xy=True,
                    style_color="theme_color",
                    )

# Add some text for info
ipg.add_text(parent_id="cont2", content="Background Theme")

# add some styling before adding container
ipg.add_styling_color("cont_color", 
                      base_color=IpgColor.DARK_BLUE,
                      text_color=IpgColor.LIGHT_BLUE,
                      border_color=IpgColor.DEEP_SKY_BLUE,
                      shadow_color=IpgColor.LIGHT_BLUE)

# Add the container with the styling
ipg.add_container(window_id=main_1, container_id="cont3",
                    parent_id="col",
                    width=200.0, height=100.0,
                    center_xy=True,
                    style_color="cont_color",
                    style_border="border",
                    style_shadow="shadow",
                    )

# Add some text to show the text_color styling
ipg.add_text(parent_id="cont3", content="Color, Border, Shadow Styling")

# **************************************************************************
# Repeating everything, except styling, in window 2 with a light theme
# ensuring things work with another theme background

# Add a container to center widgets
ipg.add_container(window_id=main_2, container_id="cont0",
                  width_fill=True, height_fill=True,
                  center_xy=True)

# Add column to hold containers
ipg.add_column(window_id=main_2, container_id="col",
               parent_id="cont0")

# add a default looking container
ipg.add_container(window_id=main_2, container_id="cont1",
                  parent_id="col",
                  width=200.0, height=100.0,
                  center_xy=True)

# Add some text to sow the text_color styling
ipg.add_text(parent_id="cont1", content="Default Styling")

# Add the container with just a border
ipg.add_container(window_id=main_2, container_id="cont2",
                    parent_id="col",
                    width=200.0, height=100.0,
                    center_xy=True,
                    style_color="theme_color",
                    )

# Add some text for info
ipg.add_text(parent_id="cont2", content="Background Theme")

# Add the container with the styling
ipg.add_container(window_id=main_2, container_id="cont3",
                    parent_id="col",
                    width=200.0, height=100.0,
                    center_xy=True,
                    style_color="cont_color",
                    style_border="border",
                    style_shadow="shadow",
                    )

# Add some text to show the text_color styling
ipg.add_text(parent_id="cont3", content="Color, Border, Shadow Styling")


# Start everthing up
ipg.start_session()
