from icedpygui import IPG, IpgCheckboxParams, IpgColumnAlignment, IpgColor
from icedpygui import IpgWindowThemes, IpgStyleStandard

#  Required to instantiate IPG
ipg = IPG()

# Add some stard styled checkboxes
def add_checkboxes():
    ipg.add_checkbox(parent_id="row", label="Primary",
                    style_standard=IpgStyleStandard.Primary,
                    )

    ipg.add_checkbox(parent_id="row", label="Success",
                    style_standard=IpgStyleStandard.Success,
                    )

    ipg.add_checkbox(parent_id="row", label="Danger",
                    style_standard=IpgStyleStandard.Danger,
                    )
    # Add a checkbox with custom styling.
    ipg.add_checkbox(parent_id="col", 
                    label="I have custon styling",
                    style_color="colors_no_border",
                    style_border="border", 
                    )
    
    # Add a checkbox with custom styling.
    ipg.add_checkbox(parent_id="col", 
                    label="I have custon styling with ugly border",
                    style_color="colors_with_border",
                    style_border="border", 
                    )


def add_info():
    text = "Using the mouse, check, uncheck, and hover to see the style changes\n" 
    text += "The standard style colors depends on the window theme color\n"
    text += "The custom style colors will not change based on the theme color\n"
    text +="For the checkbox, the base color will be the border color if it's not defined\n"

    ipg.add_text(parent_id="col", content=text)



# The styling can be added at any time before use.
# No border color is set here so the unchecked border 
# should be the base color.
ipg.add_styling_color(style_id="colors_no_border", 
                      base_color=IpgColor.BLUE,
                      icon_color=IpgColor.LIGHT_BLUE,
                      text_color=IpgColor.BLUE)

# Border color defined here.
ipg.add_styling_color(style_id="colors_with_border", 
                      base_color=IpgColor.BLUE,
                      border_color=IpgColor.YELLOW,
                      icon_color=IpgColor.LIGHT_BLUE,
                      text_color=IpgColor.BLUE)

# The default border is 1 and the radius is 2, these are used just to exagerate.
ipg.add_styling_border(style_id="border", radius=[4.0], width=2.0)


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

add_info()

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

add_info()

# Adding a row for the horizontal alignment of the checkboxes
ipg.add_row("main2", container_id="row", parent_id="col", spacing=10.0)

# Adding checkboxes
add_checkboxes()

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
