from icedpygui import IPG, IpgTextParam
from icedpygui import IpgAlignment
from icedpygui import IpgAlignment, IpgScrollableDirection, IpgScrollableParam


# To reduce typing errors when using the ids, I like to 
# use a class or define variables like below then just 
# select which one I want from the hint.

# In this way, you can change the ids if you decide to later 
# with little effort.

# However, you can just type in the string if you like.

# Currently @dataclass is not supported since it sends a tuple 
# and not a string. Hopefully I can fix it in the near future.

# These demos use a lot of callbacks to change things on widgets,
# this is probably something that you will seldom use, but it's added
# in for completeness and testing. It also makes the program a bit 
# more complex than if you simply used the widget with one callback, 
# so it's not as complex as it appears at first look.

# Note on scrolling:
# Sometimes getting the containers sized correctly takes a bit
# of patience. When using scrolling, the size of the scrollable
# should always be smaller than the container size. The container
# will be the default shrink and you set the size of the scrollable
# to be something less than the size of your data or container items.


class DemoScrollable:
    def __init__(self) -> None:
        self.ipg = IPG()
        self.wnd_width: int = 400
        self.wnd_height: int = 600

        # To help reduce spelling errors,
        # you can set up you variables
        # and select from your ide's dropdown list
        # without having to type in the string.
        self.wnd_v: str = "window_v"
        self.cont_v_top: str = "cont_v_top"
        self.scroll_v: str = "scroll_v"
        self.cont_v_middle: str = "cont_v_middle"
        self.cont_v_bottom: str = "cont_v_bottom"

        self.wnd_h: str = "window_h"
        self.cont_h_top: str = "cont_h_top"
        self.scroll_h: str = "scroll_h"
        self.cont_h_middle: str = "cont_h_middle"
        self.cont_h_bottom: str = "cont_h_bottom"

        self.wnd_b: str = "window_b"
        self.cont_b_top: str = "cont_b_top"
        self.scroll_b: str = "scroll_b"
        self.cont_b: str = "cont_b"
        self.col_b: str = "col_b"

        self.cb_text_v: int = 0
        self.cb_text_h: int = 0
        self.cb_text_b: int = 0

        self.scroll_id_1: int = 0
        self.scroll_id_3: int = 0

        self.h_bar_width: float = 10.0
        self.v_bar_width: float = 10.0
        self.h_bar_margin: float = 0.0
        self.v_bar_margin: float = 0.0
        self.h_scroller_width: float = 10.0
        self.v_scroller_width: float = 10.0

    # start_session must be the last function called
    def create_gui(self):
        self.create_scroll_vertical()
        self.create_scroll_horizontal()
        self.create_scroll_both()
        # Required to be the last widget sent to Iced,  If you start the program
        # and nothing happens, it might mean you forgot to add this command.
        self.ipg.start_session()

    # ***************Window 1-scrolling a column container vertically**********************************
    # We will create 3 containers, a header, a scrollable, and an empty bottom container.
    def create_scroll_vertical(self):

        self.ipg.add_window(
                    window_id=self.wnd_v, 
                    title="Scrollable - Vertical",
                    width=self.wnd_width, 
                    height=self.wnd_height,
                    pos_x=50, 
                    pos_y=25)

        # The header is just a title of sorts.
        self.ipg.add_container(
                    window_id=self.wnd_v, 
                    container_id=self.cont_v_top,
                    width_fill=True, 
                    height=100.0, 
                    padding=[20])

        self.ipg.add_text(
                    parent_id=self.cont_v_top, 
                    content="Try Scrolling a Column Container.")

        # A container is put into a scrollable, so it needs to be added first.
        # The width and height should be used control the size of the scrollable,
        # depending if its horizontal or vertical.  In this case, we control
        # the height by setting it a value.  If you used height_fill=True in this case,
        # the container and text widget would be pushed out of the window but you
        # could still scroll because the height is less than the data height.
        self.scroll_id_1 = self.ipg.add_scrollable(
                                        window_id=self.wnd_v, 
                                        container_id=self.scroll_v,
                                        width_fill=True, 
                                        height=150.0, 
                                        on_scroll=self.on_scroll_v)

        # A column is next added since the expectation is that you have a long list of
        # items that need to be scrolled.  This could be anything like buttons, radios, text, etc.
        # NOTE that the column height should be shrink which is the default, i.e no value.
        # The scrollable size will control the size of the scrollable container.
        self.ipg.add_column(
                    window_id=self.wnd_v, 
                    container_id=self.cont_v_middle,
                    parent_id=self.scroll_v, 
                    width_fill=True,
                    align=IpgAlignment.Center)

        for i in range(0, 25):
            self.ipg.add_text(
                        parent_id=self.cont_v_middle, 
                        content="Scroll Me Up and Down! Scroll Me Up and Down!")

        # Container for the callback text
        self.ipg.add_container(
                    window_id=self.wnd_v, 
                    container_id=self.cont_v_bottom, 
                    parent_id=self.wnd_v,
                    width_fill=True, 
                    height_fill=True)

        self.cb_text_v = self.ipg.add_text(
                                    parent_id=self.cont_v_bottom,
                                    content=f"Some data when scrolled")

        # Adding row for buttons to change things
        self.ipg.add_row(
                    window_id=self.wnd_v, 
                    container_id="row_v")
        
        self.ipg.add_button(
                    parent_id="row_v", 
                    label="Press to Change Width",
                    on_press=self.change_width)
        
        self.ipg.add_button(
                    parent_id="row_v", 
                    label="Press to Change Height",
                    on_press=self.change_height)

    def change_width(self, btn_id):
        self.ipg.update_item(
            wid=self.scroll_id_1, 
            param=IpgScrollableParam.Width, 
            value=300.0)

    def change_height(self, btn_id):
        self.ipg.update_item(
            wid=self.scroll_id_1, 
            param=IpgScrollableParam.Height, 
            value=200.0)

    # ************Window 2 scrolling a row container horizontally**********************************************

    def create_scroll_horizontal(self):
        self.ipg.add_window(
                    window_id=self.wnd_h, 
                    title="Scrollable - Horizontal", 
                    width=200,
                    height=self.wnd_height,
                    pos_x=500, 
                    pos_y=25)

        self.ipg.add_container(
                    window_id=self.wnd_h, 
                    container_id=self.cont_h_top,
                    width_fill=True, 
                    height=200.0, 
                    padding=[20])

        self.ipg.add_text(
                    parent_id=self.cont_h_top, 
                    content="Try Scrolling a Row Container.")

        # Unlike for the vertical scroller above, it's ok to use the full width
        # screen because nothing is in the way and the data is larger than the
        # window width.  However, we wanted to keep the scrollable height small
        # since there is only a single line of text.
        self.ipg.add_scrollable(
                    window_id=self.wnd_h, 
                    container_id=self.scroll_h,
                    direction=IpgScrollableDirection.Horizontal,
                    width_fill=True, 
                    height=50.0,
                    on_scroll=self.on_scroll_h)

        # NOTE: The row width and height should be left at default, no value.
        self.ipg.add_row(
                    window_id=self.wnd_h, 
                    container_id=self.cont_h_middle,
                    parent_id=self.scroll_h)

        for i in range(0, 25):
            self.ipg.add_text(
                        self.cont_h_middle, 
                        content="Scroll Me left or Right!")

        # The final mostly empty container is added at the bottom
        self.ipg.add_container(
                    window_id=self.wnd_h, 
                    container_id=self.cont_h_bottom,
                    parent_id=self.wnd_h, 
                    width_fill=True, 
                    height=200)

        self.cb_text_h = self.ipg.add_text(
                                    parent_id=self.cont_h_bottom,
                                    content=f"Some data when scrolled")

    # The data in this case in a dictionary, absolute, relative, and absolute reversed
    # {'abs_x': 0.0, 'abs_y': 0.0, 'rel_x': 0.0, 'rev_x': 0.0, 'rev_y': 0.0, 'rel_y': 0.0}
    def on_scroll_v(self, id, data):
        text = "\n" + 'abs_x = ' + str(data.get('abs_x'))
        text += "\n" + 'abs_y = ' + str(data.get('abs_y'))
        text += "\n" + 'rel_x = ' + str(data.get('rel_x'))
        text += "\n" + 'rel_y = ' + str(data.get('rel_y'))
        text += "\n" + 'rev_x = ' + str(data.get('rev_x'))
        text += "\n" + 'rev_y = ' + str(data.get('rev_y'))
 
        self.ipg.update_item(
            wid=self.cb_text_v, 
            param=IpgTextParam.Content,
            value=f"scrollable id = {id}\n{text}")

    def on_scroll_h(self, id, data):
        text = "\n" + 'abs_x = ' + str(data.get('abs_x'))
        text += "\n" + 'abs_y = ' + str(data.get('abs_y'))
        text += "\n" + 'rel_x = ' + str(data.get('rel_x'))
        text += "\n" + 'rel_y = ' + str(data.get('rel_y'))
        text += "\n" + 'rev_x = ' + str(data.get('rev_x'))
        text += "\n" + 'rev_y = ' + str(data.get('rev_y'))
        
        self.ipg.update_item(
            wid=self.cb_text_h, 
            param=IpgTextParam.Content,
            value=f"scrollable id = {id}\n{text}")

    # ***************Window 3-scrolling both directions with other property setting**********************************

    def create_scroll_both(self):
        # Add the 3rd window
        self.ipg.add_window(
                    window_id=self.wnd_b, 
                    title="Scrollable - Both",
                    width=self.wnd_width + 100, 
                    height=self.wnd_height,
                    pos_x=760, 
                    pos_y=25)

        # The container is added to center the contents below.
        self.ipg.add_container(
                    window_id=self.wnd_b, 
                    container_id=self.cont_b,
                    width_fill=True, 
                    height_fill=True)

        # Add a column to hold all the widgets
        self.ipg.add_column(
                    window_id=self.wnd_b, 
                    container_id="col", 
                    parent_id=self.cont_b,
                    spacing=10, 
                    align=IpgAlignment.Center)
        
        # Display some info
        self.ipg.add_text(
                    parent_id="col", 
                    content="You may have to press buttons many times to see the changes")
        
        # The scrollable size controls the viewport for the column container.
        self.scroll_id_3 = self.ipg.add_scrollable(
                                        window_id=self.wnd_b, 
                                        container_id=self.scroll_b,
                                        parent_id="col",
                                        width=250, 
                                        height=100.0,
                                        direction=IpgScrollableDirection.Both)

        # NOTE:  The column width and height should default to shrink, no value.
        self.ipg.add_column(
                    window_id=self.wnd_b, 
                    container_id=self.col_b, 
                    parent_id=self.scroll_b,
                    align=IpgAlignment.Center)

        for _ in range(0, 25):
            self.ipg.add_text(
                        parent_id=self.col_b, 
                        content="Scroll Me Up, Down, left, or Right!"
                                "Scroll Me Up, Down, left, or Right!")

        # Add row to hold the buttons.
        self.ipg.add_row(
                    window_id=self.wnd_b, 
                    container_id="row_1", 
                    parent_id="col")

        self.ipg.add_button(
                    parent_id="row_1", 
                    label="Press to + H Bar Width",
                    on_press=self.inc_dec_h_bar_width, 
                    padding=[5], 
                    user_data=1)
        
        self.ipg.add_button(
                    parent_id="row_1", 
                    label="Press to - H Bar Width",
                    on_press=self.inc_dec_h_bar_width, 
                    padding=[5], 
                    user_data=-1)

        self.ipg.add_row(
                    window_id=self.wnd_b, 
                    container_id="row_2", 
                    padding=[5], 
                    parent_id="col")

        self.ipg.add_button(
                    parent_id="row_2", 
                    label="Press to + V Bar Width",
                    on_press=self.inc_dec_v_bar_width, 
                    padding=[5], 
                    user_data=1)
        
        self.ipg.add_button(
                    parent_id="row_2", 
                    label="Press to - V Bar Width",
                    on_press=self.inc_dec_v_bar_width, 
                    padding=[5], 
                    user_data=-1)

        self.ipg.add_row(
                    window_id=self.wnd_b, 
                    container_id="row_3", 
                    parent_id="col")

        self.ipg.add_button(
                    parent_id="row_3", 
                    label="Press to + H Bar Margin",
                    on_press=self.inc_dec_h_bar_margin, 
                    padding=[5], 
                    user_data=1)
        
        self.ipg.add_button(
                    parent_id="row_3", 
                    label="Press to - H Bar Margin",
                    on_press=self.inc_dec_h_bar_margin, 
                    padding=[5], 
                    user_data=-1)

        self.ipg.add_row(
                    window_id=self.wnd_b, 
                    container_id="row_4", 
                    parent_id="col")

        self.ipg.add_button(
                    parent_id="row_4", 
                    label="Press to + V Bar Margin",
                    on_press=self.inc_dec_v_bar_margin, 
                    padding=[5], 
                    user_data=1)
        
        self.ipg.add_button(
                    parent_id="row_4", 
                    label="Press to - V Bar Margin",
                    on_press=self.inc_dec_v_bar_margin, 
                    padding=[5], 
                    user_data=-1)

        self.ipg.add_row(
                    window_id=self.wnd_b, 
                    container_id="row_5", 
                    parent_id="col")

        self.ipg.add_button(
                    parent_id="row_5", 
                    label="Press to + H Scroller Width",
                    on_press=self.inc_dec_h_scroller_width, 
                    padding=[5], 
                    user_data=1)
        
        self.ipg.add_button(
                    parent_id="row_5", 
                    label="Press to - H Scroller Width",
                    on_press=self.inc_dec_h_scroller_width, 
                    padding=[5], 
                    user_data=-1)

        self.ipg.add_row(
                    window_id=self.wnd_b, 
                    container_id="row_6", 
                    parent_id="col")

        self.ipg.add_button(
                    parent_id="row_6", 
                    label="Press to Change V Scroller Width",
                    on_press=self.inc_dec_v_scroller_width, 
                    padding=[5], 
                    user_data=1)
        
        self.ipg.add_button(
                    parent_id="row_6", 
                    label="Press to - V Scroller Width",
                    on_press=self.inc_dec_v_scroller_width, 
                    padding=[5], 
                    user_data=-1)

        self.ipg.add_row(
                    window_id=self.wnd_b, 
                    container_id="row_7", 
                    parent_id="col")

    def inc_dec_h_bar_width(self, btn_id, inc_dec):
        self.h_bar_width += inc_dec
        self.ipg.update_item(
            wid=self.scroll_id_3, 
            param=IpgScrollableParam.HBarWidth, 
            value=self.h_bar_width)

    def inc_dec_v_bar_width(self, btn_id, inc_dec):
        self.v_bar_width += inc_dec
        self.ipg.update_item(
            wid=self.scroll_id_3, 
            param=IpgScrollableParam.VBarWidth, 
            value=self.v_bar_width)

    def inc_dec_h_bar_margin(self, btn_id, inc_dec):
        self.h_bar_margin += inc_dec
        self.ipg.update_item(
            wid=self.scroll_id_3, 
            param=IpgScrollableParam.HBarMargin, 
            value=self.h_bar_margin)

    def inc_dec_v_bar_margin(self, btn_id, inc_dec):
        self.v_bar_margin += inc_dec
        self.ipg.update_item(
            wid=self.scroll_id_3, 
            param=IpgScrollableParam.VBarMargin, 
            value=self.v_bar_margin)

        def inc_dec_h_scroller_width(self, btn_id: int, inc_dec: float):
        self.h_scroller_width += inc_dec
        self.ipg.update_item(
            wid=self.scroll_id_3, 
            param=IpgScrollableParam.HScrollerWidth, 
            value=self.h_scroller_width
        )
    
    def inc_dec_v_scroller_width(self, btn_id: int, inc_dec: float):
        self.v_scroller_width += inc_dec
        self.ipg.update_item(
            wid=self.scroll_id_3, 
            param=IpgScrollableParam.VScrollerWidth, 
            value=self.v_scroller_width
        )


# instantiate the class
ds = DemoScrollable()

ds.create_gui()
