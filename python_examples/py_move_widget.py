from icedpygui import IPG

move_ids = []
target_ids = []



def move_widget(btn_id, item):
    # equate the tuple items to help interpretation
    move_id = item[0]
    target_id = item[1]
    # move the widget
    ipg.move_widget("main", move_id, "col_1", target_id)
    # clean up the indexes to maintain sync with the gui items
    move_ids.remove(move_id)
    
    if target_id is None:
        target_ids.append(move_id)
    else:
        # Note: inserting elements into a list will cause undefined behaviors
        # unless you exit immediately after.  If you need to continue for some reason,
        # note the index and then insert after leaving to loop.
        print(target_ids)
        for index, id in enumerate(target_ids):
            if id == target_id:
                target_ids.insert(index, move_id)
                break
        print(target_ids)

ipg = IPG()

ipg.add_window(window_id="main",
               title="Move Widget",
               width=400.0,
               height=600.0,
               pos_centered=True,
               )

ipg.add_row(window_id="main",
            container_id="row",
            )

ipg.add_column(window_id="main",
               container_id="col_1",
               parent_id="row",
               )

for i in range(0, 10):
    target_ids.append(ipg.add_text(parent_id="col_1", content=f"{i}"))

ipg.add_space(parent_id="row", width=100.0)

ipg.add_column(window_id="main",
               container_id="col_2",
               parent_id="row",
               )

for i in range(0, 10):
    move_ids.append(ipg.add_text(parent_id="col_2", content=f"{i}"))


ipg.add_column(window_id="main",
               container_id="move_btns",
               )

ipg.add_button(parent_id="move_btns",
               label="Move number 5 to end",
               on_press=move_widget,
               user_data=(move_ids[5], None)
               )

ipg.add_button(parent_id="move_btns",
               label="Move number 6 after 4",
               on_press=move_widget,
               user_data=(move_ids[6], target_ids[5])
               )


ipg.start_session()
