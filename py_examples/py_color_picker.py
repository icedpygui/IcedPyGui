from icedpygui import IPG, IpgTextParams


ipg = IPG()

def color_submitted(id, color):
    value= f"Color selected = {color}"
    ipg.update_item(color_id, IpgTextParams.Content, value=value)


ipg.add_window(window_id="main", title="ColorPicker Demo", width=800, height=800, 
                                    pos_x=500, pos_y=100)

ipg.add_container(window_id="main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col", parent_id="cont")

ipg.add_color_picker("col", on_submit=color_submitted)

color_id = ipg.add_text("col", content="Color selected =")

ipg.start_session()