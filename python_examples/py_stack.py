import os
import random
from icedpygui import IPG, IpgColor, IpgStackParam, IpgMousePointer, IpgImageContentFit
from icedpygui import IpgTextParam



def card_selected(card_id, name):
    ipg.update_item(
        wid=text_id, 
        param=IpgTextParam.Content, 
        value=f"Card selected is {name}")


ipg=IPG()        

cwd = os.getcwd()
path = path = cwd + "/python_examples/resources/cards/hearts/"

names = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"]

ipg.add_window(
        window_id="main",
        title="Stack",
        width=400.0,
        height=800.0,
        pos_centered=True)

ipg.add_container(
        window_id="main",
        container_id="cont",
        width_fill=True,
        height_fill=True)

ipg.add_column(
        window_id="main",
        container_id="main_col",
        parent_id="cont",
        height_fill=True)

text_id = ipg.add_text(
                parent_id="main_col", 
                content="Card Selected is None")

# Adds the stack container to the window.
stack_id = ipg.add_stack(
                    window_id="main",
                    container_id="stack",
                    parent_id="main_col",
                    width=200.0,
                    height=750.0)

for i in range(1, 14):
    # Adds the column to the stack to hold the space and card.
    ipg.add_column(
            window_id="main",
            container_id=f"col_{i}",
            parent_id="stack")

    file = f"{path}{i}.png"

    # The space, whcich grows with each card, allows for an offset
    # to be able to see all of the cards.  If not used, they are 
    # stacked on top of each other.
    ipg.add_space(
            parent_id=f"col_{i}",
            height=35*i-35)

    ipg.add_image(
            parent_id=f"col_{i}", 
            image_path=file,
            width=200.0, 
            height=300.0,
            content_fit=IpgImageContentFit.Fill,
            mouse_pointer=IpgMousePointer.Grabbing,
            on_press=card_selected,
            user_data=f"{names[i-1]}")


ipg.start_session()

