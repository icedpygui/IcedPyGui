from types import Callable, Union, List


class IPG:
    """
    Main class that is instantiated and that calls the corresponding rust file to implement the widgets
    """
    def __init__(self) -> None:
        ...

    def start_session(self) -> None:
        """
        Starts the gui session.  Must be the last called.
        """

    def generate_id(self) -> None:
        """
        Generates an id for some future widget
        """

    def add_window(self,
                    window_id: str,
                    title: str,
                    width: int,
                    height: int,
                    pos_x: float=None,
                    pos_y: float=None,
                    pos_centered: bool=False,
                    resizable:str="primary",
                    theme: str="dark",
                    debug: bool=False,
                    show: bool=True,
                    ) -> None:
        """
        Calls the rust lib main window function which starts the iced gui.
        This function needs to be placed at the end of the python program.

        Args:
            window_id (str, required): Window id used to place widget or container into.
            title (str, required): Title placed in the top bar of the main window.
            width (int, required): Width of the main window.
            height (int, required): Height of the main window.
            ** The window position will be defaulted to a system default unless position is set.
            pos_x (float, optional): x position of window
            pos_y (float, optional): y position of window
            pos_centered (bool, optional): default=False; Centered position of window.
            resizable (bool, optional): default=True; Whether the window can be resized.
            theme (str, optioonal): default=dark; Optional=white. More to come later.
            show (bool, optional): default=True; First window always true, others can be set.
            debug (bool, optional): default=False; Draws a box around widgets to see layout.
        """

    def add_container(self,
                        window_id: str,
                        container_id: str,
                        *,
                        parent_id: str=None,
                        width: float=None,
                        height: float=None,
                        width_fill: bool=False,
                        height_fill: bool=False,
                        max_height: float=float('inf'),
                        max_width: float=float('inf'),
                        align_x: str="Left",
                        align_y: str="Top",
                        padding: List=[10.0], 
                        show: bool=True,
                        ) -> int:
        """
        Adds a generic container to the gui
        **Note: A container unlike a row or column can only have 1 child.
                The container is used to help with widget alignments.
                You can align 1 widget or many if you add a row or column to the
                container and use the horizontal and vertical alignments.
        Args:
            window_id (str, required): Id of the window to place container in.
            container_id (str, required): id used by widgets for placement must be unique.
            parent_id (str, optional): If parent_id == window_id then not required, 
                                        if another container then required.
            width (int, optional): defailt=None; Width of the widget.
            height (int, optional): default=None; Height of the widget.
            width_fill (bool, optional): default=Shrink; Fills the parent container 
                                            or shrinks to wrap the child widgets.
            height_fill (bool, optional): default=Shrink; Fills the parent container 
                                            or shrinks to wrap the child widgets.
            max_width (float, optional): default=inf; Sets a max width.
            max_height (float, optional): default=inf; Sets a max height.
            align_x (str, optional): default="left"; "center", or "right"
            align_y (str, optional): default="top"; "center", or "bottom"
            padding ([float], optional): default=[10.0], Padding for container.
                                        use [float] for all sides,
                                        use [float, float] for [top&bottom, left&right]
                                        use [float, float, float, float] for [top, right, bottom, left]
            show (bool, Optional): default=True, Shows or hides container and all of its contents.
            
        Return: 
            int: internal id of container and can be used by user if equated.
        """

    def add_column(self,
                    window_id: str,
                    container_id: str,
                    *,
                    parent_id: str=None,
                    align_items: str="start",
                    width: float=None,
                    height: float=None,
                    width_fill: bool=False,
                    height_fill: bool=False,
                    max_width: float=float('inf'),
                    padding: List=[10.0], 
                    spacing: float=20.0,
                    show: bool=True,
                    ) -> int:

        """ Adds a Column which is a container that distributes its contents vertically
        
        Args:
            window_id (str, required): Id of the window to place container in.
            container_id (str, required): id used by widgets for placement must be unique.
            parent_id (str, optional): If parent_id == window_id then not required, 
                                        if another container then required.
            align_items (str, optional): "start"(default), "center", or "end".
            width (int, optional): defailt=None; Width of the widget.
            height (int, optional): default=None; Height of the widget.
            width_fill (bool, optional): default=Shrink; Fills the parent container 
                                            or shrinks to wrap the child widgets.
            height_fill (bool, optional): default=Shrink; Fills the parent container 
                                            or shrinks to wrap the child widgets.
            max_width (float, optional): default=float('inf'), Container used all of the available width.
            padding ([float], optional): default=[10.0], Padding for container.
                                        use [float] for all sides,
                                        use [float, float] for [top&bottom, left&right]
                                        use [float, float, float, float] for [top, right, bottom, left]
            spacing (float, optional): default=20.0, spacing between items in column.
                        user_id (str, optional): literal used to identify container instead of using the 
                                     returned integer, if variable is inconvenient.
            show (bool, Optional): default=True, Shows or hides container and all of its contents.
            
        Return: 
            int: internal id of container and can be used by user if equated
        """

    def add_row(self,
                window_id: str,
                container_id: str,
                *,
                parent_id: str=None,
                align_items: str= "Start",
                width: float=None,
                height: float=None,
                width_fill: bool=False,
                height_fill: bool=False,
                padding: List=[10.0], 
                spacing: float=20.0,
                show: bool=True,
                ) -> int:

        """
        Adds a row container to the gui.  Aligns widget horizontally.
        
        Args:
            window_id (str, required): Id of the window to place container in.
            container_id (str, required): id used by widgets for placement must be unique.
            parent_id (str, optional): If parent_id == window_id then not required, 
                                        if another container then required.
            align_items (str, optional): "Start"(default), "Center", or "End".
            width (int, optional): defailt=None; Width of the widget.
            height (int, optional): default=None; Height of the widget.
            width_fill (bool, optional): default=Shrink; Fills the parent container 
                                            or shrinks to wrap the child widgets.
            height_fill (bool, optional): default=Shrink; Fills the parent container 
                                            or shrinks to wrap the child widgets.
            padding ([float], optional): default=[10.0], Padding for container.
                                        use [float] for all sides,
                                        use [float, float] for [top&bottom, left&right]
                                        use [float, float, float, float] for [top, right, bottom, left]
            
            spacing (float, optional): default=20.0, spacing between items in column.
            user_id (str, optional): literal used to identify container instead of using the
                                     returned integer, if variable is inconvenient.
            width (tuple, optional): 
                                    ("fill", 0): Fills container space.
                                    ("fillportion", int): 1-> fills all, 2-> fills half, etc.
                                    ("shrink", 0): Fills the least amount of space.
                                    ("fixed", float): Fixed amount of space.
            show (bool, Optional): default=True, Shows or hides container and all of its contents.
            
        Return: 
            int: internal id of container and can be used by user if equated.
        """

    # def add_pane_grid(self,
    #                     window_id: str,
    #                     container_id: str,
    #                     *,
    #                     parent_id: str=None,
    #                     width: tuple=("shrink", 0),
    #                     height: tuple=("shrink", 0),
    #                     spacing: float=20.0,
    #                     padding: List=[10.0],
    #                     show: bool=True,
    #                   ) -> int:
    #     """
    #     Adds a pane grid to the gui.
        
    #     Args:
    #         window_id (str, required): Id of the window to place container in.
    #         container_id: str,
    #         parent_id (str, optional): if parent_id == window_id then not required, 
    #                                     if another container then required.
    #         *,
    #         width (int, optional): defailt=None; Width of the widget.
    #         height (int, optional): default=None; Height of the widget.
    #         width_fill (bool, optional): default=Shrink; Fills the parent container 
    #                                         or shrinks to wrap the child widgets.
    #         height_fill (bool, optional): default=Shrink; Fills the parent container 
    #                                         or shrinks to wrap the child widgets.
    #         spacing: float=20.0,
    #         padding: List=[10.0],
    #         show: bool=True,
    #     """

    # def add_pane(self,
    #             window_id: str,
    #             container_id: str,
    #             parent_id: str, 
    #             add_direction: str,
    #             ratio: float,
    #              ) -> int:
    #     """
    #     Adds a pane into the pane grid of the gui.

    #     Args:
    #         window_id (str, required): Id of the window to place container in.
    #         container_id: str,
    #         parent_id (str, required): id of another container or window. 
    #         add_direction: (required: "first", "right", or "below")
    #         ratio: float,
    #     """

    def add_scrollable(self,
                        window_id: str,
                        container_id: str,
                        *,
                        parent_id: str=None,
                        width: float=None,
                        height: float=None,
                        width_fill: bool=False,
                        height_fill: bool=False,
                        direction: str="vertical",
                        on_scroll: Callable=None,
                        ) -> int:
        """
        Wraps a scrollable widget around a container.

        Args:
            window_id (str, required): Id of the window to place container in.
            container_id (str, required): id used by widgets for placement must be unique.
            parent_id (str, optional): If parent_id == window_id then not required, 
                                        if another container then required.
            width (int, optional): defailt=None; Width of the widget.
            height (int, optional): default=None; Height of the widget.
            width_fill (bool, optional): default=Shrink; Fills the parent container 
                                            or shrinks to wrap the child widgets.
            height_fill (bool, optional): default=Shrink; Fills the parent container 
                                            or shrinks to wrap the child widgets.
            direction (str, optional): default="vertical"; Other "horizontal".
            on_scroll )Callable, optional): default=None; Callback function.
            user_id (str, optional): literal used to identify container instead of using the
                                     returned integer, if variable is inconvenient.
            width (tuple, optional): 
                                    ("fill", 0): Fills container space.
                                    ("fillportion", int): 1-> fills all, 2-> fills half, etc.
                                    ("shrink", 0): Fills the least amount of space.
                                    ("fixed", float): Fixed amount of space.
            show (bool, Optional): default=True, Shows or hides container and all of its contents.
    
        Return: 
            int: internal id of container and can be used by user if equated.
        """

    def add_tool_tip(self,
                        window_id,
                        container_id: str,
                        position: str,
                        text_to_display: str,
                        *,
                        parent_id: str=None,
                        gap: int=10,
                        style: str="box",
                     ) -> int:
    
        """
        Adds a tooltip to the widget
        Args:
            window_id (str, required): Id of the window to place container in.
            container_id: (str, required): id of the tooltip.
            position: (str, required): position of the tooltip, bottom, top, left, right, or float.
            text_to_display: (str, required): What the tooltip message is.
            parent_id: (str, optional): default=window+id of parent.
            gap: (int, Optional): defaults=10, Distance from widget.
            style: (str, optional): default=box, Optional="transparent".
        """

    def add_button(self,
                    parent_id: str,
                    label: str,
                    *,
                    id: int=None,
                    on_press: Callable=None,
                    width: float=None,
                    height: float=None,
                    width_fill: bool=False,
                    height_fill: bool=False,
                    padding: List=[10.0],
                    corner_radius: float=15.0,
                    style: any=Union[None | IpgButtonStyles.Primary],
                    arrow_style: any=Union[None | IpgButtonArrows],
                    user_data: any=None,
                    show: bool=True, 
                    ) -> int:
        """
        Adds a button to the gui
        
        Parameters
        ----------
            parent_id: str
                id of another container.
            label: str
                label of button, this field is ignored when arrow_style is used.
            id: int
                The only allowable entry for this id is the one generated by ipg.generate_id().
            on_press: Callable
                The function called when the button is pressed.
           width: float, optional)
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            height: float
                Sets the height of the widget.   
            height_fill: bool
                Sets the heigth to fill the available space, overrides height.
            padding: List[float]
                Sets the padding for widget.
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
            corner_radius: float
                Sets the roundness of the button box corners.  The effective range 0 to ~25.
            style: IpgButtonStyle
                Primary, Secondary, Positive, Destructive, Text,
            arrow_style: IpgButtonArrows
                See dropdown list when IpgButtonArrow. is typed in when period is typed.
            user_data: Any 
                Any data in any form needed by user to be passed through as a callback. 
            show: bool
                Shows or hides widget.
            
        Return:
        ------- 
            int: internal id of widget and can be used by user if equated.
        """

    def add_card(self,
                    parent_id, 
                    head, 
                    body, 
                    foot: str=None,
                    id: int=None,
                    is_open: bool=True, 
                    close_size: float=0.0, 
                    on_close=None,
                    width: float=None, 
                    height: float=None, 
                    max_width: float="inf",
                    max_height: float="inf", 
                    padding_head: float=5.0, 
                    padding_body: float=5.0, 
                    padding_foot: float=5.0,
                    show: bool=True,
                    style: str=IPG.card_style(primary=1), 
                    user_data: any=None, 
                ) -> int:
        """
        Adds a card to hold text strings

        Parameters
        ----------
            parent_id (str, required): id of another container or window.
            head (str, required): Text in the header of the card.
            body (str, required): Text in the body of the card.
            foot (str, optional): Text in the footer of the card.

        Return
        ------
            int: internal id of widget and can be used by user if equated.
        """

    def add_checkbox(self,
                     parent_id: str,
                     *,
                     on_toggle: Callable = None,
                     is_checked: bool=False,
                     label: str=None,
                     id: int=None,
                     width: float=None,
                     width_fill: bool=False,
                     size: float=16.0,
                     spacing: float = 15.0,
                     text_line_height: float=1.3,
                     text_shaping: str = "basic",
                     text_size: float=16.0,
                     icon_x: bool=False,
                     icon_size: float=25.0,
                     user_data: any=None,
                     show: bool=True,
                     ) -> int:
        """
        Adds a checkbox to the gui
        
        Parameters
        ----------
            parent_id: str
                id of another container or window.
            on_toggle: Callable
                The function called when checkbox is toggled.
            is_checked: bool
                Whether checked or not.
            label: str
                Sets the label to the right of the checkbox.
            id: int
                The only allowable entry for this id is the one generated by ipg.generate_id().
            width: float, optional)
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            size: float
                Sets the size of checkbox.
            spacing: float
                Sets the spacing between the Checkbox and the text.
            text_line_height: float
                Sets the text Line Height of the Checkbox.
            text_shaping: str
                When set to "Advanced", requires adding fonts.
            text_size: float
                Sets the size of the text beside the checkbox.
            icon_x: bool
                If true, uses the x versus the checkmark icon.
            icon_size: float
                Sets the size of either the check or x icon.
            user_data: Any 
                Any data in any form needed by user to be passed through as a callback. 
            show: bool
                Shows or hides widget.
           
        Return
        ------- 
            internal id of widget and can be used by user if equated.
        """
    
    def add_color_picker(
                        self,
                        parent_id: str,
                        *,
                        label: str="Set Color",
                        on_submit: Callable=None,
                        width: float=None,
                        height: float=None,
                        width_fill: bool=False,
                        height_fill: bool=False,
                        padding: List=[10.0],
                        corner_radius: float=0.0,
                        style: str="primary",
                        user_data: any=None,
                        show: bool=True, 
                        ) -> int:
        """
        Add a color picker.  The args just style the intial button.
        No styling for the Date Picker itself is available at this time.
        
        Args:
            parent_id (str, required): id of another container or window.
            label (str, required): label of button.
            callback (Callable, optional):  executed when button pressed.
            width (float, optional): default=None, width of the widget.
            height (float, optional): default=None, height of the widget.   
            width_fill (bool, optional): default=False, Fill available width.
            height_fill (bool, optional): default=False, Fill available height.
            padding ([float], optional): default=[10.0], Padding for container.
                                        use [float] for all sides,
                                        use [float, float] for [top&bottom, left&right]
                                        use [float, float, float, float] for [top, right, bottom, left]
            corner_radius (float, optional): default=15.0, How rounded the corner is, effective range 0 to ~25.
            style (str, optional): default="primary", others "secondary", "positive", "destructive", "text"
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback. 
            show (bool, optional): default=True, Shows or hides widget.
            
        Return: 
            int: internal id of widget and can be used by user if equated.
        """

    def add_date_picker(self,
                        parent_id: str,
                        *,
                        label: str="Calendar",
                        size_factor: float=1.0,
                        padding: List=[5.0],
                        on_submit: Callable=None,
                        user_data: any=None,
                        show=False, 
                        )  -> int:
        
        """
        Adds a date_picker widget.

        Args:
            parent_id (str, required): id of another container or window.
            label (str, optional): label to the right of the checkbox.
            size_factor (float, optional): default=1.0; Smallest size available.
                                            Can only be > 1,0.
            padding ([float], optional): default=[5.0], Padding for container.
                                        use [float] for all sides,
                                        use [float, float] for [top&bottom, left&right]
                                        use [float, float, float, float] for [top, right, bottom, left]
            on_submit (Callable, optional): Callback function date selected is submitted.
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.
            show (bool, optional): default=True, Shows or hides widget.
            
        Return 
            internal id of widget and can be used by user if equated.
        """
    
    def add_image(self, 
                    parent_id: str, 
                    image_path: str,
                    *, 
                    on_press: Callable=None, 
                    on_release: Callable=None, 
                    on_right_press: Callable=None, 
                    on_right_release: Callable=None, 
                    on_middle_press: Callable=None, 
                    on_middle_release: Callable=None,
                    on_enter: Callable=None,
                    on_move: Callable=None,
                    on_exit: Callable=None, 
                    width: float=None,
                    height: float=None,
                    width_fill: bool=False,
                    height_fill: bool=False, 
                    show: bool=True,  
                    user_data: any=None,
                    ) -> int:
        """
        Adds an image widget.  The image is selectable using callbcks for all 3 mouse buttons.

        Args:
            parent_id (str, required): id of another container or window. 
            image_path (str, required): A full path of where the image is located.
            on_press (Callable, optional): default=None, function to call after left mouse button pressed. 
            on_release (Callable, optional): default=None, function to call after left mouse button released. 
            on_right_press (Callable, optional): default=None, function to call after right mouse button pressed. 
            on_right_release (Callable, optional): default=None, function to call after right mouse button released.  
            on_middle_press (Callable, optional): default=None, function to call after middle mouse button pressed. 
            on_middle_release (Callable, optional): default=None, function to call after middle mouse button released.
            on_enter (Callable, optional): default=None, function to call after mouse enters image.
            on_move (Callable, optional): default=None, function to call after mouse moves in image.
            on_exit (Callable, optional): default=None, function to call after mouse exits image.
            width (float, optional): default=None, width of the widget.
            height (float, optional): default=None, height of the widget.   
            width_fill (bool, optional): default=False, Fill available width.
            height_fill (bool, optional): default=False, Fill available height.
            show (bool, optional) default=True, To show the widget or not.
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.   
        Return: 
            int: internal id of widget and can be used by user if equated.
        """

    def add_pick_list(self,
                        parent_id: str,
                        options: List=[str],
                        *,
                        on_select: Callable=None,
                        width: float=None,
                        width_fill: bool=False,
                        padding: List=[5.0],
                        placeholder: str=None,
                        selected: str=None,
                        text_size: float=15.0,
                        text_line_height: float="default",
                        text_shaping: str="basic",
                        user_data: any=None,
                        show: bool=True,
                      ) -> int:
        """
        Adds a pick list to the gui.

        Args:
            parent_id (str, required): id of another container or window.
            options: (List, required): List of items to select from.
            *
            on_select (Callable, optional): function executed when item selected.
            width (float, optional): default=None, width of the widget.
            width_fill (bool, optional): default=False, Fill available width.
            padding ([float], optional): default=[10.0], Padding for container.
                                        use [float] for all sides,
                                        use [float, float] for [top&bottom, left&right]
                                        use [float, float, float, float] for [top, right, bottom, left]
            placeholder: (str, optional): default=None, Instructions to user, i.e. Select Name.
            selected: (str, optional): default=None, Any preselected item.
            
            text_size: (float, optional): defalyt=15.0, Size of text.
            text_line_height: (float, optional): default=default, shrinks to text height as default.
            text_shaping: (str, optional): default="basic,
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.
            show: (bool, optional) default=True, Show or hides the widget.
    
        return: id of the widget
        """

    def add_progress_bar(self,
                         parent_id: str,
                         min: float,
                         max: float,
                         value: float=0.0,
                         *,
                         width: float=None,
                         width_fill: bool=False,
                         height: float=1.0,
                         user_data: any=None,
                         show: bool=True,
                         ) -> int:
        """
        Adds a progress bar to the gui
        
        Args:
            parent_id (str, required): id of another container or window.
            min (float, required): minimum value of bar.
            max (float, required): maximum value of bar.
            value (float, required): starting value of bar.
            width (float, optional): default=None, width of the widget.
            width_fill (bool, optional): default=False, Fill available width.
            height (float, optional ): default=1.0, height of bar
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.
            show: (bool, optional) default=True, Show or hides the widget.
            
        return: id of the widget
        """

    def add_radio(self,
                    parent_id: str,
                    labels: List=[str],
                    *,
                    direction: str="vertical",
                    spacing: float=10.0,
                    padding: List=[10.0],
                    width: float=None,
                    width_fill: bool=False,
                    on_select: Callable=None,
                    selected_index: int=None,
                    size: float=20.0,
                    text_spacing: float=15.0,
                    text_size: float=16.0,
                    text_line_height: float=1.3,
                    text_shaping: str="basic",
                    user_data=None,
                    show: bool=True,
                  ) -> int:
        """
        Adds a radio button to the gui
        
        Args:
            parent_id (str, required): id of another container or window.
            labels (List[str], required): labels of radio buttons, use List[label] for only one radio.
            direction (str, optional): default="vertical" or "horizontal", direction for the radio group.
            spacing (float, optional): default=20.0, spacing between items in column.
            padding ([float], optional): default=[10.0], Padding for container.
                                        use [float] for all sides,
                                        use [float, float] for [top&bottom, left&right]
                                        use [float, float, float, float] for [top, right, bottom, left]
            width (float, optional): default=None, width of the widget.
            width_fill (bool, optional): default=False, Fill available width.
            on_select (Callable, optional): default=None, function executed when radio pressed.
            selected_index (int, optional): default=None, Any pre-selected label index.
            line_height (float, optional): defalt=1.0, sets the line height of the radio.
            selected (bool, optional): default=False, initial state of the radio.
            show (bool, optional): default=True, shows or hides widget.
            size (f32, optional): size of the round radio.
            spacing (f32, optional): default=15.0, spacing around the radio.
            text_shaping (str, optional): default="Basic", other value is "Advanced", requires adding fonts.
            text_size (f32, optional): default=15.0, size of the text.
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.
            show: (bool, optional) default=True, Show or hides the widget.
    
        Return: 
            int: internal id of widget and can be used by user if equated.
        """

    def add_selectable_text(self, 
                            parent_id, 
                            text,
                            *, 
                            on_press: Callable=None, 
                            on_release: Callable=None, 
                            on_right_press: Callable=None, 
                            on_right_release: Callable=None, 
                            on_middle_press: Callable=None, 
                            on_middle_release: Callable=None,
                            on_enter: Callable=None,
                            on_move: Callable=None,
                            on_exit: Callable=None, 
                            width: float=None,
                            height: float=None,
                            width_fill: bool=False,
                            height_fill: bool=False, 
                            h_align: str="left",
                            v_align: str="top", 
                            line_height: float=1.3, 
                            shaping: str="basic",
                            size: float=16.0, 
                            show: bool=True,  
                            user_data: any=None,
                            ) -> int:
        """
        Adds a selectable text widget.  This selectable text allows more mouse interaction than
        a button with a style of text only.

        Args:
            parent_id (str, required): id of another container or window. 
            text (str, required): The text as wanted.
            on_press (Callable, optional): default=None, function to call for left mouse button pressed. 
            on_release (Callable, optional): default=None, function to call for left mouse button released. 
            on_right_press (Callable, optional): default=None, function to call for right mouse button pressed. 
            on_right_release (Callable, optional): default=None, function to call for right mouse button released.  
            on_middle_press (Callable, optional): default=None, function to call for middle mouse button pressed. 
            on_middle_release (Callable, optional): default=None, function to call for middle mouse button released.
            on_enter (Callable, optional): default=None, function to call for mouse enters in text area.
            on_move (Callable, optional): default=None, function to call for mouse moves in text area.
            on_exit (Callable, optional): default=None, function to call for mouse exits text area.
            width (float, optional): default=None, width of the widget.
            height (float, optional): default=None, height of the widget.   
            width_fill (bool, optional): default=False, Fill available width.
            height_fill (bool, optional): default=False, Fill available height.
            h_align (str, optional): default="left", Horizontal alignment, left, center, right.
            v_align (str, optional): default="top", vertical alignment, top, center, bottom.
            line_height (float, optional): default=1.3, size of the box the text is in.
            shaping (str, optional): shaping of text.
            size (float, optional) default=16.0, text size.
            show (bool, optional) default=True, To show the widget or not.
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.   
        Return: 
            int: internal id of widget and can be used by user if equated.
        """

    def add_slider(self, 
                   parent_id: str, 
                   min: float, 
                   max: float, 
                   step: float, 
                   value: float,
                   *, 
                   connect_progress_bar: int=None, 
                   show: bool=True, 
                   on_change: Callable=None, 
                   on_release: Callable=None, 
                   user_data=None, 
                   width: float=100.0,
                   width_fill: bool=False,
                   height: float=20.0
                  ) -> int:
        """
        Adds a slider widget which changes value as the mouse moves the slider.
        The resulting float value can be used by the callbacks to produce the desired results.
        If on_change is used, then the callback is called many times, but only once for the on_release.
        If one wants to connect to a progress bar, then give the connect_progress_bar the id of the pg bar.

        Args:
            parent_id (str, required): id of another container or window. 
            min (float, required): The minimum value wanted.
            max (float, required): The maximum value wanted.
            step (float, required): The step size. 
            value (float, required): The starting value. 
            connect_progress_bar (int, optional): default=None, If connected then the pg_bar's id. 
            show (bool, optional) default=True, Tos show or not.
            on_change (Callable, optional): default=None, If set, will use callback with each change. 
            on_release (Callable, optional): default=None, If set, will use callback when released. 
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.
            width (float, optional): default=100.0   
            width_fill (bool, optional): default=False;  Fill overides width when set to True.
            height (float, optional): default=20.0; The height of the slider.
        Return: 
            int: internal id of widget and can be used by user if equated.
        """

    def add_space(self,
                  parent_id: str,
                  *,
                  width: float=None,
                  height: float=None,
                  width_fill: bool=False,
                  height_fill: bool=False,
                  ) -> int:
        """
        Add a space between elements for alignment and aesthetics.

        Args:
            parent_id (str, required): id of another container or window.
            width (float, optional): default= None, Horizontal size of the sapce.
            height (float, optional): default=None, Vertical size of the space.
            width_fill (bool, optional): default=False, Fill the available space horizontally.
            height_fill (bool, optional): default=False, Fill the available space vertically.
        """

    def add_table(self,
                  parent_id: str,
                  title: str,
                  data: List,
                  width: float,
                  height: float,
                  *,
                  callback: Callable=None,
                  column_widths: List=[float, ...],
                  show: bool=True, 
                  user_data=None,
                  ) -> int:

        """
        Adds a table to the gui.

        Args:
            parent_id (str, required): id of another container or window.
            title (str, required): Title used for table.
            data (List[Dict], required): A list of dictionaries.
            width (float, required): Width of the table.
            height (float, required): Height of the table.
            callback (Callable, optional): defalut=None, callback used when actions on table are intiated.
            column_widths (List[float], optional): If only one value is supplied [20.0], thats the default.
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.
            show (bool, optional): default=show, whether to show the widget or not.
            
        Return: 
            int: internal id of widget and can be used by user if equated.
        """

    def update_table(self,
                     id: int, 
                     title: str=None, 
                     columns: List=None, 
                     data: List=None, 
                     callback: Callable=None
                     ) -> int:
        """
        Updates a table
        """

    def add_text(self,
                 parent_id: str,
                 content: str,
                 *,
                 id: int=None,
                 width: float=None,
                 height: float=None,
                 width_fill: bool=False,
                 height_fill: bool=False,
                 h_align: str="Left",
                 v_align: str="Top",
                 line_height: str="default",
                 size: float=16.0,
                 shaping: str="basic",
                 show: bool=True,
                 ) -> int:
        """
        Adds a text widget to the gui.

        Parameters
        ----------
            parent_id: str
                id of another container or window to place the widget in.
            content: str
                Sets the text of the widget.
            width: float, optional)
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            height: float
                Sets the height of the widget.   
            height_fill: bool
                Sets the heigth to fill the available space, overrides height.
            h_align: str
                Aligns text horizontally "Left", "Center", or "Right", width needs to be larger than text.
            v_align: str
                Aligns text vertically "Left", "Center", or "Right", height needs to be larger than text.
            line_height: float
                Sets the line height of the text.
            size: float
                Sets the text size.
            shaping: str
                Sets the shape of the text using added fonts, uUse "Basic"(None) or "Advanced".
            show:
                shows or hides widget.
        
        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.

        """

    def add_text_editor(self,
                        parent_id: str,
                        file_name: str,
                        ) -> int:
        """
        Adds a text editor widget to the gui.

        Parameters
        ----------
            parent_id: str
                id of another container or window to place the widget in.
            file_name: str
                Path to the file to be used.
        
        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.
        """

    def add_text_input(self,
                       parent_id: str,
                       placeholder: str,
                       width: float,
                       *,
                       on_submit: Callable=None,
                       on_input: Callable=None,
                       on_paste: Callable=None,
                       line_height: str="default",
                       
                       width_fill: bool=False,
                       padding: List=[10.0],
                       size: Union[float, None]=None,
                       user_data=None,
                       is_secure: bool=False,
                       ) -> int:
        """
        Adds a text_input widget to the gui.  Callbacks on_input and on_submit are
        required for the widget to work.  Using a long lived variable, value = self.value,
        as the text is entered, the value will be updated and displayed via a callback.

        Parameters
        ----------
            parent_id: str
                id of another container or window to place the widget in.
            placeholder: str
                text used for instructions in the input box.
            on_submit: Callable
                Calls a function when the enter key is pressed, submitting the text.
            on_input: Callable
                Calls a function each time a letter is enter into the text box.
            on_paste: Callable
                Calls a function when the text is pasted into the text box, pressing enter will also submit.
            line_height: float
                Sets the line height of the text.
            width: float
                Sets the width of the text box..   
            width_fill: bool
                Sets the width to fill available space, overrides width.
            padding: List[float]
                Sets the padding for widget.
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
            size: float
                Sets the text size.
            user_data: Any
                Any data that might be needed in the callback function.
            is_secure: bool
                Hides the entered text, for passwords, etc.
            
        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.
        """
    
    # *******************************events*************************************************************
    def add_event_keyboard(self,
                            enabled: bool,
                            *,
                            on_key_press: Callable=None,
                            on_key_release: Callable=None,
                            user_data=None, 
                           ) -> int:
        """
        Add a keyboard event handler to process keyboard actions.

        Parameters
        ----------
        enabled: bool
            Enables the event
        on_key_press: Callable
            Calls a function when a key is pressed.
        on_key_release: Callable
            Calls a function when a key is released.
        user_data: Any
            Any data that might be needed in the callback function.

        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.
        """

    def add_event_mouse(self,
                            enabled: bool,
                            *,
                            on_move: Callable=None,
                            on_left_press: Callable=None,
                            on_left_release: Callable=None,
                            on_middle_press: Callable=None,
                            on_middle_release: Callable=None,
                            on_right_press: Callable=None,
                            on_right_release: Callable=None,
                            on_middle_scroll: Callable=None,
                            user_data=None,
                          ) ->int:
        """
        Add a mouse button handlers to process mouse actions.

        Parameters
        ----------
        enabled: bool
            Enables the event
        on_move: Callable
            Calls a function when the mouse is moved.
        on_left_press: Callable
            Calls a function when the left mouse button is pressed.
        on_left_release: Callable
            Calls a function when the left mouse button is released.
        on_middle_press: Callable
            Calls a function when the middle mouse button is pressed.
        on_middle_release: Callable
            Calls a function when the middle mouse button is released.
        on_right_press: Callable
            Calls a function when the right mouse button is pressed.
        on_right_release: Callable
            Calls a function when the right mouse button is released.
        on_middle_scroll: Callable
            Calls a function when the middle mouse scroll is scrolled.
        user_data: Any
            Any data that might be needed in the callback function.
        
        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.
        """

    def add_event_window(self,
                         enabled: bool,
                         *,
                        on_open: Callable=None,
                        on_close: Callable=None,
                        on_moved: Callable=None,
                        on_resized: Callable=None,
                        user_data=None,
                         ) -> int:
        """
        Adds event to the window other than those in the add_window method.

        Parameters
        ----------
        enabled: bool
            Enables the event
        on_open: Callable
            Calls a function when window is opened.
        on_close: Callable
            Calls a function when the window is closed.
        on_moved: Callable
            Calls a function when the window is moved.
        on_resized: Callable
            Calls a function when the window id resized.
        user_data: Any
            Any data that might be needed in the callback function.
        
        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.
        """


    # *******************************all item ops**************************************
        
    def delete_item(self, id: int):
        """
        Deletes an item using the int id
        Example: btn_id = add_button("Button")
                 delete_item(btn_id)

        """

    def update_item(self, 
                    id: int, 
                    param: str, 
                    value):
        """
        Update a widget by supplying the id, the parameter to update and values as a str or number
        """

    def window_theme(self, Light=False, Dark=False, Dracula=False, Nord=False,SolarizedLight=False,
                        SolarizedDark=False, GruvboxLight=False,GruvboxDark=False,CatppuccinLatte=False,
                        CatppuccinFrappe=False, CatppuccinMacchiato=False, CatppuccinMocha=False,
                        TokyoNight=False,TokyoNightStorm=False, TokyoNightLight=False, KanagawaWave=False,
                        KanagawaDragon=False, KanagawaLotus=False, Moonfly=False,Nightfly=False,Oxocarbon=False):
        """
        Gets the window them.  Set any theme to True to active.  
        If you activate more than one, the first item will be chosen.
        Define this first by equating to a variable them put in the window theme parameter.
        ALternately, put the function directly in the style parameter.
        """


class IpgButtonStyles:
    Primary=0
    Secondary=0
    Positive=0
    Destructive=0
    Text=0

class IpgButtonParams:
    ArrowStyle=0,
    CornerRadius=0,
    Height=0,
    HeightFill=0,
    Label=0,
    Padding=0,
    Show=0,
    Style=0,
    Width=0,
    WidthFill=0


class IpgCardStyles: 
    Primary=0
    Secondary=0
    Success=0
    Danger=0
    Warning=0
    Info=0
    Light=0
    Dark=0
    White=0
    Default=0

class IpgCardParams:
    Body=0,
    Foot=0,
    Head=0,
    IsOpen=0,
    Style=0,

class IpgCheckboxParams:
    IconSize=0,
    IconX=0,
    IsChecked=0,
    Label=0,
    Show=0,
    Size=0,
    Spacing=0,
    Style=0,
    TextLineHeight=0,
    TextShaping=0,
    TextSize=0,
    Width=0,
    WidthFill=0,


class IpgDatePickerParams:
    Label=0,
    Padding=0,
    SizeFactor=0,
    Show=0,


class IpgImageParams:
    Height=0,
    HeightFill=0,
    ImagePath=0,
    Padding=0,
    Show=0,
    Width=0,
    WidthFill=0,


class IpgPickListParams:
    Options=0,
    Placeholder=0,
    Padding=0,
    Show=0,
    TextSize=0,
    TextLineHeight=0,
    Width=0,


class IpgProgressBarParams:
    Height=0,
    Min=0,
    Max=0,
    Show=0,
    Value=0,
    Width=0,
    WidthFill=0,

class RadioDirection:
    Horizontal=0,
    Vertical=0,


class RadioParams:
    Direction=0,
    Labels=0,
    Padding=0,
    SelectedIndex=0,
    Show=0,
    Size=0,
    Spacing=0,
    TextSpacing=0,
    TextSize=0,
    TextLineHeight=0,
    UserData=0,
    Width=0,
    WidthFill=0,
    Height=0,
    HeightFill=0,


class IpgTextParams:
    Content=0,
    Height=0,
    HeightFill=0,
    HzAlignLeft=0,
    HzAlignCenter=0,
    HzAlignRight=0,
    LineHeight=0,
    Size=0,
    VtAlignTop=0,
    VtAlignCenter=0,
    VtAlignBottom=0,
    Width=0,
    WidthFill=0,


class IpgButtonArrows:
    ArrowBarLeft=0
    ArrowBarRight=0
    ArrowBarUp=0
    ArrowClockwise=0
    ArrowCounterclockwise=0
    ArrowDown=0
    ArrowDownCircle=0
    ArrowDownCircleFill=0
    ArrowDownLeft=0
    ArrowDownLeftCircle=0
    ArrowDownLeftCircleFill=0
    ArrowDownLeftSquare=0
    ArrowDownLeftSquareFill=0
    ArrowDownRight=0
    ArrowDownRightCircle=0
    ArrowDownRightCircleFill=0
    ArrowDownRightSquare=0
    ArrowDownRightSquareFill=0
    ArrowDownShort=0
    ArrowDownSquare=0
    ArrowDownSquareFill=0
    ArrowDownUp=0
    ArrowLeft=0
    ArrowLeftCircle=0
    ArrowLeftCircleFill=0
    ArrowLeftRight=0
    ArrowLeftShort=0
    ArrowLeftSquare=0
    ArrowLeftSquareFill=0
    ArrowNinezerodegDown=0
    ArrowNinezerodegLeft=0
    ArrowNinezerodegRight=0
    ArrowNinezerodegUp=0
    ArrowRepeat=0
    ArrowReturnLeft=0
    ArrowReturnRight=0
    ArrowRight=0
    ArrowRightCircle=0
    ArrowRightCircleFill=0
    ArrowRightShort=0
    ArrowRightSquare=0
    ArrowRightSquareFill=0
    ArrowThroughHeart=0
    ArrowThroughHeartFill=0
    ArrowUp=0
    ArrowUpCircle=0
    ArrowUpCircleFill=0
    ArrowUpLeft=0
    ArrowUpLeftCircle=0
    ArrowUpLeftCircleFill=0
    ArrowUpLeftSquare=0
    ArrowUpLeftSquareFill=0
    ArrowUpRight=0
    ArrowUpRightCircle=0
    ArrowUpRightCircleFill=0
    ArrowUpRightSquare=0
    ArrowUpRightSquareFill=0
    ArrowUpShort=0
    ArrowUpSquare=0
    ArrowUpSquareFill=0
    Arrows=0
    ArrowsAngleContract=0
    ArrowsAngleExpand=0
    ArrowsCollapse=0
    ArrowsCollapseVertical=0
    ArrowsExpand=0
    ArrowsExpandVertical=0
    ArrowsFullscreen=0
    ArrowsMove=0
    ArrowsVertical=0