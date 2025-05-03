from icedpygui import IPG
from icedpygui import IpgAlignment, IpgTextParam


def button_pressed(btn_id):
    print(btn_id)


def checked(_chk_id: int, checked: bool):
    if checked:
        ipg.update_item(
              wid=checked_text_id,
              param=IpgTextParam.Content,
              value="I'm checked")

    else:
        ipg.update_item(
              wid=checked_text_id,
              param=IpgTextParam.Content,
              value="I'm not checked")


ipg = IPG()

#  add the window centered on the screen
ipg.add_window(
      window_id="main", 
      title="Demo Window",
      width=600,
      height=500,
      pos_centered=True)

# container alignment defaults to centering
ipg.add_container(
      window_id="main", 
      container_id="cont",
      width_fill=True, 
      height_fill=True)

# The column width will shrink to the size of the largest widget by default.
ipg.add_column(
      window_id="main", 
      container_id="col", 
      parent_id="cont",
      align=IpgAlignment.Center)

ipg.add_button(
      parent_id="col", 
      label="Press Me!", 
      on_press=button_pressed)

ipg.add_checkbox(
      parent_id="col", 
      label="Check Me!!!", 
      on_toggle=checked)

checked_text_id = ipg.add_text(
                    parent_id="col",
                    content="This will change when I'm checked")

ipg.start_session()
