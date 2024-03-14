from icedpygui.icedpygui import IPG


ipg = IPG()

# Callback used by the picklist.  The id is the picklist id.
# The data returns the item selected and can be named anything.
# The update items uses the text widget id and the "content" parameter
# to update.  In this case since data is a str, value_str is used
# as opposed to value_float, etc.
def picked_item(id, name, data):
    ipg.update_item(text_id, "content", value=f"You picked <{data}>")


def picked_item_with_ud(id, name, data, user_data):
    ipg.update_item(text_id_with_ud, "content", value=f"You picked <{data}> with user data <{user_data}>")

# Add window must be the first widget. Other windows can be added
# at anytime, as long as their widgts follow.
ipg.add_window("main", "Pick List Demo", 800, 800, 
                                    500, 100)

# all widgets need to be added to a container, so a container
# is the second widget needed.
ipg.add_column("main", container_id="col", align_items="center", width_fill=True)

ipg.add_space(parent_id="col", height=100.0)

ipg.add_text(parent_id="col", content="Select an item to see the results.  The 2nd pick list uses uer_data")

options = ["One", "Two", "Three"]

# For picklist, you just need a list of your items, a callback and ny placeholder
# The callback can update another widget or call your needed method.
ipg.add_pick_list(parent_id="col", options=options, callback=picked_item, 
                    placeholder="Choose a Number...")

ipg.add_space(parent_id="col", height=20.0)

# For this demo, a text is needed to put the selected text into,
# therefore, an id was needed.
text_id = ipg.add_text(parent_id="col", content="Nothing Picked Yet")

ipg.add_space(parent_id="col", height=20.0)

# user dta used in the case
ipg.add_pick_list(parent_id="col", options=options, callback=picked_item_with_ud, 
                    placeholder="Choose a Number...", user_data="Some user data")

ipg.add_space(parent_id="col", height=20.0)

text_id_with_ud = ipg.add_text(parent_id="col", content="Nothing Picked Yet")

# Always the last item processed.
ipg.start_session()
