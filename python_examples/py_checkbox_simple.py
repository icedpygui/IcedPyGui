from icedpygui import IPG, IpgCheckboxParam, IpgAlignment

# This is a simple demo to change the checkmark of the checkbox to an x.
ipg = IPG()

# A technique explored in another demo where the
# id is generated ahead of time.  Useful in some cases.
x_id = ipg.generate_id()


# The callback used for the checkbox.
# This callback id for this is not used since we are
# changing the other checkbox.  Therefore we needed to know
# its id.  Normally you would probably use a class to store theses
# needed ids for later use.
# The is_checked is a boolean which will toggle each time the
# checkbox is clicked.
def on_toggle(_chkbx_id: int, is_checked: bool, user_data: any):
    ipg.update_item(
            wid=x_id, 
            param=IpgCheckboxParam.IconX, 
            value=is_checked)


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
        height_fill=True)

# Since a container can only hold one widget, use a column to hold the
# two checkboxes.  We let the width and height default to shrink, so no entry.
# The alignment defaults to Start but for demonstration purpose, we
# added the IpgColumnAlignment.Start
ipg.add_column(
        window_id="main", 
        container_id="col", 
        parent_id="cont",
        align=IpgAlignment.Start)

# Add the first checkbox with the callback on_toggle.
ipg.add_checkbox(
        parent_id="col", 
        label="Check Me!!!",
        on_toggle=on_toggle,
        user_data="Something") # not used in this demo

# Add the second checkbox.  This has no callback since it not used.
ipg.add_checkbox(
        parent_id="col", 
        gen_id=x_id, 
        label="See my check check change to an x", 
        is_checked=True)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
