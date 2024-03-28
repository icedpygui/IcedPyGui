from icedpygui import IPG, IpgTextParams


ipg = IPG()


# Since the id is the value of the calling widget which in this case
# is the event widget, not a window, another way is needed to determine which
# window was used.  Two approachs, one use the event name as show below or 
# use the user data.  The user_data was used for the text widget ids but you 
# could have also added the window ids to it.  You could have even used a dictionary
# if you wanted a name for each id.
# if you print the name, you'll see a version of this "Window Id(0) Opened".  The id
# for a window uses the Iced id system which is not an integer that is used for the 
# other widgets.  It is unique and the integer part of the id will increment
# as you add windows.  Unless you change the order that you added the window,
# it will always be the same for each window.  You could get the interger id by
# equating the window widget.  The integer ids start at 1 and the Iced ids start at Id(0).
def on_open(id, name, data, user_data):
    # The same text widget was used for of the events
    if "0" in name:
        ipg.update_item(user_data[0], IpgTextParams.Content, value=f"Window 0 position {data}")
    else:
        ipg.update_item(user_data[1], IpgTextParams.Content, value=f"Window 1 position {data}")

# The on_close event only works for other windows, not window 0.  Closing window 0 shuts
# down the system so no callback cn be performed.  If needed, maybe a special on_close event
#  would do a calback and then shut the window down. 
def on_close(id, name, user_data):
    ipg.update_item(user_data[0], IpgTextParams.Content, value=f"Window 1 was closed")

def on_moved(id, name, data, user_data):
    if "0" in name:
        ipg.update_item(user_data[0], IpgTextParams.Content, value=f"Window 0 position {data}")
    else:
        ipg.update_item(user_data[1], IpgTextParams.Content, value=f"Window 1 position {data}")

# Resizable defaults to True so to disable this event, set it to False.
def on_resized(id, name, data, user_data):
    if "0" in name:
        ipg.update_item(user_data[0], IpgTextParams.Content, value=f"Window 0 size {data}")
    else:
        ipg.update_item(user_data[1], IpgTextParams.Content, value=f"Window 1 size {data}")

# *******************Window 0*****************************************************
# Add the first window, 
ipg.add_window(window_id="main1", title="Window Handler Demo", 
                pos_x=300, pos_y=300, width=400, height=400)

ipg.add_column(window_id="main1", container_id="col", align_items="center", 
                                width_fill=True, height_fill=True)

ipg.add_text(parent_id="col", content="Try moving and resizing either window. If you close the right window you will see the message in the left window")

ipg.add_space(parent_id="col", height=50.0)

# text id obtained for the callbacks
wnd_1_text = ipg.add_text(parent_id="col", content="Window messages will show here")

# ***********************Window 1*********************************************

ipg.add_window(window_id="main2", title="Window Handler Demo", width=400, height=400, 
                                    pos_x=800, pos_y=300)

ipg.add_column("main2", container_id="col", align_items="center", 
                                width_fill=True, height_fill=True)

ipg.add_space(parent_id="col", height=100.0)

wnd_2_text = ipg.add_text(parent_id="col", content="Window messages will show here")


# Add the event, the callbacks are all optional, just use the ones you want
ipg.add_event_window(enabled=True, 
                     on_open=on_open, 
                     on_close=on_close, 
                     on_moved=on_moved,
                     on_resized=on_resized, 
                     user_data=[wnd_1_text, wnd_2_text])

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
