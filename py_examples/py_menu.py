from icedpygui.icedpygui import IPG

ipg = IPG()


def menu_1_pressed(id, data, user_data_str, user_data_flt, user_data_int):
   print(id)

ipg.add_column("main", align_items="center")

ipg.add_menu_bar(parent_id="main", items=["Menu1", "Menu2", "Menu3"])
ipg.add_menu_item("Menu1", item="Menu1", callback=menu_1_pressed)

ipg.main_window("Python Wrapper of Rust Iced", 800, 800, 
                                    (500, 100), True, debug=False)


