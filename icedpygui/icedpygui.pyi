from types import Callable, Union, List
from typing import OrderedDict, Tuple



class IPG:
    """
    Main class that is instantiated and that calls the corresponding rust file to implement the widgets
    """
    def __init__(self) -> None:
        ...

    def start_session(self) -> None:
        """
        Starts the gui session.  Must be the last called.

        Returns
        -------
        None
        """

    def generate_id(self) -> int:
        """
        Generates an id for some future widget

        Returns
        -------
        id: int
            Pre-generated id to use for a widget with parameter gen_id.
        """

    def add_window(self,
                    window_id: str,
                    title: str,
                    width: int,
                    height: int,
                    pos_x: Union[None | float]=None,
                    pos_y: Union[None | float]=None,
                    pos_centered: bool=False,
                    resizable: bool=True,
                    theme: IpgWindowThemes=IpgWindowThemes.Dark,
                    debug: bool=False,
                    show: bool=True,
                    ) -> int:
        """
        Adds a window to the gui.

        Parameters
        ----------
            window_id: str
                Id of the window to place container in.
            title: str
                Sets the title placed in the top bar of the window.
            width: int
                Sets the width of the window.
            height: int
                Sets the height of the window.
            pos_x: float
                Sets the x position of window.  The window position will be defaulted to 
                a system default unless position is set.
            pos_y: float
                Sets the y position of window.  The window position will be defaulted to 
                a system default unless position is set.
            pos_centered: bool
                Sets the position of window to be centered.
            resizable: bool
                Sets whether the window can be resized.
            theme: IpgWindowThemes
                Sets the style of the window.
            show: bool
                Sets whether the window is showm or hides, the first window is always shown.
            debug: bool
                If set, draws a box around widgets to see the layout.
        
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_container(self,
                        window_id: str,
                        container_id: str,
                        *,
                        parent_id: Union[None | str]=None,
                        width: Union[None | float]=None,
                        height: Union[None | float]=None,
                        width_fill: bool=False,
                        height_fill: bool=False,
                        max_height: float=float('inf'),
                        max_width: float=float('inf'),
                        align_x: IpgContainerAlignment=IpgContainerAlignment.Center,
                        align_y: IpgContainerAlignment=IpgContainerAlignment.Center,
                        padding: List=[10.0], 
                        show: bool=True,
                        ) -> int:
        """
        Adds a generic container to the gui
        Note: A container unlike a row or column can only have 1 child.
            The container is used to help with widget alignments.
            You can align 1 widget or many if you add a row or column to the
            container and use the horizontal and vertical alignments.

        Parameters
        ----------
            window_id: str
                Id of the window to place container in.
            container_id: str
                The id of the container.
            parent_id: str
                If parent_id == window_id then not required, 
                If another container then required.
            width: float
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            height: float
                Sets the height of the widget.   
            height_fill: bool
                Sets the heigth to fill the available space, overrides height.
            max_width: float
                Sets the maximum width the container is allowed to be.
            max_height: float
                Sets a maximum height the container is allowed to be.
            align_x: IpgContainerAlignment
                Aligns the container horizontally; Start, Center, End
            align_y: IpgContainerAlignment
                Aligns the container vertically; Start, Center, End
            padding: List[float]
                Sets the padding for container.
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
            show: bool
                Shows or hides container and all of its contents.
            
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_column(self,
                    window_id: str,
                    container_id: str,
                    *,
                    parent_id: Union[None | str]=None,
                    align_items: IpgColumnAlignment=IpgColumnAlignment.Start,
                    width: Union[None | float]=None,
                    height: Union[None | float]=None,
                    width_fill: bool=False,
                    height_fill: bool=False,
                    max_width: float=float('inf'),
                    padding: List=[10.0], 
                    spacing: float=20.0,
                    show: bool=True,
                    ) -> int:

        """ 
        Adds a Column which is a container that distributes its contents vertically
        
        Parameters
        ----------
            window_id: str
                Id of the window to place container in.
            container_id: str
                The id of the container.
            parent_id: str
                If parent_id == window_id then not required, 
                If another container then required.
            align_items: IpgColumnAlignment
                Sets the vertical alignment of the items in the column; Start, Center, End.
            width: float
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            height: float
                Sets the height of the widget.   
            height_fill: bool
                Sets the heigth to fill the available space, overrides height.
            max_width: float
                Sets the maximum width the container is allowed to be.
            padding: List[float]
                Sets the padding for container.
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
            spacing: float
                Sets the spacing between items in column.
            show: bool
                Shows or hides widget.
            
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_mousearea(self,
                        window_id: str,
                        container_id: str,
                        *,
                        parent_id: Union[None | str]=None,
                        gen_id: Union[None | int]=None,
                        on_press: Union[None | Callable]=None, 
                        on_release: Union[None | Callable]=None, 
                        on_right_press: Union[None | Callable]=None, 
                        on_right_release: Union[None | Callable]=None, 
                        on_middle_press: Union[None | Callable]=None, 
                        on_middle_release: Union[None | Callable]=None,
                        on_enter: Union[None | Callable]=None,
                        on_move: Union[None | Callable]=None,
                        on_exit: Union[None | Callable]=None,  
                        show: bool=True,  
                        user_data: Union[None | any]=None,
                      ) -> int:
        """
        Adds a mouse area where the mouse actions have a callback.  
        The mouse area is like a container, you can add one or more items
        into the area, either widgets, containers, or containers with widgets.  
        The mouse area will assume the size of the items or items
        you add.  Typically you'll probably only add one item but you could do
        more, if needed.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            image_path: str
                Path to where the image is.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            on_press: Callable
                Function to call for left mouse button pressed. 
            on_release: Callable
                Function to call for left mouse button released. 
            on_right_press: Callable
                Function to call for right mouse button pressed. 
            on_right_release: Callable
                Function to call for right mouse button released.  
            on_middle_press: Callable
                Function to call for middle mouse button pressed. 
            on_middle_release: Callable
                Function to call for middle mouse button released.
            on_enter: Callable
                Function to call for mouse enters in text area.
            on_move: Callable
                Function to call for mouse moves in text area.
            on_exit: Callable
                Function to call for mouse exits text area.
            show: bool
                To show the widget or not.
            user_data: any
                Any data that might be needed in the callback function.

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_row(self,
                window_id: str,
                container_id: str,
                *,
                parent_id: Union[None | str]=None,
                align_items: IpgRowAlignment=IpgRowAlignment.Start,
                width: Union[None | float]=None,
                height: Union[None | float]=None,
                width_fill: bool=False,
                height_fill: bool=False,
                padding: List=[10.0], 
                spacing: float=20.0,
                show: bool=True,
                ) -> int:

        """
        Adds a row container to the gui.  Aligns widget horizontally.
        
        Parameters
        ----------
            window_id: str
                Id of the window to place container in.
            container_id: str
                The id of the container.
            parent_id: str
                If parent_id == window_id then not required, 
                If another container then required.
            align_items: IpgRowAlignment
                Sets the alignment Start, Center, or End.
            width: float
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            height: float
                Sets the height of the widget.   
            height_fill: bool
                Sets the heigth to fill the available space, overrides height.
            padding: List[float]
                Sets the padding for container.
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
            spacing: float
                Sets the spacing between items in row.
            show: bool
                Shows or hides widget.
            
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_scrollable(self,
                        window_id: str,
                        container_id: str,
                        *,
                        parent_id: str=None,
                        width: Union[None | float]=None,
                        height: Union[None | float]=None,
                        width_fill: bool=False,
                        height_fill: bool=False,
                        direction: IpgScrollableDirection=IpgScrollableDirection.Vertical,
                        h_bar_width: float=10.0,
                        h_bar_margin: float=0.0,
                        h_scroller_width: float=10.0,
                        h_bar_alignment: IpgScrollableAlignment=IpgScrollableAlignment.Start,
                        v_bar_width: float=10.0,
                        v_bar_margin: float=0.0,
                        v_scroller_width: float=10.0,
                        v_bar_alignment: IpgScrollableAlignment=IpgScrollableAlignment.Start,
                        on_scroll: Union[None | Callable]=None,
                        ) -> int:
        """
        Wraps a scrollable widget around a container.

        Parameters
        ----------
            window_id: str
                Id of the window to place container in.
            container_id: str
                The id of the scrollable.
            parent_id: str
                If parent_id == window_id then not required, 
                If another container then required.
            width: float
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            height: float
                Sets the height of the widget.   
            height_fill: bool
                Sets the heigth to fill the available space, overrides height.
            direction: IpgScrollableDirection
                Sets the direction of the scrollable, Vertical, Horizontal, Both.
            h_bar_width: float
                Sets the horizontal bar width.
            h_bar_margin: float
                Sets the horizontal bar margin.
            h_scroller_width: float
                Sets the horizontal scroller bar width
            h_bar_alignment: IpgScrollableAlignment
                Sets the horizontal bar alignment Start or End
            v_bar_width: float
                Sets the vertical bar width.
            v_bar_margin: float
                Sets the vertical bar margin.
            v_scroller_width: float
                Sets the vertical scroller bar width
            v_bar_alignment: IpgScrollableAlignment
                Sets the vertical bar alignment Start or End
            on_scroll: Callable
                The callback function that is called when scrolling occurs
            user_data: any 
                Any data in any form needed by user to be passed through as a callback. 
            show: bool
                Shows or hides widget.
    
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_tool_tip(self,
                        window_id,
                        container_id: str,
                        position: str,
                        text_to_display: str,
                        *,
                        parent_id: Union[None | str]=None,
                        gap: int=10,
                        style: str="box",
                     ) -> int:
    
        """
        Adds a tooltip container to the widget.

        Parameters
        ----------
            window_id: str
                Id of the window to place container in.
            container_id: str
                The id of the tooltip.
            position: str
                The position of the tooltip, bottom, top, left, right, or float.
            text_to_display: str
                Sets the tooltip message.
            parent_id: str
                Id of another container, if not placing in a window.
            gap: int
                Sets the distance away from the widget.
            style: str
                Sets the style of the tooltip.

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_button(self,
                    parent_id: str,
                    label: str,
                    *,
                    gen_id: Union[None | int]=None,
                    on_press: Union[None | Callable]=None,
                    width: Union[None | float]=None,
                    height: Union[None | float]=None,
                    width_fill: bool=False,
                    height_fill: bool=False,
                    padding: List=[10.0],
                    corner_radius: float=15.0,
                    style: IpgButtonStyles=IpgButtonStyles.Primary,
                    arrow_style: any=Union[None | IpgButtonArrows],
                    user_data: Union[None | any]=None,
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
            gen_id: int
                The only allowable entry for this id is the one generated by ipg.generate_id().
            on_press: Callable
                The function called when the button is pressed.
            width: float
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
            user_data: any 
                Any data in any form needed by user to be passed through as a callback. 
            show: bool
                Shows or hides widget.
            
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_card(self,
                    parent_id, 
                    head, 
                    body, 
                    foot: Union[None | str]=None,
                    gen_id: Union[None | int]=None,
                    is_open: bool=True, 
                    close_size: float=0.0, 
                    on_close: Union[None | Callable]=None,
                    width: Union[None | float]=None, 
                    height: Union[None | float]=None, 
                    max_width: float="inf",
                    max_height: float="inf", 
                    padding_head: float=5.0, 
                    padding_body: float=5.0, 
                    padding_foot: float=5.0,
                    show: bool=True,
                    style: IpgCardStyles=IpgCardStyles.Primary, 
                    user_data: Union[None | any]=None, 
                ) -> int:
        """
        Adds a card to hold text strings.  No widgets can be added at this time.

        Parameters
        ----------
            parent_id: str
                id of another container or window.
            head: str
                Sets the text in the header of the card.
            body: str
                Sets the text in the body of the card.
            foot: str
                Sets the text in the footer of the card.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            is_open: bool
                Determines if the card is opened or minimized.
            close_size: float
                The size of the close icon.
            on_close: Union[None | Callable]
                The callback function called when the card is minimized.
            width: Union[None | float]
                Sets the width of the card 
            height: Union[None | float]
                Sets the height of the card.
            max_width: float
                Sets the maximum width of the card.
            max_height: float
                Sets the maximum height of the card.
            padding_head: float
                Sets the padding around the head. 
            padding_body: float
                Sets the padding around the body. 
            padding_foot: float
                Sets the padding around the footer.
            show: bool
                Shows or hides the card.
            style: IpgCardStyles
                Sets the style of the card. 
            user_data: any 
                Any data in any form needed by user to be passed through as a callback. 

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_checkbox(self,
                     parent_id: str,
                     *,
                     on_toggle: Union[None | Callable] = None,
                     is_checked: bool=False,
                     label: Union[None | str]=None,
                     gen_id: Union[None | int]=None,
                     width: Union[None | float]=None,
                     width_fill: bool=False,
                     size: float=16.0,
                     spacing: float = 15.0,
                     text_line_height: float=1.3,
                     text_shaping: str = "basic",
                     text_size: float=16.0,
                     icon_x: bool=False,
                     icon_size: float=25.0,
                     user_data: Union[None | any]=None,
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
            gen_id: int
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
            user_data: any 
                Any data in any form needed by user to be passed through as a callback. 
            show: bool
                Shows or hides widget.
           
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """
    
    def add_color_picker(
                        self,
                        parent_id: str,
                        *,
                        label: str="Set Color",
                        gen_id: Union[None | int]=None,
                        on_submit: Union[None | Callable]=None,
                        width: Union[None | float]=None,
                        height: Union[None | float]=None,
                        width_fill: bool=False,
                        height_fill: bool=False,
                        padding: List=[10.0],
                        corner_radius: float=0.0,
                        style: str="primary",
                        user_data: Union[None | any]=None,
                        show: bool=True, 
                        ) -> int:
        """
        Adds a color picker.  The args for style and such are for the activation button.
        No styling for the Date Picker itself is available at this time.
        
        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            label: str
                The label for the button which activates the picker.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            on_submit: Callable
                The function that executs when the submit button is pressed.
            width: float
                Sets the width of the button.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            height: float
                Sets the height of the button.   
            height_fill: bool
                Sets the heigth to fill the available space, overrides height.
            padding: List[float]
                Sets the padding for the widget.
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
            corner_radius: float
                Sets the roundness of the corners of the button.
            style: str
                Sets the style of the button.
            show: bool
                To show the widget or not.
            user_data: any
                Any data that might be needed in the callback function.
            
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_date_picker(self,
                        parent_id: str,
                        *,
                        label: str="Calendar",
                        gen_id: Union[None | int]=None,
                        size_factor: float=1.0,
                        padding: List=[5.0],
                        on_submit: Union[None | Callable]=None,
                        user_data: Union[None | any]=None,
                        show=False, 
                        )  -> int:
        
        """
        Adds a date_picker widget.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            label: str
                The label name of the button that activates the date picker.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            size_factor: float
                The size of the displayed calendar, must be > 1.0.
            padding: List[float]
                The padding around the calendar.
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
            on_submit: Callable
                Callback function selected date is submitted.
            show: bool
                To show the widget or not.
            user_data: any
                Any data that might be needed in the callback function.

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """
    
    def add_image(self, 
                    parent_id: str, 
                    image_path: str,
                    *,
                    gen_id: int=None, 
                    on_press: Union[None | Callable]=None, 
                    on_release: Union[None | Callable]=None, 
                    on_right_press: Union[None | Callable]=None, 
                    on_right_release: Union[None | Callable]=None, 
                    on_middle_press: Union[None | Callable]=None, 
                    on_middle_release: Union[None | Callable]=None,
                    on_enter: Union[None | Callable]=None,
                    on_move: Union[None | Callable]=None,
                    on_exit: Union[None | Callable]=None, 
                    width: Union[None | float]=None,
                    height: Union[None | float]=None,
                    width_fill: bool=False,
                    height_fill: bool=False, 
                    show: bool=True,  
                    user_data: Union[None | any]=None,
                    ) -> int:
        """
        Adds an image widget.  The image is selectable using callbacks for all 3 mouse buttons.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            image_path: str
                Path to where the image is.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            on_press: Callable
                Function to call for left mouse button pressed. 
            on_release: Callable
                Function to call for left mouse button released. 
            on_right_press: Callable
                Function to call for right mouse button pressed. 
            on_right_release: Callable
                Function to call for right mouse button released.  
            on_middle_press: Callable
                Function to call for middle mouse button pressed. 
            on_middle_release: Callable
                Function to call for middle mouse button released.
            on_enter: Callable
                Function to call for mouse enters in text area.
            on_move: Callable
                Function to call for mouse moves in text area.
            on_exit: Callable
                Function to call for mouse exits text area.
            width: float
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            height: float
                Sets the height of the widget.   
            height_fill: bool
                Sets the heigth to fill the available space, overrides height.
            show: bool
                To show the widget or not.
            user_data: any
                Any data that might be needed in the callback function.

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_menu(self, 
                 parent_id: str,  
                 items: OrderedDict[str, list[str]],
                 widths: list,
                 spacing: list,
                 separators: Union[None | List[Tuple(int, int, IpgMenuSepTypes)]]=None,
                 sep_types: Union[None | List[IpgMenuSepTypes]]=None,
                 sep_label_names: Union[None | List[str]]=None, 
                 on_select: Union[None | Callable]=None, 
                 gen_id: Union[None | int]=None,
                 user_data: Union[None | any]=None,
                 ) -> int:
        """
        Add a menu dropdown list to the gui.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            items: OrderedDict[str, list[str]]
                Sets the menu items with the key being the bar item and values the menu items.
            widths: list[float]
                The widths of the bar items.
            spacing: list[float]
                The spacing between the menu items in each bar item.
            separators: Union[None | List[Tuple(int, int, IpgMenuSepTypes)]]
                A list of tuples [(bar_index, menu_index, separator type)].  The separator is added
                after the menu index.
            sep_types: Union[None | List[IpgMenuSepTypes]]
                Sets the type of separator line, dot, or label.
            sep_label_names: Union[None | List[str]]
                Sets the separators label names, if type label is used.
            on_select: Union[None | Callable]
                The cllback for when the menu item is selected.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            user_data: any
                Any data in any form needed by user to be passed through as a callback.

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_pick_list(self,
                        parent_id: str,
                        options: List=[str],
                        *,
                        gen_id: Union[None | int]=None,
                        on_select: Union[None | Callable]=None,
                        width: Union[None | float]=None,
                        width_fill: bool=False,
                        padding: List=[5.0],
                        placeholder: Union[None | str]=None,
                        selected: Union[None | str]=None,
                        text_size: float=15.0,
                        text_line_height: float="default",
                        text_shaping: str="basic",
                        user_data: Union[None | any]=None,
                        show: bool=True,
                      ) -> int:
        """
        Adds a pick list to the gui.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            options: List
                List of items to select from.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            on_select: Callable
                Function executed when item selected.
            width: float
                Sets the width of the widget.
            width_fill: bool
                If set, the widget fills the available space.
            padding: List[float]
                Sets the padding for widget.
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
            placeholder: str
                Sets the dimmed text in the box for user info.
            selected: str
                Sets any preselected item.
            text_size: float
                Sets the size of text.
            text_line_height: float
                Sets the height of the box around the text.
            text_shaping: str
                Sets the shape of the text.
            user_data: any
                Any data in any form needed by user to be passed through as a callback.
            show: bool
                Shows or hides the widget.
    
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_progress_bar(self,
                         parent_id: str,
                         min: float,
                         max: float,
                         *,
                         value: float=0.0,
                         gen_id: Union[None | int]=None,
                         width: Union[None | float]=None,
                         width_fill: bool=False,
                         height: float=1.0,
                         user_data: Union[None, any]=None,
                         show: bool=True,
                         ) -> int:
        """
        Adds a progress bar to the gui.
        
        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            min: float
                Sets the minimum value of bar.
            max: float
                Sets the maximum value of bar.
            value: float 
                Sets the starting value of bar.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            width: float
                Sets the width of the widget.
            width_fill: bool
                If set, the widget fills the available space.
            height: float
                Sets the height of the bar.
            user_data: any
                Any data in any form needed by user to be passed through as a callback.
            show: bool
                Shows or hides the widget.
            
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_radio(self,
                    parent_id: str,
                    labels: List=[str],
                    *,
                    gen_id: Union[None | int]=None,
                    direction: IpgRadioDirection=IpgRadioDirection.Vertical,
                    spacing: float=10.0,
                    padding: List=[10.0],
                    width: Union[None | float]=None,
                    width_fill: bool=False,
                    on_select: Union[None | Callable]=None,
                    selected_index: Union[None | int]=None,
                    size: float=20.0,
                    text_spacing: float=15.0,
                    text_size: float=16.0,
                    text_line_height: float=1.3,
                    text_shaping: str="basic",
                    user_data: Union[None, any]=None,
                    show: bool=True,
                  ) -> int:
        """
        Adds a radio button to the gui
        
        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            labels: List[str]
                A list of labels for the radio buttons.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            direction: IpgRadioDirection
                Sets the direction for the radio group.
            spacing: float
                Sets spacing between the radio buttons in the group.
            padding: List[float]
                Sets the padding around the radio group..
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
            width: float
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            on_select: Callable
                Function executed when radio button selected.
            selected_index: int
                A pre-selected label index to be selected.
            line_height:float
                Sets the line height for the box around the radio labels.
            show: bool
                Shows or hides the widget.
            size: float,
                Radius of the round radio button.
            spacing: float
                Spacing between the radio buttons.
            text_shaping: str
                Sets the text shape.
            text_size: float
                Sets the size of the text.
            user_data: any
                Any data that might be needed in the callback function.
            show: bool
                Shows or hides the widget.
    
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_rule_horizontal(self, 
                            parent_id, 
                            *,
                            width: Union[None | float]=None, 
                            width_fill: bool=True
                            ) -> int:
        """
        Add a horizontal line divider.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            width: Union[None | float]
                Defines the horizontal length of the dividing line.
            width_fill: bool
                If set, fills the available space for the horizontal length, overides width.

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_rule_vertical(self, 
                          parent_id,
                          *, 
                          height: Union[None | float]=None, 
                          height_fill: bool=True
                          ) -> int:
        """
        Add a vertical line divider.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            height: Union[None | float]
                Defines the vertical length of the dividing line.
            height_fill: bool
                If set, fills the available space for the vertical length, overides height.

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_selectable_text(self, 
                            parent_id, 
                            text,
                            *,
                            gen_id: Union[None | int]=None, 
                            on_press: Union[None | Callable]=None, 
                            on_release: Union[None | Callable]=None, 
                            on_right_press: Union[None | Callable]=None, 
                            on_right_release: Union[None | Callable]=None, 
                            on_middle_press: Union[None | Callable]=None, 
                            on_middle_release: Union[None | Callable]=None,
                            on_enter: Union[None | Callable]=None,
                            on_move: Union[None | Callable]=None,
                            on_exit: Union[None | Callable]=None, 
                            width: Union[None | float]=None,
                            height: Union[None | float]=None,
                            width_fill: bool=False,
                            height_fill: bool=False, 
                            h_align: str="left",
                            v_align: str="top", 
                            line_height: float=1.3, 
                            shaping: str="basic",
                            size: float=16.0, 
                            show: bool=True,  
                            user_data: Union[None | any]=None,
                            ) -> int:
        """
        Adds a selectable text widget.  This selectable text allows more mouse interaction than
        a button with a style of text only.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            text: str 
                The text needed.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            on_press: Callable
                Function to call for left mouse button pressed. 
            on_release: Callable
                Function to call for left mouse button released. 
            on_right_press: Callable
                Function to call for right mouse button pressed. 
            on_right_release: Callable
                Function to call for right mouse button released.  
            on_middle_press: Callable
                Function to call for middle mouse button pressed. 
            on_middle_release: Callable
                Function to call for middle mouse button released.
            on_enter: Callable
                Function to call for mouse enters in text area.
            on_move: Callable
                Function to call for mouse moves in text area.
            on_exit: Callable
                Function to call for mouse exits text area.
            width: float
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            height: float
                Sets the height of the widget.   
            height_fill: bool
                Sets the heigth to fill the available space, overrides height.
            h_align: str
                Horizontal alignment, left, center, right.
            v_align: str
                Vertical alignment, top, center, bottom.
            line_height: float
                The size of the box the text is in.
            shaping: str
                Shaping of text.
            size: float
                The text size.
            show: bool
                To show the widget or not.
            user_data: any
                Any data that might be needed in the callback function.

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_slider(self, 
                   parent_id: str, 
                   min: float, 
                   max: float, 
                   step: float, 
                   value: float,
                   *,
                   gen_id: Union[None | int]=None, 
                   show: bool=True, 
                   on_change: Union[None | Callable]=None, 
                   on_release: Union[None | Callable]=None, 
                   user_data: Union[None | any]=None, 
                   width: float=100.0,
                   width_fill: bool=False,
                   height: float=20.0
                  ) -> int:
        """
        Adds a slider widget which changes value as the mouse moves the slider.
        The resulting float value can be used by the callbacks to produce the desired results.
        If on_change is used, then the callback is called many times, but only once for the on_release.
        If one wants to connect to a progress bar, then give the connect_progress_bar the id of the pg bar.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            min: float
                The minimum value wanted.
            max: float
                The maximum value wanted.
            step: float 
                The step size. 
            value: float
                The starting value. 
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id(). 
            show:
                shows or hides the widget.
            on_change: Callable
                If set, will use callback with each change. 
            on_release: Callable
                If set, will use callback when released. 
            user_data: any
                Any data that might be needed in the callback function.
            width: float
                Width of the widget.
            height: float
                Height of the widget.
            width_fill: bool 
                Fills the available space horizontally.
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_space(self,
                  parent_id: str,
                  *,
                  gen_id: Union[None | int]=None,
                  width: Union[None | float]=None,
                  height: Union[None | float]=None,
                  width_fill: bool=False,
                  height_fill: bool=False,
                  ) -> int:
        """
        Adda a space between elements for alignment and aesthetics.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            width: float
                Width of the widget.
            height: float
                Height of the widget.
            width_fill: bool 
                Fills the available space horizontally.
            height_fill: bool
                Fills the available space vertically.

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_svg(self, 
                    parent_id: str, 
                    svg_path: str,
                    *,
                    gen_id: int=None, 
                    on_press: Union[None | Callable]=None, 
                    on_release: Union[None | Callable]=None, 
                    on_right_press: Union[None | Callable]=None, 
                    on_right_release: Union[None | Callable]=None, 
                    on_middle_press: Union[None | Callable]=None, 
                    on_middle_release: Union[None | Callable]=None,
                    on_enter: Union[None | Callable]=None,
                    on_move: Union[None | Callable]=None,
                    on_exit: Union[None | Callable]=None, 
                    width: Union[None | float]=None,
                    height: Union[None | float]=None,
                    width_fill: bool=False,
                    height_fill: bool=False, 
                    show: bool=True,  
                    user_data: Union[None | any]=None,
                    ) -> int:
        """
        Adds an image widget.  The image is selectable using callbacks for all 3 mouse buttons.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            image_path: str
                Path to where the image is.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            on_press: Callable
                Function to call for left mouse button pressed. 
            on_release: Callable
                Function to call for left mouse button released. 
            on_right_press: Callable
                Function to call for right mouse button pressed. 
            on_right_release: Callable
                Function to call for right mouse button released.  
            on_middle_press: Callable
                Function to call for middle mouse button pressed. 
            on_middle_release: Callable
                Function to call for middle mouse button released.
            on_enter: Callable
                Function to call for mouse enters in text area.
            on_move: Callable
                Function to call for mouse moves in text area.
            on_exit: Callable
                Function to call for mouse exits text area.
            width: float
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            height: float
                Sets the height of the widget.   
            height_fill: bool
                Sets the heigth to fill the available space, overrides height.
            show: bool
                To show the widget or not.
            user_data: any
                Any data that might be needed in the callback function.

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """
    
    def add_table(self,
                  parent_id: str,
                  title: str,
                  data: List,
                  width: float,
                  height: float,
                  *,
                  row_highlight: Union[None | TableRowHighLight]=None,
                  highlight_amount: float=0.15,
                  widget_header: Union[None | str]=None,
                  widget: Union[None | TableWidget]=None,
                  widget_column: int=0,
                  widget_column_length: int=0,
                  widget_label: Union[None | str]=None,
                  gen_id: Union[None | int]=None,
                  callback: Union[None | Callable]=None,
                  column_widths: List=[20.0],
                  show: bool=True, 
                  user_data: Union[None | any]=None,
                  ) -> int:

        """
        Adds a table to the gui.

        Parameters
        ----------
            parent_id: : Anystr
                Id of another container to place the widget in.
            title: str
                Title used for table.
            data: List[Dict]
                A list of dictionaries, each dictionary contains only one type.
            width: float
                Width of the table.
            height: float
                Height of the table.
            row_highlight: TableRowHighLight
                Highlights alternate row by either drkening or lightening them up.
            highligh_amount: float
                Amount of highlighting to use if row_highlight is set.
            widget_header: str
                If widgets are added, the column header name.
            widget: TableWidget
                The type of widget tp put in column.
            widget_column: int
                The column position where the widgets are placed.
                widget_label: str
                The label for the widget, if any needed. 
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            callback: Callable
                Callback used when actions on table are initiated.
            column_widths: List[float]
                A list of value for the column widths, if only one value is supplied then it will 
                be the default for all columns.
            user_data: any
                Any data that might be needed in the callback function.
            show:: Any
                shows or hides the widget.
            
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """


    def add_text(self,
                 parent_id: str,
                 content: str,
                 *,
                 gen_id: Union[None | int]=None,
                 width: Union[None | float]=None,
                 height: Union[None | float]=None,
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
                id of another container to place the widget in.
            content: str
                Sets the text of the widget.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
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
                shows or hides the widget.
        
        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.

        """

    def add_text_editor(self,
                        parent_id: str,
                        file_name: str,
                        gen_id: Union[None | str]=None,
                        ) -> int:
        """
        Adds a text editor widget to the gui.

        Parameters
        ----------
            parent_id: str
                id of another container to place the widget in.
            file_name: str
                Path to the file to be used.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
        
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
                       gen_id: Union[None | int]=None,
                       on_submit: Union[None | Callable]=None,
                       on_input: Union[None | Callable]=None,
                       on_paste: Union[None | Callable]=None,
                       line_height: str="default",
                       width_fill: bool=False,
                       padding: List=[10.0],
                       size: Union[None, float],
                       user_data: Union[None | any]=None,
                       is_secure: bool=False,
                       ) -> int:
        """
        Adds a text_input widget to the gui.  Callbacks on_input and on_submit are
        required for the widget to work.  Using a long lived variable, value = self.value,
        as the text is entered, the value will be updated and displayed via a callback.

        Parameters
        ----------
            parent_id: str
                id of another container to place the widget in.
            placeholder: str
                text used for instructions in the input box.
           gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
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
            user_data: any
                Any data that might be needed in the callback function.
            is_secure: bool
                Hides the entered text, for passwords, etc.
            
        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.
        """

    def add_timer(self,
                        parent_id: str,
                        duration_ms: int,
                        *,
                        on_start: Union[None | Callable]=None,
                        on_stop: Union[None | Callable]=None,
                        on_tick: Union[None | Callable]=None,
                        start_label: str="Start Timer",
                        stop_label: str="Stop Timer",
                        width: Union[None | float]=None,
                        height: Union[None | float]=None,
                        width_fill: bool=False,
                        height_fill: bool=False,
                        padding: list=[10.0],
                        corner_radius: float=15.0,
                        style: IpgButtonStyles=IpgButtonStyles.Primary,
                        arrow_style: Union[None | IpgButtonArrows]=None,
                        user_data: any=None,
                        ) -> int:
        """
        Adds a timer event in millisecond duration.

        Parameters
        ----------
        parent_id: str
            Id of another container.
        duration_ms: int
            The time when the on_tick function fires.
        on_start: Union[None | Callable]
            The optional function that execute when the timer starts.
        on_stop: Union[None | Callable]
            The optional function that executes when the timer stops.
        on_tick: Union[None | Callable]
            The optional function that executes on every timer tick as indicated by duration_ms.
        start_label: str="Start Timer"
            The default start label of the timer button.
        stop_label: str="Stop Timer"
            The default stop label of the timer button.
        width: Union[None | float]
            Width of the button, the default is the size of the label.
        height: Union[None | float]
            Height of the button, the default is the size of the label.
        width_fill: bool
            Sets the width to fill the container, overrides width.
        height_fill: bool
            Sets the heigth to fill the container, overrides height.
        padding: list
            Sets the padding for widget.
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
        corner_radius: float
            The rounding of the button corners.
        style: IpgButtonStyles
            The button style, defaults to Primary.
        arrow_style: Union[None | IpgButtonArrows]
            Determines if the button is an arrow.
        user_data: any
            Any data the user may need during a callback.

        Return:
        ------- 
            int: internal id of widget and can be used by user if equated.
        """


    def add_toggler(self,
                    parent_id: str,
                    *,
                    label: Union[None | str]=None,
                    gen_id: Union[None | str]=None,
                    toggled: Union[None | Callable]=None,
                    width: Union[None | float]=None,
                    width_fill: bool=False,
                    user_data: Union[None | any]=None,
                    show: bool=True, 
                    ) -> int:
        """
        Adds a toggler to the gui
        
        Parameters
        ----------
            parent_id: str
                Id of another container.
            label: str
                label of toggler.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            toggled: Callable
                The function called when the button is pressed.
            width: float
                Sets the width of the widget.
            width_fill: bool
                Sets the width to fill the available space, overrides width.
            height: float
                Sets the height of the widget.   
            user_data: any 
                Any data in any form needed by user to be passed through as a callback. 
            show: bool
                Shows or hides widget.
            
        Return:
        ------- 
            int: internal id of widget and can be used by user if equated.
        """
    
    # *******************************events*************************************************************
    def add_event_keyboard(self,
                            enabled: bool,
                            *,
                            on_key_press: Union[None | Callable]=None,
                            on_key_release: Union[None | Callable]=None,
                            user_data: Union[None | any]=None, 
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
        user_data: any
            Any data that might be needed in the callback function.

        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.
        """

    def add_event_mouse(self,
                            enabled: bool,
                            *,
                            on_move: Union[None | Callable]=None,
                            on_left_press: Union[None | Callable]=None,
                            on_left_release: Union[None | Callable]=None,
                            on_middle_press: Union[None | Callable]=None,
                            on_middle_release: Union[None | Callable]=None,
                            on_right_press: Union[None | Callable]=None,
                            on_right_release: Union[None | Callable]=None,
                            on_middle_scroll: Union[None | Callable]=None,
                            user_data: Union[None | any]=None,
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
        user_data: any
            Any data that might be needed in the callback function.
        
        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.
        """

    def add_event_window(self,
                         enabled: bool,
                         *,
                        on_open: Union[None | Callable]=None,
                        on_close: Union[None | Callable]=None,
                        on_moved: Union[None | Callable]=None,
                        on_resized: Union[None | Callable]=None,
                        user_data: Union[None | any]=None,
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
        user_data: any
            Any data that might be needed in the callback function.
        
        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.
        """


    # *******************************all item ops**************************************
        
    def delete_item(self, wid: int):
        """
        Deletes an item using the widgets id.
        Example: btn_id = add_button("Button")
                 delete_item(btn_id)

        Parameters
        ----------
        wid: int
            The widget id of the widget to be updated.

        Returns
        -------
        None
        """

    def update_item(self, 
                    wid: int, 
                    param: str, 
                    value: any,
                    ):
        """
        Update a widget by supplying the widget id, wid, the parameter to update, 
        a class property value, and a value based on the type of value used by the widget.
        
        Parameters
        ----------
        wid: int
            The widget id of the widget to be updated.
        param: class property
            Example: a button has a style class IpgButtonParams with properties of Primary, ...
        value: any 
            Any value which matches that used by the widget.  For example, to set a checkbox to true,
            param=IpgCheckboxParams.IsChecked, value=True  

        Returns
        -------
        None
        """


class IpgAlignment:
    Left=0
    Center=0
    Right=0


class IpgWindowThemes:
    Dark=0
    Light=0
    CatppuccinLatte=0
    CatppuccinFrappe=0
    CatppuccinMacchiato=0
    CatppuccinMocha=0
    Dracula=0
    Ferra=0
    GruvboxLight=0
    GruvboxDark=0
    KanagawaWave=0
    KanagawaDragon=0
    KanagawaLotus=0
    Moonfly=0
    Nightfly=0
    Nord=0
    Oxocarbon=0
    SolarizedLight=0
    SolarizedDark=0
    TokyoNight=0
    TokyoNightStorm=0
    TokyoNightLight=0
    
    
class IpgButtonStyles:
    Primary=0
    Secondary=0
    Positive=0
    Destructive=0
    Text=0


class IpgButtonParams:
    ArrowStyle=0
    CornerRadius=0
    Height=0
    HeightFill=0
    Label=0
    Padding=0
    Show=0
    Style=0
    Width=0
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
    Body=0
    Foot=0
    Head=0
    IsOpen=0
    Style=0


class IpgCheckboxParams:
    IconSize=0
    IconX=0
    IsChecked=0
    Label=0
    Show=0
    Size=0
    Spacing=0
    Style=0
    TextLineHeight=0
    TextShaping=0
    TextSize=0
    Width=0
    WidthFill=0


class IpgColumnAlignment:
    Start=0
    Center=0
    End=0


class IpgContainerAlignment:
    Start=0
    Center=0
    End=0


class IpgContainerTheme:
    Default=0
    Custom=0


class IpgDatePickerParams:
    Label=0
    Padding=0
    SizeFactor=0
    Show=0


class IpgImageParams:
    Height=0
    HeightFill=0
    ImagePath=0
    Padding=0
    Show=0
    Width=0
    WidthFill=0


class IpgMenuParams:
    MenuUpdate=0
    Separators=0
    Spacing=0
    Widths=0


class IpgMenuSepTypes:
    Line=0
    Dot=0
    Label=0


class IpgMouseAreaParams:
    show=0


class IpgPickListParams:
    Options=0
    Placeholder=0
    Padding=0
    Show=0
    TextSize=0
    TextLineHeight=0
    Width=0
    Delete=0


class IpgProgressBarParams:
    Height=0
    Min=0
    Max=0
    Show=0
    Value=0
    Width=0
    WidthFill=0


class IpgRadioDirection:
    Horizontal=0
    Vertical=0


class IpgRadioParams:
    Direction=0
    Labels=0
    Padding=0
    SelectedIndex=0
    Show=0
    Size=0
    Spacing=0
    TextSpacing=0
    TextSize=0
    TextLineHeight=0
    UserData=0
    Width=0
    WidthFill=0
    Height=0
    HeightFill=0


class IpgRowAlignment:
    Start=0
    Center=0
    End=0


class IpgScrollableDirection:
    Vertical=0
    Horizontal=0
    Both=0


class IpgScrollableAlignment:
    Start=0
    End=0


class IpgScrollableParams:
    Width=0
    Height=0
    HBarWidth=0
    HBarMargin=0
    HScrollerWidth=0
    HBarAlignment=0
    VBarWidth=0
    VBarMargin=0
    VScrollerWidth=0
    VBarAlignment=0


class IpgSelectableTextParams:
    Text=0
    Width=0
    WidthFill=0
    Height=0
    HeightFill=0
    HorizontalAlign=0
    VerticalAlign=0
    LineHeight=0
    Size=0
    Show=0


class IpgSelectableTextHorAlign:
    Left=0
    Center=0
    Right=0


class IpgSelectableTextVertAlign:
    Top=0
    Center=0
    Bottom=0


class IpgSliderParams:
    Min=0
    Max=0
    Step=0
    Value=0
    Width=0
    WidthFill=0
    Height=0
    Show=0


class IpgSvgParams:
    SvgPath=0
    Width=0
    WidthFill=0
    Height=0
    HeightFill=0
    Show=0


class TableRowHighLight:
    Darker=0
    Lighter=0


class TableWidget:
    Button=0
    Checkbox=0
    Image=0
    Radio=0
    SelectableText=0
    Svg=0

class IpgTextInputParams:
    Placeholder=0
    Value=0
    IsSecure=0
    Width=0
    Padding=0
    Size=0
    LineHeight=0


class IpgTextParams:
    Content=0
    Height=0
    HeightFill=0
    HzAlignLeft=0
    HzAlignCenter=0
    HzAlignRight=0
    LineHeight=0
    Size=0
    VtAlignTop=0
    VtAlignCenter=0
    VtAlignBottom=0
    Width=0
    WidthFill=0
    Show=0


class IpgTogglerParams:
    Alignment=0
    Label=0
    LineHeight=0
    Show=0
    Size=0
    TextSize=0
    Width=0
    WidthFill=0


class IpgWindowParams:
    Debug=0
    Theme=0


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