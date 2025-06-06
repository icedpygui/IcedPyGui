from icedpygui import IPG, IpgTextParam, IpgRadioDirection
from icedpygui import IpgButtonParam, IpgProgressBarParam
from icedpygui import IpgAlignment
import polars as pl
import random

"""
    IcedPyGui is based on Rust Iced gui at https://github.com/iced-rs/iced.
    Some code is used from Iced_aw at https://github.com/iced-rs/iced_aw.
    Pyo3 is used as the python wrapper at https://github.com/pyo3/pyo3.
    Maturin is used to build and publish the module at https://github.com/PyO3/maturin.
    
    IPG is easy to use.  The syntax used follows closely with
    that found using dearpygui, my inspiration for this.  
    IPG has a backend of Rust versus c++ which does result in some
    differences.  The key difference will be in the way
    certain data is structured.  Rust doesn't allow mixed types in lists or 
    dictionaries, but this difference has mostly been shielded from the user.
    The table is where you will mostly see this.  For example, instead of
    using a dictionary for the data, List of dictionaries where used and 
    have distinct data types like {string, List[int]} or 
    {String, List[float]}, etc.  The only mixed list is a tuple but even
    that has to be strictly defined in rust like (int, string) or (string, int).
    The user data is special because it is only passed through to rust and 
    back out as a PyObject or PyAny.  Therefore any python data can be used
    since it is never extracted into a rust type.     
    
    A few simple rules need to be followed.   
        import IPG as indicated above in this demo.      
        
        The last line of code to execute must be ipg.start_session().
        Any code after that will not be executed because rust Iced is now
        running.  You can place it anywhere, just make sure its last executed.
        If you start your program and nothing happens, it might mean that
        you aren't executing start_session() or you forgot to add it in.    
        
        Every widget needs to have a parent container previously defined and 
        every container needs to have a window and optionally a parent container 
        defined.  If the container is placed into a window then no parent_id is required.
        Therefore at least one window needs to be added first and at least 
        one container needs to be added to the window before any widgets are
        added.  As long as you have defined a parent, you can add to it.

    The organization of your program is your choice.  You can use a class
    or just functions for simple programs.  A @dataclass is not supported
    at this time but should be soon.  You can add all of your containers 
    at once if you want or add the container and all or some of its widgets.  
    The key is that a container needs to be added first.  
    
    The major containers are Container, Column, and Row.  A Container can 
    have only one widget.  Therefore, if you have more than one, you need
    to add them to a Column or a Row first and then place the Column or
    Row into the Container, if needed.

    There are some non-obvious containers like Scrollable.  It's not only a 
    container but a widget too, because it has callbacks. 

    Besides width and height, the Container defaults to centering which
    aligns the item in the center of the Container.  This is very handy
    for the centering of your Column or Row.  Other options are available.

    A Column aligns your widget vertically.  So as you add widgets, they are
    placed top to bottom.  The Column has a spacing parameter but you can add
    the spacing widget, if you have other spacing requirements.

    A Row is like the Column except it aligns the widgets horizontally.
    As you place your widgets into a row, they are placed left to right.

    The alignment depends a lot on the width and height of the container.
    The 3 basic options for setting the width and height are:
        1. Shrink (default): container shrinks to the size of the largest widget. 
        2. Specific value using a float.
        3. Setting the width_fill  or height_fill parameters to True which 
            overrides the float, fills available space container its in.

    The interaction of the above setting can be a bit difficult to figure out
    sometimes.  However, by using the debug=True option in the window parameters, 
    you will be able to see how the layout looks.  If you don't see your widget
    on the screen, its because certain combinations cause the fill to
    exceed the windows width or height and your widgets are off screen.
    I find placing everything into a Container and centering usually
    brings it back on the screen.  You can also set a specific width
    and height to help you figure how things are placed.  
    
    The nice thing about using fill as much as possible is that when you
    resize your your window, everything resizes and you don't have to
    go back and recalculate your sizes based on the window size.
    In some cases you will need to do this so use the window
    on_resize to get the width and height then recalculate your sizes
    as needed in your callback using the update_item.
    
    If you hide a widget, currently a small placeholder remains
    future additions will add hidden with and without a placeholder.

    A big part of constructing your gui layout is using the id's of
    the widgets, containers, and windows.  The ids are a central part of
    how IPG operates.  During the execution of the python program,
    the functions are called in rust and a structure having all of the
    necessary information for each widget is stored in a global list
    based on the window id and the id of all the widgets.  
    
    Once the session is started, Iced is started up, the empty windows are
    created, unique container ids are determined, and a recursive function
    processes all of the nested children.  
    
    When the window needs to be updated, the update routine in Iced 
    determines which widget type needs to be updated by processing a generic 
    enum structure and then the widget type module is called.
     
    The module for the widget type determines which widget needs changing
    based on the id, makes changes and returns to the iced update function
    which sends any new messages and/or updates the windows.

    Since the window_id, container_id, and parent_id are strings, typo's
    can occur throughout your program and changing them can be tedious 
    for large programs.  Therefore for larger programs, I prefer to assign
    my ids in the class such as self.wnd_1: str="window_id_1",  for example.
    Then your IDE will supply a dropdown of your variables and hopefully
    reduce typos.  If you group them together properly, you might find
    that your naming could be improved and easily changed.

    Callbacks are the only way to update your windows, as discussed above,
    Iced uses a messaging system and these are processed and sent back to 
    python by calling the specified function set by the user.  For example,
    a button has an on_press=user supplied function, on_press=button_pressed.
    
    The returning callback data varies depending on the widget.  
    For example, a button has no data so the callback only sends back 
    an id of the button.  A color_picker sends back a list of the rgba 
    values and so on.
    
    The callbacks, as you'll see in the below program, have up to 3 returning
    pieces of data, widget id, some data, and user_data.  Keep in mind that the id is the
    id of the calling widget, which may or may not be the id you want to use
    for updating an item.  Try not to use the term id in the parameter list because
    that is a python reserved name.  Also, name the id after the calling widget so that
    you remember what the widget is and if that's the id you want to use.

    For example, if you have a callback for the button widget and want to change
    a text widget to read "Button Pressed" then to update the text widget,
    you'll need the text widget id.  You can get this by equating the text
    widget to a variable which you would use as the id in update_item.

    def create_button_and_text():
        ipg.add_button(parent_id="col", label="Press Me", on_press=button_pressed)
        text_id = ipg.add_text(parent_id="col", content="Some text")

    Your callback function
    def button_pressed(btn_id: int):
        ipg.update(text_id, IpgTextParam.Content, "Button was Pressed")

    In this callback function you only have one returning parameter, btn_id.
    Most other widgets have a second data parameter.  The user_data parameters 
    depends on if you use the user_data option on the widget.  If you don't use 
    the user_data option,make sure and not to put that parameter in the callback function 
    or you'll get an error.  You'll also get an error if you use the user_data and forget 
    to add that parameter to your callback function.  The names of the parameters
    can be whatever you like, the order is the most important:
    calling widget id, data(if present), user_data(if present).

    It's important to look through all the demos to get a feel for how things operate.
    I tried to vary up things to include different ideas.  However, a demo doesn't
    really do much just use a lot of text widgets to show the results.  Give it a try with
    with a real program and let me know through the git repository or discord if you have problems, 
    questions, or suggestions.
    
    Have fun with IPG!! 
"""


class Demo:
    def __init__(self) -> None:
        self.ipg=IPG()  # initialize IPG, must use

        # window ids
        self.wnd_1: str="main_1"
        self.wnd_2: str="main_2"

        # containers for window 1
        self.row_1: str="row_1"
        self.l_col_1: str="left_col_1"
        self.r_col_1: str="right_col_1"

        # widgets in window 1
        # 0 is not a valid id so if not initialized,
        # you'll get an error of not finding
        # the id.
        self.btn_id: int=0
        self.button_presses: int=0
        self.btn_text_id: int=0
        self.text_id_chkbox: int=0
        self.bar_id: int=0
        self.slider_text_id: int=0
        self.picklist_text_id: int=0
        self.radio_1_text_id: int=0
        self.radio_2_text_id: int=0
        self.selectable_text_id: int=0
        self.text_input_id: int= 0

        # containers for window 2
        self.l_col_2: str="left_col_2"
        self.r_col_2: str="right_col_2"

        # Widgets in window 2
        self.date_text_id: int=0

    def start_gui(self):
        self.construct_window_1()
        self.construct_button()
        self.construct_checkbox()
        self.construct_slider_and_progress_bar()
        self.construct_pick_list()
        self.construct_radio_buttons_v()
        self.construct_radio_buttons_h()
        self.construct_selectable_text()
        self.construct_text_input()

        self.construct_window_2()
        self.construct_date_picker()
        self.construct_table()

        # required to be last executed
        self.ipg.start_session()

    def construct_window_1(self):
        self.ipg.add_window(
                    window_id=self.wnd_1, 
                    title="Demo Window 1 - Iced Wrapped in Python",
                    width=500, 
                    height=500, 
                    pos_x=100, 
                    pos_y=25)

        # Add container to center everything
        self.ipg.add_container(
                    window_id=self.wnd_1,
                    container_id="cont1",
                    width_fill=True,
                    height_fill=True)
        
        # add row to hold the 2 columns
        self.ipg.add_row(
                    window_id=self.wnd_1, 
                    container_id=self.row_1,
                    parent_id="cont1")
        
        # This column will be on the left side
        self.ipg.add_column(
                    window_id=self.wnd_1, 
                    container_id=self.l_col_1, 
                    parent_id=self.row_1)
        
        # This column will be on the right side
        self.ipg.add_column(
                    window_id=self.wnd_1, 
                    container_id=self.r_col_1, 
                    parent_id=self.row_1)

    # A button is defined, a text is defined with info.
    # The callback function follows where the content of the text
    # is replaced by the user_data.
    def construct_button(self):
        self.btn_id = self.ipg.add_button(
                                parent_id=self.l_col_1, 
                                label="Press Me!",
                                on_press=self.button_pressed)
        
        self.btn_text_id = self.ipg.add_text(
                                    parent_id=self.l_col_1,
                                    content=f"A text can count too {self.button_presses}")

    def button_pressed(self, btn_id: int):
        self.button_presses += 1
        self.ipg.update_item(
                    wid=btn_id, 
                    param=IpgButtonParam.Label, 
                    value=f"You Pressed {self.button_presses} times!")
        
        self.ipg.update_item(
                    wid=self.btn_text_id, 
                    param=IpgTextParam.Content,
                    value=f"A text can count too: {self.button_presses} times!")

    # A checkbox is defined and a text is defined with the show value of False.
    # Unlike the button above, in this case we hid the text and then will show it
    # when the box is checked.
    def construct_checkbox(self):
        self.ipg.add_checkbox(
                    parent_id=self.l_col_1, 
                    label="Check Me",
                    on_toggle=self.box_checked_id)

        self.text_id_chkbox = self.ipg.add_text(
                                        parent_id=self.l_col_1,
                                        content="You Checked the box above",
                                        show=False)  # note: show is False

    def box_checked_id(self, _chk_id: int, data: bool):
        self.ipg.update_item(
                    wid=self.text_id_chkbox, 
                    param=IpgTextParam.Show, 
                    value=data)  # show set to True

    # a progress bar and a slider are defined and connected together via the callbacks
    def construct_slider_and_progress_bar(self):
        self.bar_id = self.ipg.add_progress_bar(
                                parent_id=self.l_col_1, 
                                min=0.0, 
                                max=100.0,
                                value=50.0, 
                                width_fill=True)

        self.ipg.add_slider(
                    parent_id=self.l_col_1, 
                    min=0.0, 
                    max=100.0,
                    step=1.0, 
                    value=50.0, 
                    width_fill=True,
                    on_change=self.slider_on_change,
                    on_release=self.slider_on_release)

        self.slider_text_id = self.ipg.add_text(
                                    parent_id=self.l_col_1, 
                                    content="Slider content here.")

    # Both callbacks were used in this case for demonstration but it is
    # expected that you probably only will use the release mostly.
    # if you have a costly calculation you are using, you may want to
    # not use the on_change or filter it by using a counter to select
    # only a few changes.
    def slider_on_change(self, _slider_id: int, data: float):
        self.ipg.update_item(
                    wid=self.slider_text_id, 
                    param=IpgTextParam.Content, 
                    value=f"Slide = {data}")
        
        self.ipg.update_item(
                    wid=self.bar_id, 
                    param=IpgProgressBarParam.Value, 
                    value=data)

    def slider_on_release(self, slider_id, data: float):
        print(slider_id, data)
        
        
        # A picklist is defined here width a place holder. The option list holder the selections.
    def construct_pick_list(self):
        self.ipg.add_pick_list(
                    parent_id=self.l_col_1, 
                    options=["one", "two", "three"],
                    on_select=self.picked_item,
                    placeholder="Choose a string number")

        self.picklist_text_id = self.ipg.add_text(
                                        parent_id=self.l_col_1, 
                                        content="You picked:")

    def picked_item(self, id: int, data: str):
        self.ipg.update_item(
                    wid=self.picklist_text_id, 
                    param=IpgTextParam.Content, 
                    value=f"You Picked: {data}")

    # *****************Right Column in Window 1*************************
    # Two groups of radio buttons are defined, one vertical and one horizontal
    # Currently there is a limit of 26 buttons per group.
    # This set of radio  buttons will be vertical
    def construct_radio_buttons_v(self):
        labels = ["Radio A", "Radio B", "Radio C"]
        self.ipg.add_radio(
                    parent_id=self.r_col_1, 
                    labels=labels, 
                    on_select=self.radio_selected_v)

        self.radio_1_text_id = self.ipg.add_text(
                                    parent_id=self.r_col_1, 
                                    content="You selected:")

    # The radio on_select returns a tuple (index, label)
    def radio_selected_v(self, _radio_id: int, data: str):
        self.ipg.update_item(
                    wid=self.radio_1_text_id, 
                    param=IpgTextParam.Content, 
                    value=f"You selected: {data}")

    # This set of radio buttons will be horizontal
    def construct_radio_buttons_h(self):
        self.ipg.add_radio(
                    parent_id=self.r_col_1, 
                    labels=["A", "B", "C"],
                    direction=IpgRadioDirection.Horizontal,
                    on_select=self.radio_selected_h)

        self.radio_2_text_id = self.ipg.add_text(
                                    parent_id=self.r_col_1, 
                                    content="You selected:")

        # The radio on_select returns a tuple (index, label)
    def radio_selected_h(self, _radio_id: int, data: str):
        self.ipg.update_item(
                    wid=self.radio_2_text_id, 
                    param=IpgTextParam.Content, 
                    value=f"You selected: {data}")

        # A button style can act as a selectable text but has only one callback.
        # A selectable text has a number of different callbacks for all the mouse buttons and
        # mouse enter and exit.

    def construct_selectable_text(self):
        self.ipg.add_selectable_text(
                    self.r_col_1, "My Selectable Text",
                    on_press=self.selecting_text,
                    on_release=self.selecting_text,
                    on_middle_press=self.selecting_text,
                    on_middle_release=self.selecting_text,
                    on_right_press=self.selecting_text,
                    on_right_release=self.selecting_text,
                    on_enter=self.selecting_text,
                    on_move=self.selecting_text_with_point,
                    on_exit=self.selecting_text
                    )

        self.selectable_text_id = self.ipg.add_text(
                                        parent_id=self.r_col_1, 
                                        content="Selectable actions:")

    def selecting_text(self, sel_txt_id):
        self.ipg.update_item(
                    wid=self.selectable_text_id, 
                    param=IpgTextParam.Content, 
                    value=f"Selectable id: {sel_txt_id}")

    def selecting_text_with_point(self, _sel_txt_id, data):
        x = round(data[1], 1)
        y = round(data[3], 1)
        self.ipg.update_item(
                    wid=self.selectable_text_id, 
                    param=IpgTextParam.Content, 
                    value=f"point: x={x} y={y}")

    def construct_text_input(self):
        self.ipg.add_text_input(
                    parent_id=self.r_col_1,
                    placeholder="My Placeholder",
                    width=200.0,
                    on_submit=self.text_input_submitted,
                    on_input=self.text_on_input)

        self.text_input_id = self.ipg.add_text(
                                parent_id=self.r_col_1, 
                                content="Will fill while typing")

        # Only one callback used in this case (two could be used). 
        # Determining which callback is based on name.
        # Maybe helpful in some cases where callbacks are similar or there are many.
    def text_input_submitted(self, _input_id, data: str):
        self.ipg.update_item(
                    wid=self.text_input_id, 
                    param=IpgTextParam.Content,
                    value=f"You submitted: {data}")

    def text_on_input(self, _input_id, data: str):
        self.ipg.update_item(
                    wid=self.text_input_id, 
                    param=IpgTextParam.Content,
                    value=f"Adding while typing: {data}")

    # **********************window_2*****************************************************

    def construct_window_2(self):
        self.ipg.add_window(
                    window_id=self.wnd_2, 
                    title="Demo Window 2 - Iced Wrapped in Python",
                    width=600, 
                    height=500,
                    pos_x=650, 
                    pos_y=25)

        # Add container to center everything
        self.ipg.add_container(
                    window_id=self.wnd_2,
                    container_id="cont2",
                    width_fill=True,
                    height_fill=True)

        # add a column to hold multiple widgets
        self.ipg.add_column(
                    window_id=self.wnd_2, 
                    container_id=self.l_col_2,
                    parent_id="cont2",
                    width_fill=True, 
                    align=IpgAlignment.Center)

    # A date picker is defined and the results are put in a text widget.
    def construct_date_picker(self):
        self.ipg.add_date_picker(
                    parent_id=self.l_col_2, 
                    on_submit=self.date_selected)

        self.date_text_id = self.ipg.add_text(
                                    parent_id=self.l_col_2,
                                    content="")

    def date_selected(self, _date_id, date: str):
        self.ipg.update_item(
                    wid=self.date_text_id, 
                    param=IpgTextParam.Content, 
                    value=f"You selected: {date}")

    def construct_table(self):
        # define the column widths
        column_widths = [100.0] * 4
        # create the data dictionary
        data = {
            "str": ["H", "e", "l", "l", "o", " ", "W", "o", "r", "l", "d"],
            "one": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0],
            "two": [2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22],
            "three": [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33],
            }

        # make the dataframe
        df = pl.DataFrame(data)
        
        # Add the table.
        self.ipg.add_table(
                window_id=self.wnd_2,
                table_id="table",
                polars_df=df,
                parent_id=self.l_col_2,
                column_widths=column_widths,
                height=150.0,
                custom_footer_rows=1,
                )

        footer = ["This", "is", "a", "footer"]
        
        for i in range(len(footer)):
            self.ipg.add_text(
                    parent_id="table",
                    content=footer[i],
                    size=14.0)

    def widget_button(self, tbl_id: int, wid_index: tuple[int, int]):
        print(tbl_id, wid_index)

    def widget_checkbox(self, tbl_id: int, wid_index: tuple[int, int], is_checked: bool):
        print(tbl_id, wid_index, is_checked)

    def on_text_enter(self, tbl_id, text_index: tuple[int, int]):
        print(tbl_id, text_index)



demo = Demo()
demo.start_gui()
