from icedpygui.icedpygui import IPG

ipg = IPG()

main = ipg.add_column("main", align_items="center")

grid = ipg.add_pane_grid("pg", parent_id="main")

pane1 = ipg.add_pane("pane_1", "pg", "first", ratio=0.5)
pane2 = ipg.add_pane("pane_2", "pg", add_direction="right", ratio=0.5)

text1 = ipg.add_text("pane_1", "Pane 1")
text2 = ipg.add_text("pane_2", "Pane 2")

print(main, grid, pane1, pane2, text1, text2)

ipg.main_window("Pane Grid Demo", 800, 800, 
                                    (500, 100), True, debug=False)
