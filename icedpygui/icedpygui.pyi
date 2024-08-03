from pickletools import uint2
from typing import Any, Callable, Dict, List, OrderedDict, Optional, Tuple, Union



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
                    pos_x: Optional[float]=None,
                    pos_y: Optional[float]=None,
                    pos_centered: bool=False,
                    resizable: bool=True,
                    theme: IpgWindowTheme=IpgWindowTheme.Dark,
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
                        parent_id: Optional[str]=None,
                        width: Optional[float]=None,
                        height: Optional[float]=None,
                        width_fill: bool=False,
                        height_fill: bool=False,
                        max_height: float=float('inf'),
                        max_width: float=float('inf'),
                        horizontal_alignment: Optional[IpgAlignment]=None,
                        vertical_alignment: Optional[IpgAlignment]=None,
                        padding: List=[10.0],
                        clip: bool=False,
                        show: bool=True,
                        style_id: Optional[str]=None,
                        style_standard: Optional[IpgStyleStandard]=None
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
            horizontal_alignment: IpgHorizontalAlignment
                Aligns the container horizontally; Start, Center=default, End
            vertical_alignment: IpgVerticalAlignment
                Aligns the container vertically; Start, Center=default, End
            padding: List[float]
                Sets the padding for container.
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
            clip: bool
                Whether to clip any text if size > container.
            show: bool
                Shows or hides container and all of its contents.
            style_id: str
                style_id of the add_container_style.
            style_Standard: str
                IpgStyleStandard class.
            
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_container_style(self,
                            style_id: str, 
                            *,
                            background_color: Optional[IpgColor]=None,
                            background_rgba: Optional[list[float, 4]]=None,
                            border_color: Optional[IpgColor]=None, 
                            border_rgba: Optional[list[float, 4]]=None,
                            border_radius: list[float]=[0.0], 
                            border_width: float=1.0,
                            shadow_color: Optional[IpgColor]=None, 
                            shadow_rgba: Optional[list]=None,
                            shadow_offset_x: float=0.0, 
                            shadow_offset_y: float=0.0,
                            shadow_blur_radius: float=1.0,
                            text_color: Optional[IpgColor]=None, 
                            text_rgba: Optional[list[float, 4]]=None,
                        ) -> int:
        """
        Adds styling to container

        Parameters
        ----------
            style_id: str
                Id of container_style. 
            background_color: IpgColor]
                The Color::TRANSPARENT is used unless defined.
            background_rgba: list,
                The color in rgba format [float; 4] used as state above.
            border_color: IpgColor
                Color used for the border.
            border_rgba: list
                The color in rgba format [float; 4] used as state above.
            border_radius: list
                The radius of the 4 corners, [float]=all corners, 
                [floate;4] top-left, top-right, bottom-right, bottom-left.
            border_width: float
                Border width.
            shadow_color: IpgColor
                The color of the shadow.
            shadow_rgba: list
                The color in rgba format [float; 4] used as state above.
            shadow_offset_x: float
                Shadow offset in the horizontal direction.
            shadow_offset_y: float
                Shadow offset in the vertical direction.
            shadow_blur_radius: float
                The blur radius of the shadow.
            text_color: IpgColor
                The text color, if not defined, will either be a Black or White variation based on theme background.
            text_rgba: list]
                The color in rgba format [float; 4] used as state above.
        """

    def add_column(self,
                    window_id: str,
                    container_id: str,
                    *,
                    parent_id: Optional[str]=None,
                    align_items: IpgAlignment=IpgAlignment.Start,
                    width: Optional[float]=None,
                    height: Optional[float]=None,
                    width_fill: bool=False,
                    height_fill: bool=False,
                    max_width: float=float('inf'),
                    padding: list=[10.0], 
                    spacing: float=20.0,
                    clip: bool=False,
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
            align_items: IpgAlignment
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
            clip: bool
                Whether to clip any text if size > container.
            show: bool
                Shows or hides widget.
            
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_modal(self,
                    window_id: str,
                    container_id: str,
                    label: str,
                    *,
                    parent_id: Optional[str]=None,
                    on_open: Optional[Callable]=None,
                    align_items: IpgAlignment=IpgAlignment.Start,
                    width: Optional[float]=None,
                    height: Optional[float]=None,
                    width_fill: bool=False,
                    height_fill: bool=False,
                    max_width: float=float('inf'),
                    padding: List=[10.0], 
                    spacing: float=20.0,
                    clip: bool=False,
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
            label: str
                Label for the button to open the modal.
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
            clip: bool
                Whether to clip any text if size > container.
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
                        parent_id: Optional[str]=None,
                        gen_id: Optional[int]=None,
                        on_press: Optional[Callable]=None, 
                        on_release: Optional[Callable]=None, 
                        on_right_press: Optional[Callable]=None, 
                        on_right_release: Optional[Callable]=None, 
                        on_middle_press: Optional[Callable]=None, 
                        on_middle_release: Optional[Callable]=None,
                        on_enter: Optional[Callable]=None,
                        on_move: Optional[Callable]=None,
                        on_exit: Optional[Callable]=None,  
                        show: bool=True,  
                        user_data: Optional[any]=None,
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
                parent_id: Optional[str]=None,
                align_items: IpgAlignment=IpgAlignment.Start,
                width: Optional[float]=None,
                height: Optional[float]=None,
                width_fill: bool=False,
                height_fill: bool=False,
                padding: List=[10.0], 
                spacing: float=20.0,
                clip: bool=False,
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
                        width: Optional[float]=None,
                        height: Optional[float]=None,
                        width_fill: bool=False,
                        height_fill: bool=False,
                        direction: IpgScrollableDirection=IpgScrollableDirection.Vertical,
                        h_bar_width: float=10.0,
                        h_bar_margin: float=0.0,
                        h_scroller_width: float=10.0,
                        h_bar_alignment: IpgAlignment=IpgAlignment.Start,
                        v_bar_width: float=10.0,
                        v_bar_margin: float=0.0,
                        v_scroller_width: float=10.0,
                        v_bar_alignment: IpgAlignment=IpgAlignment.Start,
                        on_scroll: Optional[Callable]=None,
                        style_id: Optional[str]=None,
                        user_data: Optional[any]=None,
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
            style_id: str
                Style_id from add_scrollable_style
            user_data: any 
                Any data in any form needed by user to be passed through as a callback. 
            show: bool
                Shows or hides widget.
    
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_scrollable_style(self,
                            style_id: str,
                            background_color: Optional[IpgColor]=None,
                            background_rgba: Optional[list[float, 4]]=None,
                            border_color: Optional[IpgColor]=None,
                            border_rgba: Optional[list[float, 4]]=None,
                            border_radius: list[float]=0.0,
                            border_width: float=1.0,
                            shadow_color: Optional[IpgColor]=None,
                            shadow_rgba: Optional[list[float, 4]]=None,
                            shadow_offset_x: float=0.0,
                            shadow_offset_y: float=0.0,
                            shadow_blur_radius: float=2.0,
                            text_color: Optional[IpgColor]=None,
                            text_rgba: Optional[list[float, 4]]=None,
                            scrollbar_color: Optional[IpgColor]=None,
                            scrollbar_rgba: Optional[list[float, 4]]=None,
                            scrollbar_border_radius: list[float]=[2.0],
                            scrollbar_border_width: float=1.0,
                            scrollbar_border_color: Optional[IpgColor]=None,
                            scrollbar_border_rgba: Optional[list[float, 4]]=None,
                            scroller_color: Optional[IpgColor]=None,
                            scroller_rgba: Optional[list[float, 4]]=None,
                            scroller_color_hovered: Optional[IpgColor]=None,
                            scroller_rgba_hovered: Optional[list[float, 4]]=None,
                            scroller_color_dragged: Optional[IpgColor]=None,
                            scroller_rgba_dragged: Optional[list[float, 4]]=None,
                            gen_id: Optional[int]=None,
                            ) -> int:
        """
        Add styling to the Scrollable widget.

        Parameters
        ----------
        style_id: str,
            Id used in the add_scrollable widget
        background_color: Optional[IpgColor]=None
            Background color of the container.
        background_rgba: Optional[list[float, 4]]=None
            Background color of the container in rgba format.
        border_color: Optional[IpgColor]=None
            Color of the border.
        border_rgba: Optional[list[float, 4]]=None
            Color of the border in rgba format.
        border_radius: list[float]=0.0
            Radius a list of 1=all or 4=[top-left, top-right, bottom-right, bottom-left]
        border_width: float=1.0
            Width of border.
        shadow_color: Optional[IpgColor]=None
            Color of shadow.
        shadow_rgba: Optional[list[float, 4]]=None
            Color of shadow in rgba format.
        shadow_offset_x: float=0.0
            The offset of the show in the x direction.
        shadow_offset_y: float=0.0
            The offset of the show in the y direction.
        shadow_blur_radius: float=2.0
            How much to blur the shadow.
        text_color: Optional[IpgColor]=None
            Color of the text.
        text_rgba: Optional[list[float, 4]]=None
            Color of the text in rgba format.
        scrollbar_color: Optional[IpgColor]=None
            Color of the scrollbar.
        scrollbar_rgba: Optional[list[float, 4]]=None
            Color of the scrollbar in rgba format.
        scrollbar_border_radius: list[float]=[2.0]
            Radius a list of 1=all or 4=[top-left, top-right, bottom-right, bottom-left]
        scrollbar_border_width: float=1.0
            Width of the scrollbar.
        scrollbar_border_color: Optional[IpgColor]=None
            Color of the scrollbar.
        scrollbar_border_rgba: Optional[list[float, 4]]=None
            Color of the scrollbar in rgba format.
        scroller_color: Optional[IpgColor]=None
            Color of the scroller
        scroller_rgba: Optional[list[float, 4]]=None
            Color of the scroller in rgba format.
        scroller_color_hovered: Optional[IpgColor]=None
            Color of the scroller when the mouse hovers.
        scroller_rgba_hovered: Optional[list[float, 4]]=None
            Color of the scroller when the mouse hovers in rgba format.
        scroller_color_dragged: Optional[IpgColor]=None
            Color of the scroller when the mouse drags the scroller.
        scroller_rgba_dragged: Optional[list[float, 4]]=None
            Color of the scroller when the mouse drags the scroller in rgba format.
        gen_id: Optional[int]=None
            The only allowable entry for this id is the value generated by ipg.generate_id().
        """

    def add_tool_tip(self,
                        window_id,
                        container_id: str,
                        position: str,
                        text_to_display: str,
                        *,
                        parent_id: Optional[str]=None,
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
                    gen_id: Optional[int]=None,
                    on_press: Optional[Callable]=None,
                    width: Optional[float]=None,
                    height: Optional[float]=None,
                    width_fill: bool=False,
                    height_fill: bool=False,
                    padding: List=[10.0],
                    clip: bool=False, 
                    style: Optional[str]=None,
                    style_standard: Optional[IpgStyleStandard]=None,
                    style_arrow: Optional[IpgButtonArrow]=None,
                    user_data: Optional[any]=None,
                    show: bool=True, 
                    ) -> int:
        """
        Adds a button to the gui
        
        Parameters
        ----------
            parent_id: str
                id of another container.
            label: str
                label of button, this field is ignored when style_arrow is used.
            gen_id: int
                The only allowable entry for this id is the value generated by ipg.generate_id().
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
            clip: bool
                Whether to clip the label if width exceeded.
            style: str
                style_id of the add_button_style.
            style_standard: class
                IpgStyleStandard: Primary, Success, Danger, Text
            style_arrow: IpgButtonArrows
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
    def add_button_style(self,
                            style_id: str, 
                            *,
                            background_color: Optional[IpgColor]=None,
                            background_rgba: Optional[list[float, 4]]=None,
                            background_color_hovered: Optional[IpgColor]=None,
                            background_rgba_hovered: Optional[list[float, 4]]=None,
                            border_color: Optional[IpgColor]=None, 
                            border_rgba: Optional[list[float, 4]]=None,
                            border_radius: list[float | float, 4]=[0.0], 
                            border_width: float=1.0,
                            shadow_color: Optional[IpgColor]=None, 
                            shadow_rgba: Optional[list]=None,
                            shadow_offset_x: float=0.0, 
                            shadow_offset_y: float=0.0,
                            shadow_blur_radius: float=1.0,
                            text_color: Optional[IpgColor]=None, 
                            text_rgba: Optional[list[float, 4]]=None
                        ) -> int:
        """
        Adds styling to container

        Parameters
        ----------
            style_id: str
                Id of container_style. 
            background_color: Optional[IpgColor]=None
                Color of the background.
            background_rgba: Optional[list[float, 4]]=None
                Color of the background in rgba format.
            border_color: Optional[IpgColor]=None
                Color used for the border.
            border_rgba: list[float; 4]=None
                Color of the border in rgba format.
            border_radius: Optional[list[float]]=None
                The radius border, [float]=all corners, 
                [float, 4]=[top-left, top-right, bottom-right, bottom-left].
            border_width: float
                Border width.
            shadow_color: IpgColor
                The color of the shadow.
            shadow_rgba: list
                The color in rgba format [float; 4] used as state above.
            shadow_offset_x: float
                Shadow offset in the horizontal direction.
            shadow_offset_y: float
                Shadow offset in the vertical direction.
            shadow_blur_radius: float
                The blur radius of the shadow.
            text_color: IpgColor
                The text color, if not defined, will either be a Black or White variation based on theme background.
            text_rgba: list]
                The color in rgba format [float; 4] used as state above.
        """

    def add_card(self,
                    parent_id, 
                    head, 
                    body, 
                    foot: Optional[str]=None,
                    gen_id: Optional[int]=None,
                    is_open: bool=True, 
                    close_size: float=0.0, 
                    on_close: Optional[Callable]=None,
                    width: Optional[float]=None, 
                    height: Optional[float]=None, 
                    max_width: float="inf",
                    max_height: float="inf", 
                    padding_head: float=5.0, 
                    padding_body: float=5.0, 
                    padding_foot: float=5.0,
                    show: bool=True,
                    style: IpgCardStyle=IpgCardStyle.Primary, 
                    user_data: Optional[any]=None, 
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
            on_close: Optional[Callable]
                The callback function called when the card is minimized.
            width: Optional[float]
                Sets the width of the card 
            height: Optional[float]
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
                    on_toggle: Optional[Callable]=None,
                    is_checked: bool=False,
                    label: Optional[str]=None,
                    gen_id: Optional[int]=None,
                    width: Optional[float]=None,
                    width_fill: bool=False,
                    size: float=16.0,
                    spacing: float = 15.0,
                    text_line_height: float=1.3,
                    text_shaping: str = "basic",
                    text_size: float=16.0,
                    icon_x: bool=False,
                    icon_size: float=25.0,
                    style: Optional[str]=None,
                    style_standard: Optional[IpgStyleStandard]=None,
                    user_data: Optional[Any]=None,
                    show: bool=True,
                    ) -> int:
        """
        Adds a checkbox to the gui.
        When styling, the base color appears during a mouse hover and when checked.  
        If border color is not assigned, the border color will be the base color.
        
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
            style: str
                style_id of the add_checkbox_style.
            style_standard: class
                IpgStyleStandard: Primary, Success, Danger, Text(not valid)
            style_border: str
                style_id of the add_styling_border, radius, width.
            user_data: any 
                Any data in any form needed by user to be passed through as a callback. 
            show: bool
                Shows or hides widget.
           
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """
    
    def add_checkbox_style(self,
                            style_id: str, 
                            *,
                            background_color: Optional[IpgColor]=None,
                            background_rgba: Optional[list]=None,
                            accent_color: Optional[IpgColor]=None,
                            accent_color_hovered: Optional[IpgColor]=None,
                            border_color: Optional[IpgColor]=None, 
                            border_rgba: Optional[list]=None,
                            border_radius: list=[0.0], 
                            border_width: float=1.0,
                            text_color: Optional[IpgColor]=None, 
                            text_rgba: Optional[list]=None
                        ) -> int:
        """
        Adds styling to container

        Parameters
        ----------
            style_id: str
                Id of container_style. 
            background_color: Optional[IpgColor]=None
                The background color of the box.
            background_rgba: list,
                The background color of the box in rgba format.
            accent_color: Optional[IpgColor]=None
                The color of the box when checked.
            accent_rgba: Optional[list[float, 4]]=None
                The color of the box in rgba format in rgba format.
            accent_color_hovered: Optional[IpgColor]=None
                The color used for the mouse hover over the box that is checked.
            accent_rgba_hovered: Optional[list[float, 4]]=None
                The color used for the mouse hover over the box that is checked in rgba format.
            border_color: Optional[IpgColor]=None
                The color for the border.
            border_rgba: Optional[list[float, 4]]=None
                The color of the border in rgba format used as state above.
            border_radius: list
                The radius of the 4 corners, [float]=all corners, 
                [floate;4] top-left, top-right, bottom-right, bottom-left.
            border_width: float
                The border width.
            text_color: Optional[IpgColor]=None
                The text color, if not defined, will either be a Black or White variation based on theme background.
            text_rgba: Optional[list[float, 4]]=None
                The text color in rgba format.
        """

    # def add_color_picker(
    #                     self,
    #                     parent_id: str,
    #                     *,
    #                     label: str="Get Color",
    #                     gen_id: Optional[int]=None,
    #                     on_submit: Optional[Callable]=None,
    #                     width: Optional[float]=None,
    #                     height: Optional[float]=None,
    #                     width_fill: bool=False,
    #                     height_fill: bool=False,
    #                     padding: List=[10.0],
    #                     corner_radius: float=0.0,
    #                     style_standard: IpgStyleStandard.Primary,
    #                     user_data: Optional[Any]=None,
    #                     show: bool=True, 
    #                     ) -> int:
    #     """
    #     Adds a color picker.  The args for style and such are for the activation button.
    #     No styling for the Date Picker itself is available at this time.
        
    #     Parameters
    #     ----------
    #         parent_id: str
    #             Id of another container to place the widget in.
    #         label: str
    #             The label for the button which activates the picker.
    #         gen_id: int
    #             The only allowable entry for this id is that generated by ipg.generate_id().
    #         on_submit: Callable
    #             The function that executs when the submit button is pressed.
    #         width: float
    #             Sets the width of the button.
    #         width_fill: bool
    #             Sets the width to fill the available space, overrides width.
    #         height: float
    #             Sets the height of the button.   
    #         height_fill: bool
    #             Sets the heigth to fill the available space, overrides height.
    #         padding: List[float]
    #             Sets the padding for the widget.
    #             use [float] for all sides,
    #             use [float, float] for [top&bottom, left&right]
    #             use [float, float, float, float] for [top, right, bottom, left]
    #         corner_radius: float
    #             Sets the roundness of the corners of the button.
    #         style: IpgStyleStandard
    #             Sets the style of the button.
    #         show: bool
    #             To show the widget or not.
    #         user_data: any
    #             Any data that might be needed in the callback function.
            
    #     Returns
    #     -------
    #     id: int
    #         Internal id of widget and can be used by user if equated.
    #     """

    def add_date_picker(self,
                        parent_id: str,
                        *,
                        label: str="Calendar",
                        gen_id: Optional[int]=None,
                        size_factor: float=1.0,
                        padding: List=[5.0],
                        on_submit: Optional[Callable]=None,
                        user_data: Optional[Any]=None,
                        show=False,
                        button_style_standard,
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
            button_style_standard: IpgStyle_standard
                One of the standard button styles for the starting button.

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
                    on_press: Optional[Callable]=None, 
                    on_release: Optional[Callable]=None, 
                    on_right_press: Optional[Callable]=None, 
                    on_right_release: Optional[Callable]=None, 
                    on_middle_press: Optional[Callable]=None, 
                    on_middle_release: Optional[Callable]=None,
                    on_enter: Optional[Callable]=None,
                    on_move: Optional[Callable]=None,
                    on_exit: Optional[Callable]=None, 
                    width: Optional[float]=None,
                    height: Optional[float]=None,
                    width_fill: bool=False,
                    height_fill: bool=False,
                    content_fit: IpgImageContentFit=IpgImageContentFit.Contain,
                    filter_method: IpgImageFilterMethod=IpgImageFilterMethod.Linear, 
                    rotation: IpgImageRotation=IpgImageRotation.Floating,
                    rotation_radians: float=0.0,
                    opacity: float=1.0,
                    show: bool=True,  
                    user_data: Optional[Any]=None,
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
            content_fit: IpgImageContentFit
                Contain - The image will be scaled (preserving aspect ratio) so that it just fits within the window.
                Cover - Scale the image to cover all of the bounding box, cropping if needed.
                Fill - Distort the image so the widget is 100% covered without cropping.
                IpgNone - Don't resize or scale the image at all.  This is best for when you've sized the image yourself.
                ScaleDown - Scale the image down if it's too big for the space, but never scale it up.
            filter_method: IpgImageMethodFilter
                Linear - Bilinear interpolation image filtering strategy.
                Nearest - Nearest neighbor image filtering strategy.
            rotation: IpgImageRotation
                Floating - When image is roated, it floats above the container, not distoring it.
                Solid - When the image is rotated, the container resizes to fit.
            rotation_radians: float
                Amount to rotate, 180 degrees = 3.14159 radians.
            opacity: float
                How much opacity, 1=opaque, 0=transparent
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
                bar_widths: list[float],
                item_widths: list[float],  
                *,
                on_select: Optional[Callable]=None,
                bar_spacing: Optional[float]=None,
                bar_padding: Optional[list[float] | list[float, 4]]=None,
                bar_height: Optional[float]=None,
                bar_check_bounds_width=None,
                item_spacings: Optional[list[float]]=None,
                item_offsets: Optional[list[float]]=None,
                menu_bar_style: Optional[str]=None,
                menu_style: Optional[str]=None,
                button_bar_style_all: Optional[Tuple]=None,
                button_item_style_all: Optional[Tuple[IpgStyleStandard, str]]=None,
                checkbox_item_style_all: Optional[Tuple[IpgStyleStandard, str]]=None,
                circle_item_style_all: Optional[str]=None,
                dot_item_style_all: Optional[str]=None,
                label_item_style_all: Optional[str]=None,
                line_item_style_all: Optional[str]=None,
                text_item_style_all: Optional[Tuple[IpgStyleStandard, str]]=None,
                toggler_item_style_all:Optional[Tuple[IpgStyleStandard, str]]=None,
                item_styles: Optional[List[Tuple[int, int, IpgMenuType, IpgStyleStandard]]]=None, 
                gen_id: Optional[int]=None,
                user_data: Optional[Any]=None,
                ) -> int:
        """
        Add a menu dropdown list to the gui.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            items: OrderedDict[str, list[str]]
                Sets the menu items with the key being the bar item and values the menu items.
            item_widths: list[float]
                The widths of the bar items.
            item_spacings: list[float]
                The space between labels
            on_select: Callable
                The cllback for when the menu item is selected.
            bar_style_all: IpgStandardStyle or style_id
                Styles all of the menu bar items.
                if using custom style_id, use add_menu_bar_style()
            button_item_style_all: IpgStandardStyle or style_id
                Styles all of the buttons used in the menu items, if added.
                if using custom style_id, use the add_button_style()
            checkbox_item_style_all: IpgStandardStyle or style_id
                Styles all of the checkboxes used in the menu items, if added.
                if using custom style_id, use the add_checkbox_style()
            circle_item_style_all: tyle_id
                Styles all of the circles used in the menu items, if added.
                if using custom style_id, use the add_menu_separator_style()
            dot_item_style_all: style_id
                Styles all of the dotted line separators used in the menu items, if added.
                if using custom style_id, use the add_menu_separator_style()
            label_item_style_all: style_id
                Styles all of the label separators used in the menu items, if added.
                if using custom style_id, use the add_menu_separator_style()
            line_item_style_all: style_id
                Styles all of the line separators used in the menu items, if added.
                if using custom style_id, use the add_menu_separator_style()
            text_item_style_all: IpgStandardStyle or style_id
                Styles all of the text used in the menu.
                if using custom style_id, use the add_menu_text_style()
            toggler_item_style_all: IpgStandardStyle or style_id
                Styles all of the togglers used in the menu items, if added.
                if using custom style_id, use the add_toggler_style()
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            user_data: any
                Any data in any form needed by user to be passed through as a callback.

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_menu_bar_style(self,
                            style_id: str,
                            *,
                            base_color: Optional[IpgColor]=None,
                            base_rgba: Optional[list[float, 4]]=None,
                            border_color: Optional[IpgColor]=None,
                            border_rgba: Optional[list[float, 4]]=None,
                            border_radius: Optional[list[float] | list[float, 4]]=None,
                            border_width: Optional[float]=None,
                            shadow_color: Optional[IpgColor]=None,
                            shadow_rgba: Optional[list[float, 4]]=None,
                            shadow_offset_x: Optional[float]=None,
                            shadow_offset_y:Optional[float]=None,
                            shadow_blur_radius: Optional[float]=None,
                            gen_id: Optional[int]=None,
                       ) -> int:
        """
        Adds style in the menu itself, not the menu items or bar items
        """

    def add_menu_style(self,
                       style_id: str,
                        *,
                        base_color: Optional[IpgColor]=None,
                        base_rgba: Optional[list[float, 4]]=None,
                        border_color: Optional[IpgColor]=None,
                        border_rgba: Optional[list[float, 4]]=None,
                        border_radius: Optional[list[float] | list[float, 4]]=None,
                        border_width: Optional[float]=None,
                        shadow_color: Optional[IpgColor]=None,
                        shadow_rgba: Optional[list[float, 4]]=None,
                        shadow_offset_x: Optional[float]=None,
                        shadow_offset_y:Optional[float]=None,
                        shadow_blur_radius: Optional[float]=None,
                        path_base_color: Optional[IpgColor]=None,
                        path_base_rgba: Optional[list[float, 4]]=None,
                        path_border_color: Optional[IpgColor]=None,
                        path_border_rgba: Optional[list[float, 4]]=None,
                        path_border_radius: Optional[list[float] | list[float, 4]]=None,
                        path_border_width: Optional[float]=None,
                        gen_id: Optional[int]=None,
                       ) -> int:
        """
        Adds style in the menu itself, not the menu items or bar items
        """

    def add_menu_separator_style(self,
                                style_id: str,
                                separator_type: IpgMenuSeparatorType,
                                *,
                                height: float=20.0,
                                height_fill: bool=False,
                                width: Optional[float]=None,
                                width_fill: bool=True,
                                quad_ratios: Optional[list[float]]=[0.98, 0.2],
                                separator_color: Optional[IpgColor]=None,
                                separator_rgba: Optional[list[float]]=None,
                                separator_border_color: Optional[IpgColor]=None,
                                separator_border_rgba: Optional[list[float]]=None,
                                separator_border_width: Optional[float]=None,
                                separator_border_radius: Optional[list[float]]=None,
                                separator_shadow_color: Optional[IpgColor]=None,
                                separator_shadow_rgba: Optional[list[float]]=None,
                                separator_shadow_offset: Optional[list[float]]=None,
                                separator_shadow_blur_radius: Optional[float]=None,
                                background_color: Optional[IpgColor]=None,
                                background_rgba: Optional[list[float]]=None,
                                background_border_color: Optional[IpgColor]=None,
                                background_border_rgba: Optional[list[float]]=None,
                                background_border_width: Optional[float]=None,
                                background_border_radius: Optional[list[float]]=None,
                                background_shadow_color: Optional[IpgColor]=None,
                                background_shadow_rgba: Optional[list[float]]=None,
                                background_shadow_offset: Optional[list[float]]=None,
                                background_shadow_blur_radius: Optional[float]=None,
                                gen_id: Optional[int]=None,
                                ) -> int:
        """
        
        """

    def add_pick_list(self,
                        parent_id: str,
                        options: List=[str],
                        *,
                        gen_id: Optional[int]=None,
                        on_select: Optional[Callable]=None,
                        width: Optional[float]=None,
                        width_fill: bool=False,
                        padding: List=[5.0],
                        placeholder: Optional[str]=None,
                        selected: Optional[str]=None,
                        text_size: float=15.0,
                        text_line_height: float="default",
                        text_shaping: str="basic",
                        handle: Optional[IpgPickListHandle]=None,
                        arrow_size: Optional[float]=None,
                        dynamic_closed: Union[None| IpgButtonArrow]=None,
                        dynamic_opened: Optional[IpgButtonArrow]=None,
                        custom_static: Optional[IpgButtonArrow]=None,
                        style_color: Union[None, str]=None,
                        style_border: Union[None, str]=None,
                        user_data: Optional[any]=None,
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
            handle: IpgPickListHandle
                What type of indicatpr to use for the dropdown list, arrrow, ...
            arrow_size: float,
                Size of the arrow indicator, default 16.0
            dynamic_closed: IpgArrows,
                The type of arrow wanted for when the picklist is closed.
            dynamic_opened: IpgArrows
                The type of arrow wanted for when the picklist is opened.
            custom_static: IpgArrows
                The type of arrow wanted for the picklist.
            style_color: str
                style_id of the add_styling_color.
            style_border: str
                style_id of the add_styling_border.
            user_data: any
                Any data in any form needed by user to be passed through as a callback.
            show: bool
                Shows or hides the widget.
    
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_pick_list_style(self,
                            style_id: str,
                            *,
                            background_color: Optional[IpgColor]=None,
                            background_rgba: Optional[list[float, 4]]=None,
                            text_color: Optional[IpgColor]=None,
                            text_rgba: Optional[list[float, 4]]=None,
                            handle_color: Optional[IpgColor]=None,
                            handle_rgba: Optional[list[float, 4]]=None,
                            placeholder_color: Optional[IpgColor]=None,
                            placeholder_rgba: Optional[list[float, 4]]=None,
                            border_color: Optional[IpgColor]=None,
                            border_rgba: Optional[list[float, 4]]=None,
                            border_color_hovered: Optional[IpgColor]=None,
                            border_rgba_hovered: Optional[list[float, 4]]=None,
                            border_radius: Optional[list[float]]=None,
                            border_width: Optional[float]=None,
                            gen_id: Optional[int]=None,
                            ) -> int:
        """
        Add PickList styling.

        Parameters
        -----------

            style_id: str,
            background_color: Optional[IpgColor]=None,
                Color of the background.
            background_rgba: Optional[list[float, 4]]=None,
                Color of the background in  rgba format.
            text_color: Optional[IpgColor]=None,
                Color of the text.
            text_rgba: Optional[list[float, 4]]=None,
                Color of the text in rgba format.
            handle_color: Optional[IpgColor]=None,
                Color of the handle.
            handle_rgba: Optional[list[float, 4]]=None,
                Cor of the handle in rgba format.
            placeholder_color: Optional[IpgColor]=None,
                Color of the placeholder text.
            placeholder_rgba: Optional[list[float, 4]]=None,
                Color of the placeholder text in rgba format.
            border_color: Optional[IpgColor]=None,
                Color of the border.
            border_rgba: Optional[list[float, 4]]=None,
                Color of the border in rgba format.
            border_color_hovered: Optional[IpgColor]=None,
                Color of the border when mouse hovers.
            border_rgba_hovered: Optional[list[float, 4]]=None,
                Color of the border in rgba format when mouse hovers.
            border_radius: Optional[list[float]]=None,
                Radius of the corners [one value]=all corners, [4 values]=[top-left, top_right, , bottom-right, bottom-left]
            border_width: Optional[float]=None,
                Width of the border.
            gen_id: Optional[int]=None,
                The only allowable entry for this id is that generated by ipg.generate_id().
        """

    def add_progress_bar(self,
                        parent_id: str,
                        min: float,
                        max: float,
                        *,
                        value: float=0.0,
                        gen_id: Optional[int]=None,
                        width: Optional[float]=None,
                        width_fill: bool=False,
                        height: float=1.0,
                        style_standard: Optional[IpgStyleStandard]=None,
                        style: Union[None, str]=None,
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
            style: str
                style_id of the add_progress_bar_style.
            user_data: any
                Any data in any form needed by user to be passed through as a callback.
            show: bool
                Shows or hides the widget.
            
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_progress_bar_style(self,
                                style_id: str,
                                *,
                                background_color: Optional[IpgColor]=None,
                                background_rgba: Optional[list[float, 4]]=None,
                                bar_color: Optional[IpgColor]=None,
                                bar_rgba: Optional[list[float, 4]]=None,
                                border_color: Optional[IpgColor]=None,
                                border_rgba: Optional[list[float, 4]]=None,
                                border_radius: Optional[list[float, 4]]=None,
                                border_width: Optional[float]=None,
                                gen_id: Optional[int]=None,
                               ) -> int:
        """
        Add ProgressBar style.

        Parameters
        ----------
            background_color: Optional[IpgColor]=None
                Color of the background.
            background_rgba: Optional[list[float, 4]]=None
                Color of the background in rgba format.
            bar_color: Optional[IpgColor]=None
                Color of the bar.
            bar_rgba: Optional[list[float, 4]]=None
                Color of the bar in rgba format.
            border_color: Optional[IpgColor]=None
                Color of the border.
            border_rgba: Optional[list[float, 4]]=None
                Color of the border in rgba format.
            border_radius: Optional[list[float, 4]]=None
                The radius of the corners, [one value]=all cormers, [4 values]=[top-left, top-right, bottom-right, bottom-left]
            border_width: Optional[float]=None
                Width of the border
            gen_id: Optional[int]=None
                The only allowable entry for this id is that generated by ipg.generate_id().
        """

    def add_radio(self,
                    parent_id: str,
                    labels: List=[str],
                    *,
                    gen_id: Optional[int]=None,
                    direction: IpgRadioDirection=IpgRadioDirection.Vertical,
                    spacing: float=10.0,
                    padding: List=[10.0],
                    width: Optional[float]=None,
                    width_fill: bool=False,
                    on_select: Optional[Callable]=None,
                    selected_index: Optional[int]=None,
                    size: float=20.0,
                    style_color: Optional[str]=None,
                    style_border: Optional[str]=None,
                    text_spacing: float=15.0,
                    text_size: float=16.0,
                    text_line_height_pixels: Optional[int],
                    text_line_height_relative: Optional[float]=None,
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
            text_line_height_pixels:int
                Sets the line height for the box around the radio labels in pixels.
            text_line_height_relative:float
                Sets the line height for the box around the radio labels.
            show: bool
                Shows or hides the widget.
            size: float,
                Radius of the round radio button.
            spacing: float
                Spacing between the radio buttons.
            style_color: str
                style_id of the add_styling_color.
            style_border: str
                style_id of the add_styling_border.
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

    def add_radio_style(self,
                        style_id: str,
                        *,
                        circle_inner_color: Optional[IpgColor]=None,
                        circle_inner_rgba: Optional[list[float, 4]]=None,
                        circle_inner_hover_color: Optional[IpgColor]=None,
                        hover_color_factor: Optional[float]=None,
                        border_color: Optional[IpgColor]=None,
                        border_rgba: Optional[list[float, 4]]=None,
                        border_width: Optional[float]=None,
                        dot_color: Optional[IpgColor]=None,
                        dot_rgba: Optional[list[float, 4]]=None,
                        text_color: Optional[IpgColor]=None,
                        text_rgba: Optional[list[float, 4]]=None,
                        gen_id: Optional[int]=None,
                        ) -> int:
        """
        Add Radio style.

        Parameter
        ---------
            style_id: str,
                The id of the stle to use in ipg.add_radio.
            circle_inner_color: Optional[IpgColor]=None,
                The color of the inner circle of the radio button.
                Defaults to background transparent.
            circle_inner_rgba: Optional[list[float, 4]]=None,
                The color of the inner circle of the radio button.
                Defaults to background transparent.
            circle_inner_hover_color: Optional[IpgColor]=None,
                The color of the inner circle when mouse hovers.
                Defaults to background weak color.
            hover_color_factor: Optional[float]=None,
                if only the inner color defined, then can be used to
                enhance the hover color of the defined inner color.
                Defaults to 0.1.
            border_color: Optional[IpgColor]=None,
                The color of the circle border.
                Defaults to primary.
            border_rgba: Optional[list[float, 4]]=None,
                The color of the circle border.
                Defaults to primary.
            border_width: Optional[float]=None,
                The border width of the circle.
                defaults to 1.0
            dot_color: Optional[IpgColor]=None,
                The color of the dot.
                Defaults to primary.
            dot_rgba: Optional[list[float, 4]]=None,
                The color of the dot.
                Defaults to primary.
            text_color: Optional[IpgColor]=None,
                The color of the text.
                Defaults to a contrast color of the background.
            text_rgba: Optional[list[float, 4]]=None,
                The color of the text.
                Defaults to a contrast color of the background.
            gen_id: Optional[int]=None,
                The only allowable entry for this id is that generated by ipg.generate_id().

        Returns
        -------
            id: int
                Internal id of widget and can be used by user if equated.
        """

    def add_rule_horizontal(self, 
                            parent_id, 
                            *,
                            width: Optional[float]=None, 
                            width_fill: bool=True,
                            thickness: int=1,
                            style: Optional[str]=None,
                            gen_id: Optional[int]=None,
                            ) -> int:
        """
        Add a horizontal line divider.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            width: Optional[float]=None
                Defines the horizontal length of the dividing line.
            width_fill: bool=True
                If set, fills the available space for the horizontal length, overides width.
            thickness: int=1
                The thickness of the rule.
            style: str
                The id of the add_rule_style.
            gen_id: Optional[int]=None,
                The only allowable entry for this id is that generated by ipg.generate_id().
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_rule_vertical(self, 
                            parent_id,
                            *, 
                            height: Optional[float]=None, 
                            height_fill: bool=True,
                            thickness: int=1,
                            style: Optional[str]=None,
                            gen_id: Optional[int]=None,
                          ) -> int:
        """
        Add a vertical line divider.

        Parameters
        ----------
            parent_id: str
                Id of another container to place the widget in.
            height: Optional[float]=None
                Defines the vertical length of the dividing line.
            height_fill: bool=True
                If set, fills the available space for the vertical length, overides height.
            thickness: int=1
                The thickness of the rule.
            style: Optional[str]=None,
                The id of the add_rule_style.
            gen_id: Optional[int]=None,
                The only allowable entry for this id is that generated by ipg.generate_id().

        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_rule_style(self,
                        style_id: str,
                        color: Optional[IpgColor],
                        color_rgba: Optional[list[float, 4]],
                        border_radius: Optional[list[float, 4]],
                        fillmode_percent: Optional[float],
                        fillmode_padded: Optional[int],
                        fillmode_asymmetric_padding: Optional[list[int, 2]],
                        gen_id: Optional[int],
                        ) -> int:
        """
        Add Rule styling.

        Parameters:
        -----------
            style_id: str
                The id used in the add_rule style parameter.
            color: Optional[IpgColor],
                The color of the rule, background weak if not defined.
            color_rgba: Optional[list[float, 4]]
                The color of the rule
            border_radius: Optional[list[float, 4]],
                The border radius of the rule.
            fillmode_percent: Optional[float],
                Fills the color of the rule to a percent
            fillmode_padded: Optional[uint2],
                Fills the rule with the color container padding on each end.
            fillmode_asymmetric_padding: Optional[list[uint2, 2]],
                Fills the rule with the color asymetrically.
            gen_id: Optional[int],
                The only allowable entry for this id is that generated by ipg.generate_id().
    Returns
    -------
        id: int
            Internal id of widget and can be used by user if equated.

        """

    def add_selectable_text(self, 
                            parent_id, 
                            text,
                            *,
                            gen_id: Optional[int]=None, 
                            on_press: Optional[Callable]=None, 
                            on_release: Optional[Callable]=None, 
                            on_right_press: Optional[Callable]=None, 
                            on_right_release: Optional[Callable]=None, 
                            on_middle_press: Optional[Callable]=None, 
                            on_middle_release: Optional[Callable]=None,
                            on_enter: Optional[Callable]=None,
                            on_move: Optional[Callable]=None,
                            on_exit: Optional[Callable]=None, 
                            width: Optional[float]=None,
                            height: Optional[float]=None,
                            width_fill: bool=False,
                            height_fill: bool=False, 
                            h_align: str="left",
                            v_align: str="top", 
                            line_height: float=1.3, 
                            shaping: str="basic",
                            size: float=16.0, 
                            show: bool=True,  
                            user_data: Optional[any]=None,
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
                   gen_id: Optional[int]=None, 
                   show: bool=True, 
                   on_change: Optional[Callable]=None, 
                   on_release: Optional[Callable]=None, 
                   user_data: Optional[any]=None, 
                   width: float=100.0,
                   width_fill: bool=False,
                   height: float=20.0,
                   style_id: Optional[str]=None,
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
            style: str
                Id from the add_slider_style()
        Returns
        -------
        id: int
            Internal id of widget and can be used by user if equated.
        """

    def add_slider_style(self,
                        style_id: str,
                        rail_color: Optional[IpgColor]=None,
                        rail_rgba: Optional[list[float, 4]]=None,
                        rail_color_dragged: Optional[IpgColor]=None,
                        rail_rgba_dragged: Optional[list[float, 4]]=None,
                        rail_width: Optional[float]=None,
                        rail_border_radius: Optional[list[float]]=None,
                        handle_circle_radius: Optional[float]=None,
                        handle_rectangle_width: Optional[int]=None,
                        handle_rectangle_border_radius: Optional[list[float]]=None,
                        handle_color: Optional[IpgColor]=None,
                        handle_rgba: Optional[list[float, 4]]=None,
                        handle_border_width: Optional[float]=None,
                        handle_border_color: Optional[IpgColor]=None,
                        handle_border_rgba: Optional[list[float, 4]]=None,
                        gen_id: Union[None, int]=None,
                         ) -> int:
        """
        Add styling to the Slider.

        Parameters
        ----------
            style_id: str
                The id used in the add_slider style parameter.
            rail_color: Optional[IpgColor]=None
                The color of the rail.
            rail_rgba_base: Optional[list[float, 4]]=None
                The color of the rail in rgba format.
            rail_color_dragged: Optional[IpgColor]=None
                The color of the rail when dragged.
            rail_rgba_strong: Optional[list[float, 4]]=None
                The color of the rail when dragged in rgba format.
            rail_width: Optional[float]=None
                The width of the rail.
            rail_border_radius: Optional[list[float]]=None
                rail border radius use either a single list item or a list of 4
            handle_circle_radius: Optional[float]=None
                The circle is the default shape.
                The handle circle radius default=7.0
            handle_rectangle_width: Optional[uint2]=None
                Defining either the width or radius, activates this shape.
                handle width, default=12
            handle_rectangle_border_radius: Optional[list[float]]=None
                handle rectangle border radius default=1.0
            handle_color_base: Optional[IpgColor]=None
                The handle color used during mouse hover with IpgColor class.
            handle_rgba_base: Optional[list[float, 4]]=None
                The handle color used during mouse hover in rgba format.
            handle_color_strong: Optional[IpgColor]=None
                The color used for active and dragged with IpgColor class.
            handle_rgba_strong: Optional[list[float, 4]]=None
                The color used for active and dragged in rgba format.
            handle_strong_factor: Optional[float]=None
                If strong color not defined, factor used to generate the  
                the strong color using the base.
            handle_border_width: Optional[float]=None
                Handle border width, default=0.0
            handle_border_color: Optional[IpgColor]=None
                Handle border color, default=transparent.
            handle_border_rgba: Optional[list[float, 4]]=None
                Handle border color in rgba format, default=transparent.
            gen_id: Union[None, int]=None
                The only allowable entry for this id is that generated by ipg.generate_id().
        Returns
        -------
            id: int
            Internal id of widget and can be used by user if equated.
        
        """

    def add_space(self,
                  parent_id: str,
                  *,
                  gen_id: Optional[int]=None,
                  width: Optional[float]=None,
                  height: Optional[float]=None,
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
                    on_press: Optional[Callable]=None, 
                    on_release: Optional[Callable]=None, 
                    on_right_press: Optional[Callable]=None, 
                    on_right_release: Optional[Callable]=None, 
                    on_middle_press: Optional[Callable]=None, 
                    on_middle_release: Optional[Callable]=None,
                    on_enter: Optional[Callable]=None,
                    on_move: Optional[Callable]=None,
                    on_exit: Optional[Callable]=None, 
                    width: Optional[float]=None,
                    height: Optional[float]=None,
                    width_fill: bool=False,
                    height_fill: bool=False,
                    content_fit: IpgSvgContentFit=IpgSvgContentFit.Contain,
                    rotation: IpgSvgRotation=IpgSvgRotation.Floating,
                    rotation_radians: float=0.0,
                    opacity: float=1.0, 
                    show: bool=True,  
                    user_data: Optional[Any]=None,
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
            content_fit: IpgImageContentFit
                Contain - The image will be scaled (preserving aspect ratio) so that it just fits within the window.
                Cover - Scale the image to cover all of the bounding box, cropping if needed.
                Fill - Distort the image so the widget is 100% covered without cropping.
                IpgNone - Don't resize or scale the image at all.  This is best for when you've sized the image yourself.
                ScaleDown - Scale the image down if it's too big for the space, but never scale it up.
            rotation: IpgImageRotation
                Floating - When image is roated, it floats above the container, not distoring it.
                Solid - When the image is rotated, the container resizes to fit.
            rotation_radians: float
                Amount to rotate, 180 degrees = 3.14159 radians.
            opacity: float
                How much opacity, 1=opaque, 0=transparent
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
                    window_id: str,
                    table_id: str,
                    title: str,
                    data: list[dict],
                    data_length: int,
                    width: float,
                    height: float,
                    *,
                    parent_id: Optional[str]=None,
                    row_highlight: Optional[IpgTableRowHighLight]=None,
                    highlight_amount: float=0.15,
                    column_widths: list=[20.0],
                    button_style: Optional[Dict[int: IpgStyleStandard]]=None,
                    widgets_columns: Optional[Dict[int: list[IpgTableWidget]]]=None,
                    gen_id: Optional[int]=None,
                    on_button: Optional[Callable]=None,
                    on_checkbox: Optional[Callable]=None,
                    on_modal: Optional[Callable]=None,
                    on_toggler: Optional[Callable]=None,
                    show: bool=True, 
                    user_data: Optional[Any]=None,
                  ) -> int:

        """
        Adds a table to the gui.

        Parameters
        ----------
            window_id: str
                Id of the window to place container in.
            container_id: str
                The id of the container.
            title: str
                Title used for table.
            data: List[Dict]
                A list of dictionaries, each dictionary contains only one type.
            data_length: int
                The length of the data.
            width: float
                Width of the table.
            height: float
                Height of the table.
            parent_id: Optional[str]
                If parent_id == window_id then not required, 
                If another container then required.
            row_highlight: TableRowHighLight
                Highlights alternate row by either drkening or lightening them up.
            highligh_amount: float
                Amount of highlighting to use if row_highlight is set.
            column_widths: List[float]
                A list of value for the column widths, if only one value is supplied then it will 
                be the default for all columns.
            widgets_columns: dict{int, List[IpgTableWidget]}
                The column where the values are converted to text and used as labels for the widget.
            gen_id: int
                The only allowable entry for this id is that generated by ipg.generate_id().
            on_button: Callable
                Callback when a button is pressed.
            on_checkbox: Callable
                Callback when a checkbox is toggled.
            on_modal: Callable
                Callback when a modal is pressed.
            on_toggler: Callable
                Callback when a toggler is toggled.
            user_data: any
                Any data that might be needed in the callback function.
            show:: bool
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
                 gen_id: Optional[int]=None,
                 width: Optional[float]=None,
                 height: Optional[float]=None,
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
                        gen_id: Optional[str]=None,
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
                        *,
                        gen_id: Optional[int]=None,
                        on_submit: Optional[Callable]=None,
                        on_input: Optional[Callable]=None,
                        on_paste: Optional[Callable]=None,
                        width: Optional[float]=None,
                        width_fill: bool=False,
                        padding: List=[10.0],
                        size: Optional[float]=20.0,
                        line_height_pixels: Optional[int],
                        line_height_relative: Optional[float],
                        user_data: Optional[any]=None,
                        is_secure: bool=False,
                        style: Optional[str]=None,
                        show: bool=True,
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
            width: Optional[float]
                Sets the width to a specific value.
            width_fill: bool
                Sets the width to fill available space, overrides width, Default=shrink.
            padding: List[float]
                Sets the padding for widget.
                use [float] for all sides,
                use [float, float] for [top&bottom, left&right]
                use [float, float, float, float] for [top, right, bottom, left]
            size: float
                Sets the text size.
            line_height_pixels: Optional[int],
                Sets the height of the text box in pixels.
            line_height_relative: Optional[float],
                Sets the height of the text box with a float.  
            user_data: any
                Any data that might be needed in the callback function.
            is_secure: bool
                Hides the entered text, for passwords, etc.
            style: Optional[str]=None
                The string style_id of add_text_input_style().
            show: bool=True
                Shows or hides the widget.
        Returns
        -------
        id: int
            The id of the event which can be used to modify the event through update_item.
        """

    def add_text_input_style(self,
                                style_id: str,
                                background_color: Optional[IpgColor]=None,
                                background_rgba: Optional[list[float, 4]]=None,
                                background_color_strong: Optional[IpgColor]=None,
                                background_rgba_strong: Optional[list[float, 4]]=None,
                                background_strong_factor: Optional[float]=None,
                                background_color_weak: Optional[IpgColor]=None,
                                background_rgba_weak: Optional[list[float, 4]]=None,
                                background_weak_factor: Optional[float]=None,
                                border_color: Optional[IpgColor]=None,
                                border_rgba: Optional[list[float, 4]]=None,
                                border_color_hovered: Optional[IpgColor]=None,
                                border_rgba_hovered: Optional[list[float, 4]]=None,
                                border_color_focused: Optional[IpgColor]=None,
                                border_rgba_focused: Optional[list[float, 4]]=None,
                                border_width: Optional[float]=None,
                                border_radius: Optional[list[float]]=None,
                                icon_color: Optional[IpgColor]=None,
                                icon_rgba: Optional[list[float, 4]]=None,
                                placeholder_color: Optional[IpgColor]=None,
                                placeholder_rgba: Optional[list[float, 4]]=None,
                                value_color: Optional[IpgColor]=None,
                                value_rgba: Optional[list[float, 4]]=None,
                                selection_color: Optional[IpgColor]=None,
                                selection_rgba: Optional[list[float, 4]]=None,
                                gen_id: Optional[int]=None,
                             ) -> int:
        """
        Add textInput styling.

            Parameters
            ----------
                style_id: str
                    Id used in the add_text_input() parameter style.
                background_color: Optional[IpgColor]=None

                background_rgba: Optional[list[float, 4]]=None
                background_color_strong: Optional[IpgColor]=None
                background_rgba_strong: Optional[list[float, 4]]=None
                background_strong_factor: Optional[float]=None
                background_color_weak: Optional[IpgColor]=None
                background_rgba_weak: Optional[list[float, 4]]=None
                background_weak_factor: Optional[float]=None
                border_color: Optional[IpgColor]=None
                border_rgba: Optional[list[float, 4]]=None
                border_color_hovered: Optional[IpgColor]=None
                border_rgba_hovered: Optional[list[float, 4]]=None
                border_color_focused: Optional[IpgColor]=None
                border_rgba_focused: Optional[list[float, 4]]=None
                border_width: Optional[float]=None
                border_radius: Optional[list[float]]=None
                icon_color: Optional[IpgColor]=None
                icon_rgba: Optional[list[float, 4]]=None
                placeholder_color: Optional[IpgColor]=None
                placeholder_rgba: Optional[list[float, 4]]=None
                value_color: Optional[IpgColor]=None
                value_rgba: Optional[list[float, 4]]=None
                selection_color: Optional[IpgColor]=None
                selection_rgba: Optional[list[float, 4]]=None
                gen_id: Optional[int]=None
        """

    def add_timer(self,
                        parent_id: str,
                        duration_ms: int,
                        *,
                        on_start: Optional[Callable]=None,
                        on_stop: Optional[Callable]=None,
                        on_tick: Optional[Callable]=None,
                        start_label: str="Start Timer",
                        stop_label: str="Stop Timer",
                        width: Optional[float]=None,
                        height: Optional[float]=None,
                        width_fill: bool=False,
                        height_fill: bool=False,
                        padding: list=[10.0],
                        style_background: Optional[str]=None,
                        style_border: Optional[str]=None,
                        style_shadow: Optional[str]=None,
                        style_text_color: Optional[str]=None,
                        arrow_style: Optional[IpgButtonArrow]=None,
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
        on_start: Optional[Callable]
            The optional function that execute when the timer starts.
        on_stop: Optional[Callable]
            The optional function that executes when the timer stops.
        on_tick: Optional[Callable]
            The optional function that executes on every timer tick as indicated by duration_ms.
        start_label: str="Start Timer"
            The default start label of the timer button.
        stop_label: str="Stop Timer"
            The default stop label of the timer button.
        width: Optional[float]
            Width of the button, the default is the size of the label.
        height: Optional[float]
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
        style_background: str
                style_id of the add_background_style.
        style_border: str
            style_id of the add_border_style.
        style_shadow: str
            style_id of the add_shadow_style.
        style_text_color: str
            style_id of the add_text_color_style.
        arrow_style: Optional[IpgButtonArrows]
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
                    label: Optional[str]=None,
                    gen_id: Optional[str]=None,
                    toggled: Optional[Callable]=None,
                    width: Optional[float]=None,
                    width_fill: bool=False,
                    size: float=20.0,
                    text_size: float=16.0,
                    text_line_height: float=1.3,
                    text_alignment: IpgHorizontalAlignment=IpgHorizontalAlignment.Center,
                    spacing: float=10.0,
                    user_data: Optional[Any]=None,
                    show: bool=True,
                    style: Optional[str]=None,
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
                Sets the width to fill the available space, overrides width, defaults to shrink.
            size: float
                Size of the toggler.
            text_size: float
                Size of the text label.
            text_line_height: float
                Height of the box containing the text.
            text_alignment: IpgHorizontalAlignment,
                Text alignment within the text box, left, center, right
            spacing: float
                Spacing between the toggler and the text label
            user_data: any 
                Any data in any form needed by user to be passed through as a callback. 
            show: bool
                Shows or hides widget.
            style: str
                style_id of the add_toggler_style().
            
        Return:
        ------- 
            int: internal id of widget and can be used by user if equated.
        """
    def add_toggler_style(self,
                            style_id: str,
                            background_color: Optional[IpgColor]=None,
                            background_rgba: Optional[list[float, 4]]=None,
                            background_color_toggled: Optional[IpgColor]=None,
                            background_rgba_toggled: Optional[list[float, 4]]=None,
                            background_border_color: Optional[IpgColor]=None,
                            background_border_rgba: Optional[list[float, 4]]=None,
                            background_border_width: Optional[float]=None,
                            foreground_color: Optional[IpgColor]=None,
                            foreground_rgba: Optional[list[float, 4]]=None,
                            foreground_color_toggled: Optional[IpgColor]=None,
                            foreground_rgba_toggled: Optional[list[float, 4]]=None,
                            foreground_border_color: Optional[IpgColor]=None,
                            foreground_border_rgba: Optional[list[float, 4]]=None,
                            foreground_border_width: Optional[IpgColor]=None,
                          ) -> int:
        """
        Adds a toggler to the gui
        
        Parameters
        ----------
            style_id: str,
            background_color: Optional[IpgColor]
            background_rgba: Optional[list[float, 4]]
            background_color_toggled: Optional[IpgColor]
            background_rgba_toggled: Optional[list[float, 4]]
            background_border_color: Optional[IpgColor]
            background_border_rgba: Optional[list[float, 4]]
            background_border_width: Optional[float]
            foreground_color: Optional[IpgColor]
            foreground_rgba: Optional[list[float, 4]]
            foreground_color_toggled: Optional[IpgColor]
            foreground_rgba_toggled: Optional[list[float, 4]]
            foreground_border_color: Optional[IpgColor]
            foreground_border_rgba: Optional[list[float, 4]]
            foreground_border_width: Optional[IpgColor]

        """
        

    # *******************************events*************************************************************
    def add_event_keyboard(self,
                            enabled: bool,
                            *,
                            on_key_press: Optional[Callable]=None,
                            on_key_release: Optional[Callable]=None,
                            user_data: Optional[any]=None, 
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
                            on_move: Optional[Callable]=None,
                            on_left_press: Optional[Callable]=None,
                            on_left_release: Optional[Callable]=None,
                            on_middle_press: Optional[Callable]=None,
                            on_middle_release: Optional[Callable]=None,
                            on_right_press: Optional[Callable]=None,
                            on_right_release: Optional[Callable]=None,
                            on_middle_scroll: Optional[Callable]=None,
                            user_data: Optional[Any]=None,
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
                        on_open: Optional[Callable]=None,
                        on_close: Optional[Callable]=None,
                        on_moved: Optional[Callable]=None,
                        on_resized: Optional[Callable]=None,
                        user_data: Optional[Any]=None,
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
    Start=0
    Center=0
    End=0


class IpgHorizontalAlignment:
    Left=0
    Center=0
    Right=0


class IpgVerticalAlignment:
    Top=0
    Center=0
    Bottom=0


class IpgWindowTheme:
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


class IpgButtonParam:
    ArrowStyle=0
    CornerRadius=0
    Height=0
    HeightFill=0
    Label=0
    Padding=0
    Show=0
    Style=0
    StyleStandard=0
    Width=0
    WidthFill=0


class IpgCardStyle: 
    Primary=0
    Success=0
    Danger=0
    Warning=0
    Info=0
    Light=0
    Dark=0
    White=0
    Default=0


class IpgCardParam:
    Body=0
    Foot=0
    Head=0
    IsOpen=0
    Style=0


class IpgCheckboxParam:
    IconSize=0
    IconX=0
    IsChecked=0
    Label=0
    Show=0
    Size=0
    Spacing=0
    StyleBackground=0
    StyleBorder=0
    StyleIconTextColor=0
    StyleTextColor=0
    TextLineHeight=0
    TextShaping=0
    TextSize=0
    Width=0
    WidthFill=0


class IpgColor:
    PRIMARY=0
    SUCCESS=0
    DANGER=0
    WARNING=0
    INFO=0
    LIGHT=0
    DARK=0
    BACKGROUND_THEME=0
    ALICE_BLUE=0
    ANTIQUE_WHITE=0
    AQUA=0
    AQUAMARINE=0
    AZURE=0
    BEIGE=0
    BISQUE=0
    BLACK=0
    BLANCHED_ALMOND=0
    BLUE=0
    BLUE_VIOLET=0
    BROWN=0
    BURLY_WOOD=0
    CADET_BLUE=0
    CHARTREUSE=0
    CHOCOLATE=0
    CORAL=0
    CORNFLOWER_BLUE=0
    CORNSILK=0
    CRIMSON=0
    CYAN=0
    DARK_BLUE=0
    DARK_CYAN=0
    DARK_GOLDEN_ROD=0
    DARK_GRAY=0
    DARK_GREY=0
    DARK_GREEN=0
    DARK_KHAKI=0
    DARK_MAGENTA=0
    DARK_OLIVE_GREEN=0
    DARK_ORANGE=0
    DARK_ORCHID=0
    DARK_RED=0
    DARK_SALMON=0
    DARK_SEA_GREEN=0
    DARK_SLATE_BLUE=0
    DARK_SLATE_GRAY=0
    DARK_SLATE_GREY=0
    DARK_TURQUOISE=0
    DARK_VIOLET=0
    DEEP_PINK=0
    DEEP_SKY_BLUE=0
    DIM_GRAY=0
    DIM_GREY=0
    DODGER_BLUE=0
    FIRE_BRICK=0
    FLORAL_WHITE=0
    FOREST_GREEN=0
    FUCHSIA=0
    GAINSBORO=0
    GHOST_WHITE=0
    GOLD=0
    GOLDEN_ROD=0
    GRAY=0
    GREY=0
    GREEN=0
    GREEN_YELLOW=0
    HONEY_DEW=0
    HOT_PINK=0
    INDIAN_RED=0
    INDIGO=0
    IVORY=0
    KHAKI=0
    LAVENDER=0
    LAVENDER_BLUSH=0
    LAWN_GREEN=0
    LEMON_CHIFFON=0
    LIGHT_BLUE=0
    LIGHT_CORAL=0
    LIGHT_CYAN=0
    LIGHT_GOLDEN_ROD_YELLOW=0
    LIGHT_GRAY=0
    LIGHT_GREY=0
    LIGHT_GREEN=0
    LIGHT_PINK=0
    LIGHT_SALMON=0
    LIGHT_SEA_GREEN=0
    LIGHT_SKY_BLUE=0
    LIGHT_SLATE_GRAY=0
    LIGHT_SLATE_GREY=0
    LIGHT_STEEL_BLUE=0
    LIGHT_YELLOW=0
    LIME=0
    LIME_GREEN=0
    LINEN=0
    MAGENTA=0
    MAROON=0
    MEDIUM_AQUA_MARINE=0
    MEDIUM_BLUE=0
    MEDIUM_ORCHID=0
    MEDIUM_PURPLE=0
    MEDIUM_SEA_GREEN=0
    MEDIUM_SLATE_BLUE=0
    MEDIUM_SPRING_GREEN=0
    MEDIUM_TURQUOISE=0
    MEDIUM_VIOLET_RED=0
    MIDNIGHT_BLUE=0
    MINT_CREAM=0
    MISTY_ROSE=0
    MOCCASIN=0
    NAVAJO_WHITE=0
    NAVY=0
    OLD_LACE=0
    OLIVE=0
    OLIVE_DRAB=0
    ORANGE=0
    ORANGE_RED=0
    ORCHID=0
    PALE_GOLDEN_ROD=0
    PALE_GREEN=0
    PALE_TURQUOISE=0
    PALE_VIOLET_RED=0
    PAPAYA_WHIP=0
    PEACH_PUFF=0
    PERU=0
    PINK=0
    PLUM=0
    POWDER_BLUE=0
    PURPLE=0
    REBECCA_PURPLE=0
    RED=0
    ROSY_BROWN=0
    ROYAL_BLUE=0
    SADDLE_BROWN=0
    SALMON=0
    SANDY_BROWN=0
    SEA_GREEN=0
    SEA_SHELL=0
    SIENNA=0
    SILVER=0
    SKY_BLUE=0
    SLATE_BLUE=0
    SLATE_GRAY=0
    SLATE_GREY=0
    SNOW=0
    SPRING_GREEN=0
    STEEL_BLUE=0
    TAN=0
    TEAL=0
    THISTLE=0
    TOMATO=0
    TRANSPARENT=0
    TURQUOISE=0
    VIOLET=0
    WHEAT=0
    WHITE=0
    WHITE_SMOKE=0
    YELLOW=0
    YELLOW_GREEN=0

class IpgContainerTheme:
    Default=0
    Custom=0


class IpgDatePickerParam:
    Label=0
    Padding=0
    RotationRadians=0
    SizeFactor=0
    Show=0


class IpgImageContentFit:
    Contain=0
    Cover=0
    Fill=0
    IpgNone=0
    ScaleDown=0


class IpgImageFilterMethod:
    Linear=0
    Nearest=0


class IpgImageRotation:
    Floating=0
    Solid=0


class IpgImageParam:
    Height=0
    HeightFill=0
    ImagePath=0
    Opacity=0
    Padding=0
    Show=0
    Width=0
    WidthFill=0


class IpgMenuParam:
    BarHeight=0
    BarPadding=0
    BarSpacing=0
    BarWidths=0
    CheckBoundsWidth=0
    Show=0


class IpgMenuType:
    Button=0
    Checkbox=0
    Circle=0
    Dot=0
    Label=0
    Line=0
    Text=0
    Toggler=0


class IpgMenuSeparatorType:
    Circle=0
    Dot=0
    Label=0
    Line=0


class IpgMouseAreaParam:
    show=0


class IpgPickListParam:
    Options=0
    Placeholder=0
    Padding=0
    Show=0
    StyleBackground=0
    StyleBorder=0
    StyleHandleColor=0
    StyleTextColor=0
    TextSize=0
    TextLineHeight=0
    Width=0
    Delete=0


class IpgPickListHandle:
    Arrow=0
    Dynamic=0
    HandleNone=0
    Static=0


class IpgProgressBarParam:
    Height=0
    Min=0
    Max=0
    Show=0
    StyleBackground=0
    StyleBorder=0
    StyleBarColor=0
    Value=0
    Width=0
    WidthFill=0



class IpgRadioDirection:
    Horizontal=0
    Vertical=0


class IpgRadioParam:
    Direction=0
    Labels=0
    Padding=0
    SelectedIndex=0
    Show=0
    Size=0
    Spacing=0
    Style=0
    TextSpacing=0
    TextSize=0
    LineHeightPixels=0
    LineHeightRelative=0
    UserData=0
    Width=0
    WidthFill=0
    Height=0
    HeightFill=0


class IpgScrollableDirection:
    Vertical=0
    Horizontal=0
    Both=0


class IpgScrollableParam:
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


class IpgSelectableTextParam:
    Text=0
    Width=0
    WidthFill=0
    Height=0
    HeightFill=0
    HorizontalAlign=0
    VerticalAlign=0
    LineHeight=0
    Size=0
    TectColor=0
    TextRgba=0
    Show=0

class IpgSliderParam:
    Min=0
    Max=0
    Step=0
    Value=0
    Width=0
    WidthFill=0
    Height=0
    Style=0
    Show=0


class IpgStyleParam:
    Background=0
    BarColor=0
    Border=0
    DotColor=0
    FillMode=0
    HandleColor=0
    IconColor=0
    Shadow=0


class IpgStyleStandard:
    Primary=0
    Success=0
    Danger=0
    Text=0


class IpgSvgParam:
    SvgPath=0
    Width=0
    WidthFill=0
    Height=0
    HeightFill=0
    Show=0

class IpgSvgContentFit:
    Contain=0
    Cover=0
    Fill=0
    IpgNone=0
    ScaleDown=0


class IpgSvgRotation:
    Floating=0
    Solid=0


class IpgTableRowHighLight:
    Darker=0
    Lighter=0


class IpgTableWidget:
    Button=0
    Checkbox=0
    Modal=0
    Toggler=0


class IpgTextInputParam:
    Placeholder=0
    Value=0
    IsSecure=0
    Width=0
    Padding=0
    Size=0
    LineHeightPixels=0
    LineHeightRelative=0


class IpgTextParam:
    Content=0
    Height=0
    HeightFill=0
    HzAlignLeft=0
    HzAlignCenter=0
    HzAlignRight=0
    LineHeight=0
    Size=0
    TextColor=0
    TextRgba=0
    VtAlignTop=0
    VtAlignCenter=0
    VtAlignBottom=0
    Width=0
    WidthFill=0
    Show=0


class IpgTogglerParam:
    Alignment=0
    Label=0
    LineHeight=0
    Show=0
    Size=0
    TextSize=0
    Width=0
    WidthFill=0


class IpgWindowParam:
    Debug=0
    Theme=0


class IpgButtonArrow:
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
