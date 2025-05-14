from icedpygui import IPG, IpgTextParam

# This is a simple demo for text with wrapping
ipg = IPG()


def change_text (sldr_id: int, value: float):
    ipg.update_item(
        wid=txt1,
        param=IpgTextParam.Width,
        value=value)
    ipg.update_item(
        wid=txt2,
        param=IpgTextParam.Width,
        value=value)



# Add a window first
ipg.add_window(
        window_id="main", 
        title="CheckBox Demo",
        width=600, 
        height=600,  
        pos_centered=True,
        debug=True)

# Add a container to center the widgets in the middle
ipg.add_container(
        window_id="main", 
        container_id="cont", 
        width_fill=True,
        height_fill=True,
        centered=True)

ipg.add_column(
    window_id="main",
    container_id="col",
    parent_id="cont")

ipg.add_slider(
    parent_id="col",
    min=0.0, 
    max=200.0,
    step=1.0,
    value=200.0,
    on_change=change_text,
    width=175.0)

txt1 = ipg.add_text(
        parent_id="col",
        content="This is some very very very very very very very very long text.",
        width=160.0)

txt2 = ipg.add_text(
        parent_id="col",
        content="This is some very very very very very very very very long text.",
        width=160.0,
        font="FiraSans-Regular")


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
