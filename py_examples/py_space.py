from icedpygui import IPG

ipg = IPG()

ipg.add_window("main1", "Space Demo 1", 500, 600, 
                                    200, 100, debug=True)

ipg.add_column("main1", container_id="col1", align_items="center", width_fill=True, height_fill=True)

ipg.add_space(parent_id="col1", width=50.0, height=200.0)

ipg.add_text(parent_id="col1", content="\nThere is a space outlined above, \nwidth=100.0, height=50.0\n ")


ipg.add_window("main2", "Space Demo 2", 500, 600, 
                                    800, 100, debug=True)

ipg.add_column("main2", container_id="col2", align_items="center", width_fill=True, height_fill=True)

ipg.add_space(parent_id="col2", width_fill=True, height=100.0)

ipg.add_text(parent_id="col2", content="\nThere is a space outlined above, \nwidth_fill=True, height=100.0\n ")

ipg.add_text(parent_id="col2", content="\nif you drag the window width, the spacing grows because width_fill=True\n ")



ipg.start_session()

