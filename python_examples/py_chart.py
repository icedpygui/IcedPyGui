from icedpygui import IPG, IpgColor
import math

ipg = IPG()


ipg.add_window(
        window_id="main", 
        title="Chart",
        width=700.0,
        height=500.0,
        pos_centered=True)

ipg.add_bar_chart(
        window_id="main", 
        chart_id="chart",
        width_fill=True, 
        height_fill=True)




ipg.start_session()
