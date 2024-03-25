from icedpygui import IPG, IpgButtonStyles, IpgCardStyles, IpgCardUpdate


ipg = IPG()


def update_card(id):
    ipg.update_item(card_id, IpgCardUpdate.Head, "This is a new head with Danger style")
    ipg.update_item(card_id, IpgCardUpdate.Body, "This is a new body.")
    ipg.update_item(card_id, IpgCardUpdate.Foot, "This is a new foot")
    ipg.update_item(card_id, IpgCardUpdate.Style, IpgCardStyles.Danger)


def minimize_card(id):
    ipg.update_item(id, IpgCardUpdate.IsOpen, False)


def maximize_card(id):
    ipg.update_item(card_id, IpgCardUpdate.IsOpen, True)


wnd_height = 800

ipg.add_window(window_id="main", title="Card Demo", width=800, height=wnd_height, 
                                    pos_x=500, pos_y=100)

ipg.add_container(window_id="main", container_id="btn_cont", align_x="center", 
                  align_y="center", width_fill=True)

ipg.add_button("btn_cont", label="Pressing this will change the updatable items in the first card\n if you close card 1, restore it by pressing on the bottom button.",
                                    on_press=update_card)

ipg.add_container(window_id="main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

ipg.add_scrollable(window_id="main", container_id="scroller", parent_id="cont", height_fill=True)
ipg.add_column(window_id="main", container_id="col", parent_id="scroller", 
               align_items="center", width=400.0, spacing=0.0)

ipg.add_row("main", "bottom_row", parent_id="main", 
            width_fill=True, spacing=0.0)

ipg.add_button("bottom_row", "Card 1", style=IpgButtonStyles.Primary,
               on_press=maximize_card)

head = "Python Iced_aw Card"
body = "\nThis is the body of the card.  \nNote how the style is add style=IpgCardStyles.Primary.  This method should cut down on typo errors versus having to type in parameters that need to match exactly."

# Styles are set by import the appropiate module, in the case IpgCardStyles, and selecting
# the needed style from your IDE dropdown list.
card_id = ipg.add_card("col", head, "Primary: "+ body, foot="Foot", 
                       style=IpgCardStyles.Primary,
                       on_close=minimize_card)
ipg.add_card("col", head, "Secondary: " + body, foot="Foot", 
             style=IpgCardStyles.Secondary)
ipg.add_card("col", head, "Success: " + body, foot="Foot", 
             style=IpgCardStyles.Success)
ipg.add_card("col", head, "Danger: " + body, foot="Foot", 
             style=IpgCardStyles.Danger)
ipg.add_card("col", head, "Warning: " + body, foot="Foot", 
             style=IpgCardStyles.Warning)
ipg.add_card("col", head, "Info: " + body, foot="Foot", 
             style=IpgCardStyles.Info)
ipg.add_card("col", head, "Light: " + body, foot="Foot", 
             style=IpgCardStyles.Light)
ipg.add_card("col", head, "Dark: " + body, foot="Foot", 
             style=IpgCardStyles.Dark)
ipg.add_card("col", head, body="White: " + body, foot="Foot", 
             style=IpgCardStyles.White)
ipg.add_card("col", head, "Default: " + body, foot="Foot", 
             style=IpgCardStyles.Default)

# if you use no style, them this is what you get, which is Default.
ipg.add_card("col", head, "Default: If you use no style setting.\n" + body, foot="Foot")

ipg.start_session()
