from icedpygui import IPG, IpgColor
from icedpygui import IpgColumnAlignment


ipg = IPG()


ipg.add_window("main", "Scollable - Styling",
                            600, 600,
                            pos_centered=True)


# Add a container just to get some spacing from the top
ipg.add_container(window_id="main", container_id="cont",
                        width_fill=True, height=100.0, padding=[20])

# Let's add some styling for the scrollable before adding it.
# Let's style the bar first with a dark color
ipg.add_styling_background("s_bar_bkg", color=IpgColor.DARK_BLUE)

# Add another bkg for the scroller
ipg.add_styling_background("scroller_bkg", color=IpgColor.BLUE)

# Add another bkg for the container background, since only a Container has
# has any styling and Column and Row do not.  You could have added the column
# to the container and styled it but this saves a step.
ipg.add_styling_background(style_id="cont_bkg", color=IpgColor.DARK_BLUE)

# Need another style for the text color
ipg.add_styling_text_color("cont_text", color=IpgColor.ORANGE)

# Add the scrollable restricting the width so it can center and height 
# wich is less than the length of the data in the column for scrolling.
scroll_id_1 = ipg.add_scrollable(window_id="main", container_id="scroll",
                                width=500, height=150.0,
                                scroll_bar_style_background="s_bar_bkg",
                                scroller_style_background="scroller_bkg",
                                container_style_background="cont_bkg",
                                container_style_text_color="cont_text")

# Add the column for the data
ipg.add_column(window_id="main", container_id="col",
                            parent_id="scroll", width_fill=True,
                            align_items=IpgColumnAlignment.Center)

#  Add some content to scroll
for i in range(0, 25):
    ipg.add_text("col", content="Scroll Me Up and Down! Scroll Me Up and Down!")




ipg.start_session()