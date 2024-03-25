from icedpygui import IPG, IpgButtonStyles, IpgButtonArrows, IpgButtonUpdate


ipg = IPG()

text_id = 0

# Since the widget construction and callbacks are handled differently
# and at different times during this process, making a new widget in 
# a callback is not allowed because of clashing Gils.  Therefore,
# the only IPG commands allowed in a callback are the ones 
# starting with update_
def update_button(id):
    # changing the radius using a float
    ipg.update_item(radius_btn, IpgButtonUpdate.CornerRadius, 5.0)
    ipg.update_item(radius_btn, IpgButtonUpdate.Label, "Corner Radius Changed")
    # changing the label
    ipg.update_item(label_btn, IpgButtonUpdate.Label, "Label Changed")
    # Changing the width
    ipg.update_item(width_btn, IpgButtonUpdate.Width, 300.0)
    ipg.update_item(width_btn, IpgButtonUpdate.Label, "Width Changed")

    # Changing the heigth
    ipg.update_item(height_btn, IpgButtonUpdate.Height, 100.0)
    ipg.update_item(height_btn, IpgButtonUpdate.Label, "Heigth Changed")

    # Changing the padding around the label
    ipg.update_item(padding_btn, IpgButtonUpdate.Padding, [30.0])
    ipg.update_item(padding_btn, IpgButtonUpdate.Label, "Padding Changed")

    # Changing the style
    ipg.update_item(style_btn, IpgButtonUpdate.Style, IpgButtonStyles.Secondary)
    ipg.update_item(style_btn, IpgButtonUpdate.Label, "Styling Changed")

    # Changing the Arrow
    ipg.update_item(arrow_btn, IpgButtonUpdate.ArrowStyle, IpgButtonArrows.ArrowDown)

    # Hide the button
    ipg.update_item(show_btn, IpgButtonUpdate.Show, False)



ipg.add_window("main", "Button Update", width=500, height=700, 
                                    pos_centered=True)

ipg.add_container("main", "cont", align_x="center", align_y="center", width_fill=True)

ipg.add_column("main", container_id="col", parent_id="cont", align_items="center")

ipg.add_button("col", "Press to Change Buttons Below", on_press=update_button)

radius_btn = ipg.add_button("col", "Radius Will Change")

label_btn = ipg.add_button("col", "Label Will Change")

width_btn = ipg.add_button("col", "Width Will Change")

height_btn = ipg.add_button("col", "Heigth Will Change")

padding_btn = ipg.add_button("col", "Padding Will Change")

style_btn = ipg.add_button("col", "Styling Will Change")

arrow_btn = ipg.add_button("col", "", corner_radius=0.0, arrow_style=IpgButtonArrows.ArrowUp)

show_btn = ipg.add_button("col", "This button will disappear")



ipg.start_session()
