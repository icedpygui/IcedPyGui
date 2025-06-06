from icedpygui import IPG, IpgRadioParam, IpgRadioDirection, IpgTextParam
from icedpygui import IpgAlignment, IpgHorizontalAlignment, IpgColor



# Simple demo to show that the limit of the radios
# is 26 groups of 26 radio buttons.
# If you need more, request an increase.

ipg = IPG()

def radio_cb(id: int, selected: tuple):
    radio_index = selected[0]
    radio_label=selected[1]
    print(id, radio_index, radio_label)

ipg.add_window(
        window_id="main", 
        title="Radio Demo", 
        width=1400, 
        height=600,
        pos_x=100, 
        pos_y=25)

ipg.add_scrollable(
        window_id="main", 
        container_id="scroller")

ipg.add_column(
        window_id="main", 
        container_id="col",
        parent_id="scroller",
        align=IpgAlignment.Start, 
        padding=[5])

# The radio button limits are 26 groups with 26 items
for i in range(0, 25):
    ipg.add_row(
            window_id="main", 
            container_id=f"row{i}", 
            parent_id="col")
    ipg.add_text(
            parent_id=f"row{i}", 
            content=F"Row-{i}")
    ipg.add_radio(
            parent_id=f"row{i}", 
            labels=["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", 
                    "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"],
            direction=IpgRadioDirection.Horizontal,
            size=15.0,
            on_select=radio_cb)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
