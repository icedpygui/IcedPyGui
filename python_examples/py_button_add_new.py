from icedpygui import IPG


ipg = IPG()

count = 0
def on_press(btn_id):
    global count
    count += 1
    ipg.add_button(
        parent_id="col", 
        label=f"Button_{count}", 
        on_press=on_press)


# Add the windows
ipg.add_window(
            window_id="main", 
            title="Button", 
            width=400, 
            height=400,  
            pos_centered=True)

 # Add a container to hold everything aligning all in the center
ipg.add_container(
            window_id="main", 
            container_id="cont", 
            width_fill=True, 
            height_fill=True,
            centered=True)

# Add a column to hold multiple widgets, vertically.
ipg.add_column(
            window_id="main", 
            container_id="col", 
            parent_id="cont")

ipg.add_button(
            parent_id="col", 
            label="Press Me to Add A Button", 
            on_press=on_press)

ipg.start_session()
