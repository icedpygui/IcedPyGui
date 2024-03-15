from icedpygui.icedpygui import IPG


ipg = IPG()



ipg.add_window(window_id="main", title="Card Demo", width=800, height=800, 
                                    pos_x=500, pos_y=100)

ipg.add_container(window_id="main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

ipg.add_scrollable(window_id="main", container_id="scroller", parent_id="cont")
ipg.add_column(window_id="main", container_id="col", parent_id="scroller", align_items="center")

head = "Python Iced_aw Card"
body = "This is the body of the card.  Note how the style is add style=ipg.card_style(primary=1).  The opton was to type in the word promary and 8 others or use a function where the IDE show you the options.  All of the paramters are set to None.  If you give any one an integer value, that will be the style selected.  This should cut down on type errors."

ipg.add_card("col", head, "Pimary: "+body, foot="Foot", style=ipg.card_style(primary=1))
ipg.add_card("col", head, "Secondary: " + body, foot="Foot", style=ipg.card_style(secondary=1))
ipg.add_card("col", head, "Success: " + body, foot="Foot", style=ipg.card_style(success=1))
ipg.add_card("col", head, "Danger: " + body, foot="Foot", style=ipg.card_style(danger=1))
ipg.add_card("col", head, "Warning: " + body, foot="Foot", style=ipg.card_style(warning=1))
ipg.add_card("col", head, "Info: " + body, foot="Foot", style=ipg.card_style(info=1))
ipg.add_card("col", head, "Light: " + body, foot="Foot", style=ipg.card_style(light=1))
ipg.add_card("col", head, "Dark: " + body, foot="Foot", style=ipg.card_style(dark=1))
ipg.add_card("col", head, body="White: " + body, foot="Foot", style=ipg.card_style(white=1))
ipg.add_card("col", head, "Default: " + body, foot="Foot", style=ipg.card_style(default=1))
ipg.add_card("col", head, "Default: " + body, foot="Foot")

ipg.start_session()
