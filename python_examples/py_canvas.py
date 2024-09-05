from icedpygui import IPG, IpgColor
import math

ipg = IPG()


ipg.add_window(window_id="main", title="Canvas",
               width=400.0, height=400.0,
               debug=True)

ipg.add_canvas(window_id="main", canvas_id="canvas",
               width_fill=True, height_fill=True)

ipg.add_arc(canvas_id="canvas",
               center=(300.0, 100.0),
               radius=50.0,
               start_angle=math.pi,
               end_angle=math.pi*2,
               stroke_width=2.0)

ipg.add_bezier(canvas_id="canvas",
               points=((50.0, 200.0), (50.0, 125.0), (150.0, 200.0)),
               radius=50.0,
               stroke_width=2.0)

ipg.add_circle(canvas_id="canvas",
               center_xy=(100.0, 100.0),
               radius=25.0,
               stroke_width=1.0,
               color=IpgColor.DARK_OLIVE_GREEN,
               fill=True)

ipg.add_ellipse(canvas_id="canvas",
                center=(100.0, 275.0),
                radii=(60.0, 10.0),
                rotation=math.pi,
                start_angle=0.0,
                end_angle=math.pi*2,
                stroke_width=2.0,
                )

ipg.add_line(canvas_id="canvas",
             points=[(50.0, 350.0), (300.0, 370.0)],
             color=IpgColor.YELLOW,
             stroke_width=2.0,)

ipg.add_rectangle(canvas_id="canvas",
               top_left_xy=(250.0, 200.0),
               width=50.0,
               height=75.0,
               stroke_width=2.0,)


ipg.start_session()
