import os
from icedpygui import IPG, IpgColor
import math

ipg = IPG()


ipg.add_window(
        window_id="main", 
        title="Chart",
        width=800.0,
        height=600.0,
        pos_centered=True)


ipg.add_bar_chart(
    window_id="main",
    chart_id="chart",
    width=600.0,
    height=400.0,
)


ipg.start_session()
