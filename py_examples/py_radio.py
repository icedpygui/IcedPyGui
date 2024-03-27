from icedpygui import IPG, RadioParams, RadioDirection, IpgTextUpdate


ipg = IPG()


def selected_radio_1(rd_id, data, user_data):
    ipg.update_item(rd_text_id_1, IpgTextUpdate.Content, 
        f"Radio callback id = {rd_id}, \n index = {data[0]}, label = {data[1]} \n user_data = {user_data}")


def selected_radio_2(rd_id, data, user_data):
    ipg.update_item(rd_text_id_2, IpgTextUpdate.Content, 
        f"Radio callback id = {rd_id}, \n index = {data[0]}, label = {data[1]} \n user_data = {user_data}")


def change_direction(btn_id, radio_ids):
    ipg.update_item(radio_ids[0], RadioParams.Direction, RadioDirection.Horizontal)
    ipg.update_item(radio_ids[1], RadioParams.Direction, RadioDirection.Vertical)


    # for the user_data in this example, the selection choice was added also.
    # This method could have been used for the avoce too.  A list was used here but
    # any data type can be used.  Maybe a dirctionary would be better for more complex data.
def change_selection(btn_id, radio_data):
    ipg.update_item(radio_data[0], RadioParams.SelectedIndex, radio_data[1])
    ipg.update_item(radio_data[2], RadioParams.SelectedIndex, radio_data[3])


def change_labels(btn_id, radio_data):
    ipg.update_item(radio_data[0][0], RadioParams.Labels, radio_data[1])
    ipg.update_item(radio_data[0][1], RadioParams.Labels, radio_data[2])
    ipg.update_item(radio_data[0][0], RadioParams.SelectedIndex, None)
    ipg.update_item(radio_data[0][1], RadioParams.SelectedIndex, None)


ipg.add_window("main", "Radio Demo", 800, 800, 
                                    500, 100,)

ipg.add_container("main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col", parent_id="cont", align_items="center")

ipg.add_text(parent_id="col", content="Press the radio buttons to see the selection")


# Add a row for the two groupds of buttons
ipg.add_row("main", "row1", "col", spacing=100.0)

radio_left_id = ipg.add_radio(parent_id="row1", labels=["one", "two", "three", "four"], 
                                on_select=selected_radio_1, user_data="Some data")

radio_right_id = ipg.add_radio(parent_id="row1", labels=["four", "five", "six"], 
                               direction=RadioDirection.Horizontal,
                                on_select=selected_radio_2, user_data="Some data")


# add a row for the text associated with the above radio groups
ipg.add_row("main", "row2", "col", spacing=50.0)

rd_text_id_1 = ipg.add_text("row2", content="Radio callback data:")
rd_text_id_2 = ipg.add_text("row2", content="Radio callback data:")


ipg.add_button("col", "Press Me to change direction of radios",
               on_press=change_direction, user_data=[radio_left_id, radio_right_id])

ipg.add_button("col", "Press Me to change the selected radios",
               on_press=change_selection, user_data=[radio_left_id, 2, radio_right_id, 2])

ipg.add_button("col", "Press Me to change the labels",
               on_press=change_labels, user_data=[[radio_left_id, radio_right_id], 
                                                  ["seven", "eight", "nine", "ten"], 
                                                  ["cat", "dog", "horse"]])



ipg.start_session()
