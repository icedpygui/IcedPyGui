from icedpygui import IPG, IpgTextParam


# Fires when the timer is started
def on_start(btn_id, counter):
    ipg.update_item(text_id, IpgTextParam.Content, f"Count: {counter}")


# Fires on every tick
def on_tick(timer_id, counter):
    ipg.update_item(text_id, IpgTextParam.Content, f"Count: {counter}")


# Fires on stopping
def on_stop(timer_id, counter):
    ipg.update_item(text_id, IpgTextParam.Content, f"Count stopped at {counter}")

ipg = IPG()

# Add a window
ipg.add_window(window_id="main", title="Timer Demo",
               width=400.0, height=400.0,  
               pos_centered=True)

# Add the container to center everything
ipg.add_container(window_id="main", 
                  container_id="cont", 
                  width_fill=True,
                  height_fill=True,
                  )

# Add the column to hold the widgets
ipg.add_column(window_id="main", container_id="col", parent_id="cont")

# Add the time, the duration is supplied a milliseconds, so 1 second interval, in this case.
# The two callbacks are defined.
# The button is part of the timer and you can style it as need, just like the button.
ipg.add_timer(parent_id="col", duration_ms=1000,
              on_start=on_start, on_tick=on_tick, on_stop=on_stop)

# Add a text widget to display the count.
text_id = ipg.add_text(parent_id="col", content="Count: 0")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
