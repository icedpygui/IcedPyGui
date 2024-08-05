from icedpygui import IPG, IpgRadioParam, IpgRadioDirection, IpgTextParam
from icedpygui import IpgAlignment, IpgHorizontalAlignment, IpgColor


ipg = IPG()

def radio_cb(id: int, selected: tuple):
    radio_index = selected[0]
    radio_label=selected[1]
    print(id, radio_index, radio_label)

# **************Window Constructions Starts Here*************************

ipg.add_window("main", "Radio Demo", 1400, 600,
                pos_x=100, pos_y=25)

ipg.add_scrollable(window_id="main", container_id="scroller")

ipg.add_column(window_id="main", container_id="col",
               parent_id="scroller",
               align_items=IpgAlignment.Start, 
               padding=[5])

# The radio button limits are 25 groups with 25 items
for i in range(0, 25):
    ipg.add_row(window_id="main", container_id=f"row{i}", parent_id="col")
    ipg.add_text(parent_id=f"row{i}", content=F"Row-{i}")
    ipg.add_radio(parent_id=f"row{i}", 
                    labels=["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", 
                            "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"],
                    direction=IpgRadioDirection.Horizontal,
                    size=15.0,
                    on_select=radio_cb)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
