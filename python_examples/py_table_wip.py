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
    "Edit": [1, 2, 3, 4, 5],
    "one": [1, 2, 3, 4, 5],
    "two": [1, 2, 3, 4, 5],
    "three": [1, 2, 3, 4, 5],
    "four": [1, 2, 3, 4, 5],
    "five": [1, 2, 3, 4, 5],
}
df = pl.DataFrame(data)
df_ids = pl.DataFrame()


def checkbox(tbl_id: int, on_toggle: bool):
    global total_checks
    if on_toggle:
        total_checks += 1
    else:
        total_checks -= 1
        
    ipg.update_item(total_id, IpgTextParam.Content, f"Total Checked = {total_checks}")


def open_modal(btn_id: int, index: tuple[int, int]):
    ipg.update_item(modal_id, IpgContainerParam.Show, True)
    ipg.update_item(modal_title, IpgTextParam.Content, f"Modal for Row {index[0]}")


# def close_modal(btn_id: int):
#     ipg.update_item(modal_id, IpgContainerParam.Show, False)


def filter(pick_id: int, select: str):
    if selected == "None":
        for id in list_ids:
            ipg.s("main", id, True)
        return
        
    # filter the df
    df = df.filter(pl.col('Author').str.to_lowercase().str.starts_with(selected.lower()))
    print(df)
    # select only the columns with the ids
    keepers = df.select(column_id_names)
    print(df)
    list_to_keep = []
    for column in keepers.iter_columns():
        list_to_keep.extend(column.to_list())
    print(len(list_to_keep))
    for id in list_ids:
        if id not in list_to_keep:
            ipg.show_items("main", id, False)
        else:
            # else used because the table might have already been filtered
            ipg.show_items("main", id, True)


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
        parent_id="stack",
        title="My Table",
        column_widths=column_widths,
        height=300.0,
        # footer_enabled=True,
        table_width_fixed=True, # defaults to True, change to False to see the effect
        )

# create headers
for i, column in enumerate(df.columns):
    ipg.add_column(
            window_id="main",
            container_id=f"header{i}",
            parent_id="table",
            width_fill=True)
    ipg.add_text(
        parent_id=f"header{i}",
        content=column,
        align_x=IpgHorizontalAlignment.Center,
        align_y=IpgVerticalAlignment.Center,
        width_fill=True)
    if i == 2:
        ipg.add_pick_list(
            parent_id=f"header{i}",
            options=["None", "1", "2", "3", "4", "5"],
            on_select=filter,
            selected=selected,
            width_fill=True)

# Add the rows
# make a new dataframe for ids so that the rows can be edited/filtered/updated
ids = {}
scheme = {}
for i in range(0, len(column_widths)):
    ids[str(i)] = []
    scheme[str(i)] = pl.Int64

df_ids = pl.DataFrame(ids, scheme)

# iter through the rows to create the needed widgets
# is are loaded into a df
for i in range(0, len(df)):
    row = df.row(i)
    ids = {}
    for k in range(0, len(row)):
        ids[str(k)] = []
    for j in range(0, len(row)):
        if j == 0:
            ids[str(j)] = ipg.add_button(
                            parent_id="table",
                            label=f"Edit{i}",
                            width=column_widths[0],
                            style_id=btn_style,
                            text_align_x=IpgHorizontalAlignment.Center,
                            on_press=open_modal,
                            padding=[0.0],
                            user_data=i)
        else:
            ids[str(j)] = ipg.add_text(
                            parent_id="table",
                            content=str(row[j]),
                            width_fill=True,
                            align_x=IpgHorizontalAlignment.Center,
                            align_y=IpgVerticalAlignment.Center,
                            )
    
    # add this row id ids into the main df
    new_df = pl.DataFrame(ids)
    print(new_df)
    df_ids = pl.concat([df_ids, new_df])

    # extend the list of ids for easier use in the update methods
    list_ids.extend(list(ids.values()))

# finally, concat the ids with the data vertically so that they remain together
df = pl.concat([df, df_ids], how="horizontal", rechunk=True)
print(df)

# add footer
# for i in range(0, len(headers)):
#     if i == 1:
#         total_id = ipg.add_text(
#                         parent_id="table",
#                         content=f"Total Checked = {total_checks}",
#                         width_fill=True,
#                         align_x=IpgHorizontalAlignment.Center,
#                         align_y=IpgVerticalAlignment.Center,
#                         )
#     else:
#         ipg.add_text(
#                 parent_id="table",
#                 content="",
#                 width_fill=True)       

# modal_style = ipg.add_container_style(
#                     background_color=IpgColor.DARK_SLATE_GRAY
#                     )

# modal_id = ipg.add_container(
#                     window_id="main",
#                     container_id="stack_base",
#                     parent_id="stack",
#                     width_fill=True,
#                     height_fill=True,
#                     centered=True,
#                     show=False,
#                     )

# ipg.add_container(
#         window_id="main",
#         container_id="modal",
#         parent_id="stack_base",
#         width=200.0,
#         height=300.0,
#         style_id=modal_style,
#         )

# ipg.add_column(
#         window_id="main",
#         container_id="modal_col",
#         parent_id="modal",
#         width_fill=True,
#         height_fill=True,
#         align_x=IpgAlignment.Center,
#         )

# modal_title = ipg.add_text(
#                     parent_id="modal_col",
#                     content="Modal",
#                     )

# ipg.add_button(
#         parent_id="modal_col",
#         label="Close Modal",
#         on_press=close_modal,
#         )

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
