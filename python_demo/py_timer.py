from icedpygui import IPG, IpgTextParam

ipg = IPG()


# Fires when the timer is started
def on_start(btn_id, counter):
    ipg.update_item(counter_id, IpgTextParam.Content, f"Count: {counter}")


# Fires on every tick
def on_tick(timer_id, counter):
    ipg.update_item(counter_id, IpgTextParam.Content, f"Count: {counter}")


# Fires on stopping
def on_stop(timer_id, counter):
    ipg.update_item(counter_id, IpgTextParam.Content, f"Count stopped at {counter}")


# Add a window
ipg.add_window("main", "Timer Demo",
               400, 400,  pos_x=100, pos_y=25)

# Add the container to center everything
ipg.add_container(window_id="main", container_id="cont", 
                  width_fill=True,
                  height_fill=True,
                  center_xy=True)

# Add the column to hold the widgets
ipg.add_column(window_id="main", container_id="col", parent_id="cont")

# Add the time, the duration is supplied a milliseconds, so 1 second interval, in this case.
# The two callbacks are defined.
# The button is part of the timer and you can style it as need, just like the button.
ipg.add_timer("col", 1000,
              on_start=on_start, on_tick=on_tick, on_stop=on_stop)

# Add a text widget to display the count.
counter_id = ipg.add_text("col", "Count: 0")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
