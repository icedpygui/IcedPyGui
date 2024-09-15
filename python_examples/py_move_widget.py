from icedpygui import IPG

ids = []


def move_widget(btn_id, item):
    # equate the tuple items to help interpretation
    widget_id = item[0]
    container_id = item[1]
    move_after = item[2]
    move_before = item[3]

    # move the widget
    ipg.move_widget("main", widget_id, container_id, move_after, move_before)
    

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
    ids.append(ipg.add_text(parent_id="col_1", content=f"{i}"))

ipg.add_space(parent_id="row", width=100.0)

ipg.add_column(window_id="main",
               container_id="col_2",
               parent_id="row",
               )


ipg.add_column(window_id="main",
               container_id="move_btns",
               )

ipg.add_button(parent_id="move_btns",
               label="Move number 5 to end",
               on_press=move_widget,
               user_data=(ids[5], "col_1", None, None)
               )

ipg.add_button(parent_id="move_btns",
               label="Move number 5 after 0",
               on_press=move_widget,
               user_data=(ids[5], "col_1", ids[1], None)
               )

ipg.add_button(parent_id="move_btns",
               label="Move number 5 before 0",
               on_press=move_widget,
               user_data=(ids[5], "col_1", None, ids[0])
               )

ipg.start_session()
