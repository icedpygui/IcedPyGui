from icedpygui import IPG, IpgImageParam, IpgTextParam, IpgSvgParam
from icedpygui import IpgAlignment, IpgMousePointer, IpgTimerParam
import os, math


ipg = IPG()

# Note: The image is put into a mouse area container, within IPG, where these
# callbacks can be executed.  If you are using the event_mouse,
# things will get confusing.  Therefore, you'll need to disable
# the event_mouse on image entering and then enabling it on image exit.


# Setting up the image path
cwd = os.getcwd()

ferris = cwd + "/python_examples/resources/ferris_0.png"
tiger = cwd + "/python_examples/resources/tiger_0.svg"

# Global var for the ids.
ferris_ids = []
tiger_ids = []

show_ferris = [True, True, True, True, True]
show_tiger = [False, False, False, False, False]


# Callback for when the image is selected
def image_selected(image_id):
    # Get the index of the image which is the index of the text widget
    try:
        index = ferris_ids.index(image_id)
    except:
        index = tiger_ids.index(image_id)

    ipg.update_item(text_ids[index], IpgTextParam.Content, "You Pressed Me!")


# Callback for when the mouse is moving over the image.
def on_mouse_move(image_id, point):
    try:
        index = ferris_ids.index(image_id)
    except:
        index = tiger_ids.index(image_id)

    x = '{:{}.{}}'.format(point.get('x'), 10, 4)
    y = '{:{}.{}}'.format(point.get('y'), 10, 4)
    ipg.update_item(text_points[index], IpgTextParam.Content, f"x={x}\ny={y}\n")


# On exit, reset the text widget
def on_mouse_exit(image_id):
    try:
        index = ferris_ids.index(image_id)
    except:
        index = tiger_ids.index(image_id)
    ipg.update_item(text_points[index], IpgTextParam.Content, "Point")


# On right_press, ferris shows
# We need to try both because the image_id could be either the tiger or ferris
def toggle_images(image_id):
    global show_ferris, show_tiger

    try:
        index = ferris_ids.index(image_id)
    except:
        index = tiger_ids.index(image_id)

    show_ferris[index] = not show_ferris[index]
    show_tiger[index] = not show_tiger[index]

    ipg.update_item(ferris_ids[index], IpgImageParam.Show, show_ferris[index])
    ipg.update_item(tiger_ids[index], IpgSvgParam.Show, show_tiger[index])


def increment_radians(timer_id: int, counter: int):
    radians = counter*0.048481
    ipg.update_item(ferris_ids[0], IpgImageParam.RotationRadians, radians)
    ipg.update_item(ferris_ids[1], IpgImageParam.RotationRadians, radians)
    ipg.update_item(ferris_ids[2], IpgImageParam.RotationRadians, radians)
    ipg.update_item(ferris_ids[3], IpgImageParam.RotationRadians, radians)

    ipg.update_item(tiger_ids[0], IpgSvgParam.RotationRadians, radians)
    ipg.update_item(tiger_ids[1], IpgSvgParam.RotationRadians, radians)
    ipg.update_item(tiger_ids[2], IpgSvgParam.RotationRadians, radians)
    ipg.update_item(tiger_ids[3], IpgSvgParam.RotationRadians, radians)


def change_label(timer_id: int, counter: int):
    ipg.update_item(timer_id, IpgTimerParam.Label, "Stop Ferris")

# Add the window
ipg.add_window(window_id="main", title="Date Picker Demo", width=600, height=500,
               pos_x=100, pos_y=25)

# Add a column to hold the widgets
ipg.add_column(window_id="main", container_id="col", parent_id="main",
               width_fill=True, height_fill=True,
               align_items=IpgAlignment.Center)

# Add a space for readability
ipg.add_space(parent_id="col", height=50.0)

# Add some text info
ipg.add_text("col",
             "Pressing the left mouse button, while over an image, will display a message.  "
             "Pressing the right mouse button, while over the "
             "image, will toggle between ferris and the tiger.  "
             "While the mouse is over an image the the mouse position will be displayed.",
             width=600.0)

# adding a row for the line of images
ipg.add_row(window_id="main", container_id="row1", parent_id="col", spacing=0)

# Looping to add the images, each will have the same callback
# but they could be different depending on your needs.
for i in range(0, 4):

    ferris_ids.append(ipg.add_image(parent_id="row1", image_path=ferris,
                                   width=100.0, height=50.0,
                                   on_press=image_selected,
                                   on_move=on_mouse_move,
                                   on_exit=on_mouse_exit,
                                   on_right_press=toggle_images,
                                   mouse_pointer=IpgMousePointer.Pointer,
                                   show=True))
    
    tiger_ids.append(ipg.add_svg(parent_id="row1", svg_path=tiger,
                                   width=100.0, height=50.0,
                                   on_press=image_selected,
                                   on_move=on_mouse_move,
                                   on_exit=on_mouse_exit,
                                   on_right_press=toggle_images,
                                   mouse_pointer=IpgMousePointer.Pointer,
                                   show=False))
    
    # Spacing was added last because because the two images occupy the same space
    # So spacing is between the pairs
    ipg.add_space(parent_id="row1", width=10.0)

# add a row for the information
ipg.add_row(window_id="main", container_id="row2", parent_id="col")

# Using some global variables for the ids needed for the callbacks
text_ids = []
text_points = []

# Add the text below each image.  There are a number of ways this could be done,
# Another way is to add a column with the image, info, and points then put the columns into row.
for i in range(0, 4):
    text_ids.append(ipg.add_text(parent_id="row2", content="Press image above me", width=100.0))

# adding a final row for the points display
ipg.add_row(window_id="main", container_id="row3", parent_id="col")

for i in range(0, 4):
    text_points.append(ipg.add_text(parent_id="row3", content="Point", width=100.0))

ipg.add_timer(
        parent_id="col",
        label="Rotate Ferris",
        duration_ms=300, 
        on_tick=increment_radians,
        on_start=change_label)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
