from icedpygui import IPG, IpgAlignment, IpgTogglerParams, IpgTextParams


ipg = IPG()

def toggled(tog_id, is_toggled):
    ipg.update_item(is_tog_id, IpgTextParams.Content, f"The toggler is {is_toggled}.")


def update_label(btn_id):
    ipg.update_item(tog_id, IpgTogglerParams.Label, "New Toggle Label")

def update_width(btn_id):
    ipg.update_item(tog_id, IpgTogglerParams.Width, 100.0)

def update_width_fill(btn_id):
    ipg.update_item(tog_id, IpgTogglerParams.WidthFill, True)

def update_alignment(btn_id):
    ipg.update_item(tog_id, IpgTogglerParams.Alignment, IpgAlignment.Left)

def update_size(btn_id):
    ipg.update_item(tog_id, IpgTogglerParams.Size, 30.0)

def update_text_size(btn_id):
    ipg.update_item(tog_id, IpgTogglerParams.TextSize, 30.0)

def update_line_height(btn_id):
    ipg.update_item(tog_id, IpgTogglerParams.LineHeight, 2.0)



ipg.add_window("main", "Rule Demo", 900, 625, pos_centered=True)

ipg.add_column("main", "col", width_fill=True, height_fill=True, 
               align_items="center", spacing=5.0)

ipg.add_column("main", container_id="col_top", parent_id="col", align_items="center",
               height=110.0, spacing=5.0, padding=[5.0], width_fill=True)

tog_id = ipg.add_toggler("col_top", toggled=toggled)

is_tog_id = ipg.add_text("col_top", "The toggler is False.")

ipg.add_column("main", container_id="col_bot", parent_id="col", 
               align_items="center", spacing=10.0, padding=[5.0])

ipg.add_text('col_bot', "Press the buttons, below, in order to best see the effects, top to bottom, left to right")

ipg.add_button(parent_id="col_bot", label="Update Label", on_press=update_label)

ipg.add_button(parent_id="col_bot", label="Update Width\n the width will shrink stacking the label", on_press=update_width)

ipg.add_button(parent_id="col_bot", label="Setting WidthFill=True\n The toggler expand the width of the container. the label is centered, the default setting", on_press=update_width_fill)

ipg.add_button(parent_id="col_bot", label="Setting the alignment to Left\n This caused the label to move left", on_press=update_alignment)

ipg.add_button(parent_id="col_bot", label="Setting the size\n This makes the toggler bigger", on_press=update_size)

# putting last two buttons in a row to make more room on screen
ipg.add_row(window_id="main", container_id="row", parent_id="col_bot", align_items="center",
            width_fill=True)

ipg.add_button(parent_id="row", label="Increasing the TextSize", on_press=update_text_size)

# The text line height basically makes the outer box container the widget bigger
ipg.add_button(parent_id="row", label="Increasing the TextLineHeight\n Set window debug=True to see this better,", on_press=update_line_height)

ipg.start_session()
