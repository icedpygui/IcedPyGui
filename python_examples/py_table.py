from icedpygui import IPG, IpgAlignment, IpgTableRowHighLight, IpgTableWidget
import random, os

ipg = IPG()


def widget_button(tbl_id: int, index: tuple[int, int]):
    print(tbl_id, index)


def widget_checkbox(tbl_id: int, index: tuple[int, int], on_toggle: bool):
    print(tbl_id, index, on_toggle)


def widget_toggler(tbl_id: int, index: tuple[int, int], on_toggle: bool):
    print(tbl_id, index, on_toggle)


# Add the window
ipg.add_window(window_id="main", title="Table Demo", width=800, height=800,
               pos_x=100, pos_y=25)

# Add the container, since the table requires a width and height,
# the container can shrink(default) to fit.
ipg.add_container(window_id="main", container_id="cont",
                  width_fill=True, height_fill=True)

# Initialize the lists.
col0 = []
col1 = []
col2 = []
col3 = []
col4 = []
col5 = []
col6 = []

# Add some random data of different types
for i in range(0, 10):
    # labels for the buttons
    col0.append("Button")
    # labels for the checkboxes
    col1.append("")
    # labels for togglers
    col2.append("Toggle Me")
    # make a float random number
    col3.append(random.randrange(10, 99) + random.randrange(10, 99) / 100)
    col4.append(random.choice(["one", "two", "three", "four", "five", "six", "seven"]))
    col5.append(random.randrange(10, 99))
    col6.append(random.choice([True, False]))

# Create the table, the requirement is a list of dictionaries.
# Rust does not have dictionaries but a similar type is called a HashMap.
# The reason for the list of dictionaries is that you cannot extract a
# mixed dictionary into a Rust HashMap.  The HashMap has to have predefined
# types.  In this case they are <String, Vec<f64>>, <String, Vec<String>>,
# <String, Vec<i64>>, and <String, Vec<bool>>.  As one iterates through the list,
# each type is tested to see if it can be extracted in one of the types above.  If found,
# the extraction occurs and life is wonderful.  If no existing type is found, then an error occurs.
# Currently, not every variation is covered but that can be improved in future versions.
# This probably covers the vast majorities needs.  If you need that mixed column, convert
# the list to a string.  When the final version is displayed, it's converted to  a string anyway.
data = [{"Button": col0},
        {"CheckBox": col1},
        {"Toggler": col2},
        {"Col3": col3},
        {"Col4": col4},
        {"Col5": col5},
        {"Col6": col6}]

# The column widgets are prepared
buttons = []
checkboxes = []
togglers = []
selectables = []

for _ in range(0, len(col0)):
    buttons.append(IpgTableWidget.Button)
    checkboxes.append(IpgTableWidget.Checkbox)
    togglers.append(IpgTableWidget.Toggler)

# It's best to make them the same,
col_widths = [75, 75, 150, 75, 75, 75, 75]
table_width = sum(col_widths)

# The table is added.
ipg.add_table(window_id="main",
              table_id="table",
              parent_id="cont",
              title="My Table",
              data=data,
              data_length=len(col1),
              column_widths=col_widths,
              width=table_width,
              height=300.0,
              row_highlight=IpgTableRowHighLight.Lighter,
              button_fill_columns=[0],
              checkbox_fill_columns=[1],
              toggler_fill_columns=[2],
              on_button=widget_button,
              on_checkbox=widget_checkbox,
              on_toggler=widget_toggler,
              )

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
