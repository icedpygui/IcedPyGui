from icedpygui import IPG, IpgTextParams


ipg = IPG()

scroll_total = 0

# The mouse move callback will fire when the window opens.
def mouse_move(id, name, point, user_data):
    ipg.update_item(text_for_moved, IpgTextParams.Content, f"{name} {point}")

def button_pressed(id, name, user_data):
    ipg.update_item(text_for_pressed, IpgTextParams.Content, f"{name}")
    ipg.update_item(text_for_user_data, IpgTextParams.Content, f"user data = {user_data}")

def button_released(id, name, user_data):
    ipg.update_item(text_for_released, IpgTextParams.Content, f"{name}")


def button_scrolled(id, name, scroll, user_data):
    global scroll_total
    scroll_total += scroll.get("y")
    ipg.update_item(text_for_scroll, IpgTextParams.Content, f"{name} {scroll} total = {scroll_total}")



ipg.add_event_mouse(enabled=True,   
                    on_move=mouse_move, 
                    on_left_press=button_pressed,
                    on_left_release=button_released,
                    on_middle_press=button_pressed,
                    on_middle_release=button_released,
                    on_right_press=button_pressed,
                    on_right_release=button_released,
                    on_middle_scroll=button_scrolled,
                    user_data="Some Data",
                    )

ipg.add_window("main", "Mouse Handler Demo", 800, 800, 
                                    500, 100, True)

ipg.add_column("main", container_id="col", align_items="center", 
                                width_fill=True, height_fill=True)

ipg.add_space(parent_id="col", height=150.0)

text_for_moved = ipg.add_text(parent_id="col", content="Mouse position will be here")
text_for_pressed = ipg.add_text(parent_id="col", content="Button presses will show here")
text_for_released = ipg.add_text(parent_id="col", content="Button releases will show here")
text_for_scroll = ipg.add_text(parent_id="col", content="Button scroll data will show here")
text_for_user_data = ipg.add_text(parent_id="col", content="Button user data will show here")

ipg.start_session()