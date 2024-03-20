from icedpygui.icedpygui import IPG, IpgCardStyles


ipg = IPG()


def update_card(id, name):
    ipg.update_item(card_id, "head", "This is a new head with Danger style")
    ipg.update_item(card_id, "body", "This is a new body.")
    ipg.update_item(card_id, "foot", "This is a new foot")
    ipg.update_item(card_id, "style", IpgCardStyles.Danger)


ipg.add_window(window_id="main", title="Card Demo", width=800, height=800, 
                                    pos_x=500, pos_y=100)

ipg.add_container(window_id="main", container_id="btn_cont", align_x="center", 
                  align_y="center", width_fill=True)

ipg.add_button("btn_cont", label="Pressing this will change the updatable items in first card",
                                    on_press=update_card)

ipg.add_container(window_id="main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

ipg.add_scrollable(window_id="main", container_id="scroller", parent_id="cont")
ipg.add_column(window_id="main", container_id="col", parent_id="scroller", 
               align_items="center", width=400.0)


head = "Python Iced_aw Card"
body = "\nThis is the body of the card.  \nNote how the style is add style=ipg.get_card_style(primary=True).  \nSetting any of the dropdown values to True will select it.  This method should cut down on typo errors versus having to type in parameters that need to match exactly."

# Styles are set by import the appropiate module, in the case IpgCardStyles, and selecting
# the needed style from your IDE dropdown list.
card_id = ipg.add_card("col", head, "Primary: "+ body, foot="Foot", style=IpgCardStyles.Primary)
ipg.add_card("col", head, "Secondary: " + body, foot="Foot", style=IpgCardStyles.Secondary)
ipg.add_card("col", head, "Success: " + body, foot="Foot", style=IpgCardStyles.Success)
ipg.add_card("col", head, "Danger: " + body, foot="Foot", style=IpgCardStyles.Danger)
ipg.add_card("col", head, "Warning: " + body, foot="Foot", style=IpgCardStyles.Warning)
ipg.add_card("col", head, "Info: " + body, foot="Foot", style=IpgCardStyles.Info)
ipg.add_card("col", head, "Light: " + body, foot="Foot", style=IpgCardStyles.Light)
ipg.add_card("col", head, "Dark: " + body, foot="Foot", style=IpgCardStyles.Dark)
ipg.add_card("col", head, body="White: " + body, foot="Foot", style=IpgCardStyles.White)
ipg.add_card("col", head, "Default: " + body, foot="Foot", style=IpgCardStyles.Default)
# if you use no style, them this is what you get, which is Default.
ipg.add_card("col", head, "Default: If you use no style setting.\n" + body, foot="Foot")

ipg.start_session()
