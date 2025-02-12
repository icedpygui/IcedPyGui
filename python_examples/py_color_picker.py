from icedpygui import IPG, IpgColor

ipg = IPG()


def color_selected(cp_id: int, color: list):
    print(color)


def cp_opened(cp_id: int):
    print("color picker opened")
    

def cp_canceled(cp_id: int):
    print("color picker canceled")


cp_style = ipg.add_color_picker_style(
                background_color=IpgColor.LIGHT_YELLOW,
                background_color_hovered=IpgColor.YELLOW,
                text_color=IpgColor.BLACK,
                border_radius=[10.0],
                border_width=5.0,
                border_color=IpgColor.YELLOW)


# Add a window first
ipg.add_window(
        window_id="main", 
        title="Canvas",
        width=500.0, 
        height=500.0,
        pos_centered=True)

# Add the container to center both x and y.  Holds only one widget.
ipg.add_container(
        window_id="main", 
        container_id="cont",
        width_fill=True, 
        height_fill=True)

ipg.add_color_picker(
        parent_id="cont",
        on_submit=color_selected,
        on_press=cp_opened,
        on_cancel=cp_canceled,
        style_id=cp_style)

ipg.start_session()