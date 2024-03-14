from icedpygui.icedpygui import IPG

ipg = IPG()


ipg.add_column("main", align_items="center", width=("fixed", 800), height=("fixed", 800))

ipg.add_text_editor(parent_id="main", file_name="/home/charles/Documents/rust/icedpygui_project/IcedPyGui/py_examples/py_text_editor.py")

ipg.main_window("Python Wrapper of Rust Iced", 800, 800, 
                                    (500, 100), True, debug=False)
