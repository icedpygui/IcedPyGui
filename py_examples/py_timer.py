from icedpygui import IPG, IpgTextParams


ipg = IPG()

def on_start(btn_id, counter):
    ipg.update_item(counter_id, IpgTextParams.Content, f"Count: {counter}")

def on_tick(timer_id, counter):
    ipg.update_item(counter_id, IpgTextParams.Content, f"Count: {counter}")



ipg.add_window("main", "Timer Demo",
                                600, 600, pos_centered=True)
        
ipg.add_container(window_id="main", container_id="cont", width_fill=True,
                        height_fill=True, align_x="center", align_y="center")

ipg.add_column(window_id="main", container_id="col", parent_id="cont")

ipg.add_timer("col", 1000, on_start=on_start, on_tick=on_tick)

counter_id = ipg.add_text("col", "Count: 0")

ipg.start_session()
