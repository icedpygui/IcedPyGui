from icedpygui import IPG
from icedpygui import IpgColor, IpgWindowTheme
from icedpygui import IpgTableParam, IpgTextParam, IpgButtonParam, IpgCheckboxParam
import polars as pl

import random



def table_column_resize(tbl_id: int, index: int, values: list[float]):
    print(index, values)


def sum_of_column() -> str:
    return str(df["three"].sum())    


def sort_list(pkl_id: int, selected: str):
    global df, table_id
    match selected:
        case "Sort(a-z)":
            df = df.sort("str") 
        case "Sort(z-a)":
            df = df.sort("str", descending=True)
            
    # The dataframe is a bit special so use the update_dataframe
    # and not update_item
    ipg.update_dataframe(table_id, IpgTableParam.PolarsDf, df)


def math_op(pkl_id: int, selected: str, index: int):
    global df, table_id, footer_ids
    match selected:
        case "Add":
            value = f"Sum={df['three'].sum()}"
        case "Count":
            value = f"Count={df['three'].count()}"
        case "Mean":
            value = f"Mean={df['three'].mean()}"
    # The header and footer are not part of the dataframe,
    # so update the item, not the dataframe.
    ipg.update_item(footer_ids[index], IpgTextParam.Content, value)



# Normally, filtering is easy without control coulmns,
# you just pass back the new df.  However with control columns
# you'll need to do some extra work since they are not part of
# dataframe.
# The approach that can be taken is to store the ids of the widgets 
# in the dataframe and then filter out the unselected widgets
# and the update the control rows, as done below.  If you don't need
# this connection, keep the ids in a python list, get the slice of 
# the bottom ones and use update_item to either hide or delete them.
# if you want to unfilter the df later, just reshow them versus deleting them.

# Sometimes it's esy to work with a original_df and a working_df.
# In doing this your working_df is the filtered one and you can further select 
# the columns you want to show.  It also makes it easier to back out of the 
# filtering operation to get the unfiltered version.
def filtering(pkl_id: int, selected: str):
    global df, table_id
    match selected:
        case "True": 
            value = True
        case "False":
            value = False
        case "None":
            ipg.update_dataframe(table_id, IpgTableParam.PolarsDf, df)
            for wid in button_ids:
                ipg.update_item(wid, IpgButtonParam.Show, True)
            for id_tf in checkbox_ids:
                ipg.update_item(id_tf[0], IpgCheckboxParam.Show, True)
            return
    # get the filtered df
    filtered_df = df.filter(pl.col("Checks") == value)

    # now we need to hide all of the ids that were filtered out
    filtered_button_ids = filtered_df["Edit"].to_list()
    for wid in button_ids:
        if wid not in filtered_button_ids:
            ipg.update_item(wid, IpgButtonParam.Show, False)
        else:
            # in case it was hidden by another filter
            ipg.update_item(wid, IpgButtonParam.Show, True)
    
    # Since we don't hace a list of ids in the df for the checkboxes,
    # we use the values the list of list for the update.
    # We could have put put those values in the df but currently
    # we cannot pass a df into rust containing a list in the df.
    for id_tf in checkbox_ids:
        if id_tf[1] == value:
            ipg.update_item(id_tf[0], IpgCheckboxParam.Show, True)
        else:
            # in case it was hidden by another filter
            ipg.update_item(id_tf[0], IpgCheckboxParam.Show, False) 
               
    ipg.update_dataframe(table_id, IpgTableParam.PolarsDf, filtered_df)
   


ipg = IPG()

total_id = 0
selected = "None"
list_ids = []
row_ids = []
filered_ids = []
column_widths = [100.0] * 6
button_ids = [ ipg.generate_id() for _ in range(11) ]
checkbox_ids = [ [ipg.generate_id(), random.choice([True, False])] for _ in range(11) ]
checks = [ tup[1] for tup in checkbox_ids ]


# The control column are "Edit" and "Toggler" in this case.
# They can be any widget.  The only requirement for the DataFrame
# is that the columns must be of equal length.  Therefore, we fill
# them with zeroes.  You could fill them with gen_id for your widgets
# and use them for the widgets if you need their ids in a callback.
# However, the user_data works a bit easier for that but there may be 
# a use case out there somewhere.
data = {
    "Edit": button_ids,
    "str": ["H", "e", "l", "l", "o", " ", "W", "o", "r", "l", "d"],
    "one": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0],
    "Checks": checks,
    "two": [2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22],
    "three": [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33],
    }

# Polars df is used for tables because it's very fast and efficient
# usually you be filtering, sorting, and such so using the df makes 
# it pretty easy to do.
df = pl.DataFrame(data)
df_width = df.width
df_length = df.height

# Some styling for the widgets
btn_style = ipg.add_button_style(border_radius=[10.0])


# Add the window
ipg.add_window(
        window_id="main", 
        title="Table Demo",
        width=1000, 
        height=400,
        pos_centered=True,
        theme=IpgWindowTheme.TokyoNightStorm,
        debug=False)

# Add the container for centering the table
ipg.add_container(
        window_id="main", 
        container_id="cont",
        width_fill=True, 
        height_fill=True,
        centered=True,)

width = sum(column_widths)

# The table is added.
table_id = ipg.add_table(
        window_id="main",
        table_id="table",
        polars_df=df,
        parent_id="cont",
        column_widths=column_widths,
        height=150.0,
        # above required
        # width=300.0, # see the scroller when the table is smaller than the column widths, it defaults to sum of columns
        on_column_resize=table_column_resize, # may need in some cases where resizing causes alignment issues or other cases.
        # min_column_width=50.0, # uncomment to see effect
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

# As an example of having the widget interact with the database, note the button
# uses the user_data for the index, so when pressed, the callback has the row index.
# In the case of the checkbox, we are going to keep track of the if checked values
# update the checked column of the df and use a picklist to filter the df.
for i in range(df_length):
    ipg.add_button(
        parent_id="table",
        label="Edit",
        padding=[0.0, 20.0, 0.0, 20.0],
        style_id=btn_style,
        user_data=i,
        gen_id=button_ids[i])
    
    ipg.add_checkbox(
        parent_id="table",
        label="Check Me",
        is_checked=checks[i],
        user_data=i,
        gen_id=checkbox_ids[i][0])


# add the custom header row.  If you have another row,
# just repeat the process below making sure that the row
# count matches the header count.
header = [""] * df_width
for i in range(df_width):
    if i == 1:
        ipg.add_pick_list(
            parent_id="table",
            options=["Sort(a-z)", "Sort(z-a)"],
            placeholder="Sort",
            on_select=sort_list)
    elif i == 3:
        ipg.add_pick_list(
            parent_id="table",
            options=["True", "False", "None"],
            placeholder="Filter",
            on_select=filtering) 
    elif i == 5:
        ipg.add_pick_list(
            parent_id="table",
            options=["Add", "Count", "Mean"],
            placeholder="Math",
            on_select=math_op,
            user_data=5) # the footer index
    else:    
        ipg.add_text(
            parent_id="table",
            content=header[i],
            size=16.0)


# The custom footer is basically the same as the header.
# The footer ids are needed for the table footer update
# which is just a text update.
footer_ids = []
column_three_sum = df["three"].sum()
footer = [""] * 6
footer[5] = f"Sum={column_three_sum}"
for i in range(df_width):
    footer_ids.append(ipg.add_text(
                    parent_id="table",
                    content=footer[i],
                    size=14.0))




# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
