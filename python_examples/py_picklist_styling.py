from icedpygui import IPG, IpgPickListHandle
from icedpygui import IpgAlignment, IpgButtonArrow, IpgColor


ipg = IPG()


# The data returns the item selected and can be named anything.
# The update items uses the text widget id and the "content" parameter
# to update.  The value is what you want the content parameter to equal.
def picked_item(pl_id: int, data: str):
    print(f"pl_id = {pl_id} data = {data}")


def picked_item_with_user_data(pl_id: int, data: str, user_data: any):
    print(f"pl_id = {pl_id} data = {data}, user_data = {user_data}")
    

# Add window must be the first widget. Other windows can be added
# at anytime.
ipg.add_window(
        window_id="main", 
        title="Pick List Demo", 
        width=700, 
        height=400,
        pos_centered=True)

# all widgets need to be added to a container, so a container
# is the second widget needed.
ipg.add_column(
        window_id="main", 
        container_id="col",
        align=IpgAlignment.Start, 
        width_fill=True)

ipg.add_space(
        parent_id="col", 
        height=50.0)

# add a row for picklist and a button to change option
ipg.add_row(
        window_id="main", 
        container_id="row1", 
        parent_id="col")

# A PickList requires that the options you want to select be in a list.
# The list cannot be of mixed types, all will be converted to strings
options = ["One", "Two", "Three"]

# A PickList is added and the handle is not defined which
# will result in a down arrow used in the PickList box.
ipg.add_pick_list(
        parent_id="row1", 
        options=options,
        placeholder="Choose a Number...",
        on_select=picked_item)

ipg.add_text(
        parent_id="row1", 
        content="Default Arrow Style used")

# add a row for picklist and a button to change option
ipg.add_row(
        window_id="main", 
        container_id="row2", 
        parent_id="col")

# Another PickList is added and the handle is defined as an arrow which
# will result in a down arrow used again but the size can be changed.
ipg.add_pick_list(
        parent_id="row2", 
        options=options, 
        placeholder="Choose a Number...", 
        handle=IpgPickListHandle.Arrow,
        arrow_size=30.0,
        on_select=picked_item)

ipg.add_text(
        parent_id="row2", 
        content="Arrow Style with down arrow sized to 30")

# add another row for picklist
ipg.add_row(
        window_id="main", 
        container_id="row3", 
        parent_id="col")

# Another PickList is added and the handle is defined as Dynamic which
# will result in an arrow used for the open and closed position.
ipg.add_pick_list(
        parent_id="row3", 
        options=options, 
        placeholder="Choose a Number...", 
        handle=IpgPickListHandle.Dynamic,
        arrow_size=20.0, # the dynamic arrows can be sized too, if needed
        dynamic_closed=IpgButtonArrow.ArrowUp,
        dynamic_opened=IpgButtonArrow.ArrowDown,
        on_select=picked_item)

ipg.add_text(
        parent_id="row3", 
        content="Arrow Style with right and down arrow sized to 20")

# add another row for picklist
ipg.add_row(
        window_id="main", 
        container_id="row4", 
        parent_id="col")

# Another PickList is added and the handle is defined as an Custom which
# will result in a down arrow used again but the size can be changed.
ipg.add_pick_list(
        parent_id="row4", 
        options=options, 
        placeholder="Choose a Number...", 
        handle=IpgPickListHandle.Static,
        arrow_size=20.0, # the custom arrows can be sized too, if needed
        custom_static=IpgButtonArrow.ArrowNinezerodegDown,
        on_select=picked_item)

ipg.add_text(
        parent_id="row4", 
        content="Arrow Custom Style sized to 20.")


# Add some styling
colors = ipg.add_pick_list_style( 
                background_color=IpgColor.DARK_OLIVE_GREEN,
                border_color=IpgColor.ANTIQUE_WHITE,
                border_color_hovered=IpgColor.BLUE,
                handle_color=IpgColor.DARK_SEA_GREEN,
                text_color=IpgColor.LIGHT_GREEN,
                border_width=3.0,)

# add another row for picklist
ipg.add_row(
        window_id="main", 
        container_id="row5", 
        parent_id="col")

# Another PickList is added and the handle is defined as an Custom which
# will result in a down arrow used again but the size can be changed.
ipg.add_pick_list(
        parent_id="row5", 
        options=options, 
        arrow_size=25.0,
        placeholder="Choose a Number...",
        style_id=colors,
        on_select=picked_item_with_user_data,
        user_data="Some data")

ipg.add_text(
        parent_id="row5", 
        content="Background, Border, Handle, and Text styling")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
