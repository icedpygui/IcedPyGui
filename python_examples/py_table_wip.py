from icedpygui import IPG, IpgHorizontalAlignment, IpgVerticalAlignment
from icedpygui import IpgTableRowHighLight, IpgColor, IpgTextParam
from icedpygui import IpgContainerParam, IpgAlignment
import polars as pl


ipg = IPG()

global total_checks
total_checks = 0
total_id = 0
selected = "None"
list_ids = []
row_ids = []
filered_ids = []
column_widths = [100.0, 150.0, 150.0, 150.0, 150.0, 150.0]
data = {
    "str": ["H", "e", "l", "l", "o"],
    "one": [1.0, 2.0, 3.0, 4.0, 5.0],
    "two": [1, 2, 3, 4, 5],
    "three": [1, 2, 3, 4, 5],
    "four": [1, 2, 3, 4, 5],
    "five": [1, 2, 3, 4, 5],
}
df = pl.DataFrame(data)
df_ids = pl.DataFrame()




# def open_modal(btn_id: int, index: tuple[int, int]):
#     ipg.update_item(modal_id, IpgContainerParam.Show, True)
#     ipg.update_item(modal_title, IpgTextParam.Content, f"Modal for Row {index[0]}")


# def close_modal(btn_id: int):
#     ipg.update_item(modal_id, IpgContainerParam.Show, False)


def filter(pick_id: int, select: str):
    print()


btn_style = ipg.add_button_style(border_radius=[10.0])
chk_style = ipg.add_checkbox_style(border_width=3.0)
tog_style = ipg.add_toggler_style(
                    background_border_width=2.0, 
                    background_border_color=IpgColor.YELLOW)


# Add the window
ipg.add_window(
        window_id="main", 
        title="Table Demo",
        width=1000, 
        height=600,
        pos_centered=True,
        debug=False)

# Add the container for centering the table
ipg.add_container(
        window_id="main", 
        container_id="cont",
        width_fill=True, 
        height_fill=True,
        padding=[20.0])

width = sum(column_widths)

ipg.add_stack(
        window_id="main",
        container_id="stack",
        parent_id="cont")

# The table is added.
ipg.add_table(
        window_id="main",
        table_id="table",
        title="My Table",
        polars_df=df,
        parent_id="stack",
        column_widths=column_widths,
        height=300.0,
        # footer_enabled=True,
        table_width_fixed=True, # defaults to True, change to False to see the effect
        )


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
