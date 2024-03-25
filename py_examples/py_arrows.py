from icedpygui import IPG
from arrows import arrows, styles

import random

ipg = IPG()

def update(id):
    global btn
    ipg.update_item(btn, "label", "new label")



ipg.add_window("main", "CheckBox Demo",
                800, 800, 500, 100)
        
ipg.add_column(window_id="main", container_id="col", width_fill=True,
                        height_fill=True, align_items="center", spacing=5)


count = len(arrows)
print(count)
for j in range(0, 100, 10):
    if j > count: break
    ipg.add_row(window_id="main", container_id=f"row{j}", parent_id="col", spacing=10, padding=[5])

    for i in range(0, 10):
        if i+j < count:
            ipg.add_button(f"row{j}", "", arrow_style=arrows[i+j],
                            corner_radius=0.0, style=random.choice(styles))


ipg.start_session()



