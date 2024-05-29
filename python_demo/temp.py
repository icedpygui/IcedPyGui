from icedpygui import IPG, IpgColor, IpgButtonStyles

ipg = IPG()


# Styling must be added before they are used since the widget or container
# needs the styling_id.  The style_id does not need to be unique
# between the different styles, only within the style type.
# The same style type can be used for many widgets and containers.
ipg.add_styling_background(style_id="button1", color=IpgColor.PRIMARY)
ipg.add_styling_background(style_id="button2", color=IpgColor.DARK_GREEN)
ipg.add_styling_border(style_id="button", radius=[12.0])
ipg.add_styling_shadow(style_id="button2", color=IpgColor.LIGHT_GREEN, 
                       offset_x=5.0, offset_y=5.0, blur_radius=10.0)
ipg.add_styling_text_color(style_id="button", color=IpgColor.FLORAL_WHITE)


# Add the window first
ipg.add_window("main", "Menu", 500, 600,  pos_x=100, pos_y=25)

ipg.add_container(window_id="main", container_id="cont", 
                  center_xy=True, width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col", parent_id="cont")

ipg.add_button(parent_id="col", label="Press Me", 
               style_background="button1", 
               style_border="button",
               style_text_color="button")

ipg.add_button(parent_id="col", label="Press Me", 
               style_background="button2", 
               style_border="button",
               style_text_color="button",
               style_shadow="button2")




ipg.start_session()
