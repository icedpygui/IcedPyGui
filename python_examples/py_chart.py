from icedpygui import IPG, IpgChartTheme


ipg = IPG()


ipg.add_window(
        window_id="main", 
        title="Chart",
        width=800.0,
        height=600.0,
        pos_centered=True)

data = [
        ("Evaporation", [2.0, 4.9, 7.0, 23.2, 25.6, 76.7, 135.6]),
        ("Precipitation", [2.6, 5.9, 9.0, 26.4, 28.7, 70.7, 175.6]),
        ("Temperature", [2.0, 2.2, 3.3, 4.5, 6.3, 10.2, 20.3]),
        ]

x_axis_labels = [
            "Mon",
            "Tue",
            "Wed",
            "Thu",
            "Fri",
            "Sat",
            "Sun",
        ]


ipg.add_chart(
    window_id="main",
    chart_id="chart",
    series=data,
    x_axis_labels=x_axis_labels,
    theme=IpgChartTheme.GrafanaTheme
)

ipg.construct_chart(["chart"])

ipg.start_session()
