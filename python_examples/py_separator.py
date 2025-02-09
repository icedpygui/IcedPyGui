from icedpygui import IPG, IpgAlignment, IpgHorizontalAlignment, IpgVerticalAlignment, IpgSeparatorType
from icedpygui import IpgColor


# This is a simple demo to change the checkmark of the checkbox to an x.
ipg = IPG()

# styles


# The default style results in a dot, but by styling
# the dot, you can make the center transparent
# the add_seorator will need to have the border_width set
dot_style = ipg.add_separator_style(
                        ipg_color=IpgColor.TRANSPARENT,
                        border_ipg_color=IpgColor.WHITE)


# Add a window first
# if you make debug=True, you can see 
# the outline of the box containing
# the separators and the effect of the 
# width and height settings.
ipg.add_window(
        window_id="main", 
        title="Separator Demo",
        width=600, 
        height=600,  
        pos_centered=True,
        debug=False)

ipg.add_container(
        window_id="main",
        container_id="cont",
        width_fill=True,
        height_fill=True)

# Add a column to hold the widgets
ipg.add_column(
        window_id="main", 
        container_id="col",
        parent_id="cont")

# for the dot, the width and height are not used
# unless one wants a sort of padding around the circle.
ipg.add_separator(
        parent_id="col",
        separator_type=IpgSeparatorType.Dot,
        dot_radius=5.0,
        dot_count=10,
        spacing=5.0,)

# By using the styling and the border width, 
# one can make a circle
ipg.add_separator(
        parent_id="col",
        separator_type=IpgSeparatorType.Dot,
        dot_radius=5.0,
        dot_count=10,
        dot_border_width=2.0,
        spacing=5.0,
        style_id=dot_style)

# The height is needed here and needs to be
# larger than the height of the text.
ipg.add_separator(
        parent_id="col",
        separator_type=IpgSeparatorType.Label,
        height=22.0,
        label="Some Label",
        spacing=5.0)

ipg.add_separator(
        parent_id="col",
        separator_type=IpgSeparatorType.Label,
        height=22.0,
        label="Some Label",
        spacing=5.0)

# # The line needs both the width and the height
# ipg.add_separator(
#         parent_id="col",
#         width=200.0,
#         height=20.0,
#         separator_type=IpgSeparatorType.Line)




# ipg.add_separator(
#         parent_id="col",
#         width=200.0,
#         height=20.0,
#         separator_type=IpgSeparatorType.Line)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
