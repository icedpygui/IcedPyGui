from icedpygui import IPG, IpgCheckboxParam, IpgAlignment

# This is a simple demo for trich_text
ipg = IPG()



# Add a window first
ipg.add_window(
        window_id="main", 
        title="CheckBox Demo",
        width=600, 
        height=600,  
        pos_centered=True)

# Add a container to center the widgets in the middle
ipg.add_container(
        window_id="main", 
        container_id="cont", 
        width_fill=True,
        height_fill=True)


# rt = ipg.add_rich_text([
#         ipg.add_span("I am red!").color(color!(0xff0000)),
#         ipg.add_span(" "),
#         ipg.add_span("And I am bold!").font(Font { weight: font::Weight::Bold, ..Font::default() }),
#         ])




# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
