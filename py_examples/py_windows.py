from icedpygui.icedpygui import IPG

ipg = IPG()

# The debug is set to True in thsi case to allow you to see the outline of
# the widgets.  This is useful for trouble shooting widget placement.
# The second window was added after the first windows widgets were added,
# but the windows can be added at any time, as long as they are added before 
# before their widgets.

# Get the window theme and use in the add_window.  Select the one you want
# and set it to true.  This technique reduces typo errors.
theme_1 = ipg.get_window_theme(Nord=True)

#  The default position is center so a specific position is used to avoid overlaying.
ipg.add_window("window1", "Window 1", 400, 400, 
                                    300, 100,
                                    theme=theme_1, debug=True)

# A container is added first since all widgets must be placed into a container, column, or row.
# A container can have only one widget.  Use a column or row for more than one.
ipg.add_container(window_id="window1", container_id="cont1",
                    align_x="center", align_y="center", width_fill=True, height_fill=True)

ipg.add_text(parent_id="cont1", content="Window 1")

ipg.add_container(window_id="window1", container_id="cont2", 
                    align_x="center", align_y="center", width_fill=True, height_fill=True)

ipg.add_text(parent_id="cont2", content="Some text")

# *********************************************************************************
#  Get second theme
theme_2 = ipg.get_window_theme(SolarizedLight=True)

# Second window added with the light theme
ipg.add_window("window2", "Window 2", 400, 400, 
                                    800, 100,
                                    theme=theme_2, debug=True)


ipg.add_container("window2", container_id="col2", 
               align_x="center", align_y="center", width_fill=True, height_fill=True)

ipg.add_text(parent_id="col2", content="Window 2")

ipg.add_container(window_id="window2", container_id="cont2", 
                    align_x="center", align_y="center", width_fill=True, height_fill=True)

ipg.add_text(parent_id="cont2", content="Some text")

ipg.start_session()
