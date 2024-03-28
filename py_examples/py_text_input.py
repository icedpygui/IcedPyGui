from icedpygui import IPG, IpgTextParams

ipg = IPG()


def on_input(id, data):
    ipg.update_item(text_on_input_id, IpgTextParams.Content, value=data)


def on_submit(id, data):
    ipg.update_item(text_on_submit_id, IpgTextParams.Content, value=data)

def on_paste(id, data):
    ipg.update_item(text_on_paste_id, IpgTextParams.Content, value=data)

ipg.add_window("main", "Text Input Demo", 800, 800, 
                                pos_centered=True)

ipg.add_column("main", container_id="col", align_items="center",
                                height_fill=True, width_fill=True)

ipg.add_space(parent_id="col", height=150.0)

ipg.add_text_input(parent_id="col", placeholder="Input Some Text",
                                    width=200.0,
                                    on_input=on_input,
                                    on_submit=on_submit,
                                    on_paste=on_paste)

text_on_input_id = ipg.add_text(parent_id="col", content="Text here will be added when typed")

text_on_submit_id = ipg.add_text(parent_id="col", content="Text here will be added when submitted")

text_on_paste_id = ipg.add_text(parent_id="col", content="Text here will be added when pasted")

ipg.start_session()
