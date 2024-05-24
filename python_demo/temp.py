from icedpygui import IPG, IpgColumnAlignment, TableRowHighLight, TableWidget
import random, os


ipg = IPG()



def widget_button(tbl_id: int, wid_index: tuple[int, int]):
    print(tbl_id, wid_index)


def widget_checkbox(tbl_id: int, wid_index: tuple[int, int], is_checked: bool):
    print(tbl_id, wid_index, is_checked)


def image_on_something(tbl_id: int, wid_index: tuple[int, int], ):
    print(tbl_id, wid_index)


def image_move(tbl_id: int, point: tuple[float, float]):
    print(tbl_id, point)


def on_text_enter(tbl_id, text_index: tuple[int, int]):
    print(tbl_id, text_index)


# Add the window
ipg.add_window(window_id="main", title="Table Demo", width=700, height=800,
                pos_x=100, pos_y=25, debug=False)

# Add the container, since the table requires a width and height,
# the container can shrink(default) to fit.
ipg.add_column(window_id="main", container_id="col",
                  width_fill=True, height_fill=True,
                  align_items=IpgColumnAlignment.Center,
                  spacing=75)

# Initialize the lists.
col0 = []
col1 = []
col2 = []
col3 = []
col4 = []
col5 = []
col6 = []

# Add some random data of different types
for i in range(0, 20):
    # labels for the button widget
    col0.append("Button")
    # labels for the checkboxes
    col1.append("")
    # make a selectable text
    col2.append("Select Me")
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
        {"ChkBox": col1},
        {"Selectable": col2},
        {"Col3": col3},
        {"Col4": col4},
        {"Col5": col5},
        {"Col6": col6}]


# The column widgets are prepared
btn_widgets = []
chkbox_widgets = []
selectable = []
for _ in range(0, len(col0)):
    btn_widgets.append(TableWidget.Button)
    chkbox_widgets.append(TableWidget.Checkbox)
    selectable.append(TableWidget.Text)

# The table is added.
ipg.add_table("col", "My Table", data, 
              width=600.0, height=300.0, 
              row_highlight=TableRowHighLight.Lighter,
              table_length=len(col1),
              widgets_using_columns= {0: btn_widgets, 1: chkbox_widgets, 2: selectable},
              on_press_button=widget_button,
              on_toggle_checkbox=widget_checkbox,
              on_enter=on_text_enter,
              )



# Setting up the image path
cwd = os.getcwd()
ferris_root_path = cwd + "/python_demo/resources/ferris"
tiger_root_path = cwd + "/python_demo/resources/tiger"
ferris = []
tiger = []
ferris_type = []
tiger_type = []
data = []

for i in range(0, 5):
    ferris.append(f"{ferris_root_path}_{i}.png")
    tiger.append(f"{tiger_root_path}_{i}.svg")
    ferris_type.append(TableWidget.Image)
    tiger_type.append(TableWidget.Image)

data_img = [
            {"Ferris": ferris},
            {"Tiger": tiger}
            ]

# The table is added for svg and png images.
ipg.add_table("col", "My Images", data_img, 
              width=500.0, height=300.0, 
            #   row_highlight=TableRowHighLight.Lighter,
              table_length=len(ferris),
              widgets_using_columns= {0: ferris_type, 1: tiger_type},
              image_width=[100.0, 75.0], image_height=[100.0, 75.0],
              on_enter=image_on_something,
              on_move=image_move,
              )

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
