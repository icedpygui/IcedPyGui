from icedpygui import IPG, IpgHorizontalAlignment, IpgVerticalAlignment
from icedpygui import IpgTableRowHighLight, IpgColor, IpgTextParam
from icedpygui import IpgContainerParam, IpgAlignment

ipg = IPG()

global total_checks
total_checks = 0
total_id = 0


def checkbox(tbl_id: int, on_toggle: bool):
    global total_checks
    if on_toggle:
        total_checks += 1
    else:
        total_checks -= 1
        
    ipg.update_item(total_id, IpgTextParam.Content, f"Total Checked = {total_checks}")


def show_modal(btn_id: int, index: tuple[int, int]):
    ipg.update_item(modal_id, IpgContainerParam.Show, True)
    ipg.update_item(modal_title, IpgTextParam.Content, f"Modal for Row {index[0]}")


def close_modal(btn_id: int):
    ipg.update_item(modal_id, IpgContainerParam.Show, False)


btn_style = ipg.add_button_style(border_radius=[10.0])
chk_style = ipg.add_checkbox_style(border_width=3.0)
tog_style = ipg.add_toggler_style(
                    background_border_width=2.0, 
                    background_border_color=IpgColor.YELLOW)

# Add the window
ipg.add_window(
        window_id="main", 
        title="Table Demo",
        width=1000, 
        height=600,
        pos_centered=True,
        debug=False)

# Add the container for centering the table
ipg.add_container(
        window_id="main", 
        container_id="cont",
        width_fill=True, 
        height_fill=True,
        padding=[20.0])

column_widths = [100.0, 200.0, 150.0, 150.0, 150.0]
width = sum(column_widths)

ipg.add_stack(
        window_id="main",
        container_id="stack",
        parent_id="cont")

# ipg.add_opaque_container(
#         window_id="main",
#         container_id="opaque",
#         parent_id="stack")

# The table is added.
ipg.add_table(
        window_id="main",
        table_id="table",
        parent_id="stack",
        title="My Table",
        column_widths=column_widths,
        height=400.0,
        footer_enabled=True,
        table_width_fixed=True, # defaults to True, change to False to see the effect
        )

# create headers
headers = ["one", "two", "three", "four", "five"]
for i, head in enumerate(headers):
    ipg.add_text(
        parent_id="table",
        content=head,
        align_x=IpgHorizontalAlignment.Center,
        align_y=IpgVerticalAlignment.Center,
        width_fill=True)

# fill in the table rows
for i in range(0, 1):
    for j in range(0, len(headers)):
        if j == 0:
            ipg.add_button(
                parent_id="table",
                label="Edit",
                width=column_widths[0],
                style_id=btn_style,
                text_align_x=IpgHorizontalAlignment.Center,
                on_press=show_modal,
                user_data=(i, j)
                )
        elif j == 1:
            checked = True
            if i % 5 == 0:
                checked = False
            else:
                total_checks += 1
            # container used for center the widget
            # Since the checkbox does't have a label
            # and that the table column widht is longer 
            # due to the footer, a container was needed 
            # to help the alignment in this case.
            ipg.add_container(
                    window_id="main",
                    container_id=f"chk_cont{i}",
                    parent_id="table",
                    width_fill=True,
                    centered=True
                )
            ipg.add_checkbox(
                parent_id=f"chk_cont{i}",
                on_toggle=checkbox,
                is_checked=checked,
                )
        else:
            ipg.add_text(
                parent_id="table",
                content=str(i),
                width_fill=True,
                align_x=IpgHorizontalAlignment.Center,
                align_y=IpgVerticalAlignment.Center,
                )

# add footer
for i in range(0, len(headers)):
    if i == 1:
        total_id = ipg.add_text(
                        parent_id="table",
                        content=f"Total Checked = {total_checks}",
                        width_fill=True,
                        align_x=IpgHorizontalAlignment.Center,
                        align_y=IpgVerticalAlignment.Center,
                        )
    else:
        ipg.add_text(
                parent_id="table",
                content="",
                width_fill=True)       

modal_style = ipg.add_container_style(
                    background_color=IpgColor.DARK_SLATE_GRAY
                    )

modal_id = ipg.add_container(
                    window_id="main",
                    container_id="stack_base",
                    parent_id="stack",
                    width_fill=True,
                    height_fill=True,
                    centered=True,
                    show=False,
                    )

ipg.add_container(
        window_id="main",
        container_id="modal",
        parent_id="stack_base",
        width=200.0,
        height=300.0,
        style_id=modal_style,
        )

ipg.add_column(
        window_id="main",
        container_id="modal_col",
        parent_id="modal",
        width_fill=True,
        height_fill=True,
        align_x=IpgAlignment.Center,
        )

modal_title = ipg.add_text(
                    parent_id="modal_col",
                    content="Modal",
                    )

ipg.add_button(
        parent_id="modal_col",
        label="Close Modal",
        on_press=close_modal,
        )

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
