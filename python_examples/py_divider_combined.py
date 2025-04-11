from icedpygui import IPG, IpgDividerParam, IpgContainerParam, IpgColor, IpgTextParam

# This is a demo to show how the divider_horizontal is used.
# Just put the cursor over the highlighted boder and drag

ipg = IPG()


def divider_row_change(div_id: int, index: int, value: float):
    # get the difference
    diff = rows[index] - value
    # update left
    rows[index] = value
    # check if a right then add diff to right
    if index < len(rows)-1:
        rows[index+1] += diff
    ipg.update_item(
            wid=div_id,
            param=IpgDividerParam.Heights,
            value=rows) 
    
    for i in range(0, len(columns)):
        for (j, height) in enumerate(rows):
            ipg.update_item(
                wid=row_cont_ids[i][j],
                param=IpgContainerParam.Height,
                value=height)

    # Since the width of the row may have changed,
    # i.e. the handle on the end was used.  You
    # can recalc every time or just check the index
    if index == len(rows)-1:
        ipg.update_item(
            wid=div_col_id,
            param=IpgDividerParam.HandleHeight,
            value=sum(rows)
        )
    
 
def divider_col_change(div_id: int, index: int, value: float):
    # get the difference
    diff = columns[index] - value
    # update left
    columns[index] = value
    # check if a right then add diff to right
    if index < len(columns)-1:
        columns[index+1] += diff
    ipg.update_item(
            wid=div_id,
            param=IpgDividerParam.Widths,
            value=columns) 
    
    for i in range(0, len(rows)):
        for (j, width) in enumerate(columns):
            ipg.update_item(
                wid=row_cont_ids[i][j],
                param=IpgContainerParam.Width,
                value=width)

    # Since the width of the row may have changed,
    # i.e. the handle on the end was used.  You
    # can recalc every time or just check the index
    if index == len(columns)-1:
        ipg.update_item(
            wid=div_row_id,
            param=IpgDividerParam.HandleWidth,
            value=sum(columns)
        )
    
    

# It can be easy visualize to use row/column vs widths/heights
rows = [100.0, 100.0, 100.0]
columns = [150.0, 150.0]
row_cont_ids = []
row_text_ids = []

row_handle_width = sum(columns)  
row_handle_height = 4.0

col_handle_width = 4.0
col_handle_height = sum(rows)

     
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

for i, height in enumerate(rows):
    ipg.add_row(
        window_id="main",
        container_id=f"row{i}",
        parent_id="col",
        spacing=0)
    text_ids = []
    cont_ids = []
    for j, width in enumerate(columns):
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
       
# Make the vertical divider (rows)
div_row_id = ipg.add_divider_vertical(
        parent_id="stack",
        heights=rows,
        handle_width=row_handle_width,
        handle_height=row_handle_height,
        on_change=divider_row_change,
        # use the style to see just the outline and not the divider
        # style_id=divider_style_id
        )

#Make the horizontal divider (columns)
div_col_id = ipg.add_divider_horizontal(
    parent_id="stack",
    widths=columns,
    handle_width=col_handle_width,
    handle_height=col_handle_height,
    on_change=divider_col_change)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
