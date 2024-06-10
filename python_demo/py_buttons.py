
from icedpygui import IPG, IpgColor
from icedpygui import IpgButtonArrows, IpgButtonParams
from icedpygui import IpgColumnAlignment, IpgRowAlignment
from icedpygui import IpgTextParams, IpgStyleParam


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
        self.ipg.add_window(window_id=self.wnd_id, title="Button Demo", width=800, height=700,
                            pos_x=100, pos_y=25)
        # Setup the styling section.
        self.setup_button_styles()

        # Setup the padding section.
        self.setup_padding_section()

        # Setup the change color button
        self.setup_change_color()

        # Setup the chnage in border
        self.setup_change_border()

        # Setup the change in shadow
        self.setup_change_shadow()

        # Setup the change in text color
        self.setup_change_text_color()

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
        background = [IpgColor.PRIMARY, IpgColor.SECONDARY, IpgColor.SUCCESS,
                     IpgColor.DANGER, IpgColor.TRANSPARENT]
        # add a border styling
        self.ipg.add_styling_border(style_id="border", radius=[12.0])

        for i, style in enumerate(style_text):
            self.ipg.add_styling_background(style_id=f"cont{i}", color=background[i])
            self.button_style_ids.append(self.ipg.add_button(parent_id=self.style_id,
                                                             label=style,
                                                             style_background=f"cont{i}",
                                                             style_border="border",
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
                                                             style_arrow=arrows_ipg[i],
                                                             style_background=f"cont{i}",
                                                             user_data=arrow))

        # This is the text that will change when a button is pressed therefore the id is needed.
        # This could have been obtained at the time of it's creation by just setting the
        # widget equal to self.btn_info.  This was just a demo of another technique.
        self.ipg.add_text(parent_id=self.style_col_id, content="This will change when a button is pressed",
                          gen_id=self.btn_info)

    def setup_padding_section(self):

        # A column is added for center alignment of the remaining items
        # One could have used a single column for everything, it's just a matter of
        # how you want to group things and your needs 
        self.ipg.add_column(window_id=self.wnd_id, container_id=self.sld_col,
                            align_items=IpgColumnAlignment.Center)

        #  Text widgets are added for padding info.
        self.ipg.add_text(self.sld_col, content="The padding effect is shown below.")
        self.ipg.add_text(self.sld_col, content="T = Top, B=Bottom, L=Left, R=Right")

        # A row container is added to align the buttons.
        self.ipg.add_row(window_id=self.wnd_id, container_id=self.row_id, 
                         parent_id=self.sld_col, align_items=IpgRowAlignment.Center)

        # All of the buttons below have different padding values.  The padding parameter
        # has 3 different type of values.  A list of a single item sets padding on all sides.
        # A list of 2 values sets padding on left and right, respectively.
        # A list of 4 values sets padding specifically on each of the sides,
        # top, right, bottom, left, respectively. A clockwise pattern.
        # if you wanted just the right for example you would use all four and just adjust
        #  the second item in the list.  You will have to supply the default value for the others
        self.ipg.add_button(self.row_id, label="Padding all 0", padding=[0])

        self.ipg.add_button(self.row_id, label="Padding all,10", padding=[10])

        self.ipg.add_button(self.row_id, label="Padding top/bot 20, l/r 10", padding=[20, 10])

        self.ipg.add_button(self.row_id, label="Padding t10, r20, b5 l15", padding=[10, 20, 5, 15])

        
    def set_corner_radius(self, _slider_id: int, value: float, user_data: any):
        # This is the slider callback with it's id (not used), value and user_data.
        # These parameter names can be anything you like, the order is always
        # the id of the widget, a value, if used, a user -data, if used.
        # The user_data can be anything, since it's just passed through Rust.
        #  The list of all the ids were stored and each button can be changed based on their id.
        for btn_id in user_data:
            self.ipg.update_item(btn_id, IpgButtonParams.StyleBorder, float(value))

        self.ipg.update_item(self.text_id, IpgTextParams.Content, f"Slider Value {value}")

    def setup_change_color(self):
        self.ipg.add_row(window_id=self.wnd_id, 
                         container_id="color_row",
                         width_fill=True, 
                         align_items=IpgRowAlignment.End)
        
        # initial stye for button
        self.ipg.add_styling_background("bkg", color=IpgColor.PRIMARY)
        # style after button is pressed
        self.ipg.add_styling_background("new_bkg", color=IpgColor.DANGER)

        self.ipg.add_button(parent_id="color_row", 
                            label="Press Me to\nChange Background Color",
                            style_background="bkg",
                            on_press=self.change_color)
        
    def setup_change_border(self):
        #initial button border will be defaulted, new one below
        self.ipg.add_styling_border(style_id="border_update", radius=[12.0], color=IpgColor.YELLOW, width=5.0)

        self.ipg.add_button(parent_id="color_row", 
                            label="Press Me to\nChange Border",
                            style_border="border",
                            on_press=self.change_border)
    
    def setup_change_shadow(self):
        # Add future shadow style
        self.ipg.add_styling_shadow(style_id="shadow", color=IpgColor.SALMON, blur_radius=15.0)

        self.ipg.add_button(parent_id="color_row", 
                            label="Press Me to\nChange Shadow",
                            on_press=self.change_shadow)
    
    def setup_change_text_color(self):
        # Add future text style
        self.ipg.add_styling_text_color(style_id="text", color=IpgColor.BLACK)

        self.ipg.add_button(parent_id="color_row", 
                            label="Press Me to \nChange Text Color",
                            on_press=self.change_text_color)

    def button_pressed(self, _btn_id: int, user_data: any):
        # This is a callback that occurs when the button is pressed
        # The btn_id equals the button that was pressed and the user_data, in this case,
        # is a string for the text widget below.  The btn_id is not used.
        self.ipg.update_item(self.btn_info, IpgTextParams.Content, f"Last button pressed was {user_data}")

    def change_color(self, btn_id):
        self.ipg.update_item(btn_id, IpgButtonParams.StyleBackground, 
                             value="new_bkg")

    def change_border(self, btn_id):
        self.ipg.update_item(btn_id, IpgButtonParams.StyleBorder, 
                             value="border_update")
        
    def change_shadow(self, btn_id):
        self.ipg.update_item(btn_id, IpgButtonParams.StyleShadow, 
                             value="shadow")
        
    def change_text_color(self, btn_id):
        self.ipg.update_item(btn_id, IpgButtonParams.StyleTextColor, 
                             value="text")


# instantiates the class
demo = ButtonDemo()
# call to setup the gui
demo.setup_gui()
