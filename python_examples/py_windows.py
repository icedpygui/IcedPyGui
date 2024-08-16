from icedpygui import IPG, IpgWindowParam

ipg = IPG()


def change_scale(input_id, value):
    ipg.update_item(wnd1, IpgWindowParam.ScaleFactor, value)


# Add the 1st window
wnd1 = ipg.add_window(window_id="main1",
                      title="Window 1",
                      width=200.0, height=200.0, 
                      pos_x=100, pos_y=25,
                      )

ipg.add_container(window_id="main1", container_id="cont")
ipg.add_text_input(parent_id="cont", placeholder="Input scale factor", on_submit=change_scale)


# Add the 2nd window
wnd2 = ipg.add_window(window_id="main2", 
                        title="Window 2",
                        width=200.0, height=200.0,  
                        pos_x=350, pos_y=25,
                        )

ipg.add_container(window_id="main2", container_id="cont")

ipg.start_session()
