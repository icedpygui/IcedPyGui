from icedpygui import IPG, IpgHorizontalAlignment, IpgVerticalAlignment, IpgTableRowHighLight, IpgColor
import random, os

ipg = IPG()


def button(tbl_id: int, index: tuple[int, int]):
    print(tbl_id, index)


def checkbox(tbl_id: int, on_toggle: bool, index: tuple[int, int]):
    print(tbl_id, index, on_toggle)


def toggler(tbl_id: int, on_toggle: bool, index: tuple[int, int]):
    print(tbl_id, index, on_toggle)


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
        pos_x=100, 
        pos_y=25,
        debug=False)

# Add the container, since the table requires a width and height,
# the container can shrink(default) to fit.
ipg.add_container(
        window_id="main", 
        container_id="cont",
        width_fill=True, 
        height_fill=True)


# The table is added.
ipg.add_table(
        window_id="main",
        table_id="table",
        parent_id="cont",
        title="My Table",
        column_widths=[150] * 5,
        height=400.0,
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
for i in range(0, 30):
    for j in range(0, len(headers)):
        if j == 0:
            ipg.add_button(
                parent_id="table",
                label="Edit",
                width_fill=True,
                padding=[0.0],
                style_id=btn_style
                )
        else:
            ipg.add_text(
                parent_id="table",
                content=str(i),
                width_fill=True,
                align_x=IpgHorizontalAlignment.Center,
                align_y=IpgVerticalAlignment.Center,
                )
        

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
ipg.start_session()
