
import polars as pl
from icedpygui import IPG, IpgWindowTheme



# data = {
#     "ids": [1, 2, 3],
#     "values": [
#         {"id": 1, "value": True},
#         {"id": 2, "value": True},
#         {"id": 3, "value": False},
#     ],
# }

data = {
    "ids": [1, 2, 3],
    "values": [(1, 0), [2, 1], [3, 2, 1]]
}
df = pl.DataFrame(data, strict=False)
print(df)
# df = pl.DataFrame(
#     {
#         "int": [1, 2],
#         "str": ["a", "b"],
#         "bool": [True, None],
#         "list": [[1, 2], [3]],
#     }
# )
# df.select(pl.struct(pl.all()).alias("my_struct"))




# ┌─────┬───────────┐
# │ ids ┆ values    │
# │ --- ┆ ---       │
# │ i64 ┆ struct[2] │
# ╞═════╪═══════════╡
# │ 1   ┆ {1,true}  │
# │ 2   ┆ {2,true}  │
# │ 3   ┆ {3,false} │
# └─────┴───────────┘


ipg = IPG()

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

column_widths = [100.0] * 2

# The table is added.
table_id = ipg.add_table(
        window_id="main",
        table_id="table",
        polars_df=df,
        parent_id="cont",
        column_widths=column_widths,
        height=150.0)


ipg.start_session()