from icedpygui import IPG, IpgCheckboxParams, IpgColumnAlignment, IpgColor
from icedpygui import IpgWindowThemes, IpgStyleStandard

# This is a simple demo to change the checkmark of the checkbox to an x.
ipg = IPG()

# A technique explored in another demo where the
# id is generated ahead of time.  Useful in some cases.
# This id will work across windows so if you want the same
# behavior for a windget that is on multiple windows
# then use the gen_id on those widgets to change them all.
# However, this id cannot be used more than once within a window.
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

def add_checkboxes():
    ipg.add_checkbox(parent_id="row", label="Primary",
                    style_standard=IpgStyleStandard.Primary,
                    on_toggle=on_toggle)

    ipg.add_checkbox(parent_id="row", label="Success",
                    style_standard=IpgStyleStandard.Success,
                    on_toggle=on_toggle)

    ipg.add_checkbox(parent_id="row", label="Danger",
                    style_standard=IpgStyleStandard.Danger,
                    on_toggle=on_toggle)
    
    # Add the second checkbox.  This has no callback since it not used.
    ipg.add_checkbox(parent_id="col", gen_id=x_id, 
                    label="See my check change to an x and color change",
                    style_color="unchecked_colors",
                    style_border="border", 
                    is_checked=True)


# Unlike widgets and containers which need a window, styling can be added
# at any time before use.
ipg.add_styling_color(style_id="checked_colors", 
                      base_color=IpgColor.BLUE,
                      border_color=IpgColor.GOLDEN_ROD,
                      icon_color=IpgColor.LIGHT_BLUE,
                      text_color=IpgColor.BLUE)

ipg.add_styling_color(style_id="unchecked_colors", 
                      base_color=IpgColor.YELLOW,
                      border_color=IpgColor.RED,
                      icon_color=IpgColor.RED,
                      )

# Standard style, even though Text is listed in the hint, there is
# no Text for a checkbox
ipg.add_styling_standard("std_p", IpgStyleStandard.Primary)
ipg.add_styling_standard("std_s", IpgStyleStandard.Success)
ipg.add_styling_standard("std_d", IpgStyleStandard.Danger)


ipg.add_styling_border(style_id="border", radius=[3.0], width=3.0)


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

# Adding a row for the horizontal alignment of the checkboxes
ipg.add_row("main", container_id="row", parent_id="col", spacing=10.0)

# Adding checkboxes
add_checkboxes()


# Let's add another window with a different background theme
# to see how things look
ipg.add_window("main2", "CheckBox Demo",
               600, 600,  pos_x=750, pos_y=25,
               theme=IpgWindowThemes.SolarizedLight)

ipg.add_container(window_id="main2", container_id="cont", width_fill=True,
                  height_fill=True, center_xy=True)

ipg.add_column(window_id="main2", container_id="col", parent_id="cont",
               align_items=IpgColumnAlignment.Center)

# Adding a row for the horizontal alignment of the checkboxes
ipg.add_row("main2", container_id="row", parent_id="col", spacing=10.0)

# Adding checkboxes
add_checkboxes()

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
