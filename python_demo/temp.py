from icedpygui import IPG, IpgTextParams
from icedpygui import IpgButtonParams
from icedpygui import IpgColumnAlignment



def btn_pressed(btn_id):
    ipg.update_item(txt_id, IpgTextParams.Content, f"id = {btn_id}")
    val = 30/0


ipg = IPG()

ipg.add_window(window_id="main", title="Main", width=400, height=400, pos_centered=True)

ipg.add_column(window_id="main", container_id="col")

ipg.add_button(parent_id="col", label="Press Me", on_press=btn_pressed)

txt_id = ipg.add_text(parent_id="col", content="id = ")

ipg.start_session()
