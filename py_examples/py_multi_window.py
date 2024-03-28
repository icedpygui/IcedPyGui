from icedpygui import IPG

ipg = IPG()

ipg.add_window("main1", "MultWindow 1", 500, 600, 
                                200, 100)

ipg.add_column("main1", container_id="col1", parent_id="main1", 
               align_items="center", width=800.0, height=800.0)

ipg.add_space(parent_id="col1", width=100.0, height=100.0)

ipg.add_text(parent_id="col1", content="There is a space outlined above me")


ipg.add_window("main2", "MultWindow 2", 500, 600, 
                                800, 100, show=True)

ipg.add_column("main2", container_id="col2", parent_id="main2", 
               align_items="center", width=800.0, height=800.0)

ipg.add_space(parent_id="col2", width=100.0, height=100.0)

ipg.add_text(parent_id="col2", content="There is a space outlined above me")

ipg.start_session()

