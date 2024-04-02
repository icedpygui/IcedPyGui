from icedpygui import IPG, IpgMenuSepTypes

ipg = IPG()


def menu_pressed(id, data):
   print(id, data)

   
ipg.add_window("main", "Menu", 500, 500
               , pos_centered=True)

ipg.add_column("main", container_id="col", align_items="start")

items = {"Menu1": ["item1-1", "item1-2", "item1-3"],
         "Menu2": ["item2-1", "item2-2", "item2-3"],
         "Menu3": ["item3-1", "item3-2", "item3-3"]}

labels = list(items.keys())
widths = [90.0, 90.0, 90.0]

ipg.add_menu("col", labels, items, widths, 
            separators=[(0, 0, IpgMenuSepTypes.Dot), (1, 1, IpgMenuSepTypes.Line), (2, 0, IpgMenuSepTypes.Label)],
             sep_label_names=["Label"], on_select=menu_pressed)



ipg.start_session()

