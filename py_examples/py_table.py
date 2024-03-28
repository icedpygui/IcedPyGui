from icedpygui import IPG
import random

ipg = IPG()

ipg.add_window("main", "Table Demo", width=500, height=600, 
                                    pos_centered=True)

ipg.add_container(window_id="main", container_id="cont", width=500.0, height=600.0)

col1 = []
col2 = []
col3 = []
col4 = []

for i in range(0, 50):
    # make a float random number
    col1.append(random.randrange(10, 99) + random.randrange(10, 99)/100)
    col2.append(random.choice(["one", "two", "three", "four", "five", "six", "seven"]))
    col3.append(random.randrange(10, 99))
    col4.append(random.choice([True, False]))

data = [{"Col1": col1},
        {"Col2": col2},
        {"Col3": col3},
        {"Col4": col4}]

ipg.add_table("cont", "My Table", data, width=700.0, height=600.0)

ipg.start_session()
