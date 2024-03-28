from icedpygui import IPG, IpgTextParams


def button_pressed(id):
    print(id)


def checked(id, checked, user_data):
    if checked:
        ipg.update_item(checked_text_id, 
                        IpgTextParams.Content, 
                        "I'm checked")

    else:
        ipg.update_item(checked_text_id, 
                        IpgTextParams.Content, 
                        "I'm not checked")

    ipg.update_item(user_data_text_id, IpgTextParams.Content, user_data)


    

ipg = IPG()

ipg.add_window(window_id="main", title="Demo Window", 
                                width=600, height=500, 
                                pos_x=150, pos_y=100)

ipg.add_container("main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col", parent_id="cont")

ipg.add_button(parent_id="col", label="Press Me!", on_press=button_pressed)

ipg.add_checkbox(parent_id="col", label="Check Me!!!", 
                                    on_toggle=checked,
                                    user_data="Some string data")

checked_text_id = ipg.add_text(parent_id="col", 
                                content="This will change when I'm checked")

user_data_text_id = ipg.add_text(parent_id="col", 
                                content="User data here")

ipg.start_session()