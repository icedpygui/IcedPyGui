from icedpygui import IPG, IpgButtonArrow, IpgButtonParam, IpgStyleStandard
from icedpygui import IpgAlignment, IpgAlignment, IpgButtonStyleParam, IpgTimerParam


# Normally in larger project, a class or classes would be used.
# Some of the widgets examples do use a class, the IPG() can be
# used inside a class.

# This command needs to be first or in the __init__ of the class
ipg = IPG()


# Making a new widget in a callback is not allowed at this time.
# The only ipg command allowed is the update command.  If you need to
# have more widgets show up, then during creation set the show value to
# false and then update it to true in the callback.
# A callback id is the id of the widget making the callback.  If you need
# update other widgets, use their ids as show in this callback.
def update_button(btn_id: int):
    # changing the radius using a float
    # Since the radius is a style, then we are adding a styling id
    # which is defined below.
    ipg.update_item(button_id, 
                    IpgButtonParam.StyleId, 
                    style_id)
    
    ipg.update_item(button_id, 
                    IpgButtonParam.Label, 
                    "Corner Radius Changed")

    # changing the label
    ipg.update_item(label_btn, 
                    IpgButtonParam.Label, 
                    "Label Changed")

    # Changing the width
    ipg.update_item(width_btn, 
                    IpgButtonParam.Width, 
                    300.0)
    
    ipg.update_item(width_btn, 
                    IpgButtonParam.Label, 
                    "Width Changed")

    # Changing the height
    ipg.update_item(height_btn, 
                    IpgButtonParam.Height, 
                    100.0)
    
    ipg.update_item(height_btn, 
                    IpgButtonParam.Label, 
                    "Height Changed")

    # Changing the padding around the label
    ipg.update_item(padding_btn, 
                    IpgButtonParam.Padding, 
                    [30.0])
    
    ipg.update_item(padding_btn, 
                    IpgButtonParam.Label, 
                    "Padding Changed")

    # Changing the style
    ipg.update_item(style_btn, 
                    IpgButtonParam.StyleStandard, 
                    value=IpgStyleStandard.Danger)
    
    ipg.update_item(style_btn, 
                    IpgButtonParam.Label, 
                    "Styling Changed")

    # Changing the Arrow
    ipg.update_item(arrow_btn, 
                    IpgButtonParam.ArrowStyle, 
                    IpgButtonArrow.ArrowDown)

    # Hide the button
    ipg.update_item(show_btn, 
                    IpgButtonParam.Show, 
                    False)

def on_tick(timer_id: int, counter: int):
    if counter >= 10:
        ipg.update_item(timer_id, IpgTimerParam.Counter, 0)
    ipg.update_item(style_id, IpgButtonStyleParam.BorderRadius, [float(counter)*3.0])


# Add styling to change the border radius or many other properties.
style_id = ipg.add_button_style(border_radius=[20.0])

# A window widget needs to be added first, except for styles.
ipg.add_window(window_id="main", 
               title="Button Update", 
               width=500, height=650,
               pos_centered=True)

# Adding a container helps in aligning widgets since it has an x and y centering.
# The IpgContainerAlignment.Center is used to center widgets.  The container defaults
# to center so these are not needed in this case but put in to show use.
# A container can have only one widget, so generally a column or row follows.
# THis container may or may not be needed, it depends on your layout.
ipg.add_container(window_id="main", 
                  container_id="cont",
                  width_fill=True, 
                  height_fill=True)

# A column is added next because the plan is to arrange then in a column.
# If you  set the width_fill or height_fill to true when the outer container
# is also true usually doesn't work, the column or row will expand out of view.
# The containers width_fill defaults to shrink to keep it that way unless needed.
# Sometime you'll need to give them specific amounts to get alignments correct for your layout
ipg.add_column("main", container_id="col", parent_id="cont",
               align_items=IpgAlignment.Center)

# This is the only active button needed for this demo, so it's the only one with a callback
# On some IDE setting, when you type in the callback name, it puts a () after the name.
# If this happens, simply remove the ().  If you leave it in, you will get an error
# about missing parameters.  This is not a function that is called but a python object
# passed to rust to let it know what function needs to be called from rust.
ipg.add_button(parent_id="col", 
               label="Press to Change Buttons Below", 
               on_press=update_button)

# This timer button will change the button style widget
ipg.add_timer(parent_id="col", 
              duration_ms=500,
              label="Continually change the button radius by press this button",
              on_tick=on_tick)

# The radius of this button is styled by using the style_id defined above.

button_id = ipg.add_button(parent_id="col", 
                            label="Corner Radius Will Change",
                            )

label_btn = ipg.add_button(parent_id="col", 
                           label="Label Will Change")

width_btn = ipg.add_button(parent_id="col", 
                           label="Width Will Change")

height_btn = ipg.add_button(parent_id="col", 
                            label="Height Will Change")

padding_btn = ipg.add_button(parent_id="col", 
                             label="Padding Will Change")

style_btn = ipg.add_button(parent_id="col", 
                           label="Styling Will Change")

# On many parameters that are updated, you will need to import the proper
# dataclass so that the parameter can be selected.  In this case, you are working
# with a button arrow, so import the IpgButtonArrow and select the one you want.
# This method greatly cuts down on typos, if you had to use strings for the parameters.
arrow_btn = ipg.add_button(parent_id="col", 
                           label="",
                           style_arrow=IpgButtonArrow.ArrowUp)

show_btn = ipg.add_button(parent_id="col", 
                          label="This button will disappear")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
