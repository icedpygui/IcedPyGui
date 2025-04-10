from icedpygui import IPG, IpgDividerParam, IpgContainerParam, IpgColor, IpgTextParam

# This is a demo to show how the divider_horizontal is used.
# Just put the cursor over the highlighted boder and drag

ipg = IPG()


def divider_row_change(div_id: int, index: int, value: float):
    # Both the columns and the divider need to be updated
    for i in range(0, len(heights)):
        ipg.update_item(
                wid=row_cont_ids[index][i],
                param=IpgContainerParam.Height,
                value=value)
        
        ipg.update_item(wid=row_text_ids[index][i],
                    param=IpgTextParam.Content,
                    value=f"Height={value}")

    heights[index] = value
    ipg.update_item(
            wid=div_id,
            param=IpgDividerParam.Heights,
            value=heights)
    
 
def divider_col_change(div_id: int, index: int, value: float):
    # get the difference
    diff = widths[index] - value
    # update left
    widths[index] = value
    # check if a right then add diff to right
    if index < len(widths)-1:
        widths[index+1] += diff
    ipg.update_item(
            wid=div_id,
            param=IpgDividerParam.Widths,
            value=widths) 
    
    for i in range(0, len(heights)):
        for (j, width) in enumerate(widths):
            ipg.update_item(
                wid=row_cont_ids[i][j],
                param=IpgContainerParam.Width,
                value=width)

    
    


heights = [100.0, 100.0]
widths = [150.0, 150.0]
row_cont_ids = []
row_text_ids = []

row_handle_width = sum(widths)  
row_handle_height = 4.0

col_handle_width = 4.0
col_handle_height = sum(heights)

     
cont_style_id = ipg.add_container_style(border_color=IpgColor.WHITE,
                                        border_width=1.0)

divider_style_id = ipg.add_divider_style(background_transparent=True)


# Add a window first
ipg.add_window(
        window_id="main", 
        title="CheckBox Demo",
        width=600, 
        height=600,  
        pos_centered=True,
        # debug=True
        )

# Add a container to center the widgets in the middle
ipg.add_container(
        window_id="main", 
        container_id="main_cont", 
        width_fill=True,
        height_fill=True,
        centered=False,
        padding=[100, 0, 0, 100])

# add a column to hold the text and the stack
ipg.add_column(
        window_id="main",
        container_id="main_col",
        parent_id="main_cont",
        spacing=30)

content = "Pace the cursor over the highlighted divider and drag"

ipg.add_text(
        parent_id="main_col",
        content=content)

# make the stack to lay the dividers over the containers
ipg.add_stack(
        window_id="main",
        container_id="stack",
        parent_id="main_col")


# make a column to hold the two columns
# this is added to stack
ipg.add_column(
        window_id="main",
        parent_id="stack",
        container_id="col",
        spacing=0,
        padding=[0],
        width=row_handle_width)

for i, height in enumerate(heights):
    ipg.add_row(
        window_id="main",
        container_id=f"row{i}",
        parent_id="col",
        spacing=0)
    text_ids = []
    cont_ids = []
    for j, width in enumerate(widths):
        cont_ids.append(ipg.add_container(
                        window_id="main",
                        container_id=f"cont{i} {j}",
                        parent_id=f"row{i}",
                        width=width,
                        height=height,
                        style_id=cont_style_id))
        
        text_ids.append(ipg.add_text(
                        parent_id=f"cont{i} {j}",
                        content=f"Height={height}\n width={width}"))
    
    row_text_ids.append(text_ids)
    row_cont_ids.append(cont_ids)
       
# Make the vertical divider
ipg.add_divider_vertical(
        parent_id="stack",
        heights=heights,
        handle_width=row_handle_width,
        handle_height=row_handle_height,
        on_change=divider_row_change,
        # use the style to see just the outline and not the divider
        # style_id=divider_style_id
        )

#Make the horizontal divider
ipg.add_divider_horizontal(
    parent_id="stack",
    widths=widths,
    handle_width=col_handle_width,
    handle_height=col_handle_height,
    on_change=divider_col_change)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
