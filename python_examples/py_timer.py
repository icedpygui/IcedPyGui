from icedpygui import IPG, IpgTextParam, IpgTimerParam, IpgButtonStyleParam
from icedpygui import IpgCheckboxStyleParam, IpgColorPickerStyleParam, IpgPickListStyleParam
from icedpygui import IpgColor, IpgProgressBarStyleParam, IpgRadioStyleParam
from icedpygui import IpgRuleStyleParam, IpgSliderStyleParam


colors = [IpgColor.PRIMARY, IpgColor.SECONDARY, IpgColor.SUCCESS, IpgColor.DANGER, IpgColor.WARNING, 
          IpgColor.INFO, IpgColor.LIGHT, IpgColor.DARK, IpgColor.ALICE_BLUE, IpgColor.ANTIQUE_WHITE, 
          IpgColor.AQUA, IpgColor.AQUAMARINE, IpgColor.AZURE, IpgColor.BEIGE, IpgColor.BISQUE, 
          IpgColor.BLACK, IpgColor.BLANCHED_ALMOND, IpgColor.BLUE, IpgColor.BLUE_VIOLET, IpgColor.BROWN, 
          IpgColor.BURLY_WOOD, IpgColor.CADET_BLUE, IpgColor.CHARTREUSE, IpgColor.CHOCOLATE, IpgColor.CORAL, 
          IpgColor.CORNFLOWER_BLUE, IpgColor.CORNSILK, IpgColor.CRIMSON, IpgColor.CYAN, IpgColor.DARK_BLUE, 
          IpgColor.DARK_CYAN, IpgColor.DARK_GOLDEN_ROD, IpgColor.DARK_GRAY, IpgColor.DARK_GREY, 
          IpgColor.DARK_GREEN, IpgColor.DARK_KHAKI, IpgColor.DARK_MAGENTA, IpgColor.DARK_OLIVE_GREEN, 
          IpgColor.DARK_ORANGE, IpgColor.DARK_ORCHID, IpgColor.DARK_RED, IpgColor.DARK_SALMON, 
          IpgColor.DARK_SEA_GREEN, IpgColor.DARK_SLATE_BLUE, IpgColor.DARK_SLATE_GRAY, IpgColor.DARK_SLATE_GREY, 
          IpgColor.DARK_TURQUOISE, IpgColor.DARK_VIOLET, IpgColor.DEEP_PINK, IpgColor.DEEP_SKY_BLUE, 
          IpgColor.DIM_GRAY, IpgColor.DIM_GREY, IpgColor.DODGER_BLUE, IpgColor.FIRE_BRICK, IpgColor.FLORAL_WHITE, 
          IpgColor.FOREST_GREEN, IpgColor.FUCHSIA, IpgColor.GAINSBORO, IpgColor.GHOST_WHITE, IpgColor.GOLD, 
          IpgColor.GOLDEN_ROD, IpgColor.GRAY, IpgColor.GREY, IpgColor.GREEN, IpgColor.GREEN_YELLOW, IpgColor.HONEY_DEW, 
          IpgColor.HOT_PINK, IpgColor.INDIAN_RED, IpgColor.INDIGO, IpgColor.IVORY, IpgColor.KHAKI, IpgColor.LAVENDER, 
          IpgColor.LAVENDER_BLUSH, IpgColor.LAWN_GREEN, IpgColor.LEMON_CHIFFON, IpgColor.LIGHT_BLUE, IpgColor.LIGHT_CORAL, 
          IpgColor.LIGHT_CYAN, IpgColor.LIGHT_GOLDEN_ROD_YELLOW, IpgColor.LIGHT_GRAY, IpgColor.LIGHT_GREY, 
          IpgColor.LIGHT_GREEN, IpgColor.LIGHT_PINK, IpgColor.LIGHT_SALMON, IpgColor.LIGHT_SEA_GREEN, 
          IpgColor.LIGHT_SKY_BLUE, IpgColor.LIGHT_SLATE_GRAY, IpgColor.LIGHT_SLATE_GREY, IpgColor.LIGHT_STEEL_BLUE, 
          IpgColor.LIGHT_YELLOW, IpgColor.LIME, IpgColor.LIME_GREEN, IpgColor.LINEN, IpgColor.MAGENTA, IpgColor.MAROON, 
          IpgColor.MEDIUM_AQUA_MARINE, IpgColor.MEDIUM_BLUE, IpgColor.MEDIUM_ORCHID, IpgColor.MEDIUM_PURPLE, 
          IpgColor.MEDIUM_SEA_GREEN, IpgColor.MEDIUM_SLATE_BLUE, IpgColor.MEDIUM_SPRING_GREEN, IpgColor.MEDIUM_TURQUOISE, 
          IpgColor.MEDIUM_VIOLET_RED, IpgColor.MIDNIGHT_BLUE, IpgColor.MINT_CREAM, IpgColor.MISTY_ROSE, IpgColor.MOCCASIN, 
          IpgColor.NAVAJO_WHITE, IpgColor.NAVY, IpgColor.OLD_LACE, IpgColor.OLIVE, IpgColor.OLIVE_DRAB, IpgColor.ORANGE, 
          IpgColor.ORANGE_RED, IpgColor.ORCHID, IpgColor.PALE_GOLDEN_ROD, IpgColor.PALE_GREEN, IpgColor.PALE_TURQUOISE, 
          IpgColor.PALE_VIOLET_RED, IpgColor.PAPAYA_WHIP, IpgColor.PEACH_PUFF, IpgColor.PERU, IpgColor.PINK, IpgColor.PLUM,
          IpgColor.POWDER_BLUE, IpgColor.PURPLE, IpgColor.REBECCA_PURPLE, IpgColor.RED, IpgColor.ROSY_BROWN, 
          IpgColor.ROYAL_BLUE, IpgColor.SADDLE_BROWN, IpgColor.SALMON, IpgColor.SANDY_BROWN, IpgColor.SEA_GREEN, 
          IpgColor.SEA_SHELL, IpgColor.SIENNA, IpgColor.SILVER, IpgColor.SKY_BLUE, IpgColor.SLATE_BLUE, 
          IpgColor.SLATE_GRAY, IpgColor.SLATE_GREY, IpgColor.SNOW, IpgColor.SPRING_GREEN, IpgColor.STEEL_BLUE, 
          IpgColor.TAN, IpgColor.TEAL, IpgColor.THISTLE, IpgColor.TOMATO, IpgColor.TRANSPARENT, IpgColor.TURQUOISE, 
          IpgColor.VIOLET, IpgColor.WHEAT, IpgColor.WHITE, IpgColor.WHITE_SMOKE, IpgColor.YELLOW, IpgColor.YELLOW_GREEN]

# Fires when the timer is started
def on_start(timer_id: int, counter: int):
    ipg.update_item(timer_id, IpgTimerParam.Label, "Stop Timer")
    # Just in case of a restart, zero the counter
    ipg.update_item(timer_id, IpgTimerParam.Counter, 0)


# Fires on every tick
def on_tick(timer_id: int, counter: int):
    ipg.update_item(text_id, IpgTextParam.Content, f"Count: {counter}")
    if counter > len(colors):
        ipg.update_item(timer_id, IpgTimerParam.Counter, 0)
    ipg.update_item(btn_style, IpgButtonStyleParam.BackgroundIpgColor, colors[counter])
    ipg.update_item(chk_style, IpgCheckboxStyleParam.BackgroundIpgColor, colors[counter])
    ipg.update_item(cp_style, IpgColorPickerStyleParam.BackgroundIpgColor, colors[counter])
    ipg.update_item(pl_style, IpgPickListStyleParam.BackgroundIpgColor, colors[counter])
    ipg.update_item(pb_style, IpgProgressBarStyleParam.BackgroundIpgColor, colors[counter])
    ipg.update_item(rd_style, IpgRadioStyleParam.BackgroundIpgColor, colors[counter])
    ipg.update_item(rl_style, IpgRuleStyleParam.IpgColor, colors[counter])
    ipg.update_item(sl_style, IpgSliderStyleParam.RailIpgColor, colors[counter])
    ipg.update_item(sl_style, IpgSliderStyleParam.HandleIpgColor, colors[counter])


# Fires on stopping
def on_stop(timer_id: int, counter: int):
    ipg.update_item(text_id, IpgTextParam.Content, f"Count stopped at {counter}")
    ipg.update_item(timer_id, IpgTimerParam.Label, "Start Timer")

ipg = IPG()

# Add a window
ipg.add_window(window_id="main", title="Timer Demo",
               width=400.0, height=600.0,  
               pos_centered=True)

# Add the container to center everything
ipg.add_container(window_id="main", 
                  container_id="cont", 
                  width_fill=True,
                  height_fill=True,
                  )

# Add the column to hold the widgets
ipg.add_column(window_id="main", container_id="col", parent_id="cont")


btn_style = ipg.add_button_style(background_color=IpgColor.PRIMARY)
chk_style = ipg.add_checkbox_style(background_color=IpgColor.PRIMARY)
cp_style = ipg.add_color_picker_style(background_color=IpgColor.PRIMARY)
pl_style = ipg.add_pick_list_style(background_color=IpgColor.PRIMARY)
pb_style = ipg.add_progress_bar_style(background_color=IpgColor.PRIMARY)
rd_style = ipg.add_radio_style(background_color=IpgColor.PRIMARY)
rl_style = ipg.add_rule_style(color=IpgColor.PRIMARY)
sl_style = ipg.add_slider_style(rail_color=IpgColor.PRIMARY, handle_color=IpgColor.PRIMARY)


btn_id = ipg.add_button(parent_id="col",
                        label="Button",
                        style_id=btn_style,
                        )

chk_id = ipg.add_checkbox(parent_id="col",
                          label="Checkbox",
                          style_id=chk_style)

cp_id = ipg.add_color_picker(parent_id="col",
                             label="Color Picker",
                             style_id=cp_style)


pl_id = ipg.add_pick_list(parent_id="col",
                          options=["1", "2", "3"],
                          placeholder="Pick a number",
                          style_id=pl_style)

pb_id = ipg.add_progress_bar(parent_id="col",
                             min=0.0,
                             max=100.0,
                             value=0.0,
                             style_id=pb_style)

rd_id = ipg.add_radio(parent_id="col",
                      labels=["One", "Two", "Three"],
                      style_id=rd_style)

rl_id = ipg.add_rule_horizontal(parent_id="col",
                                width=150.0,
                                thickness=3,
                                style_id=rl_style)

sl_id = ipg.add_slider(parent_id="col",
                       min=0.0,
                       max=100.0,
                       step=1.0,
                       value=50.0,
                       width=300.0,
                       style_id=sl_style)

# The time duration is in milliseconds, so 1 second interval, in this case.
# The three callbacks start, stop, and tick are defined.
# The timer button is part of the timer method but you can style it as need, 
# just like the regular button.
ipg.add_timer(parent_id="col", 
              duration_ms=300,
              on_start=on_start, 
              on_tick=on_tick, 
              on_stop=on_stop)

# Add a text widget to display the count.
text_id = ipg.add_text(parent_id="col", content="Count: 0")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()

