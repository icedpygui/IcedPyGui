from icedpygui import IPG, IpgMenuSepTypes, IpgMenuParams, IpgTextParams
from collections import OrderedDict

ipg = IPG()

   # Simple callback where we put the label name that was pressed
   # into a text widget.  Callback params are selceted by importing
   # the appropiate widgets parameter enums and selecting the parameter.
def menu_pressed(menu_id, data, user_data):
   ipg.update_item(text_id1, IpgTextParams.Content, f"You selected menu iten - {data}")
   ipg.update_item(text_id2, IpgTextParams.Content, f"Your user data - {user_data}")


   # Update the menu by adding a new dictionary.  
   # The following functions update the widths, spacing and separators but
   # but would be done at the same time in the real world.  Alos one would have to preplan
   # the update so that the above menu_press would work with the new labels.
def update_menu(btn_id):
   new_menu = OrderedDict({"New1": ["1", "2", "3"],
                           "New2": ["4", "5", "6"]
                           })
   ipg.update_item(menu_id, IpgMenuParams.MenuUpdate, new_menu)


   # In this case since one may have updated the menu, we don't know the menu width
   # so we use the single width for all.
def update_menu_widths(btn_id):
   new_widths = [80.0];
   ipg.update_item(menu_id, IpgMenuParams.Widths, new_widths)


   # In this case since one may have updated the menu, we don't know the menu spacing
   # so we use the single spacing for all.
def update_menu_spacing(btn_id):
   new_spacings = [5.0]
   ipg.update_item(menu_id, IpgMenuParams.Spacing, new_spacings)


   # Remember to put the separator in a list.  If the separator exists then
   # its replaced, if not its added.  To delete a separator, use the Delete type.
def change_menu_separators(btn_id):
   new_separator = [(1, 1, IpgMenuSepTypes.Dot)]
   ipg.update_item(menu_id, IpgMenuParams.Separators, new_separator)


def add_menu_separators(btn_id):
   new_separator = [(0, 1, IpgMenuSepTypes.Line)]
   ipg.update_item(menu_id, IpgMenuParams.Separators, new_separator)



# As always, need to add a window first
ipg.add_window("main", "Menu", 500, 700, pos_centered=True)

# Add a column container to hold everthing
ipg.add_column("main", container_id="col", align_items="start")

# A menu needs a dictionary of text values.  The key values become the 
# menu bar labels, so one needs to use an ordered dictionary.  
items = OrderedDict({"Menu1": ["item1-1", "item1-2", "item1-3"],
                     "Menu2": ["item2-1", "item2-2", "item2-3", "item2-4"],
                     "Menu3": ["item3-1", "item3-2", "item3-3"]})

# A list of the widths which must equal the number of menu bar labels, the dict keys
# or they must b a list of 1 number, i.e. [90.0] to indicate all widths are 90.0
widths = [90.0, 100.0, 90.0]

# A list of the spacings which must equal the number of menu bar labels, the dict keys
# or they must b a list of 1 number, i.e. [5.0] to indicate all spacings are 5.0
spacing = [5.0, 10.0, 5.0]

# The spearator is a list of tuples [(bar_index, menu_index(sep added after menu item), separator type)]
separators = [(0, 0, IpgMenuSepTypes.Dot), (1, 1, IpgMenuSepTypes.Line), (2, 0, IpgMenuSepTypes.Label)]

# Finally, we add the menu.  Note how the separators are added, these are optional.
menu_id = ipg.add_menu("col", items, widths, spacing, 
                        separators=separators,sep_label_names=["Label"], 
                        on_select=menu_pressed, user_data="Any kind of data, if wanted")

ipg.add_space("col", height=100)

text_id1 = ipg.add_text("col", "You selected menu iten - ")
text_id2 = ipg.add_text("col", "Your user data - ")


# let's add a button to change the widths parameter.
ipg.add_button("col", "Update Menu Widths\n The widths will shorten", on_press=update_menu_widths)

# let add a button to change the spacing parameter.
ipg.add_button("col", "Update Menu Spacing/n The Menu2 spacing will change", on_press=update_menu_spacing)

# let add a button to change a separator.
ipg.add_button("col", "Update Menu Separator\n The Menu2 separator to a dots", on_press=change_menu_separators)

# let add a button to add a separator.
ipg.add_button("col", "Add Menu Separator\n The Menu1 added line separator ", on_press=add_menu_separators)


# let add a button to change the entire menu.
ipg.add_button("col", "Update Menu Items", on_press=update_menu)


ipg.start_session()

