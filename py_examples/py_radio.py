from icedpygui.icedpygui import IPG


ipg = IPG()


def selected_radio_1(id, name, data, user_data):
    ipg.update_item(rd_text_id_1, "content", 
        f"Radio callback id = {id}, \n name = {name}, \n index = {data[0]}, label = {data[1]} \n user_data = {user_data}")


def selected_radio_2(id, name, data, user_data):
    ipg.update_item(rd_text_id_2, "content", 
        f"Radio callback id = {id}, \n name = {name}, \n index = {data[0]}, label = {data[1]} \n user_data = {user_data}")



ipg.add_window("main", "Date Picker Demo", 800, 800, 
                                    500, 100,)

ipg.add_container("main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col", parent_id="cont", align_items="center")

ipg.add_text(parent_id="col", content="Press the radio buttons to see the selection")

ipg.add_radio(parent_id="col", labels=["one", "two", "three"], 
              on_select=selected_radio_1, user_data="Some data")

rd_text_id_1 = ipg.add_text("col", content="Radio callback data is = ")


ipg.add_radio(parent_id="col", labels=["one", "two", "three"], direction="horizontal",
              on_select=selected_radio_2, user_data="Some data")

rd_text_id_2 = ipg.add_text("col", content="Radio callback data is = ")

ipg.start_session()
