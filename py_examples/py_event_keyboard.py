from turtle import clear
from icedpygui import IPG, IpgTextParams


ipg = IPG()

# When ctrl, alt, logo, or shift are processed, they are keys.  Therefore modifier is "None".
# When a key follows the command key, then modifier is the command key and key is the key.
# So to reduce confusion with the text widget values, when the command key is first pressed,
# the modifier is set to "" versus print out a "None".
def key_pressed(id, name, key, user_data):
    modifier = key.get("modifier")
    if key.get("modifier") == "None":
        modifier = ""
    
    ipg.update_item(text_pressed, IpgTextParams.Content, f"{name}: {modifier} {key.get('key')}")
    ipg.update_item(text_user_data, IpgTextParams.Content, f"user data is {user_data}")


def key_released(id, name, key, user_data):
    ipg.update_item(text_released, IpgTextParams.Content, f"{name}: {key.get('modifier')} {key.get('key')}")


ipg.add_event_keyboard(enabled=True, on_key_press=key_pressed, on_key_release=key_released, user_data=[25])

ipg.add_window("main", "KeyBoard Handler Demo", 800, 800, 
                                    500, 100, True)

ipg.add_column("main", container_id="col", align_items="center", 
                                width_fill=True, height_fill=True)
 
ipg.add_space(parent_id="col", height=150.0)

text_pressed = ipg.add_text(parent_id="col", content="Key presses will show here")
text_released = ipg.add_text(parent_id="col", content="Key releases will show here")

ipg.add_space(parent_id="col", height=50.0)

text_user_data = ipg.add_text(parent_id="col", content="Some user data will be diplayed here")

ipg.start_session()
clear()