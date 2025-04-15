from icedpygui import IPG, IpgHorizontalAlignment
from icedpygui import IpgColor
from icedpygui import IpgTableParam
import polars as pl
import uuid


def generate_unique_id():
  return uuid.uuid1()

def table_column_resize(tbl_id: int, index: int, value: float):
    diff = column_widths[index] - value

    # Adjust the left side
    column_widths[index] = value
    
    # Adjust the right side
    if index < len(column_widths)-1:
            column_widths[index+1] += diff
    
    # update the column widths in the table
    ipg.update_item(
        wid=tbl_id,
        param=IpgTableParam.ColumnWidths,
        value=column_widths,
    )
    

ipg = IPG()

global total_checks
total_checks = 0
total_id = 0
selected = "None"
list_ids = []
row_ids = []
filered_ids = []
column_widths = [100.0] * 5
edit = [str(generate_unique_id()) for _ in range(0, 11) ]

data = {
    "Edit": edit,
    "str": ["H", "e", "l", "l", "o", " ", "W", "o", "r", "l", "d"],
    "one": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0],
    "two": [2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22],
    "three": [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33],
    }
df = pl.DataFrame(data)

# The number of footer items must match the 
# number of columns and they must be strings
footer = ["This", "is", "a", "footer", "100"]


def filter(pick_id: int, select: str):
    print()


btn_style = ipg.add_button_style(border_radius=[5.0])
chk_style = ipg.add_checkbox_style(border_width=3.0)
tog_style = ipg.add_toggler_style(
                    background_border_width=2.0, 
                    background_border_color=IpgColor.YELLOW)


# Add the window
ipg.add_window(
        window_id="main", 
        title="Table Demo",
        width=1000, 
        height=400,
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

# The table is added.
ipg.add_table(
        window_id="main",
        table_id="table",
        title="My Table",
        polars_df=df,
        parent_id="cont",
        column_widths=column_widths,
        height=300.0,
        control_columns=[0],
        header_enabled=True, #default value
        footer=footer,
        table_width_fixed=True, # defaults to True, change to False to see the effect
        on_column_resize=table_column_resize,
        )



ctrl_buttons = []
for i in range(0, 5):
    ipg.add_button(parent_id="table",
                   label="Edit",
                   padding=[0.0],
                   width_fill=True,
                #    on_press=open_modal,
                   style_id=btn_style,
                   text_align_x=IpgHorizontalAlignment.Center,
                   user_data=edit[i])




# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
