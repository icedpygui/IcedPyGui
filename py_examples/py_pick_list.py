from icedpygui.icedpygui import IPG


ipg = IPG()

# Callback used by the picklist.  The id is the picklist id.
# The data returns the item selected and can be named anything.
# The update items uses the text widget id and the "content" parameter
# to update.  The value is what you want the content parameter to equal.
def picked_item(id, data):
    ipg.update_item(text_id, "content", value=f"You picked <{data}>")

# In this case, some user_data was sent with the on_select callback.
# The user data can be any value and is it just passes through rust and back out. 
def picked_item_with_ud(id, data, user_data):
    ipg.update_item(text_id_with_ud, "content", value=f"You picked <{data}> with user data <{user_data}>")


def picked_item_with_float(id, data):
    ipg.update_item(text_id_3, "content", value=f"You picked <{data}>")


def picked_item_with_bool(id, data):
    ipg.update_item(text_id_4, "content", value=f"You picked <{data}>")

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
ipg.add_text(parent_id="col", content="Select an item to see the results.  \nThe 2nd pick list uses uer_data")

# A PickList reqires that the options you want to select be in a list.
# The list cannot be of mixed types, all will be converted to strings
options_1 = ["One", "Two", "Three"]

# For picklist, you just need a list of your items, a callback and ny placeholder
# The callback can update another widget or call your needed method.
ipg.add_pick_list(parent_id="col", options=options_1, on_select=picked_item, 
                    placeholder="Choose a Number...")

# For this demo, a text is needed to put the selected text into,
# therefore, an id was needed.
text_id = ipg.add_text(parent_id="col", content="Nothing Picked Yet")

options_2 = [1, 2, 3]

# user dta used in the case
ipg.add_pick_list(parent_id="col", options=options_2, on_select=picked_item_with_ud, 
                    placeholder="Choose a Number...", user_data="Some user data")

text_id_with_ud = ipg.add_text(parent_id="col", content="Nothing Picked Yet")

options_3 = [1.1, 2.2, 3.3]

# user dta used in the case
ipg.add_pick_list(parent_id="col", options=options_3, on_select=picked_item_with_float, 
                    placeholder="Choose a Number...")

text_id_3 = ipg.add_text(parent_id="col", content="Nothing Picked Yet")

options_4 = [True, False]

# user dta used in the case
ipg.add_pick_list(parent_id="col", options=options_4, on_select=picked_item_with_bool, 
                    placeholder="Choose a Boolean...")

text_id_4 = ipg.add_text(parent_id="col", content="Nothing Picked Yet")



# Always the last item processed.
ipg.start_session()
