from icedpygui import IPG, IpgDividerParam, IpgColumnParam, IpgColor, IpgTextParam

# This is a demo to show how the divider_horizontal is used.
# Just put the cursor over the highlighted boder and drag

ipg = IPG()


def divider_change(div_id: int, index: int, value: float):
    # Both the columns and the divider need to be updated
    ipg.update_item(
            wid=column_ids[index],
            param=IpgColumnParam.Width,
            value=value)
    
    column_widths[index] = value
    ipg.update_item(
            wid=div_id,
            param=IpgDividerParam.Widths,
            value=column_widths)
    
    ipg.update_item(wid=text_ids[index],
                    param=IpgTextParam.Content,
                    value=f"Width={value}")


column_widths = [250.0, 250.0]
column_ids = []
text_ids = []
handle_width = 4.0  # defaults to 4 just using for demo info
handle_height = 200.0
        
cont_style_id = ipg.add_container_style(border_color=IpgColor.WHITE,
                                        border_width=1.0)

divider_style_id = ipg.add_divider_style(background_transparent=True)


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
        container_id="main_cont", 
        width_fill=True,
        height_fill=True)

# add a column to hold the text and the stack
ipg.add_column(
    window_id="main",
    container_id="main_col",
    parent_id="main_cont",
)

content = "Pace the cursor over the highlighted divider and drag"

ipg.add_text(
    parent_id="main_col",
    content=content)

# make the stack to lay the dividers over the containers
ipg.add_stack(
    window_id="main",
    container_id="stack",
    parent_id="main_col")


# make a row to hold the two columns
# this is added to stack
# The outer container used in the stack 
# cannot have any padding, since divider
# cannot detect whether padding is used
# it becomes misaligned.
ipg.add_row(window_id="main",
            parent_id="stack",
            container_id="row",
            spacing=0,
            padding=[0])

for index, width in enumerate(column_widths):
    # add a container for styling purposes
    ipg.add_container(window_id="main",
                      container_id=f"cont{index}",
                      parent_id="row",
                      style_id=cont_style_id)
    
    column_ids.append(ipg.add_column(window_id="main",
                   container_id=f"col{index}",
                   parent_id=f"cont{index}",
                   width=width))
    
    text_ids.append(ipg.add_text(parent_id=f"col{index}",
                 content=f"Width={width}"))
    
    ipg.add_button(parent_id=f"col{index}",
                   label="Some Button")
    
    ipg.add_button(parent_id=f"col{index}",
                   label="Another Button")

    ipg.add_toggler(parent_id=f"col{index}",
                    label="Toggler"),

       
       
# Make the divider
ipg.add_divider_horizontal(
    parent_id="stack",
    widths=column_widths,
    handle_width=handle_width,
    handle_height=handle_height,
    on_change=divider_change,
    # use the style to see just the outline and not the divider
    # style_id=divider_style_id
    )


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
