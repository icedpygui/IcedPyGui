from icedpygui import IPG, IpgWindowParam, IpgWindowMode, IpgTextParam, IpgWindowLevel, IpgWindowTheme

ipg = IPG()


popup_id = ipg.generate_id()
wnd2 = ipg.generate_id()
wnd4 = ipg.generate_id()


# ****************Functions for modifying the window 4 parameters*************
# Debug requires a bool value therefore a toggler is used
def toggle_debug(tog_id: int, value: bool):
    ipg.update_item(wnd4, IpgWindowParam.Debug, value)


# The decorator is just toggled therefore a button works.  Supply the window id.
def toggle_decorations(btn_id: int):
    ipg.update_item(wnd4, IpgWindowParam.Decorations, wnd4)


# The resize of the window requires a list [width, height]
def toggle_resize(btn_id: int, value: bool):
    if value:
        ipg.update_item(wnd4, IpgWindowParam.Size, [300.0, 400.0])
    else:
        ipg.update_item(wnd4, IpgWindowParam.Size, [300.0, 600.0])


# The level of the window, move it over another window to see the effect
def toggle_level(tog_id: int, value: bool):
    if value:
        ipg.update_item(wnd4, IpgWindowParam.Level, IpgWindowLevel.AlwaysOnBottom)
    else:
        ipg.update_item(wnd4, IpgWindowParam.Level, IpgWindowLevel.Normal)


# Move the window to a new position, required a list[pos_x, pos_y]
def toggle_move_to(tog_id: int, value: bool):
    if value:
        ipg.update_item(wnd4, IpgWindowParam.Position, [900.0, 25.0])
    else:
        ipg.update_item(wnd4, IpgWindowParam.Position, [1000.0, 25.0])


def toggle_theme(tog_id: int, value: bool):
    if value:
        ipg.update_item(wnd4, IpgWindowParam.Theme, IpgWindowTheme.Light)
    else:
        ipg.update_item(wnd4, IpgWindowParam.Theme, IpgWindowTheme.Dark)

# ****************Functions for changes and events in window 1*****************
# Since the input value is a string, need to convert to  a float
def change_scale(input_id: int, value: str):
    ipg.update_item(wnd1, IpgWindowParam.ScaleFactor, float(value))
    ipg.update_item(wnd2, IpgWindowParam.ScaleFactor, float(value))


# The user data is a window_id
def show_window(btn_id: int, window_id: int):
    ipg.update_item(window_id, IpgWindowParam.Mode, IpgWindowMode.Windowed)
    ipg.update_item(s_h_text_id, IpgTextParam.Content, f"Window with id {window_id} is shown")


# The user data is a window_id
def close_window(btn_id: int, window_id: int):
    ipg.update_item(wnd2, IpgWindowParam.Mode, IpgWindowMode.Closed)
    ipg.update_item(s_h_text_id, IpgTextParam.Content, f"Window with id {window_id} is closed")


# Window events return a window_id and event name at the minimum.
# It's the only widget which doesn't return it's own id, since it would never be used.
def event_on_closed(wnd_id: int, event_name: str):
    ipg.update_item(event_text_id, IpgTextParam.Content, f"You closed window width id {wnd_id}")


def event_on_move(wnd_id: int, event_name: str, position: dict):
    ipg.update_item(event_text_id, IpgTextParam.Content, f"Window with id {wnd_id} \nhas position {position}")


def event_on_resize(wnd_id: int, event_name: str, size: dict):
    ipg.update_item(event_resize_id, IpgTextParam.Content, f"Window with id {wnd_id} \nhas size of {size}")


def event_focused(wnd_id, event_name: str):
    ipg.update_item(event_focused_id, IpgTextParam.Content, f"Window with id {wnd_id} has the focus")


def event_unfocused(wnd_id, event_name: str):
    ipg.update_item(event_unfocused_id, IpgTextParam.Content, f"Window with id {wnd_id} was unfocused")


def event_on_file_hovered(wnd_id: int, event_name: str, file_name: str):
    ipg.update_item(event_file_hovered_id, IpgTextParam.Content, f"File, {file_name}, was hovered over window with id {wnd_id}")


def event_on_file_dropped(wnd_id: int, event_name: str, file_name: str):
   ipg.update_item(event_file_dropped_id, IpgTextParam.Content, f"File, {file_name}, was dropped on window with id {wnd_id}")


def event_on_files_hover_left(wnd_id: int, event_name: str):
    print("here")
    ipg.update_item(event_file_hover_left_id, IpgTextParam.Content, f"File hover left window with id {wnd_id}.")


# *******************add functions for close requested ******************************
# This responds to the close request event
# IMPORTANT: Once this event is used, you must also update any
# other window with a close statement since all window are now
# calling this event function.
def event_on_close_requested(wnd_id: int, event_name: str):
    if wnd_id == wnd2:
        # show window to acknowledge close or not
        ipg.update_item(popup_id, IpgWindowParam.Mode, IpgWindowMode.Windowed)
    else:
        # If not the window of interest, close it.
        ipg.update_item(wnd_id, IpgWindowParam.Mode, IpgWindowMode.Closed)


# This responds to the button pressed in the close request window
def close_window_using_popup(btn_id: int, window_ids: list):
    # Close the requested window
    ipg.update_item(window_ids[0], IpgWindowParam.Mode, IpgWindowMode.Closed)
    # Close the popup window
    ipg.update_item(window_ids[1], IpgWindowParam.Mode, IpgWindowMode.Closed)
    

# This responds to a button pressed in the close request window
def close_window_canceled(btn_id, window_id: int):
    ipg.update_item(window_id, IpgWindowParam.Mode, IpgWindowMode.Closed)

# ****************************Add the event widget for the windows*******************************
ipg.add_event_window(enabled=True,
                     on_closed=event_on_closed,
                     on_moved=event_on_move,
                     on_resized=event_on_resize,
                     on_close_requested=event_on_close_requested,
                     on_focused=event_focused,
                     on_unfocused=event_unfocused,
                     on_file_hovered=event_on_file_hovered,
                     on_file_dropped=event_on_file_dropped,
                     on_files_hovered_left=event_on_files_hover_left,
                     )

# ******************Add the 1st window***************************
wnd1 = ipg.add_window(window_id="main1",
                      title="Window 1",
                      width=400.0, height=600.0, 
                      pos_x=100, pos_y=25,
                      )

# add a container to center things
ipg.add_container(window_id="main1", 
                  container_id="cont",
                  width_fill=True,
                  height_fill=True
                  )

# Add a column for multiple widgets
ipg.add_column(window_id="main1", 
               container_id="col", 
               parent_id="cont",
               )

# Add some text
ipg.add_text(parent_id="col", content="Input scale factor")

# add the input widget
ipg.add_text_input(parent_id="col",
                   width=200.0,
                   placeholder="scale factor (float)", 
                   on_submit=change_scale)

# Add show hide text
s_h_text_id = ipg.add_text(parent_id="col", content="Window 2 is closed")

# Add event text
event_text_id = ipg.add_text(parent_id="col", content="This will change when an event occurs")

event_resize_id = ipg.add_text(parent_id="col", content="This will change when resized event occurs")

event_close_request_id = ipg.add_text(parent_id="col", content="You have no close requests")

event_focused_id = ipg.add_text(parent_id="col", content="No window has the focus")

event_unfocused_id = ipg.add_text(parent_id="col", content="No window has the focus")

event_file_hovered_id = ipg.add_text(parent_id="col", content="No file has been hovered")

event_file_dropped_id = ipg.add_text(parent_id="col", content="No file has been dropped")

event_file_hover_left_id = ipg.add_text(parent_id="col", content="No file hover has left")

# add a button to show the 2nd window
ipg.add_button(parent_id="col",
                label="Show Window",
                on_press=show_window,
                user_data=wnd2,
                )

ipg.add_button(parent_id="col",
                label="Show Close request window",
                on_press=show_window,
                user_data=popup_id
                )

# ************Add the 2nd window ****************************
# To get a close request from this window, exit_on_close must be set to False.
# Windows default to True.
ipg.add_window(window_id="main2", 
                title="Window 2",
                width=400.0, height=400.0,  
                pos_x=600, pos_y=25,
                mode=IpgWindowMode.Closed,
                exit_on_close=False,
                gen_id=wnd2,
                )

ipg.add_container(window_id="main2", container_id="cont")
ipg.add_button(parent_id="cont", 
                label="Hide Window",
                on_press=close_window,
                user_data=wnd2)


# **************Add close request window ***********************
# Note the window is closed, acting like a modal in this case.
ipg.add_window(window_id="close_request",
               title="Close Requested",
               width=300.0, height=300.0,
               pos_centered=True,
               mode=IpgWindowMode.Closed,
               gen_id=popup_id,
               )

ipg.add_container(window_id="close_request",
                  container_id="cont",
                  width_fill=True, 
                  height_fill=True,
                  )

ipg.add_column(window_id="close_request",
               container_id="col",
               parent_id="cont",
               )

ipg.add_text(parent_id="col", content="With a little bit more programming, you could place this popup window anyplace on the screen.")
ipg.add_space(parent_id="col", height=30.0)

ipg.add_button(parent_id="col", label="Close Window 2", 
               on_press=close_window_using_popup,
               user_data=[wnd2, popup_id])

ipg.add_button(parent_id="col", label="Cancel Window 2 Closing", 
               on_press=close_window_canceled,
               user_data=popup_id
               )


# ************Add the 4th window ****************************
# This window is for changing the window parameters
ipg.add_window(window_id="main4", 
                title="Window 4",
                width=300.0, height=600.0,  
                pos_x=1000.0, pos_y=25.0,
                gen_id=wnd4,
                )

ipg.add_column(window_id="main4", 
               container_id="col"
               )

ipg.add_toggler(parent_id="col", 
               label="Toggle Debug",
               toggled=toggle_debug)

ipg.add_button(parent_id="col", 
               label="Toggle Decorations",
               on_press=toggle_decorations)

ipg.add_toggler(parent_id="col", 
               label="Toggle Window Resize",
               toggled=toggle_resize)

ipg.add_toggler(parent_id="col", 
               label="Toggle Position",
               toggled=toggle_move_to)

ipg.add_toggler(parent_id="col", 
               label="Toggle Level",
               toggled=toggle_level)

ipg.add_toggler(parent_id="col", 
               label="Toggle Theme",
               toggled=toggle_theme)

ipg.start_session()
