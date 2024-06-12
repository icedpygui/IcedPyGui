from icedpygui import IPG
from icedpygui import IpgColumnAlignment, IpgColor


ipg = IPG()

# Add the window
ipg.add_window("main", "Rule Demo", 500, 650,
                pos_x=100, pos_y=25)

# Add a container for alignment
ipg.add_container("main", "cont", width_fill=True, height_fill=True)

# Add a column to hold the wigets
ipg.add_column("main", container_id="col", parent_id="cont",
               align_items=IpgColumnAlignment.Center)

# Add some spacing
ipg.add_space(parent_id="col", width_fill=True, height=20.0)

# Add info
ipg.add_text(parent_id="col", content="Below are vertical and horizontal rules")

# add some styling
ipg.add_styling_background("bkg_v", color=IpgColor.YELLOW)
ipg.add_styling_background("bkg_h", color=IpgColor.BLUE)
# The border style only has radius as a valid entry.
ipg.add_styling_border(style_id="border", radius=[10.0])
# The fill_mode styling
ipg.add_styling_fill_mode("fill_50", percent=50.0)
ipg.add_styling_fill_mode("pad", asymmetric_padding=(10, 30))


# Add the rules
ipg.add_rule_vertical("col", height=250, thickness=5, 
                      style_background="bkg_v")
ipg.add_rule_horizontal("col", width=250, thickness=15,
                        style_background="bkg_h", 
                        style_border="border")
ipg.add_text(parent_id="col", content="Styling added to above with color and corner radius")

ipg.add_rule_horizontal("col", width=250, thickness=15,
                        style_background="bkg_h", 
                        style_fill_mode="fill_50",
                        style_border="border")
ipg.add_text(parent_id="col", content="Additional Styling added to above with 50% color fill")

ipg.add_rule_horizontal("col", width=250, thickness=15, 
                        style_background="bkg_h", 
                        style_fill_mode="pad",
                        style_border="border")
ipg.add_text(parent_id="col", content="Additional Styling added to above with unsymmetrical padding")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
