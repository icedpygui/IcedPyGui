from icedpygui import IPG, IpgColor, IpgTextParam

ipg = IPG()


def color_selected(cp_id: int, color: list):
    # Need to change the list color to a str type
    string = "["
    for i in range(0, len(color)):
        string += str(color[i]) + ", "
    string += "]"
    
    ipg.update_item(wid=text_id, 
                    param=IpgTextParam.Content, 
                    value=string)


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

# Add the container to center both x and y (default).  Holds only one widget.
ipg.add_container(
        window_id="main", 
        container_id="cont",
        width_fill=True, 
        height_fill=True)

# Add a column to hold multiple widgets
ipg.add_column(
    window_id="main",
    container_id="col",
    parent_id="cont")

ipg.add_color_picker(
        parent_id="col",
        on_submit=color_selected,
        on_press=cp_opened,
        on_cancel=cp_canceled,
        style_id=cp_style)

text_id = ipg.add_text(
            parent_id="col",
            content="Color value here")

ipg.start_session()