from icedpygui.icedpygui import IPG


ipg = IPG()

def image_selected(id, name):
    print(id, name)

ipg.add_window(window_id="main", title="Date Picker Demo", width=800, height=800, 
                                    pos_x=500, pos_y=100)

ipg.add_container(window_id="main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

path = "/home/charles/Documents/rust/icedpygui_project/IcedPyGui/resources/ferris.png"

ipg.add_image("cont", path, width=200.0, height=200.0, on_press=image_selected)

ipg.start_session()