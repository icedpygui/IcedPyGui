from icedpygui import IPG, IpgDrawMode, IpgCanvasParam
from icedpygui import IpgCanvasWidget, IpgWindowTheme
import os


def on_tick(timer_id, counter: int):
    print(counter)



ipg = IPG()


global default_file_path
cwd = os.getcwd()
sun_path = f"{cwd}/python_examples/resources/solar_system_assets/sun.png"
earth_path = f"{cwd}/python_examples/resources/solar_system_assets/earth.png"
   
ipg.add_window(window_id="main", title="Solor System",
               width=750.0, height=750.0,
               pos_centered=True,
               theme=IpgWindowTheme.Moonfly)

ipg.add_row(window_id="main",
            container_id="row",
            width_fill=True,
            height_fill=True)

ipg.add_timer(parent_id="row", duration_ms=1000,
              on_tick=on_tick,
             )

canvas_id = ipg.add_canvas(window_id="main", canvas_id="canvas",
                           parent_id="row",
                             width=600.0, height_fill=True,
                             )


sun_id = ipg.add_canvas_image(canvas_id="canvas",
                     image_path=sun_path,
                     width=140.0,
                     height=140.0,
                     position_xy=(325.0, 325.0),
                     align_center=True,
                     )

earth_id = ipg.add_canvas_image(canvas_id="canvas",
                     image_path=earth_path,
                     width=140.0,
                     height=140.0,
                     position_xy=(325.0, 325.0),
                     align_center=True,
                     )

ipg.start_session()
