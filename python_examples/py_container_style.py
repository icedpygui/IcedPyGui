from icedpygui import IPG, IpgColor
from icedpygui import IpgWindowTheme


ipg = IPG()

main_1 = "main_1"
main_2 = "main_2"

ipg.add_window(main_1, "Container - Styling",
                            400, 500,
                            pos_x=100.0, pos_y=100.0)

ipg.add_window(main_2, "Container - Styling",
                            400, 500,
                            pos_x=600.0, pos_y=100.0,
                            theme=IpgWindowTheme.SolarizedLight)

# Add just the theme background
ipg.add_container_style(style_id="theme_bkg", background_color=IpgColor.BACKGROUND_THEME)

# add some styling before adding container
ipg.add_container_style("cont_color", 
                        background_color=IpgColor.DARK_BLUE,
                        text_color=IpgColor.LIGHT_BLUE,
                        border_color=IpgColor.DEEP_SKY_BLUE,
                        shadow_color=IpgColor.LIGHT_BLUE,
                        border_radius=[12.0], border_width=5.0,
                        shadow_offset_x=10.0, shadow_offset_y=10.0, 
                        shadow_blur_radius=20.0)


# Add a container to center widgets
ipg.add_container(window_id=main_1, container_id="cont0",
                  width_fill=True, height_fill=True,)

# Add column to hold containers
ipg.add_column(window_id=main_1, container_id="col",
               parent_id="cont0")

# add a default looking container
ipg.add_container(window_id=main_1, container_id="cont1",
                  parent_id="col",
                  width=200.0, height=100.0,)

# Add some text to sow the text_color styling
ipg.add_text(parent_id="cont1", content="Default Styling")

# add a container with a theme background
ipg.add_container(window_id=main_1, container_id="cont2",
                  parent_id="col",
                  width=200.0, height=100.0,
                  style_id="theme_bkg")

# Add some text for info
ipg.add_text(parent_id="cont2", content="Theme Background")

# Add the container with the styling
ipg.add_container(window_id=main_1, container_id="cont3",
                    parent_id="col",
                    width=200.0, height=100.0,
                    style_id="cont_color",
                    )

# Add some text for info
ipg.add_text(parent_id="cont3", content="Background Color, Border, Shadow Styling")

# **************************************************************************
# Repeating everything, except styling, in window 2 with a light theme
# ensuring things work with another theme background

# Add a container to center widgets
ipg.add_container(window_id=main_2, container_id="cont0",
                  width_fill=True, height_fill=True)

# Add column to hold containers
ipg.add_column(window_id=main_2, container_id="col",
               parent_id="cont0")

# add a default looking container
ipg.add_container(window_id=main_2, container_id="cont1",
                  parent_id="col",
                  width=200.0, height=100.0)

# Add some text for info
ipg.add_text(parent_id="cont1", content="Default Styling")

# add a container with a theme background
ipg.add_container(window_id=main_2, container_id="cont2",
                  parent_id="col",
                  width=200.0, height=100.0,
                  style_id="theme_bkg")

# Add some text for info
ipg.add_text(parent_id="cont2", content="Theme Background")

# Add the container with the styling
ipg.add_container(window_id=main_2, container_id="cont3",
                    parent_id="col",
                    width=200.0, height=100.0,
                    style_id="cont_color",
                    )

# Add some text for info
ipg.add_text(parent_id="cont3", content="Background Color, Border, Shadow Styling")


# Start everything up
ipg.start_session()
