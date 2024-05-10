from icedpygui import IPG, IpgContainerAlignment
import os

# Since user data is include, it will need to be
# added to all whether used on or.
def on_press(id, _user_data):
    print("on press", id)


def on_release(id, _user_data):
    print("on release", id)


def on_right_press(id, _user_data):
    print("on right press, id")


def on_right_release(id, _user_data):
    print("on right release", id)


def on_middle_press(id, _user_data):
    print("on middle press", id)


def on_middle_release(id, _user_data):
    print("on middle release", id)


def on_enter(id, user_data):
    print("entered", id, user_data)    


def on_move(id: int, point: dict, _user_data):
    print("on move", id, point)


def on_exit(id, _user_data):
    print("on exit", id)


    

ipg = IPG()

ipg.add_window(window_id="main", title="Main", width=400, height=400, 
               pos_centered=True, debug=True)

ipg.add_container(window_id="main", container_id="cont", 
                  width_fill=True, height_fill=True,
                  align_x=IpgContainerAlignment.Center,
                  align_y=IpgContainerAlignment.Center)

# Setting up the image path
cwd = os.getcwd()
svg_path = cwd + "/resources/tiger.svg"

ipg.add_svg(parent_id="cont",
            svg_path= svg_path,
            on_enter=on_enter,
            on_exit=on_exit,
            on_move=on_move,
            on_press=on_press,
            on_release=on_release,
            on_middle_press=on_middle_press,
            on_middle_release=on_middle_release,
            on_right_press=on_right_press,
            on_right_release=on_right_release,
            user_data="Some Data")


ipg.start_session()
