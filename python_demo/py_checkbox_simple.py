from icedpygui import IPG, IpgCheckboxParams, IpgColumnAlignment, IpgColor

# This is a simple demo to change the checkmark of the checkbox to an x.
ipg = IPG()

# A technique explored in another demo where the
# id is generated ahead of time.  Useful in some cases.
x_id = ipg.generate_id()


# The callback used for the checkbox.
# This callback id for this is not used since we are
# changing the other checkbox.  Therefore we needed to know
# its id.  Normally you would probably use a class to store theses
# needed ids for later use.
# The is_checked is a boolean which will toggle each time the
# checkbox is clicked.
def on_toggle(_chkbx_id: int, is_checked: bool):
    # Changing the checkmark to  an x
    ipg.update_item(x_id, IpgCheckboxParams.IconX, is_checked)
    # changing the background using the style_id of "bkg", doesn't toggle unless
    # another styling is chosen based on the value of is_checked.
    if is_checked:
        ipg.update_item(x_id, IpgCheckboxParams.StyleColor, "checked_colors")
    else:
        ipg.update_item(x_id, IpgCheckboxParams.StyleColor, "unchecked_colors")


# Add a window first
ipg.add_window("main", "CheckBox Demo",
               600, 600,  pos_x=100, pos_y=25)

# Add a container to center the widgets in the middle
ipg.add_container(window_id="main", container_id="cont", width_fill=True,
                  height_fill=True, center_xy=True)

# Since a container can only hold one widget, use a column to hold the
# two checkboxes.  We let the width and height default to shrink, so no entry.
# The alignment defaults to Start but for demonstration purpose, we
# added the IpgColumnAlignment.Start
ipg.add_column(window_id="main", container_id="col", parent_id="cont",
               align_items=IpgColumnAlignment.Center)

ipg.add_styling_color(style_id="checked_colors", 
                      base_color=IpgColor.BLUE,
                      border_color=IpgColor.GOLDEN_ROD,
                      icon_color=IpgColor.LIGHT_BLUE,
                      text_color=IpgColor.YELLOW)

ipg.add_styling_color(style_id="unchecked_colors", 
                      base_color=IpgColor.YELLOW,
                      border_color=IpgColor.RED,
                      icon_color=IpgColor.RED,
                      text_color=IpgColor.WHITE)

ipg.add_styling_border(style_id="border", radius=[3.0], width=3.0)


# Add the first checkbox with the callback on_toggle.
ipg.add_checkbox(parent_id="col", label="Check Me!!!",
                 style_border="border",
                 on_toggle=on_toggle)

# Add the second checkbox.  This has no callback since it not used.
ipg.add_checkbox(parent_id="col", gen_id=x_id, 
                 label="See my check change to an x and color change",
                 style_color="unchecked_colors",
                 style_border="border", 
                 is_checked=True)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
