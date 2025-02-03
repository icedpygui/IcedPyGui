from icedpygui import IPG, IpgTextParam, IpgTimerParam


# Fires when the timer is started
def on_start(timer_id: int, counter: int):
    ipg.update_item(timer_id, IpgTimerParam.Label, "Stop Timer")
    # Just in case of a restart, zero the counter
    ipg.update_item(timer_id, IpgTimerParam.Counter, 0)


# Fires on every tick
def on_tick(timer_id: int, counter: int):
    ipg.update_item(text_id, IpgTextParam.Content, f"Count: {counter}")


# Fires on stopping
def on_stop(timer_id: int, counter: int):
    ipg.update_item(text_id, IpgTextParam.Content, f"Count stopped at {counter}")
    ipg.update_item(timer_id, IpgTimerParam.Label, "Start Timer")

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

# The time duration is in milliseconds, so 1 second interval, in this case.
# The three callbacks start, stop, and tick are defined.
# The timer button is part of the timer method but you can style it as need, 
# just like the regular button.
ipg.add_timer(parent_id="col", 
              duration_ms=1000,
              on_start=on_start, 
              on_tick=on_tick, 
              on_stop=on_stop)

# Add a text widget to display the count.
text_id = ipg.add_text(parent_id="col", content="Count: 0")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
