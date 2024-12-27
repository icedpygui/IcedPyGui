from icedpygui import IPG, IpgDrawMode, IpgCanvasParam, IpgCanvasWidget
import math


ipg = IPG()


def canvas_clear(btn_id: int):
    ipg.update_item(canvas_id, IpgCanvasParam.Clear, True)
    

def widget_select(radio_id: int, selected: tuple[int, str]):
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
    
    
def mode_select(id: int, selected: str):
    mode = IpgDrawMode.DrawAll
    match selected:
        case "DrawAll":
            mode = IpgDrawMode.DrawAll
        case "New":
            mode = IpgDrawMode.New
        case "Edit":
            mode = IpgDrawMode.Edit
        case "Rotate":
            mode = IpgDrawMode.Rotate
    
    ipg.update_item(canvas_id, IpgCanvasParam.Mode, mode)


ipg.add_window(window_id="main", title="Canvas",
               width=800.0, height=600.0,
               pos_centered=True)

ipg.add_row(window_id="main", container_id="row",
            width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col",
               parent_id="row",
               width=200, height_fill=True,
               padding=[10.0])

canvas_id = ipg.add_canvas(window_id="main", canvas_id="canvas",
               parent_id="row",
               width_fill=True, height_fill=True)

ipg.add_space(parent_id="col", height=10.0)

ipg.add_button(parent_id="col", label="Clear",
               on_press=canvas_clear,
               )

widget_labels = ["Arc", "Bezier", "Circle", "Ellipse", "Line", "Polygon",
                "PolyLine", "RightTriangle", "FreeHand", "Text"]

ipg.add_radio(parent_id="col", labels=widget_labels,
              on_select=widget_select)

mode_labels = ["DrawAll", "New", "Edit", "Rotate"]

ipg.add_pick_list(parent_id="col", 
                  options=mode_labels,
                  placeholder="Select Mode",
                  on_select=mode_select)




ipg.start_session()
