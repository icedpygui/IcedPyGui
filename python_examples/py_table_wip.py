from icedpygui import IPG, IpgHorizontalAlignment, IpgVerticalAlignment
from icedpygui import IpgColor
from icedpygui import IpgTableParam
import polars as pl
import uuid


def generate_unique_id():
  return uuid.uuid1()

def table_column_resize(tbl_id: int, index: int, values: list[float]):
    print(index, values)


def sum_of_column() -> str:
    return str(df["three"].sum())    


ipg = IPG()

global total_checks
total_checks = 0
total_id = 0
selected = "None"
list_ids = []
row_ids = []
filered_ids = []
column_widths = [100.0] * 6
fill = [0 for _ in range(0, 11) ]

# The control column are "Edit" and "Toggler" in this case.
# They can be any widget.  The only requirement for the DataFrame
# is that the columns must be of equal length.  Therefore, we fill
# them with zeroes.  You could fill them with gen_id for your widgets
# and use them for the widgets if you need their ids in a callback.
# However, the user_data works a bit easier for that but there may be 
# a use case out there somewhere.
data = {
    "Edit": fill,
    "str": ["H", "e", "l", "l", "o", " ", "W", "o", "r", "l", "d"],
    "one": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0],
    "Checks": fill,
    "two": [2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22],
    "three": [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33],
    }

# Polars df is used for tables because it's very fast and efficient
# usually you be filtering, sorting, and such so using the df makes 
# it pretty easy to do.
df = pl.DataFrame(data)

def filter(pick_id: int, select: str):
    print()

# Some styling for the widgets
btn_style = ipg.add_button_style(border_radius=[10.0])


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
        polars_df=df,
        parent_id="cont",
        column_widths=column_widths,
        height=150.0,
        # above required
        # width=300.0, # see the scroller when the table is smaller than the column widths, it defaults to sum of columns
        width_fill=False, # best to allow the width to be the sum of the columns but maybe there is a use case out there
        on_column_resize=table_column_resize, # may need in some cases where resizing causes alignment issues or other cases.
        custom_header_rows=1, # the number of additional header rows, default=0
        custom_footer_rows=1, # the number of footer rows, default=0
        control_columns=[0, 3], # list for the indexes of the control columns
        )


# Once the table is added, you can add the other items
# IMPORTANT:
# You must add these in order, if you have indicated
# in them in the table parameters.
# 1. control_columns
# 2. custom_header_row
# 3. custom_footer_row


# Important:
# When you add the control columns, The you must keep the rows
# together.  Below, the button was added then the toggler.
# Therefore when the table code iterates through the rows,
# each element is pulled out as needed from one vector.
# If you add all the buttons then the togglers, you'll have buttons
# in your rows until they run out then togglers.
# If you want mixed columns then you could use the index to
# determine whether it's a button or something else,
# i.e. if index == 1 add_button else add checkbox  

# For the widgets in the control columns,
# the lengths must match the dataframe or you'll
# get an error.  
# 
# In the Table code, each widget is
# placed into a container and the width of the column
# is used as the container width along with centering the
# widget.  
# 
# It's best to use the default shrink to keep things centered.  
# If you set the width to fill, some widgets may not align there labels
# correctly because it doesn't know the size.  If you use a set width, 
# the alignment works but if you resize the column, you'll need to resize all of 
# the widths too, not a big effort in the callback.  If you want a wider
# widget with a smaller label, try using just the padding on each side.  The button
# in the this table uses the padding as an example of making the button wider
# but keeping the default shrink. 
# 
# Some cases may require resizing the widget.

# You can place any widget or a combination of widgets in the table,
# just as long as you put only one widget or one parent widget in the column cell.
for i in range(0, 11):
    ipg.add_button(
        parent_id="table",
        label="Edit",
        padding=[0.0, 20.0, 0.0, 20.0],
    #    on_press=open_modal,
        style_id=btn_style,
        user_data=i)

    is_checked = False;
    if i%2 == 0: 
        is_checked = True
        
    ipg.add_checkbox(
        parent_id="table",
        label="Check Me",
        is_checked=is_checked,
        user_data=i)


# add the custom header row.  If you have another row,
# just repeat the process below making sure that the row
# count matches the header count.
header = ["this", "is", "a", "custom", "header", "row"]
for i in range(0, 6):
    ipg.add_text(
        parent_id="table",
        content=header[i])

# The custom footer is basically the same as the header.
column_three_sum = df["three"].sum()
footer = ["this", "is", "a", "custom", "footer", f"Sum={column_three_sum}"]
for i in range(0, 6):
    ipg.add_text(
        parent_id="table",
        content=footer[i])

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
