from icedpygui import IPG, IpgColor


ipg = IPG()




# Add the window first
ipg.add_window("main", "Menu", 500, 600,  pos_x=100, pos_y=25)

# Add a column container to hold everything
ipg.add_container("main", container_id="cont", width=400.0, height=200.0)

# ipg.add_styling_background(parent_id="cont", color=IpgColor.BLUE)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
