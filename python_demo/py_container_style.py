from turtle import color
from icedpygui import IPG, IpgColor
from icedpygui import IpgContainerAlignment


ipg = IPG()


ipg.add_window("main", "Scollable - Styling",
                            600, 600,
                            pos_centered=True)

# add a container for centering
ipg.add_container(window_id="main", container_id="main_cont",
                  width_fill=True, height_fill=True,
                  center_xy=True)

# add some styling before adding container
ipg.add_styling_background("bkg", color=IpgColor.DARK_BLUE)
ipg.add_styling_text_color("text", color=IpgColor.ORANGE)
ipg.add_styling_border(style_id="border", radius=[12.0], color=IpgColor.YELLOW)
ipg.add_styling_shadow(style_id="shadow", color=IpgColor.LIGHT_BLUE,
                       offset_x=10.0, offset_y=20.0, blur_radius=20.0,
                       invert=True, scale_alpha=0.5)

# Add the container with the styling
ipg.add_container(window_id="main", container_id="cont",
                    parent_id="main_cont",
                    width=200.0, height=200.0,
                    center_xy=True,
                    style_background="bkg",
                    style_text_color="text",
                    style_border="border",
                    style_shadow="shadow",
                    )

# Add some text to sow the text_color styling
ipg.add_text(parent_id="cont", content="Some content")


# Start everthing up
ipg.start_session()
