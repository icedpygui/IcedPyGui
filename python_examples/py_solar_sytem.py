from icedpygui import IPG, IpgDrawMode, IpgCanvasParam, IpgColor
from icedpygui import IpgCanvasWidget, IpgWindowTheme, IpgCanvasGeometryParam
import os
import random
import math


def point_on_circle(center_x, center_y, radius, angle):
    x = center_x + radius * math.cos(angle)
    y = center_y + radius * math.sin(angle)
    return [x, y]


def on_tick(timer_id: int, elapsed: int):
    angle = (2.0 * math.pi / 60.0) * elapsed + (2.0 * math.pi / 60_000.0) * elapsed/1000
    point = point_on_circle(canvas_width/2.0, canvas_height/2.0, 150.0, angle)
    ipg.update_canvas_item(earth_id, IpgCanvasGeometryParam.Position, point)



ipg = IPG()


global default_file_path
cwd = os.getcwd()
sun_path = f"{cwd}/python_examples/resources/solar_system_assets/sun.png"
earth_path = f"{cwd}/python_examples/resources/solar_system_assets/earth.png"
moon_path = f"{cwd}/python_examples/resources/solar_system_assets/moon.png"

canvas_width = 800.0
canvas_height = 725.0
   
ipg.add_window(window_id="main", title="Canvas",
               width=800.0, height=800.0,
               pos_centered=True,
               )

ipg.add_column(window_id="main", container_id="col",
            width_fill=True, height_fill=True)

ipg.add_canvas_timer(parent_id="col", 
                    duration_ms=1000,
                    on_tick=on_tick,
                  )

canvas_id = ipg.add_canvas(window_id="main", 
                           canvas_id="canvas",
                           parent_id="col",
                           width=canvas_width, 
                           height=canvas_height,
                           background_ipg_color=IpgColor.BLACK)

sun_id = ipg.add_canvas_image(canvas_id="canvas",
                     image_path=sun_path,
                     width=140.0,
                     height=140.0,
                     position_xy=(canvas_width/2.0, canvas_height/2.0),
                     align_center=True,
                     )

earth_id = ipg.add_canvas_image(canvas_id="canvas",
                     image_path=earth_path,
                     width=24.0,
                     height=24.0,
                     position_xy=(canvas_width/2.0, canvas_height/2.0-150.0),
                     align_center=True,
                     )

moon_id = ipg.add_canvas_image(canvas_id="canvas",
                     image_path=moon_path,
                     width=8.0,
                     height=8.0,
                     position_xy=(canvas_width/2.0+20.0, canvas_height/2.0-150.0-20.0),
                     align_center=True,
                     )

earth_obit_id = ipg.add_circle(canvas_id="canvas",
               position_xy=(canvas_width/2.0, canvas_height/2.0),
               radius=150.0,
               stroke_width=1.0,
               stroke_ipg_color=IpgColor.WHITE,
               stroke_color_alpha=0.1,
               stroke_dash_offset=0,
               stroke_dash_segments=[3.0, 6.0],
              )

# generate a random star pattern
for _ in range(0, 100):
  x = float(random.randint(0, int(canvas_width)))
  y = float(random.randint(0, int(canvas_height)))
  ipg.add_rectangle(canvas_id="canvas",
                 top_left_xy=(x, y),
                 width=0.1,
                 height=0.1,
                 fill_ipg_color=IpgColor.WHITE
                 )




ipg.start_session()
