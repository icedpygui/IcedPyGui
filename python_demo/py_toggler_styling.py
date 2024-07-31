from icedpygui import IPG, IpgColor
from icedpygui import IpgAlignment, IpgAlignment

ipg = IPG()



# Add the window
ipg.add_window("main", "Toggler Demo", 
               700, 625,  
               pos_x=100.0, pos_y=25.0)

# Add a main column to hold everything
ipg.add_column("main", "col", width_fill=True, height_fill=True,
               align_items=IpgAlignment.Center, spacing=5.0)

ipg.add_space(parent_id="col", height=50.0)

# Add some styling to the toggler
ipg.add_toggler_style(style_id="tog_style",
                      background_color=IpgColor.GREEN,
                      background_color_toggled=IpgColor.LIGHT_GREEN,
                      foreground_color=IpgColor.ANTIQUE_WHITE,
                      foreground_color_toggled=IpgColor.BLUE,
                      )

# Add the toggler and change size to see styling better
tog_id = ipg.add_toggler("col",
                         label="Some Toggler Label",
                         size=40.0,
                         text_size=25.0,
                         style="tog_style"
                         )



# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
