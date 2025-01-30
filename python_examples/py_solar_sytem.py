from icedpygui import IPG, IpgDrawMode, IpgCanvasParam
from icedpygui import IpgCanvasWidget, IpgWindowTheme
import os


ipg = IPG()


global default_file_path
cwd = os.getcwd()
sun_path = f"{cwd}/python_examples/resources/solar_system_assets/sun.png"

   
ipg.add_window(window_id="main", title="Solor System",
               width=800.0, height=800.0,
               pos_centered=True,
               theme=IpgWindowTheme.Moonfly)

canvas_id = ipg.add_canvas(window_id="main", canvas_id="canvas",
                             width_fill=True, height_fill=True,
                             )


ipg.add_canvas_image(canvas_id="canvas",
                     image_path=sun_path,
                     center_xy=(400.0, 400.0),
                     width=140.0,
                     height=140.0
                     )

ipg.start_session()
