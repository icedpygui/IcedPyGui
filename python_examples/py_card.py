from icedpygui import IPG, IpgCardStyle, IpgCardParam
from icedpygui import IpgAlignment, IpgStyleStandard


# Needed first, see other demos for using a class
ipg = IPG()


# Callback function for changing the card style
# The update function is (wid, param, value)
# wid = widget id
def update_card(_btn_id: int):
    global card_id
    # The card_id is the first card in the series.  The only one that is changed.
    # The btn_id is not used.
    ipg.update_item(card_id, IpgCardParam.Head, "This is a new head with Danger style")
    ipg.update_item(card_id, IpgCardParam.Body, "This is a new body.")
    ipg.update_item(card_id, IpgCardParam.Foot, "This is a new foot")
    ipg.update_item(card_id, IpgCardParam.Style, IpgCardStyle.Danger)


# The callback will minimizes the first card, the button at the bottom left will maximize it.
def minimize_card(card_id: int):
    # In this case the card has a built in button, it can trigger the minimization.
    # Therefore, unlike most other widgets, the id is the card_id needed.
    # The update widget will always need a type where the correct
    # parameter can be selected.  In this case is was IsOpen.
    # id you look at the Card widget docs, you will know what the value
    # type will be, in this case a boolean.
    ipg.update_item(card_id, IpgCardParam.IsOpen, False)


# Pressing the bottom button will maximize the card, returning it to the top.
# Note the callback is from the button so the card_id has to be global.
# Normally, you would use a class or dataclass to store these ids.
def maximize_card(_btn_id: int):
    global card_id
    ipg.update_item(card_id, IpgCardParam.IsOpen, True)


# window added first
ipg.add_window(window_id="main", title="Card Demo", width=800, height=600,
               pos_x=100, pos_y=25)

# add a container for the first button to center it.
# A width_fill is used but the height remains a shrink
# We have center aligned along the x axis.
ipg.add_container(window_id="main", container_id="btn_cont", width_fill=True)

# add a button with a callback on_press to update the first card.
ipg.add_button("btn_cont",
               label="Pressing this will change the updatable items in the first card\n if you close card 1, "
                     "restore it by pressing on the bottom button.",
               on_press=update_card)

# add another is added container to center the column of cards to follow
ipg.add_container(window_id="main", container_id="cont",
                  width_fill=True, height_fill=True)

# put a scrollable in the container since the column will be larger than the container
ipg.add_scrollable(window_id="main", container_id="scroller", parent_id="cont", height_fill=True)

# Put a column in the scrollable.  Note that the height of the scrollable is fill
# and then the column is made shorter that the scrollable.  This seems to work
# most of the time but in some situations you'll need to use the window debug setting
# to see how things line up and getting the contents to scroll.  Just remember the
# scrollable has to be larger than the container, column, or row.
ipg.add_column(window_id="main", container_id="col", parent_id="scroller",
               align_items=IpgAlignment.Center, width=400.0, 
               spacing=0.0, padding=[10.0])

# Add a row at the bottom to hold the button
ipg.add_row("main", "bottom_row", parent_id="main",
            width_fill=True, spacing=0.0)

# Add the button. This button could have been hidden and when the card is minimized, then show it.
# You could also have changed the label to min or max.
ipg.add_button("bottom_row", "Card 1", style_standard=IpgStyleStandard.Primary,
               on_press=maximize_card)

# define the head and body of the cards.
head = "Python Iced_aw Card"
body = ("\nThis is the body of the card.  \nNote how the style is add style=IpgCardStyles.Primary.  This method should "
        "cut down on typo errors versus having to type in parameters that need to match exactly.")

# Styles are set by importing the appropriate module, in this case IpgCardStyles, and selecting
# the needed style from your IDE dropdown list.
card_id = ipg.add_card("col", head, "Primary: " + body, foot="Foot",
                       style=IpgCardStyle.Primary,
                       on_close=minimize_card)
ipg.add_card("col", head, "Secondary: " + body, foot="Foot",
             style=IpgCardStyle.Secondary)
ipg.add_card("col", head, "Success: " + body, foot="Foot",
             style=IpgCardStyle.Success)
ipg.add_card("col", head, "Danger: " + body, foot="Foot",
             style=IpgCardStyle.Danger)
ipg.add_card("col", head, "Warning: " + body, foot="Foot",
             style=IpgCardStyle.Warning)
ipg.add_card("col", head, "Info: " + body, foot="Foot",
             style=IpgCardStyle.Info)
ipg.add_card("col", head, "Light: " + body, foot="Foot",
             style=IpgCardStyle.Light)
ipg.add_card("col", head, "Dark: " + body, foot="Foot",
             style=IpgCardStyle.Dark)
ipg.add_card("col", head, body="White: " + body, foot="Foot",
             style=IpgCardStyle.White)
ipg.add_card("col", head, "Default: " + body, foot="Foot",
             style=IpgCardStyle.Default)

# if you use no style, them this is what you get, which is Default.
ipg.add_card("col", head, "Default: If you use no style setting.\n" + body, foot="Foot")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
