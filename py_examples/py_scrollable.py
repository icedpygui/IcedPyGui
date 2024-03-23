from icedpygui.icedpygui import IPG


# To reduce typing errors when using the ids, I like to 
# use a class or define variables like below then just 
# select which one I want from the hint.

# In this way, you can change the ids if you decide to later 
# with little effort.

# However, you can just type in the string if you like.

# Currently @dataclass is not supported since it sends a tuple 
# and not a string.  Hopefully I can fixed in the future. 

# Note on scrolling:
# Sometimes getting the windows sized correctly takes a bit
# of patiences using fill, fixed, shrink, fillportion
# When using scrolling, size the scrollable and let the container,
# if horizontal be fill and the length be shrink.
# if vertical scrolling, let the container width be shrink and
#  the length can be shrink or fill.


class DemoScrollable:
    def __init__(self) -> None:
        self.ipg = IPG()
        self.wnd_width: int=400
        self.wnd_height: int=600

        # To help reduce spelling errors,
        # you can set up you variables
        # and select from your ide's dropdown list
        # without havin to type in the string.
        self.wnd_v: str="window_v"
        self.cont_v_top: str="cont_v_top"
        self.scroll_v: str="scroll_v"
        self.cont_v_middle: str="cont_v_middle"
        self.cont_v_bottom: str="cont_v_bottom"

        self.wnd_h: str="window_h"
        self.cont_h_top: str="cont_h_top"
        self.scroll_h: str="scroll_h"
        self.cont_h_middle: str="rcont_h_middle" 
        self.cont_h_bottom: str="cont_h_bottom"

        self.cb_text_v: int=0
        self.cb_text_h: int=0
        

    # start_session must be the last function called
    def start_session(self):
        self.create_scroll_vertical()
        self.create_scroll_horizontal()
        self.ipg.start_session()

    # ***************Window 1-scrolling a column container vertically**********************************
    # We will create 3 containers, a header, a scrollable, and an empty bottom container.
    def create_scroll_vertical(self):

        self.ipg.add_window(self.wnd_v, "Scollable - Vertical", 
                                self.wnd_width, self.wnd_height, 
                                200, 100, theme="dark")

        # The header is just a title of sorts.
        self.ipg.add_container(window_id=self.wnd_v, container_id=self.cont_v_top, 
                                 align_x="center", align_y="center", 
                                 width_fill=True, height=200.0, padding=[20])

        self.ipg.add_text(self.cont_v_top, "Try Scrolling a Column Container.")

        # a scrollable is the outside of a container, wraps it, so it needs to be added first
        # The width and height should controll the size and let the internal container be fill
        # If you have multiple containers inside the scrollable, try using fillportion or fixed
        # The disadvantage of using fixed is that you'll need to recalc your sizes on a callback
        # and update each container.
        self.ipg.add_scrollable(window_id=self.wnd_v, container_id=self.scroll_v,
                                width_fill=True, height=150.0, on_scroll=self.on_scroll_v)

        # A column is next added since the expectation is that you have a long list of
        # items that need to be scrolled.  The could be anything like buttons, radios, text, etc.
        # Note that the column height shoulb be shrink which is the default.  The scrollable will
        # control the size of the scrollable container.
        self.ipg.add_column(window_id=self.wnd_v, container_id=self.cont_v_middle, parent_id=self.scroll_v, 
                            width_fill=True, align_items="center")

        for i in range(0, 25):
            self.ipg.add_text(self.cont_v_middle, content="Scroll Me Up and Down!")

        # Container for the callback text
        self.ipg.add_container(window_id=self.wnd_v, container_id=self.cont_v_bottom, parent_id=self.wnd_v,
                            width_fill=True, height_fill=True, 
                            align_x="center", align_y="center")
        
        self.cb_text_v = self.ipg.add_text(parent_id=self.cont_v_bottom, 
                                      content=f"Some data when scrolled")

# ************Window 2 scrolling a row container horizontally**********************************************
    
    def create_scroll_horizontal(self):
        self.ipg.add_window(self.wnd_h, "Scollable - Horizontal", self.wnd_width, 
                                            self.wnd_height, 650, 100,
                                            theme="dark", debug=False)

        self.ipg.add_container(window_id=self.wnd_h, container_id=self.cont_h_top, 
                                align_x="center", align_y="center", 
                                width_fill=True, height=200.0, padding=[20])

        self.ipg.add_text(self.cont_h_top, "Try Scrolling a Row Container.")


        self.ipg.add_scrollable(window_id=self.wnd_h, container_id=self.scroll_h, 
                                direction="horizontal",
                                width_fill=True, height=50.0,
                                on_scroll=self.on_scroll_h)

        self.ipg.add_row(window_id=self.wnd_h, container_id=self.cont_h_middle, 
                         parent_id=self.scroll_h, align_items="start")

        for i in range(0, 25):
            self.ipg.add_text(self.cont_h_middle, content="Scroll Me left or Right!")

        # The final mostly empty container is added at the bottom
        self.ipg.add_container(window_id=self.wnd_h, container_id=self.cont_h_bottom, 
                               parent_id=self.wnd_h,width_fill=True, height=200, 
                                align_x="center", align_y="center")
        
        self.cb_text_h = self.ipg.add_text(parent_id=self.cont_h_bottom, 
                                      content=f"Some data when scrolled")

        # The data in this case in a dictionary, check the docs or print data to 
        # determine the key, value of the data.
    def on_scroll_v(self, id, data):
        text = "\n".join("{}: {}".format(k, v) for k, v in data.items())
        self.ipg.update_item(self.cb_text_v, "content", 
                             value=f"scrollable id = {id}\n{text}")
    
    def on_scroll_h(self, id, data):
        text = "\n".join("{}: {}".format(k, v) for k, v in data.items())
        self.ipg.update_item(self.cb_text_h, "content", 
                             value=f"scrollable id = {id}\n{text}")


ds = DemoScrollable()
ds.start_session()

