from icedpygui import IPG, IpgWindowParam

ipg = IPG()


def change_scale(input_id, value):
    ipg.update_item(wnd1, IpgWindowParam.ScaleFactor, value)


def show_hide_window(tog_id, value):
    ipg.update_item(wnd2, IpgWindowParam.Show, value)


# Add the 1st window
wnd1 = ipg.add_window(window_id="main1",
                      title="Window 1",
                      width=300.0, height=300.0, 
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

# add a toggler to show and hide the 2nd window
ipg.add_toggler(parent_id="col",
                label="Show/Hide Window",
                toggled=show_hide_window,
                )



# Add the 2nd window
wnd2 = ipg.add_window(window_id="main2", 
                        title="Window 2",
                        width=300.0, height=300.0,  
                        pos_x=500, pos_y=25,
                        show=False,
                        )

ipg.add_container(window_id="main2", container_id="cont")
ipg.add_text(parent_id="cont", content="Some Text")
ipg.start_session()
