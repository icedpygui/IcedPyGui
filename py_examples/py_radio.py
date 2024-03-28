from icedpygui import IPG, RadioParams, RadioDirection, IpgTextUpdate


# The radio widget has a count limitation unlike the other widgets due to the way
# the Rust enums are connected to this widget.  Therefore a limit of 26 groups of buttons with
# a limit of 26 buttons per group was set for this widget.  In the unlikely case where you need
# more, you could used checkboxes and just uncheck the all on a callback, keeping the
# one you selected as checked.

ipg = IPG()


def selected_radio_1(rd_id, data, user_data):
    ipg.update_item(rd_text_id_1, IpgTextUpdate.Content, 
        f"Radio callback id = {rd_id}, \n index = {data[0]}, \nlabel = {data[1]} \n user_data = {user_data}")


def selected_radio_2(rd_id, data, user_data):
    ipg.update_item(rd_text_id_2, IpgTextUpdate.Content, 
        f"Radio callback id = {rd_id}, \n index = {data[0]}, \nlabel = {data[1]} \n user_data = {user_data}")


def change_direction(chk_id, checked, radio_ids):
    radio1 = RadioDirection.Vertical
    radio2 = RadioDirection.Horizontal
    if checked:
        radio1 = RadioDirection.Horizontal
        radio2 = RadioDirection.Vertical
    ipg.update_item(radio_ids[0], RadioParams.Direction, radio1)
    ipg.update_item(radio_ids[1], RadioParams.Direction, radio2)


def change_selection(chk_id, checked, radio_ids):
    selected = None
    if checked:
        selected = 2
    ipg.update_item(radio_ids[0], RadioParams.SelectedIndex, selected)
    ipg.update_item(radio_ids[1], RadioParams.SelectedIndex, selected)


def change_labels(chk_id, checked, rd_ids):
    labels = [["one", "two", "three", "four"], ["five", "six", "seven"]]
    if checked:
        labels = [["ants", "flies", "bees", "wasps"], ["cat", "dog", "horse"]]

    ipg.update_item(rd_ids[0], RadioParams.Labels, labels[0])
    ipg.update_item(rd_ids[1], RadioParams.Labels, labels[1])
    # unselecting any if selected
    ipg.update_item(rd_ids[0], RadioParams.SelectedIndex, None)
    ipg.update_item(rd_ids[1], RadioParams.SelectedIndex, None)

def hide_left_radio(chk_id, checked, rd_left_id):
    show = True
    if checked:
        show = False
    ipg.update_item(rd_left_id, RadioParams.Show, show)

def change_size(chk_id, checked, rd_ids):
    size = 20.0
    if checked:
        size = 10.0
    ipg.update_item(rd_ids[0], RadioParams.Size, size)
    ipg.update_item(rd_ids[1], RadioParams.Size, size)

def change_spacing(chk_id, checked, rd_ids):
    spacing = 10.0
    if checked:
        spacing = 20.0
    ipg.update_item(rd_ids[0], RadioParams.Spacing, spacing)
    ipg.update_item(rd_ids[1], RadioParams.Spacing, spacing)

def change_text_spacing(chk_id, checked, rd_ids):
    ts = 15
    if checked:
        ts = 30.0
    ipg.update_item(rd_ids[0], RadioParams.TextSpacing, ts)
    ipg.update_item(rd_ids[1], RadioParams.TextSpacing, ts)

def change_text_size(chk_id, checked, rd_ids):
    ts = 16.0
    if checked:
        ts = 20.0
    ipg.update_item(rd_ids[0], RadioParams.TextSize, ts)
    ipg.update_item(rd_ids[1], RadioParams.TextSize, ts)

def change_text_line_heigth(chk_id, checked, rd_ids):
    tlh = 1.3
    if checked:
        tlh = 3.0
    ipg.update_item(rd_ids[0], RadioParams.TextLineHeight, tlh)
    ipg.update_item(rd_ids[1], RadioParams.TextLineHeight, tlh)

def change_width(chk_id, checked, rd_ids):
    wd = None # defaults to shrink
    if checked:
        wd = 150.0
    ipg.update_item(rd_ids[0], RadioParams.Width, wd)
    ipg.update_item(rd_ids[1], RadioParams.Width, wd)

def change_width_fill(chk_id, checked, rd_ids):
    # Width overrides WidthFill, so make sure to Width is not set
    # in this case, let's set the Width to none just in case.
    wdf = False # defaults to shrink
    if checked:
        wdf = True
    ipg.update_item(rd_ids[0], RadioParams.Width, None)
    ipg.update_item(rd_ids[1], RadioParams.Width, None)
    
    ipg.update_item(rd_ids[0], RadioParams.WidthFill, wdf)
    ipg.update_item(rd_ids[1], RadioParams.WidthFill, wdf)


def change_height(chk_id, checked, rd_ids):
    ht = None # defaults to shrink
    if checked:
        ht = 150.0
    ipg.update_item(rd_ids[0], RadioParams.Height, ht)
    ipg.update_item(rd_ids[1], RadioParams.Height, ht)

def change_height_fill(chk_id, checked, rd_ids):
    # Height overrides HeightFill, so make sure to Height is not set
    # in this case, let's set the Height to none just in case.
    htf = False # defaults to shrink
    if checked:
        htf = True
    ipg.update_item(rd_ids[0], RadioParams.Height, None)
    ipg.update_item(rd_ids[1], RadioParams.Height, None)

    ipg.update_item(rd_ids[0], RadioParams.HeightFill, htf)
    ipg.update_item(rd_ids[1], RadioParams.HeightFill, htf)


# **************Window Constructions Starts Here*************************

ipg.add_window("main", "Radio Demo", 500, 650, 
                                    300, 100)

ipg.add_container("main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col", parent_id="cont", 
               align_items="center", height=600.0, width=500.0)

ipg.add_space("col", height=100)

ipg.add_text(parent_id="col", content="Press the radio buttons to see the selection")
ipg.add_text(parent_id="col", content="Press the checkboxes in window 2 to see the effects")

# Add a row for the two groups of radio buttons
# The WidthFill and HeightFill are set True in this case to be able to demostrate 
# the same parameters for the Radio Buttons.  If you set a widget's WidthFill, for example,
# and then have it in a container with a default shrink, it will shrink instead of fill
# because a shrink's size is not defined since it depends on what it contains.
# To prevent the row from going to the bottom, a spacing with height is added.
ipg.add_row("main", "row1", "col", 
            spacing=50.0, width_fill=True, height_fill=True)

rd_left_id = ipg.add_radio(parent_id="row1", labels=["one", "two", "three", "four"], 
                                on_select=selected_radio_1, user_data="Some data")

rd_right_id = ipg.add_radio(parent_id="row1", labels=["five", "six", "seven"], 
                               direction=RadioDirection.Horizontal,
                                on_select=selected_radio_2, user_data="Some data")


# add a row for the text associated with the above radio groups
ipg.add_row("main", "row2", "col", spacing=50.0)

rd_text_id_1 = ipg.add_text("row2", content="Radio callback data:")
rd_text_id_2 = ipg.add_text("row2", content="Radio callback data:")

ipg.add_space(parent_id="col", height=90)

# **********Adding Window 2 with checkboxes****************************************
ipg.add_window("main2", "Radio Demo", 600, 650, 
                                    850, 100)

ipg.add_container("main2", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

ipg.add_column(window_id="main2", container_id="chk_col", parent_id="cont")

ipg.add_checkbox("chk_col", label="Change Direction of radios",
               on_toggle=change_direction, user_data=[rd_left_id, rd_right_id])

ipg.add_checkbox("chk_col", label="Change the selected radios to the 3rd label on each",
               on_toggle=change_selection, user_data=[rd_left_id, rd_right_id])

ipg.add_checkbox("chk_col", label="Change the Labels",
               on_toggle=change_labels, user_data=[rd_left_id, rd_right_id])


ipg.add_checkbox("chk_col", label="Change the Size", user_data=[rd_left_id, rd_right_id],
               on_toggle=change_size)

ipg.add_checkbox("chk_col", label="Change the Spacing", user_data=[rd_left_id, rd_right_id],
               on_toggle=change_spacing)

ipg.add_checkbox("chk_col", label="Change the TextSpacing", user_data=[rd_left_id, rd_right_id],
               on_toggle=change_text_spacing)

ipg.add_checkbox("chk_col", label="Change the TextSize", user_data=[rd_left_id, rd_right_id],
               on_toggle=change_text_size)

ipg.add_checkbox("chk_col", label="Change the TextLineHeight", user_data=[rd_left_id, rd_right_id],
               on_toggle=change_text_line_heigth)

ipg.add_checkbox("chk_col", label="Change the Width", user_data=[rd_left_id, rd_right_id],
               on_toggle=change_width)

ipg.add_checkbox("chk_col", label="Change the WidthFill", user_data=[rd_left_id, rd_right_id],
               on_toggle=change_width_fill)

ipg.add_checkbox("chk_col", label="Change the Height", user_data=[rd_left_id, rd_right_id],
               on_toggle=change_height)

ipg.add_checkbox("chk_col", label="Change the HeightFill", user_data=[rd_left_id, rd_right_id],
               on_toggle=change_height_fill)

ipg.add_checkbox("chk_col", label="Hide the left Radios", user_data=rd_left_id,
               on_toggle=hide_left_radio)

ipg.start_session()
