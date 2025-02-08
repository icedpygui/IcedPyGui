from icedpygui import IPG, IpgAlignment, IpgHorizontalAlignment, IpgVerticalAlignment, IpgSeparatorType
from icedpygui import IpgColor


# This is a simple demo to change the checkmark of the checkbox to an x.
ipg = IPG()

# styles


# for circle, separator_border_radius has no effect
# The background is the box in which the seaprator exists.
# If the circle is given no width or height, then the box shrinks
# to the circle otherwise the width and height act as a padding
# or some other design if styling is used.
circle_style = ipg.add_separator_style(
                        separator_color=IpgColor.RED,
                        separator_border_color=IpgColor.YELLOW,
                        separator_border_width=4.0,
                        separator_shadow_blur_radius=5.0,
                        separator_shadow_color=IpgColor.LIGHT_YELLOW,
                        separator_shadow_offset=[3.0, 7.0],
                        background_border_color=IpgColor.LIGHT_YELLOW,
                        background_shadow_blur_radius=5.0,
                        background_shadow_color=IpgColor.LIGHT_YELLOW)

dot_style = ipg.add_separator_style(
                        separator_border_color=IpgColor.YELLOW,
                        separator_border_width=4.0,
                        separator_shadow_blur_radius=5.0,
                        separator_shadow_color=IpgColor.LIGHT_YELLOW,
                        separator_shadow_offset=[3.0, 7.0],
                        background_border_color=IpgColor.LIGHT_YELLOW,
                        background_shadow_blur_radius=5.0,
                        background_shadow_color=IpgColor.LIGHT_YELLOW
                        )


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
        debug=True)

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

ipg.add_text(parent_id="col", content="Separators with default styling")

# Add all of the separators with the default styling

# for the circleand dot, the width and height are not used
# unless one wants a sort of padding around the circle.
# See the next circle and dot for an example.
ipg.add_separator(
        parent_id="col",
        separator_type=IpgSeparatorType.Dot,
        dot_radius=10.0,
        dot_count=10)


# The height is needed here and needs to be
# larger than the height of the text.
ipg.add_separator(
        parent_id="col",
        separator_type=IpgSeparatorType.Label,
        height=22.0,
        label="Some Label")

# The line needs both the width and the height
ipg.add_separator(
        parent_id="col",
        width=200.0,
        height=20.0,
        separator_type=IpgSeparatorType.Line)


# Add the all of the separators with some styling
ipg.add_text(parent_id="col", content="Separators with custom styling")

# In this case, we use the width and height as a padding around the circle
# This will also be styled just to show them
ipg.add_separator(
        parent_id="col",
        separator_type=IpgSeparatorType.Dot,
        dot_radius=10.0,
        dot_count=10,
        style_id=dot_style)

ipg.add_separator(
        parent_id="col",
        separator_type=IpgSeparatorType.Label,
        height=22.0,
        label="Some Label")

ipg.add_separator(
        parent_id="col",
        width=200.0,
        height=20.0,
        separator_type=IpgSeparatorType.Line)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
