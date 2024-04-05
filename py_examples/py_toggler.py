from icedpygui import IPG, IpgTextParams

ipg = IPG()

def toggled(tog_id, is_toggled):
    ipg.update_item(is_tog_id, IpgTextParams.Content, f"The toggler is {is_toggled}.")

ipg.add_window("main", "Rule Demo", 500, 600, 
                                    pos_centered=True)

ipg.add_container("main", "cont", width_fill=True, height_fill=True)

ipg.add_column("main", container_id="col", parent_id="cont", align_items="center")

ipg.add_space(parent_id="col", width=500, height=20.0)

ipg.add_toggler("col", toggled=toggled)

is_tog_id = ipg.add_text("col", f"The toggler is False.")


ipg.start_session()
