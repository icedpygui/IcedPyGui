from icedpygui import IPG, IpgPickListParams, IpgPickListHandle
from icedpygui import IpgColumnAlignment, IpgButtonArrows


ipg = IPG()


# The data returns the item selected and can be named anything.
# The update items uses the text widget id and the "content" parameter
# to update.  The value is what you want the content parameter to equal.
# def picked_item(_pl_id, data):
#     ipg.update_item(text_id, IpgTextParams.Content, value=f"You picked <{data}>")


# Add window must be the first widget. Other windows can be added
# at anytime.
ipg.add_window("main", "Pick List Demo", 800, 600,
                pos_x=100, pos_y=25)

# all widgets need to be added to a container, so a container
# is the second widget needed.
ipg.add_column("main", container_id="col",
               align_items=IpgColumnAlignment.Center, width_fill=True)


# add a row for picklist and a button to change option
ipg.add_row("main", "row1", parent_id="col")

# A PickList requires that the options you want to select be in a list.
# The list cannot be of mixed types, all will be converted to strings
options = ["One", "Two", "Three"]

# A PickList is added and the handle is not defined which
# will result in a down arrow used in the PickList box.
pl_id = ipg.add_pick_list(parent_id="row1", options=options,
                            placeholder="Choose a Number...")

ipg.add_text(parent_id="row1", content="Default Arrow Style used")

# add a row for picklist and a button to change option
ipg.add_row("main", "row2", parent_id="col")

# Another PickList is added and the handle is defined as an arrow which
# will result in a down arrow used again but the size can be changed.
pl_id = ipg.add_pick_list(parent_id="row2", options=options, 
                            placeholder="Choose a Number...", 
                            handle=IpgPickListHandle.Arrow,
                            arrow_size=30.0)

ipg.add_text(parent_id="row2", content="Arrow Style with down arrow sized to 30")

# add another row for picklist
ipg.add_row("main", "row3", parent_id="col")

# Another PickList is added and the handle is defined as Dynamic which
# will result in an arrow used for the open and closed position.
pl_id = ipg.add_pick_list(parent_id="row3", options=options, 
                            placeholder="Choose a Number...", 
                            handle=IpgPickListHandle.Dynamic,
                            arrow_size=20.0, # the dynamic arrows can be sized too, if needed
                            dynamic_closed=IpgButtonArrows.ArrowUp,
                            dynamic_opened=IpgButtonArrows.ArrowDown,
                            )

ipg.add_text(parent_id="row3", content="Arrow Style with rigth and down arrow sized to 20")

# add another row for picklist
ipg.add_row("main", "row4", parent_id="col")

# Another PickList is added and the handle is defined as an Custom which
# will result in a down arrow used again but the size can be changed.
pl_id = ipg.add_pick_list(parent_id="row4", options=options, 
                            placeholder="Choose a Number...", 
                            handle=IpgPickListHandle.Static,
                            arrow_size=20.0, # the custom arrows can be sized too, if needed
                            custom_static=IpgButtonArrows.ArrowNinezerodegDown
                            )

ipg.add_text(parent_id="row4", content="Arrow Custom Style sized to 20.")


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
