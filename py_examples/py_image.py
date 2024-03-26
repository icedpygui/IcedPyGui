from icedpygui import IPG, IpgTextUpdate, IpgImageUpdate


ipg = IPG()


def image_selected(id):
    index = image_ids.index(id)
    ipg.update_item(text_ids[index], IpgTextUpdate.Content, "You Pressed Me!")


def on_mouse_move(id, point):
    index = image_ids.index(id)
    x = '{:{}.{}}'.format(point.get('x'), 10, 4)
    y = '{:{}.{}}'.format(point.get('y'), 10, 4)
    ipg.update_item(text_points[index], IpgTextUpdate.Content, f"x={x}\ny={y}\n")


def on_mouse_exit(id):
    index = image_ids.index(id)
    ipg.update_item(text_points[index], IpgTextUpdate.Content, "Point")

ipg.add_window(window_id="main", title="Date Picker Demo", width=800, height=800, 
                                    pos_x=500, pos_y=100)

ipg.add_container(window_id="main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

# height was used in the column here because the default is shrink and when the 
# text widgets change in height due to the content, the column will shrink or expand
# and move back to center.  So to reduce this movement, height was set.
ipg.add_column(window_id="main", container_id="col1", parent_id="cont", height=400.0)

path = "/home/charles/Documents/icedpygui_project/IcedPyGui/resources/ferris.png"

ipg.add_row(window_id="main", container_id="row1", parent_id="col1")

image_ids = []

for i in range(0, 5):
    image_ids.append(ipg.add_image(parent_id="row1", image_path=path, 
                                   width=100.0, height=50.0, 
                                   on_press=image_selected,
                                   on_move=on_mouse_move,
                                   on_exit=on_mouse_exit))

ipg.add_row(window_id="main", container_id="row2", parent_id="col1")

text_ids = []
text_points = []

for i in range(0, 5):
    text_ids.append(ipg.add_text(parent_id="row2", content="Press image above me", width=100.0))

ipg.add_row(window_id="main", container_id="row3", parent_id="col1")

for i in range(0, 5):
    text_points.append(ipg.add_text(parent_id="row3", content="Point", width=100.0))

ipg.start_session()
