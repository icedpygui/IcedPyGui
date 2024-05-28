from icedpygui import IPG, IpgColor, IpgButtonStyles

ipg = IPG()


# Add the window first
ipg.add_window("main", "Menu", 500, 600,  pos_x=100, pos_y=25)

ipg.add_container(window_id="main", container_id="cont", 
                  center_xy=True, width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col", parent_id="cont")

ipg.add_button(parent_id="col", label="Press Me", style=IpgButtonStyles.Primary)

btn = ipg.add_button(parent_id="col", label="Press Me", style_custom=True)

ipg.add_styling_background(widget_id=btn, color=IpgColor.DARK_GREEN)
ipg.add_styling_text_color(widget_id=btn, color=IpgColor.FLORAL_WHITE)
ipg.add_styling_border(widget_id=btn, radius=[15.0])
ipg.add_styling_shadow(widget_id=btn, color=IpgColor.LIGHT_GREEN, offset_x=5.0, offset_y=5.0, blur_radius=10.0)


ipg.start_session()
