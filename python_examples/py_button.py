from icedpygui import IPG, IpgColor, IpgWindowTheme, IpgAlignment, IpgStyleStandard, IpgButtonArrow
from icedpygui import IpgHorizontalAlignment, IpgVerticalAlignment
ipg = IPG()


def on_press(btn_id):
    print("button pressed")

def add_buttons(window: str):
    # Add a container to hold everything aligning all in the center
    ipg.add_container(
            window_id=window, 
            container_id="cont", 
            width_fill=True, 
            height_fill=True)
    
    # Add a column to hold multiple widgets, vertically.
    ipg.add_column(
            window_id=window, 
            container_id="col", 
            parent_id="cont")

    # Add a row to hold widgets, horizontally.
    ipg.add_row(
            window_id=window, 
            container_id="row_btn", 
            parent_id="col",
            align=IpgAlignment.Center)

    # Add buttons
    ipg.add_button(
            parent_id="row_btn", 
            label="Default", 
            on_press=on_press)

    ipg.add_button(
            parent_id="row_btn", 
            label="Primary", 
            on_press=on_press,
            style_standard=IpgStyleStandard.Primary)

    ipg.add_button(
            parent_id="row_btn", 
            label="Success", 
            on_press=on_press,
            style_standard=IpgStyleStandard.Success)

    ipg.add_button(
            parent_id="row_btn", 
            label="Danger", 
            on_press=on_press,
            style_standard=IpgStyleStandard.Danger)

    ipg.add_button(
            parent_id="row_btn", 
            label="Text", 
            on_press=on_press,
            style_standard=IpgStyleStandard.Text)
    
    ipg.add_button(
            parent_id="row_btn", 
            label="",
            on_press=on_press,
            style_arrow=IpgButtonArrow.ArrowRight)

    ipg.add_row(
            window_id=window, 
            container_id="row_btn2", 
            parent_id="col",
            align=IpgAlignment.Center)

    dodger = ipg.add_button_style(
                background_color=IpgColor.DODGER_BLUE)

    ipg.add_button(
            parent_id="row_btn2", 
            label="Custom Base Only Defined",
            style_id=dodger)

    custom = ipg.add_button_style(
                    background_color=IpgColor.DODGER_BLUE,
                    background_color_hovered=IpgColor.BLUE,
                    border_color=IpgColor.DARK_GOLDEN_ROD,
                    shadow_color=IpgColor.DARK_ORANGE,
                    text_color=IpgColor.BLACK,
                    border_radius=[12.0], 
                    border_width=5.0,
                    shadow_offset_x=0.0, 
                    shadow_offset_y=0.0, 
                    shadow_blur_radius=15.0)
    
    ipg.add_button(
            parent_id="row_btn2", 
            label="All Colors Custom",
            style_id=custom)
    
    ipg.add_row(
            window_id=window, 
            container_id="row_btn3", 
            parent_id="col",
            align=IpgAlignment.Center)
    
    std_border = ipg.add_button_style(
                         border_color=IpgColor.GREEN,
                         border_radius=[12.0],
                         border_width=5.0,
                         shadow_color=IpgColor.DARK_GREEN,
                         shadow_blur_radius=10.0,
                         shadow_offset_x=5.0,
                         shadow_offset_y=5.0,
                         )
    
    ipg.add_button(
            parent_id="row_btn3", 
            label="Standard with Border and shadow",
            style_id=std_border,
            style_standard=IpgStyleStandard.Success)
    
    if window == "main1":
        ipg.add_button(
                parent_id="col",
                label="Alignment = Center/Center",
                width=300.0,
                height=50.0)
        
        ipg.add_button(
                parent_id="col",
                label="Alignment = Left/Bottom",
                width=300.0,
                height=50.0,
                text_align_x=IpgHorizontalAlignment.Left,
                text_align_y=IpgVerticalAlignment.Bottom)
        
        ipg.add_button(
                parent_id="col",
                label="Alignment = Right/Top",
                width=300.0,
                height=50.0,
                text_align_x=IpgHorizontalAlignment.Right,
                text_align_y=IpgVerticalAlignment.Top)

# Add the windows
ipg.add_window(
        window_id="main1", 
        title="Button Styling", 
        width=500, 
        height=600,  
        pos_x=100, 
        pos_y=25)

ipg.add_window(
        window_id="main2", 
        title="Button Styling", 
        width=500, 
        height=600,  
        pos_x=650, 
        pos_y=25,
        theme=IpgWindowTheme.GruvboxLight)

add_buttons("main1")

add_buttons("main2")

ipg.start_session()
