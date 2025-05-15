from icedpygui import IPG, IpgColor
import polars as pl

ipg = IPG()


ipg.add_window(
        window_id="main", 
        title="Chart",
        width=800.0,
        height=600.0,
        pos_centered=True)

data = {
        "Evaporation": [2.0, 4.9, 7.0, 23.2, 25.6, 76.7, 135.6],
        "Precipitation": [2.6, 5.9, 9.0, 26.4, 28.7, 70.7, 175.6],
        "Temperature": [2.0, 2.2, 3.3, 4.5, 6.3, 10.2, 20.3],
    }

axis_labels = [
            "Mon",
            "Tue",
            "Wed",
            "Thu",
            "Fri",
            "Sat",
            "Sun",
        ]

df = pl.DataFrame(data)

ipg.add_chart(
    window_id="main",
    chart_id="chart",
    width=600.0,
    height=400.0,
)


ipg.start_session()
