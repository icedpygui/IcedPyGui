from icedpygui import IPG
from icedpygui import IpgButtonArrows, IpgButtonParams, IpgButtonStyle
from icedpygui import IpgColumnAlignment
from icedpygui import IpgTextParams


# In this case a class is being used to give
# you an idea of how a more organize structure
# might like.  In some cases where you use string
# ids a lot, you might want to include those in the
# init so that when you start typing, you can select it.
# This helps cut down on typos which cause error about not
# finding the parent_id, etc.
class ButtonDemo:
    def __init__(self) -> None:
        # This needs to be in the init function
        # before any ipg commands are used.
        self.ipg = IPG()

        self.button_style_ids: list = []
        self.button_arrow_ids: list = []

        # ids - I prefer to give my string ids a variable name so that
        #  I can select them from the IDE's dropdown list, it reduces spelling errors.
        self.wnd_id: str = "main"
        self.style_col_id: str = "style_col"
        self.style_id: str = "style_id"
        # This is a special case scenario where you need an id for a widget
        # before starting.  If so, generate it and use it for the optional gen_id
        # parameter in your widget.
        self.btn_info: int = self.ipg.generate_id()
        # For the below text_id, a zero place holder is used.  Zero is an invalid id
        # so if you later miss initializing it, you will get an error.  Alternately,
        # you could use the above method.
        self.text_id = 0
        self.sld_col: str = "sld_col"
        self.row_id: str = "padding_row"
        self.arrow_row: str = "arrow_row"

    # sets the gui up and starts the session.
    def setup_gui(self):

        # The first widget added must be a window, more windows can be added at any time.
        # Windows and containers must be added before their widgets are added. 
        self.ipg.add_window(window_id=self.wnd_id, title="Button Demo", width=800, height=600,
                            pos_x=100, pos_y=25)
        # Setup the styling section.
        self.setup_button_styles()

        # Setup the slider section.
        self.setup_slider_section()

        # Setup the padding section.
        self.setup_padding_section()

        # Required to be the last widget sent to Iced,  If you start the program
        # and nothing happens, it might mean you forgot to add this command.
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
                            align_items=IpgColumnAlignment.Center, width_fill=True)

        # Just text giving info. A widget needs to be added to a container so the parent_id
        # points to column with id self.style_col_id.
        self.ipg.add_text(parent_id=self.style_col_id, content="The buttons below show the different standard styles")

        # The row is needed for the horizontal alignment of the buttons
        self.ipg.add_row(window_id=self.wnd_id, container_id=self.style_id, parent_id=self.style_col_id)

        # Adding another row for the arrows styles
        self.ipg.add_row(window_id=self.wnd_id, container_id=self.arrow_row, parent_id=self.style_col_id)

        # A row of buttons are added, the ids [b1, ...] will be used to update the style of the buttons
        # The callback is the same for each button but may be different based on needs.
        # The user_data is the style name and just used to update a text widget in the callback.
        style_text = ["Primary", "Secondary", "Success", "Danger", "Text"]
        style_ipg = [IpgButtonStyle.Primary, IpgButtonStyle.Secondary, IpgButtonStyle.Success,
                     IpgButtonStyle.Danger, IpgButtonStyle.Text]

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
        # This could have been obtained at the time of it's creation by just setting the
        # widget equal to self.btn_info.  This was just a demo of another technique.
        self.ipg.add_text(parent_id=self.style_col_id, content="This will change when a button is pressed",
                          gen_id=self.btn_info)

    def setup_slider_section(self):

        # A column is added for center alignment of the remaining items
        # One could have used a single column for everything, it's just a matter of
        # how you want to group things and your needs 
        self.ipg.add_column(window_id=self.wnd_id, container_id=self.sld_col,
                            align_items=IpgColumnAlignment.Center)

        # text widget for info
        self.ipg.add_text(parent_id=self.sld_col,
                          content="Using the slider, see the effect of changing the corner radius")

        # A slider widget is used to change a value which is sent to the callback, set_corner_radius.
        # The value for the slider is passed to the callback.  In this case we need some additional 
        # data sent so a list of integers are sent. You can send any type of data since it just
        # passes through rust with no change.
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
        # has 3 different type of values.  A list of a single item sets padding on all sides.
        # A list of 2 values sets padding on left and right, respectively.
        # A list of 4 values sets padding specifically on each of the sides,
        # top, right, bottom, left, respectively. A clockwise pattern.
        # if you wanted just the right for example you would use all four and just adjust
        #  the second item in the list.  You will have to supply the default value for the others
        self.ipg.add_button(self.row_id, label="Padding 0", padding=[0])

        self.ipg.add_button(self.row_id, label="Padding all,10", padding=[10])

        self.ipg.add_button(self.row_id, label="Padding top/bot 20, l/r 10", padding=[20, 10])

        self.ipg.add_button(self.row_id, label="Padding t10, r20, b5 l15", padding=[10, 20, 5, 15])

        # Another text widget for info.
        self.ipg.add_text(parent_id=self.sld_col, content="The height and width will be show in a separate example")

    def set_corner_radius(self, _slider_id: int, value: float, user_data: any):
        # This is the slider callback with it's id (not used), value and user_data.
        # These parameter names can be anything you like, the order is always
        # the id of the widget, a value, if used, a user -data, if used.
        # The user_data can be anything, since it's just passed through Rust.
        #  The list of all the ids were stored and each button can be changed based on their id.
        for btn_id in user_data:
            self.ipg.update_item(btn_id, IpgButtonParams.CornerRadius, float(value))

        self.ipg.update_item(self.text_id, IpgTextParams.Content, f"Slider Value {value}")

    def button_pressed(self, _btn_id: int, user_data: any):
        # This is a callback that occurs when the button is pressed
        # The btn_id equals the button that was pressed and the user_data, in this case,
        # is a string for the text widget below.  The btn_id is not used.
        self.ipg.update_item(self.btn_info, IpgTextParams.Content, f"Last button pressed was {user_data}")


# instantiates the class
demo = ButtonDemo()
# call to setup the gui
demo.setup_gui()
