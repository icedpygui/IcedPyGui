from icedpygui import IPG, IpgMenuSepTypes, IpgMenuParams, IpgTextParams
from icedpygui import IpgMenuItemType, IpgMenuItemStyle
from collections import OrderedDict


ipg = IPG()


# Simple callback where we put the label name that was pressed
# into a text widget.  Callback params are selected by importing
# the appropriate widgets parameter enums and selecting the parameter.
def menu_pressed(_menu_id, data, user_data):
    ipg.update_item(text_id1, IpgTextParams.Content, f"You selected menu iten - {data}")
    ipg.update_item(text_id2, IpgTextParams.Content, f"Your user data - {user_data}")


# The callbacks below allow you to change all of the parameters for a widget.
# They may or may not have frequent usage but it makes the gui very flexible
# when the data that may be loaded effects the placement, sizes, etc. used.
# These callbacks also demonstrate the usage of the widget parameters and
# are used in the testing of the code to make sure it behaves as expected.

# Update the menu by adding a new dictionary.
# This update may not be a common scenario but for completion, it's included.
# Since the dictionary needs to maintain its order, user OrderedDict.
# Once the dictionary is made, update the menu using the update command.
# This action will remove all separators, so these will need to be added
# along with the other menu parameters as needed.  These have been separated
# out for this demo just to show the effect more.
def update_menu(_btn_id):
    new_menu = OrderedDict({"New1": ["1", "2", "3"],
                            "New2": ["4", "5", "6"]
                            })
    ipg.update_item(menu_id, IpgMenuParams.MenuUpdate, new_menu)


# In this case since one may have updated the menu, we don't know the menu width
# so we use the single width for all.
def update_menu_widths(_btn_id):
    new_widths = [80.0]
    ipg.update_item(menu_id, IpgMenuParams.Widths, new_widths)


# Update the spacings using 5 for all
def update_menu_spacing(_btn_id):
    new_spacings = [5.0]
    ipg.update_item(menu_id, IpgMenuParams.Spacing, new_spacings)


# Remember to put the separator in a list.  If the separator exists then
# its replaced, if not its added.  To delete a separator, use the Delete type.
def change_menu_separators(_btn_id):
    new_separator = [(1, 1, IpgMenuSepTypes.Dot)]
    ipg.update_item(menu_id, IpgMenuParams.Separators, new_separator)


# Add some new separators
def add_menu_separators(_btn_id):
    new_separator = [(0, 1, IpgMenuSepTypes.Line)]
    ipg.update_item(menu_id, IpgMenuParams.Separators, new_separator)


# Add the window first
ipg.add_window("main", "Menu", 500, 600,  pos_x=100, pos_y=25)

# Add a column container to hold everything
ipg.add_column("main", container_id="col")

# A menu needs a dictionary of text values.  The key values become the 
# menu bar labels.  One also needs to use an ordered dictionary since
# the order needs to be maintained.
items = OrderedDict({"Menu0": ["item0-0", "item0-1", "item0-2"],
                     "Menu1": ["item1-0", "item1-1", "item1-2", "item1-3"],
                     "Menu2": ["item2-0", "item2-1", "item2-2"]})

# A list of the widths which must equal the number of menu bar labels, the dict keys,
# or they must be a list of 1 number, i.e. [90.0] to indicate all widths are 90.0
widths = [90.0, 100.0, 90.0]

# A list of the spacings which must equal the number of menu bar labels, the dict keys,
# or they must b a list of 1 number, i.e. [5.0] to indicate all spacings are 5.0
spacing = [5.0, 10.0, 5.0]

# The separator is a list of tuples [(bar_index, menu_index(separator is added after menu item), separator type)]
separators = [(0, 0, IpgMenuSepTypes.Dot), (1, 1, IpgMenuSepTypes.Line), (2, 0, IpgMenuSepTypes.Label)]

item_type = [(2, 1, IpgMenuItemType.Checkbox)]
item_style = [(1, 3, IpgMenuItemStyle.Primary)]

# Finally, we add the menu.  The separators are optional parameters.
menu_id = ipg.add_menu("col", items, widths, spacing,
                       item_type=item_type,
                       item_style=item_style,
                       separators=separators, sep_label_names=["Label"],
                       on_select=menu_pressed, user_data="Some user_data")

# spacing for readability
ipg.add_space("col", height=120)

# text info widgets
text_id1 = ipg.add_text("col", "You selected menu iten - ")
text_id2 = ipg.add_text("col", "Your user data - ")

# let's add a button to change the widths parameter.
ipg.add_button("col", "Update Menu Widths - The widths will shorten", on_press=update_menu_widths)

# let add a button to change the spacing parameter.
ipg.add_button("col", "Update Menu Spacing - The Menu2 spacing will change", on_press=update_menu_spacing)

# let add a button to change a separator.
ipg.add_button("col", "Update Menu Separator - The Menu2 separator to a dots", on_press=change_menu_separators)

# let add a button to add a separator.
ipg.add_button("col", "Add Menu Separator - The Menu1 added line separator ", on_press=add_menu_separators)

# let add a button to change the entire menu.
ipg.add_button("col", "Update Menu Items", on_press=update_menu)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
