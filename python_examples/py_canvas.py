import os
from icedpygui import IPG, IpgColor
import math

ipg = IPG()


ipg.add_window(
        window_id="main", 
        title="Canvas",
        width=400.0,
        height=400.0,
        pos_centered=True)

ipg.add_canvas(
        window_id="main", 
        canvas_id="canvas",
        width_fill=True, 
        height_fill=True)

ipg.add_arc(
        canvas_id="canvas",
        center_xy=(250.0, 100.0),
        radius=50.0,
        start_angle=0.0,
        end_angle=180.0,
        stroke_width=2.0)

ipg.add_bezier(
        canvas_id="canvas",
        points=((50.0, 200.0), (50.0, 125.0), (150.0, 200.0)),
        stroke_width=2.0)

ipg.add_circle(
        canvas_id="canvas",
        position_xy=(100.0, 100.0),
        radius=25.0,
        stroke_width=1.0,
        fill_ipg_color=IpgColor.DARK_OLIVE_GREEN)

ipg.add_ellipse(
        canvas_id="canvas",
        position_xy=(100.0, 275.0),
        radius_x=60.0, 
        radius_y=10.0,
        degrees=math.pi,
        stroke_width=2.0)

ipg.add_line(
        canvas_id="canvas",
        start=(50.0, 350.0), 
        end=(300.0, 370.0),
        stroke_ipg_color=IpgColor.YELLOW,
        stroke_width=2.0,)

ipg.add_rectangle(
        canvas_id="canvas",
        top_left_xy=(250.0, 200.0),
        width=50.0,
        height=75.0,
        stroke_width=2.0,)

# Setting up the image path
cwd = os.getcwd()

ferris0 = cwd + "/python_examples/resources/ferris_0.png"
ferris1 = cwd + "/python_examples/resources/ferris_1.png"

ipg.add_canvas_image(
    canvas_id="canvas",
    image_path=ferris0,
    width=50.0,
    height=50.0,
    position_xy=(275.0, 325.0)
)

ipg.add_canvas_image(
    canvas_id="canvas",
    image_path=ferris1,
    width=50.0,
    height=50.0,
    position_xy=(330.0, 325.0)
)

ipg.start_session()
