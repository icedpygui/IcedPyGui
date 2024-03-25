from icedpygui import IPG, IpgButtonStyles
from icedpygui import IpgButtonArrows


class ButtonDemo:
    def __init__(self) -> None:
        self.ipg = IPG()
        
        self.button_style_ids: list = []
        self.button_arrow_ids: list = []

        # ids - I prefer to give my string ids a variable name so that
        # it I select them from a list and it reduced spelling errors.
        self.wnd_id: str="main"
        self.style_col_id: str="style_col"
        self.style_id: str="style_id"
        self.btn_info: int=self.ipg.generate_id()
        self.sld_col: str="sld_col"
        self.row_id: str="padding_row"
        self.arrow_row: str = "arrow_row"

    # sets the gui up and starts the session.
    def setup_gui(self):

        # The first widget added must be a window, more windows can be added at any time.
        # Windows and containers must be added before their widgets are added. 
        self.ipg.add_window(self.wnd_id, "Button Demo", 800, 800, 
                                        500, 100, True)
        # Setup the styling section.
        self.setup_button_styles()

        # Setup the slider section.
        self.setup_slider_section()

        # Seup the padding section.
        self.setup_padding_section()
        
        # This function starts the gui and must be the last function that is called
        self.ipg.start_session()

    def setup_button_styles(self):

        # This column holds the widgets and is used to center the items.
        # The items are center aligned and the width is fill, which fills the window.
        # The default width is shrink which wouldn't align the items in the middle of
        # the screen since it "shrinks" to the size of the contained widgets.
        # Sometimes the alignment is not obvious but using debug=True on the window settings
        # shows the outline of the widgets which helps.  You could also used "fixed"
        # for a specific size.
        self.ipg.add_column(self.wnd_id, container_id=self.style_col_id, 
                            align_items="center", width_fill=True)

        # Just text giving info. A widget needs to be added to a container so the parent_id
        # points to column with id self.style_col_id.
        self.ipg.add_text(parent_id=self.style_col_id, content="The buttons below show the different standard styles")

        # The row is needed for the horizontal alignment of the buttons
        self.ipg.add_row(window_id=self.wnd_id, container_id=self.style_id, parent_id=self.style_col_id)

        # Adding another row for the arrows styles
        self.ipg.add_row(window_id=self.wnd_id, container_id=self.arrow_row, parent_id=self.style_col_id)

        # A row of buttons are added, the ids [b1, ...] will be used to update the style of the buttons
        # The callback is the same for each button but may be different based on needs.
        style_text = ["Primary", "Secondary", "Positive", "Destructive", "Text"]
        style_ipg = [IpgButtonStyles.Primary, IpgButtonStyles.Secondary, IpgButtonStyles.Positive,
                     IpgButtonStyles.Destructive, IpgButtonStyles.Text]
        
        for i, style in enumerate(style_text): 
            self.button_style_ids.append(self.ipg.add_button(parent_id=self.style_id, 
                                                                    label=style, 
                                                                    style=style_ipg[i], 
                                                                    on_press=self.button_pressed, 
                                                                    user_data=style))
        
        # The same approach as above is used here.
        arrows = ["UpArrow", "RightArrow", "DownArrow", "LeftArrow"]
        arrows_ipg = [IpgButtonArrows.ArrowUp, IpgButtonArrows.ArrowRight, IpgButtonArrows.ArrowDown,
                      IpgButtonArrows.ArrowLeft]

        for i, arrow in enumerate(arrows):
            self.button_style_ids.append(self.ipg.add_button(self.arrow_row, 
                                                                    "",
                                                                     on_press=self.button_pressed, 
                                                                    padding=[5.0],
                                                                    arrow_style=arrows_ipg[i],
                                                                    user_data=arrow))
        

        # This is the text that will change when a button is pressed therefore the id is needed.
        self.ipg.add_text(parent_id=self.style_col_id, content="This will change when a button is pressed",
                          id=self.btn_info)

    def setup_slider_section(self):

        # A column is added for center alignment of the remaining items
        # One could have used a single column for everything, it's just a metter of 
        # how you want to group things and your needs 
        self.ipg.add_column(window_id=self.wnd_id, container_id=self.sld_col, align_items="center")

        # text widget for info
        self.ipg.add_text(parent_id=self.sld_col, content="Using the slider, see the effect of changing the corner radius")

        # A slider wiget is used to change a value which is sent to the callback, set_corner_radius.
        # The value for the slider is passed to the callback.  In this case we need some additional 
        # data sent so a list of integers are sent. You can send int, float, sting, or bool.
        self.ipg.add_slider(parent_id=self.sld_col, min=0.0, max=20.0, step=0.5, value=10.0, 
                            width=300.0, on_change=self.set_corner_radius, 
                            user_data=self.button_style_ids)

        # Text widget for info, since it changes as the slider is moved, the id is needed.
        self.text_id = self.ipg.add_text(parent_id=self.sld_col, content="Slider Value is 10")

    def setup_padding_section(self):

        #  Text widgets are added for padding info.
        self.ipg.add_text(self.sld_col, content="The padding effect is shown below.")
        self.ipg.add_text(self.sld_col, content="T = Top, B=Bottom, L=Left, R=Right")

        # A row container is added to align the buttons.
        self.ipg.add_row(window_id=self.wnd_id, container_id=self.row_id, parent_id=self.sld_col)

        # All of the buttons below have different padding values.  The padding parameter
        # has 3 different type of values.  A list of a single item setting padding on all sides.
        # A list of 2 values sets padding on left and right, respectively.
        # A list of 4 alues sets pading specifically on each of the sides,
        # top, right, bottom, left, respectively. A clockwise pattern.
        # if you wanted just the right for example you would use all four and just adjust
        #  the second item in the list.  You will have to supply the default value for the others
        self.ipg.add_button(self.row_id, label="Padding 0", padding=[0])

        self.ipg.add_button(self.row_id, label="Padding all,10", padding=[10])

        self.ipg.add_button(self.row_id, label="Padding top/bot 20, l/r 10", padding=[20, 10])

        self.ipg.add_button(self.row_id, label="Padding t10, r20, b5 l15", padding=[10, 20, 5, 15])

        # Another text widget for info.
        self.ipg.add_text(parent_id=self.sld_col, content="The height and width will be show in a separate example")

    def set_corner_radius(self, id, value, user_data):
        # The slider uses this callback and the slider id, value and any user data is
        # returned.  These parameter names can be anything you like as long
        # as you know that the order is all the same for all callbacks.
        # The user_data can be anything.  Since the user_data is just passed through Rust,
        # some of the issure with mixed list are not seen.
        #  The list of all the ids were stored and each button can be changed based on their id.
        for id in user_data:
            self.ipg.update_item(id, "corner_radius", float(value))

        self.ipg.update_item(self.text_id, "content", f"Slider Value {value}")

    def button_pressed(self, id, user_data):
        # This is a callback that occurs when the button is pressed
        # The id equals the button that was pressed and the data
        # for the button is None but its needed as a placeholder in the fucntion.
        #  THe user_data_str is a list of any string that the user wants to use.
        #  In this case, it was just the name of the button style used in the value_str below.

        self.ipg.update_item(self.btn_info, "content", f"Last button pressed was {user_data}")

    def arrow_selected(self, id, name):
        print("UpArrow")


demo = ButtonDemo()
demo.setup_gui()
