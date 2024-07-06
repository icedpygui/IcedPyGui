from icedpygui import IPG, IpgColor, IpgRowAlignment, IpgStyleStandard

ipg = IPG()


def on_press(btn_id):
    print("button pressed")

# Styling must be added before they are used since the widget or container
# needs the styling_id.  The style_id does not need to be unique
# between the different styles, only within the style type.
# The same style type can be used for many widgets and containers.

# A border and shadow can be added as indicated below.  The color of these will
# be automatically determined based on the base color of the widget.
# If you you want a different color for these styles, you will have to use
# the styling_custom command and supply your own colors.  There is a defined_colors
# example that shows all of the current colors you can use and if those are not
# suitable, use the rgba values. 


# When using custom colors, the text color is calculated based on the 
# background but may not be visible enough, if so, add the text_color 
# of your choice.
ipg.add_button_style(style_id="custom", 
                      base_color=IpgColor.YELLOW,
                      border_color=IpgColor.DARK_GOLDEN_ROD,
                      shadow_color=IpgColor.LIGHT_YELLOW,
                      text_color=IpgColor.BLACK,
                      border_radius=[12.0], 
                      border_width=5.0,
                      shadow_offset_x=0.0, 
                      shadow_offset_y=0.0, 
                      shadow_blur_radius=15.0)

ipg.add_button_style(style_id="primary", 
                      base_color=IpgColor.PRIMARY)

# Add the window first
ipg.add_window("main", "Menu", 800, 600,  pos_x=100, pos_y=25)

# Add a container to hold everything aligning all in the center
ipg.add_container(window_id="main", container_id="cont", 
                  center_xy=True, width_fill=True, height_fill=True,
                  )
# Add a column to hold multiple widgets, vertically.
ipg.add_column(window_id="main", container_id="col", parent_id="cont")

# Add a row to hold widgtes, horizontally.
ipg.add_row(window_id="main", container_id="row_btn", parent_id="col",
            align_items=IpgRowAlignment.Center)

# Add buttoms
ipg.add_button(parent_id="row_btn", label="Default", 
               on_press=on_press)

ipg.add_button(parent_id="row_btn", label="Primary", 
               on_press=on_press,
               style_standard=IpgStyleStandard.Primary,
               )

ipg.add_button(parent_id="row_btn", label="Success", 
               on_press=on_press,
               style_standard=IpgStyleStandard.Success,
                )

ipg.add_button(parent_id="row_btn", label="Danger", 
               on_press=on_press,
               style_standard=IpgStyleStandard.Danger,
                )

ipg.add_button(parent_id="row_btn", label="Text", 
               on_press=on_press,
               style_standard=IpgStyleStandard.Text)

ipg.add_row(window_id="main", container_id="row_btn2", parent_id="col",
            align_items=IpgRowAlignment.Center)

ipg.add_button(parent_id="row_btn2", label="Custom Primary",
               style="primary", 
               )


ipg.start_session()
