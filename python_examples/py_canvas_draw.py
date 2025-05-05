from icedpygui import IPG, IpgDrawMode, IpgCanvasParam, IpgColor, IpgColorPickerParam
from icedpygui import IpgCanvasWidget, IpgTextInputParam, IpgColorPickerStyleParam
from icedpygui import IpgHorizontalAlignment, IpgVerticalAlignment
import os


ipg = IPG()


global default_file_path
cwd = os.getcwd()
default_file_path = f"{cwd}/python_examples/resources/canvas.json"

global new_file_path
new_file_path = ""



def canvas_clear(btn_id: int):
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.Clear, 
            value=True)
    

# The radio buttons return a list where the int is the index and
# the string is the label of the radio button.  You can use either
# one for the match.
def widget_select(radio_id: int, selected: list[int, str]):
    widget = IpgCanvasWidget.Line
    match selected[0]:
        case 0:
            widget = IpgCanvasWidget.Arc
        case 1:
            widget = IpgCanvasWidget.Bezier
        case 2:
            widget = IpgCanvasWidget.Circle
        case 3:
            widget = IpgCanvasWidget.Ellipse
        case 4:
            widget = IpgCanvasWidget.Line
        case 5:
            widget = IpgCanvasWidget.Polygon
        case 6:
            widget = IpgCanvasWidget.PolyLine
        case 7:
            widget = IpgCanvasWidget.RightTriangle
        case 8:
            widget = IpgCanvasWidget.FreeHand
        case 9:
            widget = IpgCanvasWidget.Text

    ipg.update_item(canvas_id, IpgCanvasParam.Widget, widget)
    

# The IpgDrawModes are set and cannot be cahnged but you
# could use any other names for the quoated strings.
def mode_select(input_id: int, selected: str):
    match selected:
        case "Display":
            mode = IpgDrawMode.Display
        case "New":
            mode = IpgDrawMode.New
        case "Edit":
            mode = IpgDrawMode.Edit
        case "Rotate":
            mode = IpgDrawMode.Rotate
    
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.Mode, 
            value=mode)


def poly_points(input_id: int, number: int):
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.PolyPoints, 
            value=number)
    
    ipg.update_item(
            wid=input_id, 
            param=IpgTextInputParam.Value, 
            value=str(number))


# by just pressing enter while in the input text for filename
# will result in the filling in of the filename below.
# One could also use a python input method to type in a new one.
# Both the input widget and the canvas widget need to be updated.
def set_file_path(input_id: int, name: str):
    if name == "":
        global new_file_path
        new_file_path = default_file_path
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.FilePath, 
            value=new_file_path)
    
    ipg.update_item(
            wid=input_id, 
            param=IpgTextInputParam.Value, 
            value=new_file_path)


def load_file(btn_id):
    global new_file_path
    if new_file_path == "":
        new_file_path = default_file_path
        ipg.update_item(
                wid=canvas_id, 
                param=IpgCanvasParam.FilePath, 
                value=new_file_path)
       
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.Load, 
            value=None)
    

def save_file(btn_id):
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.Save, 
            value=None)
 

# Since the color picker widget doesn't know the existence
# of the canvas, the draw color will need to be updated.
def submit_draw_color_picker(cp_id: int, color: list):
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.DrawColor, 
            value=color)
    # update the button to reflex the new color
    # update the color picker button color
    ipg.update_item(
        cp_id_draw_color, 
        IpgColorPickerStyleParam.BackgroundRbga, 
        color)
    
    
def submit_fill_color_picker(cp_id: int, color: list):
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.FillColor, 
            value=color)
    # update the color picker button color
    ipg.update_item(
        cp_id_fill_color, 
        IpgColorPickerStyleParam.BackgroundRbga, 
        color)

    
def submit_canvas_color_picker(cp_id: int, color: list):
    # update the canvas color
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.CanvasColor, 
            value=color)
    # update the color picker button color
    ipg.update_item(
        cp_id_bkg_color, 
        IpgColorPickerStyleParam.BackgroundRbga, 
        color)


def set_draw_width(input_id: int, width: str):
    width_float = float(width)
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.DrawWidth, 
            value=width_float)
    

# The Alignmnet names are set but you could use anything
# you want for the quoted names as long as the list matches too.
def set_horizontal_text_alignment(pick_id: int, selected: str):
    match selected:
        case "H_Left":
            align = IpgHorizontalAlignment.Left
        case "H_Center":
            align = IpgHorizontalAlignment.Center
        case "H_Right":
            align = IpgHorizontalAlignment.Right
    
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.TextAlignment, 
            value=align)    


def set_vertical_text_alignment(pick_id: int, selected: str):
    match selected:
        case "V_Top":
            align = IpgVerticalAlignment.Top
        case "V_Center":
            align = IpgVerticalAlignment.Center
        case "V_Bottom":
            align = IpgVerticalAlignment.Bottom
            
    ipg.update_item(
            wid=canvas_id, 
            param=IpgCanvasParam.TextAlignment, 
            value=align)    
    
cp_id_bkg_color = ipg.add_color_picker_style(background_color=IpgColor.PRIMARY)
cp_id_draw_color = ipg.add_color_picker_style(background_color=IpgColor.PRIMARY)
cp_id_fill_color = ipg.add_color_picker_style(background_color=IpgColor.TRANSPARENT)

ipg.add_window(
        window_id="main", 
        title="Canvas",
        width=1000.0, 
        height=800.0,
        pos_centered=True)

ipg.add_row(
        window_id="main", 
        container_id="row",
        width_fill=True, 
        height_fill=True)

ipg.add_column(
        window_id="main", 
        container_id="col",
        parent_id="row",
        width=200, height_fill=True,
        padding=[10.0])

canvas_id = ipg.add_canvas(
                window_id="main", 
                canvas_id="canvas",
                parent_id="row",
                width_fill=True, 
                height_fill=True)

ipg.add_space(
        parent_id="col", 
        height=10.0)

ipg.add_button(
        parent_id="col", 
        label="Clear",
        on_press=canvas_clear)

widget_labels = ["Arc", "Bezier", "Circle", "Ellipse", "Line", "Polygon",
                "PolyLine", "RightTriangle", "FreeHand", "Text"]

ipg.add_radio(
        parent_id="col", 
        labels=widget_labels,
        on_select=widget_select)

mode_labels = ["Display", "New", "Edit", "Rotate"]

ipg.add_pick_list(
        parent_id="col", 
        options=mode_labels,
        placeholder="Select Mode",
        on_select=mode_select)

# By using the input and the submit, one can
# set the value without using the enter key.
# When the mouse is clicked outside the input
# the input text will be submitted.
# if you were having the called function needing
# only the whole value, then you would 
# only use the submit callback function.
ipg.add_text_input(
        parent_id="col",
        placeholder="File Name",
        width=150.0,
        on_input=set_file_path,
        on_submit=set_file_path)

ipg.add_row(
        parent_id="col", 
        container_id="file_row",
        window_id="main")

ipg.add_button(
        parent_id="file_row",
        label="Load",
        on_press=load_file)

ipg.add_button(
        parent_id="file_row",
        label="Save",
        on_press=save_file)

ipg.add_color_picker(
        parent_id="col",
        label="Set Draw Color",
        on_submit=submit_draw_color_picker,
        style_id=cp_id_draw_color)

ipg.add_color_picker(
        parent_id="col",
        label="Fill Color",
        on_submit=submit_fill_color_picker,
        style_id=cp_id_fill_color)

cp_canvas_color_id = ipg.add_color_picker(
                        parent_id="col",
                        label="Set Canvas Color",
                        style_id=cp_id_bkg_color,
                        on_submit=submit_canvas_color_picker)

ipg.add_text_input(
        parent_id="col",
        placeholder="PolyPoints(3)",
        width=150.0,
        on_input=poly_points,
        on_submit=poly_points)

ipg.add_text_input(
        parent_id="col",
        placeholder="Draw Width(2.0)",
        width=150.0,
        on_input=set_draw_width,
        on_submit=set_draw_width)

ipg.add_text(
        parent_id="col",
        content="Text Alignment")

ipg.add_pick_list(
        parent_id="col",
        options=["H_Left", "H_Center", "H_Right"],
        placeholder="H_Center",
        on_select=set_horizontal_text_alignment)

ipg.add_pick_list(
        parent_id="col",
        options=["V_Top", "V_Center", "V_Bottom"],
        placeholder="V_Center",
        on_select=set_vertical_text_alignment)


ipg.start_session()
