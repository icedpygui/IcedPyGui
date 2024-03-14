from icedpygui.icedpygui import IPG

import random


"""
    IcedPyGui is based on Rust Iced gui at https://github.com/iced-rs/iced.
    Some code is used from Iced_aw at https://github.com/iced-rs/iced_aw.
    Pyo3 is used as the python wrapper at https://github.com/pyo3/pyo3.
    Maturin is used uild and publish the module at https://github.com/PyO3/maturin.
    
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
    The user data is special becuase it is only passed through to rust and 
    back out as a PyObject or PyAny.  Therefore any python data can be used
    since it is never extracted into a rusr type.     
    
    A few simple rules need to be followed.   
        import IPG as indicated above in this demo.      
        
        The last line of code to execute must be ipg.start_seesion().
        Any code after that will not be executed because rust Iced is now
        running.  You can place it anywhere, just make sure its last executed.
        If you start your program and nothing happens, it might mean that
        you aren't executing start_session() or you forgot to add it in.    
        
        Ever widget needs to have a parent container previously defined and 
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

    Besides width and height, the Container has a centering option which
    aligns the item in the center of the Container.  This is very handy
    for the centering of your Column or Row.

    A Column aligns your widget vetically.  So as you add widgets, they are
    placed top to bottom.  The Column has spacing options and you can add
    the spacing widget, if you have other spacing requirements.

    A Row is like the Column except it aligns the widgets horizontally.
    As you place your widgets into a row, they are placed left to right.

    A Column and Row have alignment options, start, center, and end.
    Start and center are obvious but end means basically align right 
    for Row or align at the bottom for Column.

    The alignment depends a lot on the width and height of the container.
    The 3 basic options for setting the width and height are:
        1. Shrink (default): container shrinks to the size of the largest widget. 
        2. Specific value using a float.
        3. Setting the width_fill  or height_fill parameters to True which 
            overides the float, fills available space container its in.

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
    python by calling the specified function set by the user.  
    
    The returning callback data varies depending on the widget.  
    For example, a button has no data so the callback only sends back 
    an id of the button and the name of the callback.  
    A color_picker sends back a list of the rgba values and so on.
    These are documented but if you are unsure or the docs are behind
    in updating, just print the data and see what it looks like.
    
    The callbacks, as you'll see in the below program, have up to 4 returning
    pieces of data, id, name, data, user_data.  Keep in mind that the id is the
    id of the calling widget, which may or may not be the id you want to use
    for updating an item.

    For example, if you have a callback for the button widget and want to change
    a text widget to read "Button Pressed" then to update the text widget,
    you'll need the text widget id.  You can get this by equating the text
    widget to a variable which you would use as the id in update_item.

    def button():
        ipg.add_button(parent_id="col", "Press Me", on_press=button_pressed)
        text_id = ipg.add_text(parent_id="col", content="Some text")

    def button_pressed(id, name):
        ipg.update(id=text_id, param="content", value="Button Pressed")

    In your callback function you must have a minimum of 3 returning parameters,
    id, name, and data, excpet for widgets like button and selectable_text
    which have no data.  The user_data parameters depends on if you use the 
    user_data option on the widget.  If you don't use the user_data option,
    make sure and not put that parameter in the callback function or you'll 
    get an error.  You'll also get an error if you use the user_data and forget 
    to add that parameter to your callback function.  The names of the parameters
    can be whatever you like.

    Have fun with the Demo!! 
"""


class Demo:
    def __init__(self) -> None:
        self.ipg = IPG() # initialize IPG, must use
        
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
        self.bar_id: int= 0
        self.slider_text_id: int=0
        self.picklist_text_id: int=0
        self.radio_1_text_id: int=0
        self.radio_2_text_id: int=0
        self.selectable_text_id: int=0
        
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

        
        self.ipg.start_session()
        

    def construct_window_1(self):
        
        self.ipg.add_window(self.wnd_1, "Demo Window 1 - Iced Wrapped in Python", 
                                                width=600, height=500, 
                                                pos_x=150, pos_y=100)
        
        self.ipg.add_row(self.wnd_1, container_id=self.row_1, width_fill=True, height_fill=True)
        self.ipg.add_column(self.wnd_1, container_id=self.l_col_1, parent_id=self.row_1)
        self.ipg.add_column(self.wnd_1, container_id=self.r_col_1, parent_id=self.row_1)


        # A button is defined, a text is defined with info.
        # The callback function follows where the content of the text
        # is replaced by the user_data.
    def construct_button(self):
        self.btn_id = self.ipg.add_button(parent_id=self.l_col_1, label="Press Me!",
                                    on_press=self.button_pressed)
        self.btn_text_id = self.ipg.add_text(self.l_col_1, 
                                             f"A text can count too {self.button_presses}")
    
    def button_pressed(self, id, name):
        self.button_presses += 1
        self.ipg.update_item(self.btn_id, "label", f"You Pressed {self.button_presses} times!")      
        self.ipg.update_item(self.btn_text_id, "content", f"A text can count too: {self.button_presses} times!")

        # A checkbox is defined and a textis defined with the show value of False.
        # Unlke the button above, in this case we hid the text and then will show it
        # when the box is checked.
    def construct_checkbox(self):
        self.ipg.add_checkbox(parent_id=self.l_col_1, label="Check Me", 
                                        on_checked=self.box_checked_id)
        
        self.text_id_chkbox = self.ipg.add_text(parent_id=self.l_col_1, 
                                                    content="You Checked the box above", 
                                                    show=False) # note: show is False
    def box_checked_id(self, id, name, data):
            self.ipg.update_item(self.text_id_chkbox, "show", data) # show set to True

        # a progress bar and a slider are defined and connected together via the callbacks
    def construct_slider_and_progress_bar(self):
        self.bar_id = self.ipg.add_progress_bar(parent_id=self.l_col_1, min=0.0, max=100.0, 
                                           value=50.0, width_fill=True)
        
        self.ipg.add_slider(parent_id=self.l_col_1, min=0.0, max=100.0, 
                                    step=1.0, value=50.0, width_fill=True, 
                                    on_change=self.slider_on_change, 
                                    on_release=self.slider_on_release)
    
        self.slider_text_id = self.ipg.add_text(self.l_col_1, "Slider content here.")

        # Both callbacks were used in this case for demeonstration but it is
        # expected that you probably only will use the release mostly.
        # if you have a costly calculation you are using, you may want to
        # not use the on_change or filter it by using a counter to select
        # only a few changes. 
    def slider_on_change(self, id, name, data):
        self.ipg.update_item(self.slider_text_id, "content", f"Slide = {data}")
        self.ipg.update_item(self.bar_id, "value", data)
    
    def slider_on_release(self, id, name, data):
        self.ipg.update_item(self.bar_id, "value", data)    

        # A picklist is defined here width a place holder. The option list holder the selections.
    def construct_pick_list(self):
        self.ipg.add_pick_list(parent_id=self.l_col_1, options=["one", "two", "three"], 
                               callback=self.picked_item, 
                                placeholder="Choose a string number")

        self.picklist_text_id = self.ipg.add_text(self.l_col_1, "You picked:")

    def picked_item(self, id, name, data):
        self.ipg.update_item(self.picklist_text_id, "content", f"You Picked: {data}")

    # *****************Right Column in Window 1*************************
        # Two groups of radio buttons are defined, one vertical and one horizontal
        # Currently there is a limit of 26 buttons per group.
        # This set of radio  buttons will be vertical
    def construct_radio_buttons_v(self):
        labels = ["Radio A", "Radio B", "Radio C"]
        self.ipg.add_radio(parent_id=self.r_col_1, labels=labels, on_select=self.radio_selected_v)

        self.radio_1_text_id = self.ipg.add_text(self.r_col_1, "You selected:")

        # The radio on_select returns a tuple (index, label)
    def radio_selected_v(self, id, name, data):
        self.ipg.update_item(self.radio_1_text_id, "content", f"You selected: {data}")

    # This set of radio buttons will be horizontal
    def construct_radio_buttons_h(self):        
        self.ipg.add_radio(parent_id=self.r_col_1, labels=["A", "B", "C"],
                           direction="horizontal", 
                           on_select=self.radio_selected_h)

        self.radio_2_text_id = self.ipg.add_text(self.r_col_1, "You selected:")

        # The radio on_select returns a tuple (index, label)
    def radio_selected_h(self, id, name, data):
        self.ipg.update_item(self.radio_2_text_id, "content", f"You selected: {data}")

        # A button style can act as a selectable text but has only one callback.
        # A selectable text has a number of different callbacks for all the mouse buttons and
        # mouse enter and exit.
    def construct_selectable_text(self):
        self.ipg.add_selectable_text(self.r_col_1, "My Selectable Text", 
                                     on_press=self.selecting_text,
                                     on_release=self.selecting_text,
                                     on_middle_press=self.selecting_text,
                                     on_middle_release=self.selecting_text,
                                     on_right_press=self.selecting_text,
                                     on_right_release=self.selecting_text)
        
        self.selectable_text_id = self.ipg.add_text(self.r_col_1, "Selectable actions:")

    def selecting_text(self, id, name):
        self.ipg.update_item(self.selectable_text_id, "content", f"Selectable action: \n{name}")

    def construct_text_input(self):
        self.ipg.add_text_input(parent_id=self.r_col_1, 
                                placeholder="My Placeholder",
                                width=200.0, 
                                on_submit=self.text_input_submitted,
                                on_input=self.text_input_submitted)
    
        self.text_input_id = self.ipg.add_text(self.r_col_1, "Will fill while typing")

        # Only one callback used in this case (two could be used). Determing which callback is based on name.
        # Maybe helpful in some cases where callbacks are similar or there are many.
    def text_input_submitted(self, id, name, data):
        if name == "on_input":
            self.ipg.update_item(self.text_input_id, "content", 
                                 f"Adding while typing: {data}")
        else:
            self.ipg.update_item(self.text_input_id, "content", 
                                 f"You submitted: {data}")

    #**********************window_2***************************************************** 
    def construct_window_2(self):
        self.ipg.add_window(self.wnd_2, "Demo Window 2 - Iced Wrapped in Python", 
                                                width=500, height=500, 
                                                pos_x=800, pos_y=100)
        
        self.ipg.add_column(window_id=self.wnd_2, container_id=self.l_col_2, 
                            width_fill=True, align_items="center")
        
        # A date picker is defined and the results are put in a text widget.
    def construct_date_picker(self):
        self.ipg.add_date_picker(self.l_col_2, on_select=self.date_selected)

        self.date_text_id = self.ipg.add_text(self.l_col_2, 
                                              "You selected:") 

    def date_selected(self, id, name, date):
         self.ipg.update_item(self.date_text_id, "content", f"You selected: {date}")

        # A table is defined with 4 columns of random items.
        # Rust does not allow types to be mixed in a list.
        # Therefore, if a mixed list is needed, convert it to a list[str].
        # The gui converts the list to strings anyway.
        #  Width and height are reuired for the table.
    def construct_table(self):
        
        col1: list[float] = []
        col2: list[str] = []
        col3: list[int] = []
        col4: list[bool] = []

        for i in range(0, 25):
            # make a float random number
            col1.append(random.randrange(10, 99) + random.randrange(10, 99)/100)
            col2.append(random.choice(["one", "two", "3", "four", "5", "six", "seven"]))
            col3.append(random.randrange(10, 99))
            col4.append(random.choice([True, False]))

        # Note this is a list of dictionaries not a single dictionary
        data = [
                {"Floats": col1},
                {"Strings": col2},
                {"Integers": col3},
                {"Booleans": col4}
                ]

        table_id = self.ipg.add_table(parent_id=self.l_col_2, 
                                      title="My Table", data=data,
                                      width=450.0, height=300.0)
        

demo = Demo()
demo.start_gui()

