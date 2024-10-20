from icedpygui import IPG, IpgCanvasMode, IpgCanvasWidget
import math

ipg = IPG()

mode = IpgCanvasMode.New


def widget_select(radio_id: int, selected: tuple[int, str]):
    match selected[0]:
        case 0:
            ipg.add_canvas_widget(canvas_id="canvas", widget=IpgCanvasWidget.Bezier, mode=mode)
        case 1:
            ipg.add_canvas_widget(canvas_id="canvas", widget=IpgCanvasWidget.Circle, mode=mode)
        case 2:
            ipg.add_canvas_widget(canvas_id="canvas", widget=IpgCanvasWidget.Line, mode=mode)
        case 3:
            ipg.add_canvas_widget(canvas_id="canvas", widget=IpgCanvasWidget.Polygon, mode=mode)
        case 4:
            ipg.add_canvas_widget(canvas_id="canvas", widget=IpgCanvasWidget.Rectangle, mode=mode)
        case 5:
            ipg.add_canvas_widget(canvas_id="canvas", widget=IpgCanvasWidget.RightTriangle, mode=mode)
        case 6:
            ipg.add_canvas_widget(canvas_id="canvas", widget=IpgCanvasWidget.Triangle, mode=mode)
    
    
def mode_select(id: int, selected: tuple[int, str]):
    match selected[0]:
        case 0:
            mode = IpgCanvasMode.New
        case 1:
            mode = IpgCanvasMode.Edit
        case 2:
            mode = IpgCanvasMode.Freehand
        case 3:
            mode = IpgCanvasMode.PicknPlace


ipg.add_window(window_id="main", title="Canvas",
               width=800.0, height=600.0,
               pos_centered=True)

ipg.add_row(window_id="main", container_id="row",
            width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col",
               parent_id="row",
               width=200, height_fill=True)

ipg.add_canvas(window_id="main", canvas_id="canvas",
               parent_id="row",
               width_fill=True, height_fill=True)

mode_labels = ["New", "Edit", "Freehand", "Pick-n-Place"]

ipg.add_radio(parent_id="col", labels=mode_labels,
              on_select=mode_select)

widget_labels = ["Bezier", "Circle", "Line", "Polygon",
                "Rectangle", "RightTriangle", "Triangle"]

ipg.add_radio(parent_id="col", labels=widget_labels,
              on_select=widget_select)

ipg.add_line(canvas_id="canvas", 
             start=(0.0, 0.0), 
             end=(50.0, 50.0),
             stroke_width=2.0)


ipg.start_session()
