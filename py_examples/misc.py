from icedpygui.icedpygui import IPG, IpgButtonStyles

ipg = IPG()



def pressed(id, name):
    ipg.update_item(id, "style",  IpgButtonStyles.Destructive)



ipg.add_window("main", "CheckBox Demo",
                                800, 800, 500, 100)
        
ipg.add_container(window_id="main", container_id="cont", width_fill=True,
                        height_fill=True, align_x="center", align_y="center")


ipg.add_button("cont", "Press Me", on_press=pressed, style=IpgButtonStyles.Text)


ipg.start_session()