from dataclasses import dataclass
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

    def add_window(self,
                    window_id: str,
                    title: str,
                    width: int,
                    height: int,
                    pos_x: float=None,
                    pos_y: float=None,
                    pos_centered: bool=False,
                    resizable: bool=True,
                    theme: str="dark",
                    scroll: bool=False,
                    scroll_width: tuple=(str, float),
                    scroll_height: tuple=(str, float),
                    scroll_direction: str="vertical",
                    on_scroll: Callable=None,
                    debug: bool=False,
                    show: bool=True,
                    ) -> None:
        """
        Calls the rust lib main window function which starts the iced gui.\n
        This function needs to be placed at the end of the python program.

        Args:
            window_id (str, required): Window id used to place widget or container into.\n
            title (str, required): Title placed in the top bar of the main window.\n
            width (int, required): Width of the main window.\n
            height (int, required): Height of the main window.\n
            ** The window position will be defaulted to a system default unless position is set.\n
            pos_x (float, optional): x position of window\n
            pos_y (float, optional): y position of window\n
            pos_centered (bool, optional): default=False; Centered position of window.\n
            resizable (bool, optional): default=True; Whether the window can be resized.\n
            theme (str, optioonal): default=dark; Optional=white. More to come later.\n
            scroll (bool, optional): default=false; Turns scrolling on or off.\n
            scroll_width (float, optional): default=16.0; Width of the scroll bar.\n
            scroll_height (float, optional): default=Fill; Fill height of window.\n
            on_scroll (Callable, optional): default=None; Callback function.\n
            show (bool, optional): default=True; First window always true, others can be set.\n
            debug (bool, optional): default=False; Draws a box around widgets to see layout.\n
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
                        user_id: str=None,
                        ) -> int:
        """
        Adds a generic container to the gui
        **Note: A container unlike a row or column can only have 1 child.\n
                The container is used to help with widget alignments.\n
                You can align 1 widget or many if you add a row or column to the\n
                container and use the horizontal and vertical alignments.\n
        Args:
            window_id (str, required): Id of the window to place container in.\n
            container_id (str, required): id used by widgets for placement must be unique.\n
            parent_id (str, optional): If parent_id == window_id then not required, \n
                                        if another container then required.\n
            width (int, optional): defailt=None; Width of the widget.\n
            height (int, optional): default=None; Height of the widget.\n
            width_fill (bool, optional): default=Shrink; Fills the parent container \n
                                            or shrinks to wrap the child widgets.\n
            height_fill (bool, optional): default=Shrink; Fills the parent container \n
                                            or shrinks to wrap the child widgets.\n
            max_width (float, optional): default=inf; Sets a max width.\n
            max_height (float, optional): default=inf; Sets a max height.\n
            align_x (str, optional): default="left"; "center", or "right"\n
            align_y (str, optional): default="top"; "center", or "bottom"\n
            padding ([float], optional): default=[10.0], Padding for container.\n
                                        use [float] for all sides,\n
                                        use [float, float] for [top&bottom, left&right]\n
                                        use [float, float, float, float] for [top, right, bottom, left]\n
            show (bool, Optional): default=True, Shows or hides container and all of its contents.\n
            user_id (str, optional): literal used to identify container instead of using the \n
                                        returned integer, if int type is inconvenient.\n
        Return: 
            int: internal id of container and can be used by user if equated or use user_id.\n
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
                    user_id: str="",
                    ) -> int:

        """ Adds a Column which is a container that distributes its contents vertically
        
        Args:
            window_id (str, required): Id of the window to place container in.\n
            container_id (str, required): id used by widgets for placement must be unique.\n
            parent_id (str, optional): If parent_id == window_id then not required, \n
                                        if another container then required.\n
            align_items (str, optional): "start"(default), "center", or "end".\n
            width (int, optional): defailt=None; Width of the widget.\n
            height (int, optional): default=None; Height of the widget.\n
            width_fill (bool, optional): default=Shrink; Fills the parent container \n
                                            or shrinks to wrap the child widgets.\n
            height_fill (bool, optional): default=Shrink; Fills the parent container \n
                                            or shrinks to wrap the child widgets.\n
            max_width (float, optional): default=float('inf'), Container used all of the available width.\n
            padding ([float], optional): default=[10.0], Padding for container.\n
                                        use [float] for all sides,\n
                                        use [float, float] for [top&bottom, left&right]\n
                                        use [float, float, float, float] for [top, right, bottom, left]\n
            spacing (float, optional): default=20.0, spacing between items in column.\n
                        user_id (str, optional): literal used to identify container instead of using the \n
                                     returned integer, if variable is inconvenient.\n
            show (bool, Optional): default=True, Shows or hides container and all of its contents.\n
            user_id (str, optional): literal used to identify container instead of using the \n
                                        returned integer, if int type is inconvenient.\n
        Return: 
            int: internal id of container and can be used by user if equated or use user_id\n
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
                user_id: str=None,
                ) -> int:

        """
        Adds a row container to the gui.  Aligns widget horizontally.
        
        Args:
            window_id (str, required): Id of the window to place container in.\n
            container_id (str, required): id used by widgets for placement must be unique.\n
            parent_id (str, optional): If parent_id == window_id then not required, \n
                                        if another container then required.\n
            align_items (str, optional): "Start"(default), "Center", or "End".\n
            width (int, optional): defailt=None; Width of the widget.\n
            height (int, optional): default=None; Height of the widget.\n
            width_fill (bool, optional): default=Shrink; Fills the parent container \n
                                            or shrinks to wrap the child widgets.\n
            height_fill (bool, optional): default=Shrink; Fills the parent container \n
                                            or shrinks to wrap the child widgets.\n
            padding ([float], optional): default=[10.0], Padding for container.\n
                                        use [float] for all sides,\n
                                        use [float, float] for [top&bottom, left&right]\n
                                        use [float, float, float, float] for [top, right, bottom, left]\n
            
            spacing (float, optional): default=20.0, spacing between items in column.\n
            user_id (str, optional): literal used to identify container instead of using the\n
                                     returned integer, if variable is inconvenient.\n
            width (tuple, optional):\n 
                                    ("fill", 0): Fills container space.\n
                                    ("fillportion", int): 1-> fills all, 2-> fills half, etc.\n
                                    ("shrink", 0): Fills the least amount of space.\n
                                    ("fixed", float): Fixed amount of space.\n
            show (bool, Optional): default=True, Shows or hides container and all of its contents.\n
            user_id (str, optional): literal used to identify container instead of using the \n
                                        returned integer, if int type is inconvenient.\n
        Return: 
            int: internal id of container and can be used by user if equated or use user_id.\n
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
    #                     user_id: str="",
    #                   ) -> int:
    #     """
    #     Adds a pane grid to the gui.
        
    #     Args:
    #         window_id (str, required): Id of the window to place container in.\n
    #         container_id: str,
    #         parent_id (str, optional): if parent_id == window_id then not required, \n
    #                                     if another container then required.\n
    #         *,
    #         width (int, optional): defailt=None; Width of the widget.\n
    #         height (int, optional): default=None; Height of the widget.\n
    #         width_fill (bool, optional): default=Shrink; Fills the parent container \n
    #                                         or shrinks to wrap the child widgets.\n
    #         height_fill (bool, optional): default=Shrink; Fills the parent container \n
    #                                         or shrinks to wrap the child widgets.\n
    #         spacing: float=20.0,
    #         padding: List=[10.0],
    #         show: bool=True,
    #         user_id: str="",
    #     """

    # def add_pane(self,
    #             window_id: str,
    #             container_id: str,
    #             parent_id: str, 
    #             add_direction: str,
    #             ratio: float,
    #             user_id: str="",
    #              ) -> int:
    #     """
    #     Adds a pane into the pane grid of the gui.

    #     Args:
    #         window_id (str, required): Id of the window to place container in.\n
    #         container_id: str,
    #         parent_id (str, required): id of another container or window.\n 
    #         add_direction: (required: "first", "right", or "below")
    #         ratio: float,
    #         user_id: str="",
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
                        user_id: str=None,
                        ) -> int:
        """
        Wraps a scrollable widget around a container.

        Args:
            window_id (str, required): Id of the window to place container in.\n
            container_id (str, required): id used by widgets for placement must be unique.\n
            parent_id (str, optional): If parent_id == window_id then not required, \n
                                        if another container then required.\n
            width (int, optional): defailt=None; Width of the widget.\n
            height (int, optional): default=None; Height of the widget.\n
            width_fill (bool, optional): default=Shrink; Fills the parent container \n
                                            or shrinks to wrap the child widgets.\n
            height_fill (bool, optional): default=Shrink; Fills the parent container \n
                                            or shrinks to wrap the child widgets.\n
            direction (str, optional): default="vertical"; Other "horizontal".\n
            on_scroll )Callable, optional): default=None; Callback function.\n
            user_id (str, optional): literal used to identify container instead of using the\n
                                     returned integer, if variable is inconvenient.\n
            width (tuple, optional):\n 
                                    ("fill", 0): Fills container space.\n
                                    ("fillportion", int): 1-> fills all, 2-> fills half, etc.\n
                                    ("shrink", 0): Fills the least amount of space.\n
                                    ("fixed", float): Fixed amount of space.\n
            show (bool, Optional): default=True, Shows or hides container and all of its contents.\n
            user_id (str, optional): literal used to identify container instead of using the \n
                                        returned integer, if int type is inconvenient.\n
        Return: 
            int: internal id of container and can be used by user if equated or use user_id.\n
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
            window_id (str, required): Id of the window to place container in.\n
            container_id: (str, required): id of the tooltip.\n
            position: (str, required): position of the tooltip, bottom, top, left, right, or float.\n
            text_to_display: (str, required): What the tooltip message is.\n
            parent_id: (str, optional): default=window+id of parent.\n
            gap: (int, Optional): defaults=10, Distance from widget.\n
            style: (str, optional): default=box, Optional="transparent".\n
        """

    def add_button(self,
                    parent_id: str,
                    label: str,
                    *,
                    on_press: Callable=None,
                    width: float=None,
                    height: float=None,
                    width_fill: bool=False,
                    height_fill: bool=False,
                    padding: List=[10.0],
                    corner_radius: float=15.0,
                    style: str="primary",
                    user_data: any=None,
                    show: bool=True, 
                    user_id: str=None,
                    ) -> int:
        """
        Adds a button to the gui
        
        Args:
            parent_id (str, required): id of another container or window.\n
            label (str, required): label of button.\n
            on_press (Callable, optional):  executed when button pressed.\n
            width (float, optional): default=None, width of the widget.\n
            height (float, optional): default=None, height of the widget.\n   
            width_fill (bool, optional): default=False, Fill available width.\n
            height_fill (bool, optional): default=False, Fill available height.\n
            padding ([float], optional): default=[10.0], Padding for container.\n
                                        use [float] for all sides,\n
                                        use [float, float] for [top&bottom, left&right]\n
                                        use [float, float, float, float] for [top, right, bottom, left]\n
            corner_radius (float, optional): default=15.0, How rounded the corner is, effective range 0 to ~25.\n
            style (str, optional): default="primary", others "secondary", "positive", "destructive", "text"\n
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n 
            show (bool, optional): default=True, Shows or hides widget.\n
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n
            
        Return: 
            int: internal id of widget and can be used by user if equated or use user_id.\n
        """

    def add_card(self,
                    parent_id, 
                    head, 
                    body, 
                    foot: str=None, 
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
                    user_id: str=None, 
                    
                ) -> int:
        """
        Adds a card to hold text strings

        Args:
            parent_id (str, required): id of another container or window.\n
            head (str, required): Text in the header of the card.\n
            body (str, required): Text in the body of the card.\n
            foot (str, optional): Text in the footer of the card.\n
            user_data_str (any, optional): Data needed by user to be passed through a callback.\n 
        Return: 
            int: internal id of widget and can be used by user if equated or use user_id.\n
        """

    def add_checkbox(self,
                     parent_id: str,
                     *,
                     callback: Callable = None,
                     is_checked: bool=False,
                     label: str=None,
                     width: float=None,
                     width_fill: bool=False,
                     size: float=16.0,
                     spacing: float = 15.0,
                     text_line_height: float=1.3,
                     text_shaping: str = "basic",
                     text_size: float=16.0,
                     user_data: any=None,
                     user_id: str = "",
                     show: bool=True,
                     ) -> int:
        """
        Adds a checkbox to the gui
        
        Args:
            parent_id (str, required): id of another container or window.\n
            callback (Callable, optional): function executed when button pressed.\n
            is_checked (bool, optional): default=False, Whether checked or not.\n
            label (str, optional): label to the right of the checkbox.\n
            width (float, optional): default=None, width of the widget.\n
            width_fill (bool, optional): default=False, Fill available width.\n
            size (float, optional): default=15.0, sets the size of checkbox.\n
            spacing (float, optional): default=20.0, Sets the spacing between the Checkbox and the text.\n
            text_line_height (float, optional): default=1.3 Sets the text Line Height of the Checkbox.
            text_shaping (str, optional): default="Basic", other value is "Advanced", requires adding fonts.\n
            text_size (float, optional): default=16.0, size of the text beside the checkbox.\n
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n
            show (bool, optional): default=True, Shows or hides widget.\n
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n
        Return 
            internal id of widget and can be used by user if equated or use user_id.\n
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
                        user_id: str=None,
                        ) -> int:
        """
        Add a color picker.  The args just style the intial button.
        No styling for the Date Picker itself is available at this time.
        
        Args:
            parent_id (str, required): id of another container or window.\n
            label (str, required): label of button.\n
            callback (Callable, optional):  executed when button pressed.\n
            width (float, optional): default=None, width of the widget.\n
            height (float, optional): default=None, height of the widget.\n   
            width_fill (bool, optional): default=False, Fill available width.\n
            height_fill (bool, optional): default=False, Fill available height.\n
            padding ([float], optional): default=[10.0], Padding for container.\n
                                        use [float] for all sides,\n
                                        use [float, float] for [top&bottom, left&right]\n
                                        use [float, float, float, float] for [top, right, bottom, left]\n
            corner_radius (float, optional): default=15.0, How rounded the corner is, effective range 0 to ~25.\n
            style (str, optional): default="primary", others "secondary", "positive", "destructive", "text"\n
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n 
            show (bool, optional): default=True, Shows or hides widget.\n
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n
            
        Return: 
            int: internal id of widget and can be used by user if equated or use user_id.\n
        """

    def add_date_picker(self,
                        parent_id: str,
                        *,
                        label: str="Calendar",
                        size_factor: float=1.0,
                        padding: List=[5.0],
                        on_select: Callable=None,
                        user_data: any=None,
                        show=False,
                        user_id: str="", 
                        )  -> int:
        
        """
        Adds a date_picker widget.

        Args:
            parent_id (str, required): id of another container or window.\n
            label (str, optional): label to the right of the checkbox.\n
            size_factor (float, optional): default=1.0; Smallest size available.\n
                                            Can only be > 1,0.\n
            padding ([float], optional): default=[5.0], Padding for container.\n
                                        use [float] for all sides,\n
                                        use [float, float] for [top&bottom, left&right]\n
                                        use [float, float, float, float] for [top, right, bottom, left]\n
            on_select (Callable, optional): Callback function when checkbox checked or unchecked.\n
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n
            show (bool, optional): default=True, Shows or hides widget.\n
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n
        Return 
            internal id of widget and can be used by user if equated or use user_id.\n
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
                    user_id: str=None,
                    show: bool=True,  
                    user_data: any=None,
                    ) -> int:
        """
        Adds an image widget.  The image is selectable using callbcks for all 3 mouse buttons.

        Args:
            parent_id (str, required): id of another container or window.\n 
            image_path (str, required): A full path of where the image is located.\n
            on_press (Callable, optional): default=None, function to call after left mouse button pressed.\n 
            on_release (Callable, optional): default=None, function to call after left mouse button released.\n 
            on_right_press (Callable, optional): default=None, function to call after right mouse button pressed.\n 
            on_right_release (Callable, optional): default=None, function to call after right mouse button released.\n  
            on_middle_press (Callable, optional): default=None, function to call after middle mouse button pressed.\n 
            on_middle_release (Callable, optional): default=None, function to call after middle mouse button released.\n
            on_enter (Callable, optional): default=None, function to call after mouse enters image.\n
            on_move (Callable, optional): default=None, function to call after mouse moves in image.\n
            on_exit (Callable, optional): default=None, function to call after mouse exits image.\n
            width (float, optional): default=None, width of the widget.\n
            height (float, optional): default=None, height of the widget.\n   
            width_fill (bool, optional): default=False, Fill available width.\n
            height_fill (bool, optional): default=False, Fill available height.\n
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n  
            show (bool, optional) default=True, To show the widget or not.\n
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n   
        Return: 
            int: internal id of widget and can be used by user if equated or use user_id.\n
        """

    def add_pick_list(self,
                        parent_id: str,
                        options: List=[str],
                        *,
                        callback: Callable=None,
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
                        user_id: str=None,
                      ) -> int:
        """
        Adds a pick list to the gui.

        Args:
            parent_id (str, required): id of another container or window.\n
            options: (List, required): List of items to select from.\n
            *
            callback (Callable, optional): function executed when item selected.\n
            width (float, optional): default=None, width of the widget.\n
            width_fill (bool, optional): default=False, Fill available width.\n
            padding ([float], optional): default=[10.0], Padding for container.\n
                                        use [float] for all sides,\n
                                        use [float, float] for [top&bottom, left&right]\n
                                        use [float, float, float, float] for [top, right, bottom, left]\n
            placeholder: (str, optional): default=None, Instructions to user, i.e. Select Name.\n
            selected: (str, optional): default=None, Any preselected item.\n
            
            text_size: (float, optional): defalyt=15.0, Size of text.\n
            text_line_height: (float, optional): default=default, shrinks to text height as default.\n
            text_shaping: (str, optional): default="basic,
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n
            show: (bool, optional) default=True, Show or hides the widget.\n
            user_id: (str, optional): default=None, A str for widget id if number inconvenient.\n
    
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
                         user_id: str=None,
                         ) -> int:
        """
        Adds a progress bar to the gui
        
        Args:
            parent_id (str, required): id of another container or window.\n
            min (float, required): minimum value of bar.\n
            max (float, required): maximum value of bar.\n
            value (float, required): starting value of bar.\n
            width (float, optional): default=None, width of the widget.\n
            width_fill (bool, optional): default=False, Fill available width.\n
            height (float, optional ): default=1.0, height of bar\n
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n
            show: (bool, optional) default=True, Show or hides the widget.\n
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n
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
                    user_id: str="",
                  ) -> int:
        """
        Adds a radio button to the gui
        
        Args:
            parent_id (str, required): id of another container or window.\n
            labels (List[str], required): labels of radio buttons, use List[label] for only one radio.\n
            direction (str, optional): default="vertical" or "horizontal", direction for the radio group.\n
            spacing (float, optional): default=20.0, spacing between items in column.\n
            padding ([float], optional): default=[10.0], Padding for container.\n
                                        use [float] for all sides,\n
                                        use [float, float] for [top&bottom, left&right]\n
                                        use [float, float, float, float] for [top, right, bottom, left]\n
            width (float, optional): default=None, width of the widget.\n
            width_fill (bool, optional): default=False, Fill available width.\n
            on_select (Callable, optional): default=None, function executed when radio pressed.\n
            selected_index (int, optional): default=None, Any pre-selected label index.\n
            line_height (float, optional): defalt=1.0, sets the line height of the radio.\n
            selected (bool, optional): default=False, initial state of the radio.\n
            show (bool, optional): default=True, shows or hides widget.\n
            size (f32, optional): size of the round radio.\n
            spacing (f32, optional): default=15.0, spacing around the radio.\n
            text_shaping (str, optional): default="Basic", other value is "Advanced", requires adding fonts.\n
            text_size (f32, optional): default=15.0, size of the text.\n
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n
            show: (bool, optional) default=True, Show or hides the widget.\n
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n
        Return: 
            int: internal id of widget and can be used by user if equated or use user_id.\n
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
                            width: float=None,
                            height: float=None,
                            width_fill: bool=False,
                            height_fill: bool=False, 
                            h_align: str="left",
                            v_align: str="top", 
                            line_height: float=1.3, 
                            shaping: str="basic",
                            size: float=16.0, 
                            user_id: str=None,
                            show: bool=True,  
                            user_data: any=None,
                            ) -> int:
        """
        Adds a selectable text widget.  This selectable text allows more mouse interaction than
        a button with a style of text only.

        Args:
            parent_id (str, required): id of another container or window.\n 
            text (str, required): The text as wanted.\n
            on_press (Callable, optional): default=None, what function to use after left mouse button pressed.\n 
            on_release (Callable, optional): default=None, what function to use after left mouse button released.\n 
            on_right_press (Callable, optional): default=None, what function to use after right mouse button pressed.\n 
            on_right_release (Callable, optional): default=None, what function to use after right mouse button released.\n  
            on_middle_press (Callable, optional): default=None, what function to use after middle mouse button pressed.\n 
            on_middle_release (Callable, optional): default=None, what function to use after middle mouse button released.\n
            width (float, optional): default=None, width of the widget.\n
            height (float, optional): default=None, height of the widget.\n   
            width_fill (bool, optional): default=False, Fill available width.\n
            height_fill (bool, optional): default=False, Fill available height.\n
            h_align (str, optional): default="left", Horizontal alignment, left, center, right.\n
            v_align (str, optional): default="top", vertical alignment, top, center, bottom.\n
            line_height (float, optional): default=1.3, size of the box the text is in.\n
            shaping (str, optional): shaping of text.\n
            size (float, optional) default=16.0, text size.\n
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n  
            show (bool, optional) default=True, To show the widget or not.\n
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n   
        Return: 
            int: internal id of widget and can be used by user if equated or use user_id.\n
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
                   user_id: str=None, 
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
            parent_id (str, required): id of another container or window.\n 
            min (float, required): The minimum value wanted.\n
            max (float, required): The maximum value wanted.\n
            step (float, required): The step size.\n 
            value (float, required): The starting value.\n 
            connect_progress_bar (int, optional): default=None, If connected then the pg_bar's id.\n 
            show (bool, optional) default=True, Tos show or not.\n
            on_change (Callable, optional): default=None, If set, will use callback with each change.\n 
            on_release (Callable, optional): default=None, If set, will use callback when released.\n 
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n 
            width (float, optional): default=100.0\n   
            width_fill (bool, optional): default=False;  Fill overides width when set to True.\n
            height (float, optional): default=20.0; The height of the slider.\n
        Return: 
            int: internal id of widget and can be used by user if equated or use user_id.\n
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
            parent_id (str, required): id of another container or window.\n
            width (float, optional): default= None, Horizontal size of the sapce.\n
            height (float, optional): default=None, Vertical size of the space.\n
            width_fill (bool, optional): default=False, Fill the available space horizontally.\n
            height_fill (bool, optional): default=False, Fill the available space vertically.\n
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
                  user_id: str=None,
                  ) -> int:

        """
        Adds a table to the gui.

        Args:
            parent_id (str, required): id of another container or window.\n
            title (str, required): Title used for table.\n
            data (List[Dict], required): A list of dictionaries.\n
            width (float, required): Width of the table.\n
            height (float, required): Height of the table.\n
            callback (Callable, optional): defalut=None, callback used when actions on table are intiated.\n
            column_widths (List[float], optional): If only one value is supplied [20.0], thats the default.\n
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n
            show (bool, optional): default=show, whether to show the widget or not.\n
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n 
        Return: 
            int: internal id of widget and can be used by user if equated or use user_id.\n
        """

    def update_table(self,
                     id: int, 
                     title: str=None, 
                     columns: List=None, 
                     data: List=None, 
                     user_id: str=None, 
                     callback: Callable=None
                     ) -> int:
        """
        Updates a table
        """

    def add_text(self,
                 parent_id: str,
                 content: str,
                 *,
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
                 user_id: str="",
                 ) -> int:
        """
        Adds a text widget to the gui.

        Args:
            parent_id (str, required): id of another container or window.\n
            content (str, required): text to use.\n
            width (float, optional): default=None, width of the widget.\n
            height (float, optional): default=None, height of the widget.\n   
            width_fill (bool, optional): default=False, Fill available width.\n
            height_fill (bool, optional): default=False, Fill available height.\n
            h_align (str, optional): "Left"(default), "Center", or "Right"\n
            v_align (str, optional): default="top", aligns items vertically.\n
            line_height (float, optional): defalt=1.0, sets the line height of the text.\n
            size (f32, optional): default=16.0, text size.\n
            shaping (str, optional): default="Basic", other value is "Advanced", requires adding fonts.\n
            text_size (f32, optional): default=16.0, size of the text.\n
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n
            show (bool, optional): default=True, shows or hides widget.\n
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n
        Return: 
            int: internal id of widget and can be used by user if equated or use user_id.\n

        """

    def add_text_editor(self,
                        parent_id: str,
                        file_name: str,
                        ) -> int:
        """
        Adds a text editor widget to the gui.
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
                       user_id: str="",
                       ) -> int:
        """
        Adds a text_input widget to the gui.  Callbacks on_input and on_submit are
        required for the widget to work.  Using a long lived variable, value = self.value,
        as the text is entered, the value will be updated and displayed via a callback.

        Args:
            parent_id (str, required): id of another container or window.\n
            placeholder (str, required): dimmed text used for instructions.\n
            on_submit (Callable, required): callback that responds on pressing enter.\n
            on_input (Callable, required): callback that responds for each typed letter.\n
            on_paste (Callable, optional): default=None, callback that responds on a paste.\n
            line_height (float, optional): defalt=1.0, sets the line height of the text.\n
            width (float, optional): default=None, width of the widget.\n   
            width_fill (bool, optional): default=False, Fill available width.\n
            padding ([float], optional): default=[10.0], Padding for container.\n
                                        use [float] for all sides,\n
                                        use [float, float] for [top&bottom, left&right]\n
                                        use [float, float, float, float] for [top, right, bottom, left]\n
            size (f32, optional): default=16.0, text size.\n
            user_data (Any, optional): Any data in any form needed by user to be passed through as a callback.\n
            is_secure (bool, optional): hides the text as typed, for passwords, etc.
            user_id (str, optional): literal used to identify widget instead of using the \n
                                     returned integer, if variable is inconvenient.\n
        Return:
            int: internal id of widget and can be used by user if equated or use user_id.\n
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
        Add a keyborad event handler to process keyboard actions.
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
        Add a mouse handler to process mouse actions.
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
        Add a window event to process window actions such as resize, move, etc.
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


    def get_card_style(self, Primary: bool=False, Secondary: bool=False, Success: bool=False, 
                        Danger: bool=False, warning: bool=False, info: bool=False, light: bool=False, 
                        dark: bool=False, white: bool=False, default: bool=False):
        """
        Gets the card style.  Set any style to True to active.
        If you activate more than one, the first item will be chosen.
        Define this first by equating to a variable them put in the card style parameter.
        ALternately, put the function in the style parameter.
        """

    def get_window_theme(self, Light=False, Dark=False, Dracula=False, Nord=False,SolarizedLight=False,
                        SolarizedDark=False, GruvboxLight=False,GruvboxDark=False,CatppuccinLatte=False,
                        CatppuccinFrappe=False, CatppuccinMacchiato=False, CatppuccinMocha=False,
                        TokyoNight=False,TokyoNightStorm=False, TokyoNightLight=False, KanagawaWave=False,
                        KanagawaDragon=False, KanagawaLotus=False, Moonfly=False,Nightfly=False,Oxocarbon=False):
        """
        Gets the window them.  Set any theme to True to active.  
        If you activate more than one, the first item will be chosen.
        Define this first by equating to a variable them put in the window theme parameter.
        ALternately, put the function in the style parameter.
        """
