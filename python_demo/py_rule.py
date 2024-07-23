from icedpygui import IPG
from icedpygui import IpgAlignment, IpgColor


ipg = IPG()

# Add the window
ipg.add_window("main", "Rule Demo", 
               500, 650,
                pos_x=100, pos_y=25,
                # debug=True
                )

# Add a container for alignment
ipg.add_container("main", "cont", 
                  width_fill=True, height_fill=True)

# Add a column to hold the wigets
ipg.add_column("main", container_id="col", parent_id="cont",
               align_items=IpgAlignment.Center)

# Add some spacing
ipg.add_space(parent_id="col", width_fill=True, height=20.0)

# Add info
ipg.add_text(parent_id="col", content="Below are vertical and horizontal rules")

# add some styling
ipg.add_rule_style("v", 
                   color=IpgColor.YELLOW, 
                   border_radius=[10.0])

ipg.add_rule_style("h", 
                   color=IpgColor.BLUE, 
                   border_radius=[10.0])

# The fill_mode styling
ipg.add_rule_style("fill_50", 
                   color=IpgColor.BLUE, 
                   fillmode_percent=50.0)

# The padding is almost like percent except it gives you
# an unsymmetrical ability. It can be seen better if you uncomment the 
# degug mode in the window.
ipg.add_rule_style("pad",
                   color=IpgColor.BLUE,
                   fillmode_asymmetric_padding=(10, 50))


# Add the rules
ipg.add_rule_vertical("col", height=250, thickness=5, 
                      style="v")

ipg.add_rule_horizontal("col", width=250, thickness=15,
                        style="h", 
                        )

ipg.add_text(parent_id="col", content="Styling added to above with color and corner radius")

ipg.add_rule_horizontal("col", width=250, thickness=15,
                        style="fill_50", 
                        )
ipg.add_text(parent_id="col", content="Additional Styling added to above with 50% color fill")

ipg.add_rule_horizontal("col", width=250, thickness=15, 
                        style="pad")

ipg.add_text(parent_id="col", content="Additional Styling added to above with unsymmetrical padding")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
