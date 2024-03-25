from icedpygui import IPG, IpgCheckboxUpdate, IpgTextUpdate


class CheckboxDemo:
    def __init__(self) -> None:
        self.ipg = IPG()

        # ids
        self.wnd_id: str="main"
        self.col: str="col"

        self.chk_main: int=0
        self.checked_id: int=0
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
        self.ipg.add_column(window_id=self.wnd_id, container_id=self.col, parent_id="cont",
                            width=400.0, height=800.0)
        
        self.ipg.add_text(parent_id=self.col, content="CHECK THE CHECKBOX TO SEE THE CHANGES")

        # All widgets that cause an action can carry user data to the callback method.
        # You can send data types a number of different data types.  
        # See the item_update example for more info.
        self.chk_main = self.ipg.add_checkbox(parent_id=self.col, label="Check Me!!!",
                                                        on_toggle=self.on_toggled,
                                                        user_data="Some string data")
        
        # These text widgets will show some content when the checkbox is checked.
        # Currently they show a placeholder of an empty string.  You could have set the 
        # show parameter to false also and have the same effect, at least for the text widget.

        self.text_id = self.ipg.add_text(parent_id=self.col, content="The callback name is show here")

        self.text_user_data_id = self.ipg.add_text(parent_id=self.col, content="The callback user data will show here")

        self.checked_id = self.ipg.add_checkbox(self.col, label="My icon will change", 
                                                is_checked=True)

        self.width_id = self.ipg.add_checkbox(self.col, label="Checkbox width will change")

        self.size_id = self.ipg.add_checkbox(self.col, label="Checkbox size will change")

        self.spacing_id = self.ipg.add_checkbox(self.col, label="Checkbox spacing will change")

        self.txt_size_id = self.ipg.add_checkbox(self.col, label="Checkbox text size will change")

        self.tlh_id = self.ipg.add_checkbox(parent_id=self.col, label="Text Line Height will change", text_line_height=1.3)

        self.show_id = self.ipg.add_checkbox(parent_id=self.col, label="This checkbox will be hidden")
        self.hide_text = self.ipg.add_text(self.col, "")


        self.ipg.start_session()

    def on_toggled(self, id, is_checked, user_data):
        # This is the callback for the checkbox.  The id parameter is the id of the checkbox
        # that is checked, so if other widgets need to be manipulated, as in this case, 
        # you must obtain their ids by equating them and storing them in a class or using them globally.  
        # To update a widget, you provide the parameter name as a string, followed by the value.
        # The type of the value must correspond to the parameter type.
        # Only function with the starting anme of update_ can be used in a callback.
        
        if is_checked:
            # changing the checkbox label.  Since this was the checkbox that was checked,
            # the id of the callback is the id to use for updating.
            self.ipg.update_item(self.chk_main, IpgCheckboxUpdate.Label, "Check Me Again!!!")

            self.ipg.update_item(self.text_id, IpgTextUpdate.Content, 
                                            f"is_checked = {is_checked}")
            self.ipg.update_item(self.text_user_data_id, IpgTextUpdate.Content, 
                                            value=f"user data = {user_data}")

            # changing the icon to a x
            self.ipg.update_item(self.checked_id, IpgCheckboxUpdate.IconX, True)
            self.ipg.update_item(self.checked_id, IpgCheckboxUpdate.Label, "My icon changed to a X")
            self.ipg.update_item(self.checked_id, IpgCheckboxUpdate.IsChecked, True)

            # Changing the width of the label with a tuple (string, float) value
            self.ipg.update_item(self.width_id, IpgCheckboxUpdate.Width, 200.0)
            self.ipg.update_item(self.width_id, IpgCheckboxUpdate.Label, "Width Shrank and label wrapped")
            self.ipg.update_item(self.width_id, IpgCheckboxUpdate.IsChecked, True)

            # Changing the size of the box with a float value
            self.ipg.update_item(self.size_id, IpgCheckboxUpdate.Size, 50.0)
            self.ipg.update_item(self.size_id, IpgCheckboxUpdate.Label, "Size Changed of box")
            self.ipg.update_item(self.size_id, IpgCheckboxUpdate.IsChecked, True)
            

            # Changing the spacing between box and label with a float value
            self.ipg.update_item(self.spacing_id, IpgCheckboxUpdate.Spacing, 100.0)
            self.ipg.update_item(self.spacing_id, IpgCheckboxUpdate.Label, "Spacing Changed")
            self.ipg.update_item(self.spacing_id, IpgCheckboxUpdate.IsChecked, True)

            # Changing the text_size with a float value
            self.ipg.update_item(self.txt_size_id, IpgCheckboxUpdate.TextSize, 20.0)
            self.ipg.update_item(self.txt_size_id, IpgCheckboxUpdate.Label, "Text size Changed")
            self.ipg.update_item(self.txt_size_id, IpgCheckboxUpdate.IsChecked, True)

            # Changing the text_line_height which increases space around the text label.
            self.ipg.update_item(self.tlh_id, IpgCheckboxUpdate.TextLineHeight, 5.0)
            self.ipg.update_item(self.tlh_id, IpgCheckboxUpdate.Label, "Text_line_Height: set window debug=True to see the size better")
            self.ipg.update_item(self.tlh_id, IpgCheckboxUpdate.IsChecked, True)

            # TODO text_shaping

            # TODO Changing the style
            

            # Hide the checkbox
            self.ipg.update_item(self.show_id, IpgCheckboxUpdate.Show, False)
            self.ipg.update_item(self.hide_text, IpgTextUpdate.Content, 
                                 "The checkbox above above me is hidden")

        if not is_checked:

            self.ipg.update_item(self.chk_main, IpgCheckboxUpdate.Label, "Check Me!!!")
        
            self.ipg.update_item(self.text_id, IpgTextUpdate.Content, 
                                                    "The callback name is show here")
            
            self.ipg.update_item(self.text_user_data_id, IpgTextUpdate.Content, 
                                        value="The callback user data will show here")

            # changing the icon to a check
            self.ipg.update_item(self.checked_id, IpgCheckboxUpdate.IconX, False)
            self.ipg.update_item(self.checked_id, IpgCheckboxUpdate.Label, "My icon will change")

            # Changing the width of the label with value=None which is shrink
            self.ipg.update_item(self.width_id, IpgCheckboxUpdate.Width, 0)
            self.ipg.update_item(self.width_id, IpgCheckboxUpdate.Label, "Checkbox width will change")
            self.ipg.update_item(self.width_id, IpgCheckboxUpdate.IsChecked, False)

            # Changing the size of the box with a float value
            self.ipg.update_item(self.size_id, IpgCheckboxUpdate.Size, 16.0)
            self.ipg.update_item(self.size_id, IpgCheckboxUpdate.Label, "Checkbox size will change")
            self.ipg.update_item(self.size_id, IpgCheckboxUpdate.IsChecked, False)

            # Changing the spacing between box and label with a float value
            self.ipg.update_item(self.spacing_id, IpgCheckboxUpdate.Spacing, 15.0)
            self.ipg.update_item(self.spacing_id, IpgCheckboxUpdate.Label, "Checkbox spacing will change")
            self.ipg.update_item(self.spacing_id, IpgCheckboxUpdate.IsChecked, False)

            # Changing the text_size with a float value
            self.ipg.update_item(self.txt_size_id, IpgCheckboxUpdate.TextSize, 16.0)
            self.ipg.update_item(self.txt_size_id, IpgCheckboxUpdate.Label, "Checkbox text size will change")
            self.ipg.update_item(self.txt_size_id, IpgCheckboxUpdate.IsChecked, False)

            # Changing the text_line_height which increases space around the text label.
            self.ipg.update_item(self.tlh_id, IpgCheckboxUpdate.TextLineHeight, 1.3)
            self.ipg.update_item(self.tlh_id, IpgCheckboxUpdate.Label, "Text_line_Height")
            self.ipg.update_item(self.tlh_id, IpgCheckboxUpdate.IsChecked, False)
            
            # TODO text_shaping

            # TODO Changing the style
            
            # Hide the checkbox
            self.ipg.update_item(self.show_id, IpgCheckboxUpdate.Show, True)
            self.ipg.update_item(self.hide_text, IpgTextUpdate.Content, "")

demo = CheckboxDemo()
demo.setup_gui()
