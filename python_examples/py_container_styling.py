from icedpygui import IPG, IpgColor, IpgContainerStyleParam

# To change the style of the container, 
# use the style id, not the container id.
def change_container_styling(btn_id):
    ipg.update_item(cont_style, IpgContainerStyleParam.BackgroundIpgColor, IpgColor.RED)
    ipg.update_item(cont_style, IpgContainerStyleParam.BorderIpgColor, IpgColor.LIGHT_SALMON)
    ipg.update_item(cont_style, IpgContainerStyleParam.BorderRadius, [5.0])
    ipg.update_item(cont_style, IpgContainerStyleParam.BorderWidth, 5.0)
    ipg.update_item(cont_style, IpgContainerStyleParam.ShadowBlurRadius, 10.0)
    ipg.update_item(cont_style, IpgContainerStyleParam.ShadowIpgColor, IpgColor.RED)
    ipg.update_item(cont_style, IpgContainerStyleParam.ShadowOffsetXY, [0.0, 0.0])
    ipg.update_item(cont_style, IpgContainerStyleParam.TextIpgColor, IpgColor.WHITE)

ipg = IPG()

# Add the styling container widget
cont_style = ipg.add_container_style(
                    background_color=IpgColor.AQUA,
                    border_color=IpgColor.BLUE,
                    border_radius=[10.0],
                    border_width=10.0,
                    shadow_color=IpgColor.YELLOW,
                    shadow_blur_radius=20.0,
                    shadow_offset_xy=[8.0, 8.0],
                    text_color=IpgColor.BLACK)

# Add the windows
ipg.add_window(
        window_id="main", 
        title="Container Styling", 
        width=500, 
        height=600,  
        pos_centered=True)

# add a container to hold the demo container 
# in the middle of the window
ipg.add_container(
        window_id="main",
        container_id="cont1",
        width_fill=True,
        height_fill=True,
        centered=True)

# add the container to work on
ipg.add_container(
        window_id="main",
        container_id="cont2",
        parent_id="cont1",
        width=200.0,
        height=200.0,
        style_id=cont_style)

# Add a column to hold the widgets
# the column has an transparent background
# so the container style shows through
ipg.add_column(
        window_id="main",
        container_id="col",
        parent_id="cont2")

# Add some text.  Since this text is not styled
# it would be a bit hard to see because the container
# will attempt to default style the text but
# won't always work best, So you can either
# style the text or use the container text_color
# to style all of the text in the container.
# This text styling will override the container 
# text color
ipg.add_text(
        parent_id="col",
        content="Some Text")

# Add a button the change the background color 
# or any of the style settings
ipg.add_button(
        parent_id="col",
        label="Change styling",
        on_press=change_container_styling)


# last thing is to start the session
ipg.start_session()