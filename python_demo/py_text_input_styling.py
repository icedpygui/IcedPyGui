from icedpygui import IPG, IpgTextParam
from icedpygui import IpgAlignment, IpgTextInputParam

ipg = IPG()


secure = False

# Currently, Ipg only has the text_input widget.
# Additional types of inputs will follow soon.
# Therefore, the return data will be a string
# that you will need to convert to whatever type you want



# add the window
ipg.add_window("main", "Text Input Demo", 600, 600,
                pos_x=100, pos_y=25)

ipg.add_container(window_id="main", container_id="cont",
                  height_fill=True,
                  width_fill=True,
                  center_xy=True,)


# add the column for the widgets, centered
ipg.add_column("main", 
               container_id="col",
               parent_id="cont",
               align_items=IpgAlignment.Center,
               height_fill=True, width_fill=True, spacing=10.0)


ipg.add_text_input_style(style_id="input_style",
                         )

# Add the text_input widget
ti_id = ipg.add_text_input(parent_id="col", 
                           placeholder="Input Some Text",
                           width=200.0,
                           
                           )






# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
