from icedpygui import IPG, IpgContainerAlignment, TableRowHighLight, TableWidget
import random, os


ipg = IPG()



def widget_button(tbl_id: int, wid_index: tuple[int, int]):
    print(tbl_id, wid_index)


def widget_checkbox(tbl_id: int, wid_index: tuple[int, int], is_checked: bool):
    print(tbl_id, wid_index, is_checked)


# Add the window
ipg.add_window(window_id="main", title="Table Demo", width=500, height=600,
                pos_x=100, pos_y=25, debug=False)

# Add the container, since the table requires a width and height,
# the container can shrink(default) to fit.
ipg.add_container(window_id="main", container_id="cont",
                  width_fill=True, height_fill=True,
                  align_x=IpgContainerAlignment.Center,
                  align_y=IpgContainerAlignment.Center)

# Initialize the lists.
col0 = []
col1 = []
col2 = []
col3 = []
col4 = []
col5 = []

# Add some random data of different types
for i in range(0, 20):
    # labels for the button widget
    col0.append("Edit")
    # labels for the checkboxes
    col1.append("")
    # make a float random number
    col2.append(random.randrange(10, 99) + random.randrange(10, 99) / 100)
    col3.append(random.choice(["one", "two", "three", "four", "five", "six", "seven"]))
    col4.append(random.randrange(10, 99))
    col5.append(random.choice([True, False]))

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
data = [{"Edit": col0},
        {"Select": col1},
        {"Col2": col2},
        {"Col3": col3},
        {"Col4": col4},
        {"Col4": col5}]


# The column widgets are prepared
btn_widgets = []
chkbox_widgets = []
for _ in range(0, len(col0)):
    btn_widgets.append(TableWidget.Button)
    chkbox_widgets.append(TableWidget.Checkbox)

# The table is added.
ipg.add_table("cont", "My Table", data, 
              width=500.0, height=200.0, 
              row_highlight=TableRowHighLight.Lighter,
              table_length=len(col1),
              widgets_using_columns= {0: btn_widgets, 1: chkbox_widgets},
              on_press_button=widget_button,
              on_toggle_checkbox=widget_checkbox,
              )

# Setting up the image path
cwd = os.getcwd()
ferris_path = cwd + "/python_demo/resources/rustacean-flat-happy.png"
ferris1 = []
ferris2 = []
ferris_type = []

for _ in range(0, 5):
    ferris1.append(ferris_path)
    ferris2.append(ferris_path)
    ferris_type.append(TableWidget.Image)

data_img = [
            {"ferris1": ferris1},
            {"ferris2": ferris2}
            ]

# The table is added for svg and .
ipg.add_table("cont", "My Table", data_img, 
              width=500.0, height=200.0, 
              row_highlight=TableRowHighLight.Lighter,
              table_length=len(ferris1),
              widgets_using_columns= {0: ferris_type, 1: ferris_type},
              image_root_name="ferris",
              image_root_pattern="_#",
              )
# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
