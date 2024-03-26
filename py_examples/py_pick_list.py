from icedpygui import IPG, IpgPickListUpdate, IpgTextUpdate, IpgButtonUpdate


ipg = IPG()

    # Callback used by the picklist.  The id is the picklist id.
    # The data returns the item selected and can be named anything.
    # The update items uses the text widget id and the "content" parameter
    # to update.  The value is what you want the content parameter to equal.
def picked_item(pl_id, data):
    ipg.update_item(text_id, IpgTextUpdate.Content, value=f"You picked <{data}>")

    # In this case, some user_data was sent with the on_select callback.
    # The user data can be any value as it just passes through rust and back out.
    # You can call the vars anything as long as you understand
    # the order, id of the calling widget is always first, followed by any data 
    # generated, followed by user_data.  If no data is generated, don't use a placeholder
    # just skip putting it in or you will get an error needing only 2 parameters.
def picked_item_with_ud(pl_id, data, user_data):
    ipg.update_item(text_id_with_ud, IpgTextUpdate.Content, value=f"You picked <{data}> with user data <{user_data}>")

    # Float data returned in user_data
def picked_item_with_float(pl_id, data):
    ipg.update_item(text_id_3, IpgTextUpdate.Content, value=f"You picked <{data}>")

    # A boolean in this case
def picked_item_with_bool(pl_id, data):
    ipg.update_item(text_id_4, IpgTextUpdate.Content, value=f"You picked <{data}>")


    # the pl_id is the user_data passed in by the button
def change_option_list(btn_id, pl_id):
    ipg.update_item(pl_id, IpgPickListUpdate.Options, ["four", "five", "six"])

    # Change the placeholder
def change_placeholder(btn_id, pl_id):
    ipg.update_item(pl_id, IpgPickListUpdate.Placeholder, "New Placeholder")

    # Change the padding, a padding is a list of values, see the picklist docs 
    # for the possible values and how they fit.
def change_padding(btn_id, pl_id):
    ipg.update_item(pl_id, IpgPickListUpdate.Padding, [20])

    # Hiding the widget
def change_show(btn_id, pl_id):
    ipg.update_item(pl_id, IpgPickListUpdate.Show, False)
    
    # Change the text_size
def change_text_size(btn_id, pl_id):
    ipg.update_item(pl_id, IpgPickListUpdate.TextSize, 30.0)

    # Change the text_line_height
def change_text_line_height(btn_id, pl_id):
    ipg.update_item(pl_id, IpgPickListUpdate.TextLineHeight, 3.0)

   # Change the width
def change_width(btn_id, pl_id):
    ipg.update_item(pl_id, IpgPickListUpdate.Width, 150.0)

def change_width_fill(btn_id, pl_id):
    ipg.update_item(pl_id, IpgPickListUpdate.WidthFill, True)


def add_first_row():
    # add a row for picklist and a button to change option
    ipg.add_row("main", "row1", parent_id="col")

    # A PickList reqires that the options you want to select be in a list.
    # The list cannot be of mixed types, all will be converted to strings
    options = ["One", "Two", "Three"]

    # For picklist, you just need a list of your items, a callback and ny placeholder
    # The callback can update another widget or call your needed method.
    pl_id = ipg.add_pick_list(parent_id="row1", options=options, on_select=picked_item, 
                        placeholder="Choose a Number...")

    # add the button to initiate the callback to replace the options
    ipg.add_button("row1", "Press me to change the options", 
                    user_data=pl_id, on_press=change_option_list)
    
    ipg.add_button("row1", "Press me to change the text size", 
                    user_data=pl_id, on_press=change_text_size)


def add_second_row():
    # add a row for picklist and a button to change option
    ipg.add_row("main", "row2", parent_id="col")

    # options must be of the same type, no mixing strings and numbers, etc.
    # if you need a mixed type, use strings for all and convert in your callback.
    options = [1, 2, 3]

    # user data used in this case
    pl_id = ipg.add_pick_list(parent_id="row2", options=options, on_select=picked_item_with_ud, 
                        placeholder="Choose a Number...", user_data="Some user data")
    
    ipg.add_button("row2", "Press me to change the placeholder",
                   on_press=change_placeholder, user_data=pl_id)
    
    ipg.add_button("row2", "Press me to change the text_line_height",
                   on_press=change_text_line_height, user_data=pl_id)


def add_third_row():
    # add a row for picklist and a button to change padding
    ipg.add_row("main", "row3", parent_id="col")

    # Options with one type, float in this case.
    options = [1.1, 2.2, 3.3]

    # user data used in the case
    pl_id = ipg.add_pick_list(parent_id="row3", options=options, on_select=picked_item_with_float, 
                                placeholder="Choose a Number...")
    ipg.add_button("row3", "Press me to change the padding",
                   on_press=change_padding, user_data=pl_id)
    
    ipg.add_button("row3", "Press me to change the width",
                   on_press=change_width, user_data=pl_id)

def add_fourth_row():
    # add a row for picklist and a button to change if shown
    ipg.add_row("main", "row4", parent_id="col")

    # booleans in this case
    options = [True, False]

    # Boolean options
    pl_id = ipg.add_pick_list(parent_id="row4", options=options, on_select=picked_item_with_bool, 
                        placeholder="Choose a Boolean...")

    ipg.add_button("row4", "Press me to hide the PickList",
                   on_press=change_show, user_data=pl_id)



# Add window must be the first widget. Other windows can be added
# at anytime.
ipg.add_window("main", "Pick List Demo", 800, 800, 
                                    500, 100)

# all widgets need to be added to a container, so a container
# is the second widget needed.
ipg.add_column("main", container_id="col", align_items="center", width_fill=True)

# Just adding som space at the top of the column
ipg.add_space(parent_id="col", height=100.0)

# Adding some explanation text.
ipg.add_text(parent_id="col", content="Select an item to see the results.")

# add a row for the picklist and a button
add_first_row()

# For this demo, a text is needed to put the selected text into,
# therefore, an id was needed.
text_id = ipg.add_text(parent_id="col", content="Nothing Picked From Above Yet")

# another row for the picklist and a button
add_second_row()

text_id_with_ud = ipg.add_text(parent_id="col", content="Nothing Picked From Above Yet")

# another row for the picklist and a button
add_third_row()

text_id_3 = ipg.add_text(parent_id="col", content="Nothing Picked From Above Yet")

# another row for the picklist and a button
add_fourth_row()

text_id_4 = ipg.add_text(parent_id="col", content="Nothing Picked From Above Yet")



# Always the last item processed.
ipg.start_session()
