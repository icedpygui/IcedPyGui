from icedpygui import IPG, IpgToolTipPosition
from icedpygui import IpgToolTipParam, IpgButtonParam, IpgColor

ipg = IPG()

positions = [
    IpgToolTipPosition.Top,
    IpgToolTipPosition.Right,
    IpgToolTipPosition.Bottom,
    IpgToolTipPosition.Left,
    IpgToolTipPosition.FollowCursor,
]

index = 0

def change_position(btn_id):
    global index
    index += 1
    if index == 5:
        index = 0
    
    ipg.update_item(
        wid=tt_id,
        param=IpgToolTipParam.Position,
        value=positions[index])
    
    match index:
        case 0:
            label = "Tool Tip On Top, Press to Change"
        case 1:
            label = "Tool Tip On Right, Press to Change"
        case 2:
            label = "Tool Tip On Bottom, Press to Change"
        case 3:
            label = "Tool Tip On Left, Press to Change"   
        case 4:
            label = "Tool Tip Follows Cursor, Press to Change"    
            
    ipg.update_item(
        wid=btn_id,
        param=IpgButtonParam.Label,
        value=label)  
    
    
ts_id = ipg.add_tooltip_style(
            background_color=IpgColor.DARK_GRAY,
            text_color=IpgColor.BLACK,
            border_radius=[5.0],
            border_color=IpgColor.WHITE,
            border_width=2.0)  
    
# Add a window first
ipg.add_window(
        window_id="main", 
        title="CheckBox Demo",
        width=600, 
        height=600,  
        pos_centered=True)

# Add a container to center the widgets in the middle
ipg.add_container(
        window_id="main", 
        container_id="cont", 
        width_fill=True,
        height_fill=True,
        centered=True)


tt_id = ipg.add_tool_tip(
    window_id="main",
    container_id="tt",
    parent_id="cont",
    text_to_display="Some Tip",
    position=IpgToolTipPosition.Top,
    padding=5.0,
    gap=20,
    style_id=ts_id)

btn_id = ipg.add_button(
    parent_id="tt",
    label="Tool Tip On Top, Press to Change",
    on_press=change_position)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
