from icedpygui.icedpygui import IPG


class CheckboxDemo:
    def __init__(self) -> None:
        self.ipg = IPG()

        # ids
        self.wnd_id: str="main"
        self.col: str="col"

        self.text_id: int=0
        self.text_user_data_id: int=0
        self.width_id: int=0
        self.size_id: int=0
        self.spacing_id: int=0
        self.txt_size_id: int=0
        self.tlh_id: int=0
        self.show_id: int=0

    # sets the gui up and starts the session.
    def setup_gui(self):
        # The window must be the first item added.  More than one window can be added
        # now or at other times.
        self.ipg.add_window(self.wnd_id, "CheckBox Demo", 
                                800, 800, 500, 100)
        
        self.ipg.add_container(window_id=self.wnd_id, container_id="cont", width_fill=True,
                               height_fill=True, align_x="center", align_y="center")

        # A coulmn is added for the widgets.  Widgets cannot be added directly to windows.
        self.ipg.add_column(window_id=self.wnd_id, container_id=self.col, parent_id="cont")
        
        self.ipg.add_text(parent_id=self.col, content="CHECK THE CHECKBOX TO SEE THE CHANGES")

        # All widgets that cause an action can carry user data to the callback method.
        # You can send data types a number of different data types.  
        # See the item_update example for more info.
        self.ipg.add_checkbox(parent_id=self.col, label="Check Me!!!", 
                                                        on_checked=self.on_checked,
                                                        user_data="Some string data")
        
        # These text widgets will show some content when the checkbox is checked.
        # Currently they show a placeholder of an empty string.  You could have set the 
        # show parameter to false also and have the same effect, at least for the text widget.

        self.text_id = self.ipg.add_text(parent_id=self.col, content="The callback name is show here")
        self.text_user_data_id = self.ipg.add_text(parent_id=self.col, content="The callback user data will show here")

        self.width_id = self.ipg.add_checkbox(self.col, label="Checkbox width will change")

        self.size_id = self.ipg.add_checkbox(self.col, label="Checkbox size will change")

        self.spacing_id = self.ipg.add_checkbox(self.col, label="Checkbox spacing will change")

        self.txt_size_id = self.ipg.add_checkbox(self.col, label="Checkbox text size will change")

        self.tlh_id = self.ipg.add_checkbox(parent_id=self.col, label="Line Height", text_line_height=1.3)

        self.show_id = self.ipg.add_checkbox(parent_id=self.col, label="This checkbox will be hidden")


        self.ipg.start_session()

    def on_checked(self, id, name, is_checked, user_data):
        # This is the callback for the checkbox.  The id parameter is the id of the checkbox
        # that is checked, so if other widgets need to be manipulated, as in this case, 
        # you must obtain their ids by equating them and storing them in a class or using them globally.  
        # To update a widget, you provide the parameter name as a string, followed by the value.
        # The type of the value must correspond to the parameter type.
        
        if is_checked:
            # changing the checkbox label.  Since this was the checkbox that was checked,
            # the id of the callback is the id to use for updating.
            self.ipg.update_item(id, "label", "Checkbox checked and label changed")

            self.ipg.update_item(self.text_id, "content", 
                                            f"Callback name = {name}     is_checked = {is_checked}")
            self.ipg.update_item(self.text_user_data_id, "content", value=f"user data = {user_data}")

            # Changing the width of the label with a tuple (string, float) value
            self.ipg.update_item(self.width_id, "width", 200.0)
            self.ipg.update_item(self.width_id, "label", "Width Shrank and label wrapped")

            # Changing the size of the box with a float value
            self.ipg.update_item(self.size_id, "size", 50.0)
            self.ipg.update_item(self.size_id, "label", "Size Changed of box")

            # Changing the spacing between box and label with a float value
            self.ipg.update_item(self.spacing_id, "spacing", 100.0)
            self.ipg.update_item(self.spacing_id, "label", "Spacing Changed")

            # Changing the text_size with a float value
            self.ipg.update_item(self.txt_size_id, "text_size", 20.0)
            self.ipg.update_item(self.txt_size_id, "label", "Text size Changed")

            # Changing the text_line_height which increases space around the text label.
            self.ipg.update_item(self.tlh_id, "text_line_height", 5.0)

            # TODO text_shaping

            # TODO Changing the style
            

            # Hide the checkbox
            self.ipg.update_item(self.show_id, "show", False)

        if not is_checked:

            self.ipg.update_item(id, "label", "Check Me Again!!!")
        
            self.ipg.update_item(self.text_id, "content", "The callback name is show here")
            
            self.ipg.update_item(self.text_user_data_id, "content", value="The callback user data will show here")

            # Changing the width of the label with a float value
            self.ipg.update_item(self.width_id, "width", None)
            self.ipg.update_item(self.width_id, "label", "Checkbox width will change")

            # Changing the size of the box with a float value
            self.ipg.update_item(self.size_id, "size", 16.0)
            self.ipg.update_item(self.size_id, "label", "Checkbox size will change")

            # Changing the spacing between box and label with a float value
            self.ipg.update_item(self.spacing_id, "spacing", 15.0)
            self.ipg.update_item(self.spacing_id, "label", "Checkbox spacing will change")

            # Changing the text_size with a float value
            self.ipg.update_item(self.txt_size_id, "text_size", 16.0)
            self.ipg.update_item(self.txt_size_id, "label", "Checkbox text size will change")

            # Changing the text_line_height which increases space around the text label.
            self.ipg.update_item(self.tlh_id, "text_line_height", 1.3)

            # TODO text_shaping

            # TODO Changing the style
            

            # Hide the checkbox
            self.ipg.update_item(self.show_id, "show", False)

demo = CheckboxDemo()
demo.setup_gui()
