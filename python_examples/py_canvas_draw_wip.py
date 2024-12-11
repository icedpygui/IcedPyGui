from icedpygui import IPG, IpgCanvasDrawMode, IpgCanvasParam, IpgCanvasWidget
import math


ipg = IPG()


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
            widget = IpgCanvasWidget.Line
        case 4:
            widget = IpgCanvasWidget.Polygon
        case 5:
            widget = IpgCanvasWidget.PolyLine
        case 6:
            widget = IpgCanvasWidget.RightTriangle

            
    ipg.update_item(canvas_id, IpgCanvasParam.Widget, widget)
    
    
def mode_select(id: int, selected: tuple[int, str]):
    mode = IpgCanvasDrawMode.DrawAll
    match selected[0]:
        case 0:
            mode = IpgCanvasDrawMode.DrawAll
        case 1:
            mode = IpgCanvasDrawMode.New
        case 2:
            mode = IpgCanvasDrawMode.Edit
        case 3:
            mode = IpgCanvasDrawMode.Rotate
    
    ipg.update_item(canvas_id, IpgCanvasParam.Mode, mode)


ipg.add_window(window_id="main", title="Canvas",
               width=800.0, height=600.0,
               pos_centered=True)

ipg.add_row(window_id="main", container_id="row",
            width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col",
               parent_id="row",
               width=200, height_fill=True)

canvas_id = ipg.add_canvas(window_id="main", canvas_id="canvas",
               parent_id="row",
               width_fill=True, height_fill=True)

mode_labels = ["DrawAll", "New", "Edit", "Rotate"]

ipg.add_radio(parent_id="col", labels=mode_labels,
              on_select=mode_select)

widget_labels = ["Arc", "Bezier", "Circle", "Line", "Polygon",
                "PolyLine", "RightTriangle"]

ipg.add_radio(parent_id="col", labels=widget_labels,
              on_select=widget_select)


ipg.start_session()
