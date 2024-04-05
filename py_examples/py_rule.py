from icedpygui import IPG

ipg = IPG()

ipg.add_window("main", "Rule Demo", 500, 600, 
                                    pos_centered=True)

ipg.add_container("main", "cont", width_fill=True, height_fill=True)

ipg.add_column("main", container_id="col", parent_id="cont", align_items="center")

ipg.add_space(parent_id="col", width=500, height=20.0)

ipg.add_rule_vertical("col", height=250)
ipg.add_rule_horizontal("col", width=250)


ipg.add_text(parent_id="col", content="There a vertical and horizontal divider above me, centered.")

ipg.start_session()
