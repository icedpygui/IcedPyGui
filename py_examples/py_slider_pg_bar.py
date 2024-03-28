from icedpygui.icedpygui import IPG, IpgProgressBarParams, IpgTextParams

ipg = IPG()


def slider_on_change(id, data):
    ipg.update_item(on_change_id, IpgTextParams.Content, f"On Change value is {data}")
    ipg.update_item(bar_id, IpgProgressBarParams.Value, data)

def slider_on_release(id, data):
    ipg.update_item(on_release_id, IpgTextParams.Content, f"On Release value is {data}")  


ipg.add_window("main", "Slider Demo", 800, 800, 
                                    500, 100)

ipg.add_column("main", container_id="col", align_items="center", 
                                width_fill=True, height_fill=True)

ipg.add_space(parent_id="col", height=150.0)

bar_id = ipg.add_progress_bar(parent_id="col", min=0.0, max=100.0, value=50.0, width=300.0)

ipg.add_slider(parent_id="col", min=0.0, max=100.0, step=0.5, value=50.0, 
                    width=300.0, on_change=slider_on_change, on_release=slider_on_release)

ipg.add_space("col", height=100.0)

on_change_id = ipg.add_text(parent_id="col", content=f"On Change value is 0")

on_release_id = ipg.add_text(parent_id="col", content=f"On Release value is 0")

ipg.start_session()
