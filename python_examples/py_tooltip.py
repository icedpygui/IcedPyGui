from icedpygui import IPG, IpgToolTipPosition, IpgToolTipParam

# This is a simple demo to change the checkmark of the checkbox to an x.
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
        param=IpgToolTipParam,
        value=positions[index])
    
    
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
    position=IpgToolTipPosition.Top
)

btn_id = ipg.add_button(
    parent_id="tt",
    label="Tool Tip Demo"
)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
