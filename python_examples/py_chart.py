from icedpygui import IPG, IpgColor
import math

ipg = IPG()


ipg.add_window(
        window_id="main", 
        title="Chart",
        width=600.0,
        height=400.0,
        pos_centered=True)

ipg.add_bar_chart(
        window_id="main", 
        chart_id="chart",
        width_fill=True, 
        height_fill=True)




ipg.start_session()
