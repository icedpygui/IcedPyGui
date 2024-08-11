from icedpygui import IPG, IpgMenuType, IpgWindowTheme
from collections import OrderedDict


ipg = IPG()


# The menu callback requires 4 parameters and an optional user_data
# if a checkbox or toggler is used, the status will be the 4th parameter
# or None if not used or selected. Use the indexes to determine which
# widget was selected.
def menu_pressed(menu_id, bar_index, menu_index, toggled, user_data):
    print(f"menu id={menu_id} bar_index={bar_index} menu_index={menu_index} toggled={toggled} user_data={user_data}")

# Adding two windows to show color contrasts 
# Add the 1st window, the default theme is Dark
ipg.add_window("main-dark", "Menu", 
               400, 400,  
               pos_x=100, pos_y=25,
               )

# Add the 2nd window with a lighter theme
ipg.add_window("main-light", "Menu", 
               400, 400,  
               pos_x=600, pos_y=25,
               theme=IpgWindowTheme.GruvboxLight
               )

# Add a column container to to each window
ipg.add_column("main-dark", container_id="col-dark")
ipg.add_column("main-light", container_id="col-light")

# A menu needs an ordered dictionary since
# the order needs to be maintained.  The key values become the 
# menu bar labels.  The values are a list of tuples which are
# the item labels, if needed.  Not all types of items need a label.
# Use the IpgMenuType to select the different types of items.
# The Text is the standard type.
items = OrderedDict({"Menu0": [(None, IpgMenuType.Dot),
                               ("item0-1", IpgMenuType.Text), 
                               ("item0-2", IpgMenuType.Text), 
                               ("item0-3", IpgMenuType.Text),
                               ],
                     "Menu1": [("item1-0", IpgMenuType.Text), 
                               (None, IpgMenuType.Line), 
                               ("item1-2", IpgMenuType.Text), 
                               ("item1-3", IpgMenuType.Button),
                               ],
                     "Menu2": [("Label-0", IpgMenuType.Label),
                               ("item2-1", IpgMenuType.Text), 
                               ("item2-2", IpgMenuType.Checkbox), 
                               ("item2-3", IpgMenuType.Toggler),
                               (None, IpgMenuType.Circle),
                               ]})

# A list of the widths which must equal the number of menu bar labels, the dict keys,
# or they must be a list of 1 number, i.e. [90.0] to indicate all widths are 90.0
item_widths = [90.0, 100.0, 90.0]
bar_widths = [100.0]

# A list of the spacings which must equal the number of menu bar labels, the dict keys,
# or they must be a list of 1 number, i.e. [5.0] to indicate all spacings of 5.0
item_spacings = [5.0]

# Finally, we add the menus to each window.
windows = ["col-dark", "col-light"]
for col in windows:
    ipg.add_menu(col, items, 
                bar_widths, 
                item_widths,
                item_spacings=item_spacings,
                on_select=menu_pressed, 
                user_data=col)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
