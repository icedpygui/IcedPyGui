from icedpygui import IPG

ipg = IPG()





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


ipg.start_session()