from icedpygui import IPG, IpgImageUpdate, IpgTextUpdate
import os

ipg = IPG()


cwd = os.getcwd()
path_happy = cwd + "/resources/rustacean-flat-happy.png"
path_orig = cwd + "/resources/rustacean-orig-noshadow.png"

image_ids = []

# Note: Since user_data is being user, it needs to be added
# to all callback methods.  However, points are only generated for
# the mouse move so there is no need to include that in all callbacks.
def image_selected(id, user_data):
    index = image_ids.index(id)
    ipg.update_item(text_ids[index], IpgTextUpdate.Content, "You Pressed Me!")


def on_mouse_move(id, point, user_data):
    index = image_ids.index(id)
    x = '{:{}.{}}'.format(point.get('x'), 10, 4)
    y = '{:{}.{}}'.format(point.get('y'), 10, 4)
    ipg.update_item(text_points[index], IpgTextUpdate.Content, f"x={x}\ny={y}\n")


def on_mouse_exit(id, user_data):
    index = image_ids.index(id)
    ipg.update_item(text_points[index], IpgTextUpdate.Content, "Point")

# On right_press, original shows
def show_original(id, index):
    ipg.update_item(image_ids[index], IpgImageUpdate.ImagePath, path_orig)

# On middle press, happy shows
def show_happy(id, index):
    ipg.update_item(image_ids[index], IpgImageUpdate.ImagePath, path_happy)


ipg.add_window(window_id="main", title="Date Picker Demo", width=800, height=800, 
                                    pos_x=500, pos_y=100)

ipg.add_container(window_id="main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

# height was used in the column here because the default is shrink and when the 
# text widgets change in height due to the content, the column will shrink or expand
# and move back to center.  So to reduce this movement, height was set.
ipg.add_column(window_id="main", container_id="col", parent_id="cont", height=400.0)

ipg.add_text("col", "Pressing the middle and right mouse buttons, while the mouse is over an image, will change the image.", width=600.0)

# adding a row for the line of images
ipg.add_row(window_id="main", container_id="row1", parent_id="col")

# Looping to add the images
for i in range(0, 5):
    image_ids.append(ipg.add_image(parent_id="row1", image_path=path_happy, 
                                   width=100.0, height=50.0, 
                                   on_press=image_selected,
                                   on_move=on_mouse_move,
                                   on_exit=on_mouse_exit,
                                   on_right_press=show_original,
                                   on_middle_press=show_happy,
                                   user_data=i))

# add a row for the information
ipg.add_row(window_id="main", container_id="row2", parent_id="col")

text_ids = []
text_points = []

for i in range(0, 5):
    text_ids.append(ipg.add_text(parent_id="row2", content="Press image above me", width=100.0))

# adding a final row for the points display
ipg.add_row(window_id="main", container_id="row3", parent_id="col")

for i in range(0, 5):
    text_points.append(ipg.add_text(parent_id="row3", content="Point", width=100.0))


ipg.start_session()
