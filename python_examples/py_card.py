from icedpygui import IPG, IpgCardParam, IpgCardStyleParam
from icedpygui import IpgAlignment, IpgColor, IpgStyleStandard


# Needed first, see other demos for using a class
ipg = IPG()

card_id = 0
# Callback function for changing the card style
# The update function is (wid, param, value)
# wid = widget id
def update_card(_btn_id: int):
    global card_id
    # The card_id is the first card in the series.  The only one that is changed.
    # The btn_id is not used.
    ipg.update_item(
            wid=card_id, 
            param=IpgCardParam.Head, 
            value="This is a new head text")
    
    ipg.update_item(
            wid=card_id, 
            param=IpgCardParam.Body, 
            value="This is a new body text.")
    
    ipg.update_item(
            wid=card_id, 
            param=IpgCardParam.Foot, 
            value="This is a new foot text")
    


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

# The style id is used in the card style_id to set the style
# In this case we create 4 simple styles for later use
colors = [IpgColor.PRIMARY, IpgColor.SUCCESS, IpgColor.DANGER, IpgColor.BLUE]
style_ids = []
for i in range(0, 4):
    id = ipg.add_card_style(
            body_background_color=colors[i],
            foot_background_color=IpgColor.DARK_GREY)
    style_ids.append(id)

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
               align=IpgAlignment.Center, width=400.0, 
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
body = "This is the body of the card."
foot = "Foot"

for i in range(0, 4):
    id = ipg.add_card(
            parent_id="col", 
            head=head, 
            body=body, 
            foot=foot,
            style_id=style_ids[i],
            on_close=minimize_card)
    if i == 0:
        card_id = id


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
