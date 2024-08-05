from icedpygui import IPG

ipg = IPG()


def on_open(id):
    print(id)

ipg.add_window(window_id="main", title="Main", 
               width=400, height=400, 
               pos_centered=True,
               debug=True)

ipg.add_container(window_id="main", container_id="cont", 
                  width_fill=True, height_fill=True,
                  )

ipg.add_modal(window_id="main", container_id="modal", 
              label="Modal", parent_id="cont",
              on_open=on_open)

ipg.add_container(window_id="main", container_id="modal_cont",
                  parent_id="modal",
                  width=200.0,
                  height=200.0)

ipg.add_text(parent_id="modal_cont", content="I'm a modal")

ipg.start_session()
