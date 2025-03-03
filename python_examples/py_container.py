from icedpygui import IPG, IpgColor, IpgContainerParam, IpgHorizontalAlignment, IpgVerticalAlignment


# Moves the text to the top left position
def change_alignment(btn_id):
    ipg.update_item(
            wid=cont2, 
            param=IpgContainerParam.AlignX, 
            value=IpgHorizontalAlignment.Left)
    
    ipg.update_item(
            wid=cont2, 
            param=IpgContainerParam.AlignY, 
            value=IpgVerticalAlignment.Top)
    
    
# Moves the text to the center position
def recenter(btn_id):
    ipg.update_item(
            wid=cont2, 
            param=IpgContainerParam.Centered, 
            value=True)
    
    
# Moves text off center because padding on the left side
# padding = [top, right, bottom, left]
def padding(btn_id):
    ipg.update_item(
            wid=cont2, 
            param=IpgContainerParam.Padding, 
            value=[0.0, 0.0, 0.0, 50.0])  
    
# change container width
def width(btn_id):
    ipg.update_item(
            wid=cont2, 
            param=IpgContainerParam.Width, 
            value=300.0)
    
    
# change container height
def height(btn_id):
    ipg.update_item(
            wid=cont2, 
            param=IpgContainerParam.Height, 
            value=300.0)
 

ipg = IPG()

# Add the styling container widget
cont_style = ipg.add_container_style(
                    background_color=IpgColor.GRAY)

# Add the windows
ipg.add_window(
        window_id="main", 
        title="Container Styling", 
        width=600, 
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

# Add a column to hold the widgets
# the column has an transparent background
# so the container style shows through
ipg.add_column(
        window_id="main",
        container_id="col",
        parent_id="cont1")

# add the container to work on
cont2 = ipg.add_container(
                window_id="main",
                container_id="cont2",
                parent_id="col",
                width=200.0,
                height=200.0,
                style_id=cont_style)

ipg.add_text(
        parent_id="cont2",
        content="Some Text")

# Add a button the change the alignment 
ipg.add_button(
        parent_id="col",
        label="Change Alignment",
        on_press=change_alignment)

# Add a button recenter the text 
ipg.add_button(
        parent_id="col",
        label="Recenter",
        on_press=recenter)

# Add a button add padding of the contained items
ipg.add_button(
        parent_id="col",
        label="Padding",
        on_press=padding)

# Add a button change the container width
ipg.add_button(
        parent_id="col",
        label="Width",
        on_press=width)

# Add a button change the container height
ipg.add_button(
        parent_id="col",
        label="Height",
        on_press=height)

# last thing is to start the session
ipg.start_session()
