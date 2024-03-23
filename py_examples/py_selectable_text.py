from icedpygui.icedpygui import IPG

ipg = IPG()

left_button_pcount = 0
left_button_rcount = 0

right_button_pcount = 0
right_button_rcount = 0

middle_button_pcount = 0
middle_button_rcount = 0


# Some specific errors in callback functions may indicate that the parameter count
# is not correct.  However, these can also occur when a non-fatal python errors
# occur like incrementing a non-initialized variable.  Comment out the global
# statement below and you should get an error about parameter count.  However, 
# this is a non-fatal python error, in this case.  You can figure this out by
# commenting out your code adding None as a placehold and running it.  If the
# error doesn't happen, look for python non-fatal error.
def on_press(id):
    global left_button_pcount
    left_button_pcount += 1
    ipg.update_item(left_pressed_id, "content", f"Left button pressed {left_button_pcount} times")
    
def on_release(id):
    global left_button_rcount
    left_button_rcount += 1
    ipg.update_item(left_released_id, "content", f"Left button pressed {left_button_rcount} times")

def on_right_press(id):
    global right_button_pcount
    right_button_pcount += 1
    ipg.update_item(right_pressed_id, "content", f"Right button pressed {right_button_pcount} times")
    
def on_right_release(id):
    global right_button_rcount
    right_button_rcount += 1
    ipg.update_item(right_released_id, "content", f"Right button pressed {right_button_rcount} times")


def on_middle_press(id):
    global middle_button_pcount
    middle_button_pcount += 1
    ipg.update_item(middle_pressed_id, "content", f"Middle button pressed {middle_button_pcount} times")
    
def on_middle_release(id):
    global middle_button_rcount
    middle_button_rcount += 1
    ipg.update_item(middle_released_id, "content", f"Middle button pressed {middle_button_rcount} times")


# Add window must be the first widget. Other windows can be added
# at anytime, as long as their widgts follow.
ipg.add_window("main", "Selectable Text Demo", 800, 800, 
                                    500, 100, True)

# All widgets need to be added to a container, so a container
# is the second widget needed.
ipg.add_column("main", container_id="col", align_items="center", 
                                width_fill=True, height_fill=True)

ipg.add_space(parent_id="col", height=150.0)
# ipg.add_container("con", parent_id="main", align_x="center", align_y="center",
#                                         height=("fixed", 400), width=("fill", 0))

# A selectable is a bit more versatile than using a button styled as text.
# In this case, you can detect the left, right, and muddle buttons of the mouse.
# A single callback was used in this case but you could use indiviual ones also.
# Any are none of these callbacks can be used.
ipg.add_selectable_text(parent_id="col", text="Click on Me You Mouse Buttons! To see the changes below.", 
                                                on_press=on_press,
                                                on_release=on_release,
                                                on_right_press=on_right_press,
                                                on_right_release=on_right_release,
                                                on_middle_press=on_middle_press,
                                                on_middle_release=on_middle_release,
                                                )

ipg.add_space(parent_id="col", height=10.0)

left_pressed_id = ipg.add_text(parent_id="col", content=f"Left button pressed {left_button_pcount} times")
left_released_id = ipg.add_text(parent_id="col", content=f"Left button released {left_button_rcount} times")

right_pressed_id = ipg.add_text(parent_id="col", content=f"Right button pressed {right_button_pcount} times")
right_released_id = ipg.add_text(parent_id="col", content=f"Left button released {right_button_rcount} times")

middle_pressed_id = ipg.add_text(parent_id="col", content=f"Middle button pressed {middle_button_pcount} times")
middle_released_id = ipg.add_text(parent_id="col", content=f"Middle button released {middle_button_rcount} times")
                       

# Always the last item processed.
ipg.start_session()
