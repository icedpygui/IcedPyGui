from icedpygui import IPG, IpgTextParam
from icedpygui import IpgAlignment


ipg = IPG()


# Windows are a little unique in how Iced uses them.  They are assigned
# a unique id Id(some number).  This id is converted to a string in IPG and
# supplied through the callback as indicated by name in the parameters below.
# The number part of the id is incremented for each window and is unique.
# You can distinguish the id by simply checking if the string
# matches the window you are interested in, if "0" in name or if "Id(0)" == name, etc.
# Another way to distinguish the windows would be to equate the window,
# as done for widgets, and use that integer id as a unique value to
# determine which window you are using.  In this demo, I decided to use the Iced id.
# Instead of using a global variable for the text as in most other demos except for the
# ones using a class, I supplied those through the user_data.
def on_open(_event_id: int, name: str, data: dict, user_data: any):
    # The same text widget was used for of the events
    if "Id(0)" == name:
        ipg.update_item(user_data[0], IpgTextParam.Content, value=f"Window 0 position {data}")
    else:
        ipg.update_item(user_data[1], IpgTextParam.Content, value=f"Window 1 position {data}")


# The on_close event only works for other windows, not window 0.  Closing window 0 shuts
# down the system so no callback can be performed.  A special callback may be implemented
# in the future if there are requests.
def on_close(_event_id: int, name: str, user_data: any):
    ipg.update_item(user_data[0], IpgTextParam.Content, value=f"Window 1 was closed")


# A slightly different if statement was used in this case just to demonstrate another way.
def on_moved(event_id: int, name: str, data: dict, user_data: any):
    if "0" in name:
        ipg.update_item(user_data[0], IpgTextParam.Content, value=f"Window 0 position {data}")
    else:
        ipg.update_item(user_data[1], IpgTextParam.Content, value=f"Window 1 position {data}")


# The resized event
def on_resized(event_id: int, name: str, data: dict, user_data: any):
    if "0" in name:
        ipg.update_item(user_data[0], IpgTextParam.Content, value=f"Window 0 size {data}")
    else:
        ipg.update_item(user_data[1], IpgTextParam.Content, value=f"Window 1 size {data}")


# *******************Window 0*****************************************************
# Add the first window, 
ipg.add_window(window_id="main1", title="Window Handler Demo",
               pos_x=100, pos_y=25, width=400, height=400)

# add column to hold the widgets
ipg.add_column(window_id="main1", container_id="col",
               align_items=IpgAlignment.Center,
               width_fill=True, height_fill=True)

# Add some text
ipg.add_text(parent_id="col",
             content="Try moving and resizing either window. If you close the right window you will see the message "
                     "in the left window")

# Add space for readability
ipg.add_space(parent_id="col", height=50.0)

# text id obtained for the callbacks
wnd_1_text = ipg.add_text(parent_id="col", content="Window messages will show here")

# ***********************Window 1*********************************************
# Add second window
ipg.add_window(window_id="main2", title="Window Handler Demo", width=400, height=400,
               pos_x=600, pos_y=25)

# Add the column for the widgets
ipg.add_column("main2", container_id="col",
               align_items=IpgAlignment.Center,
               width_fill=True, height_fill=True)

ipg.add_space(parent_id="col", height=100.0)

# Get the second text id
wnd_2_text = ipg.add_text(parent_id="col", content="Window messages will show here")

# Add the event, the callbacks are all optional, just use the ones you want
# In this case I used the user_data to hold the text_ids, normally you would
# probably use a class
ipg.add_event_window(enabled=True,
                     on_open=on_open,
                     on_close=on_close,
                     on_moved=on_moved,
                     on_resized=on_resized,
                     user_data=[wnd_1_text, wnd_2_text])


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
