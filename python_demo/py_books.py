import random

from icedpygui import IPG, IpgTableRowHighLight, IpgTableWidget, IpgTableParam, IpgColor
from icedpygui import IpgTextParam, IpgAlignment, IpgHorizontalAlignment, IpgVerticalAlignment, IpgTextInputParam
import polars as pl
from datetime import datetime, date

# Just to demo how one might use a large table, I have supplied my book list that
# I have read over the last few years.  You may note that I'm a scifi reader but
# am trying to exapnd more into thrillers and mystery with a touch of other genres.

# Note:: The modal is not working at this time but should be up and going in about
# a week after this is published.  When you press a button you will just get a dimmed
# window for now.  I'll be adding a control row beneath the header for sorting and 
# other types of actions or incoporate it into the her row itself.

# Note: You will need to install polars and have the resource folder with the csv books file.
#  I use polars vs pandas because its much faster for larger tables.

class Books:
    def __init__(self):
        self.ipg = IPG()
        self.df = pl.DataFrame
        self.book_list = []
        self.column_names = []
        self.table_list = pl.DataFrame()
        #          Edit,Title,Series,Num,Author,Status,Returned,Source,Url
        self.widths = [100, 200, 200, 40, 150, 100, 100, 150, 100]

        self.table_id=0

        self.modal_col_ids = []

        self.sort_list = ["None", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
                          "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"]
        self.status_list = ["None", "Read", "To Be Read", "Maybe Continue Series", "Final", "Publish Date",
                            "Won't Finish", "Checked"]
        self.source_list = ["None", "Amazon Unlimited", "Amazon", "Library"]

    def start(self):
        self.load()
        self.create_table()
        self.create_modal()
        self.ipg.start_session()

    def load(self):
        self.df = pl.read_csv("./python_demo/resources/books.csv")
        self.book_list = []
        self.column_names = self.df.columns

        for name in self.column_names:
            self.book_list.append({name: self.df.get_column(name).to_list()})

    def create_table(self):
        self.ipg.add_window(window_id="main", title="Books",
                            width=1200, height=600,
                            pos_x=100, pos_y=50,
                            )

        modals = []
        for _ in range(0, len(self.df)):
            modals.append(IpgTableWidget.Button)

        self.table_id = self.ipg.add_table(window_id="main",
                                            table_id="table",
                                            title="Books",
                                            data=self.book_list,
                                            data_length=len(self.df),
                                            width=1100.0, height=600.0,
                                            column_widths=self.widths,
                                            row_highlight=IpgTableRowHighLight.Lighter,
                                            highlight_amount=0.1,
                                            widgets_columns={0: modals},
                                            on_button=self.open_modal,
                                            )
        
    def create_modal(self):

        self.ipg.add_container_style(style_id="modal_style", 
                                     background_color=IpgColor.DARK_GRAY,
                                     text_color=IpgColor.BLACK,
                                     )

        self.ipg.add_container(window_id="main", 
                               container_id="modal",
                               parent_id="table",
                               width=400.0, height=400.0,
                               style_id="modal_style")
        
        self.ipg.add_column(window_id="main", 
                            container_id="modal_column",
                            parent_id="modal", spacing=2.0,
                            align_items=IpgAlignment.Start)
        
        
        # create all of the text items in left and right columns
        for i, name in enumerate(self.column_names):
            self.ipg.add_row(window_id="main", 
                                container_id=f"row_{i}",
                                parent_id="modal_column", 
                                align_items=IpgAlignment.Start)
            self.ipg.add_text(parent_id=f"row_{i}", 
                                content=f"{name}:",
                                width=75.0,
                                horizontal_alignment=IpgHorizontalAlignment.Left
                                )
            if name == "row":
                id = self.ipg.add_text(parent_id=f"row_{i}",
                                    content="",
                                    )
            else:
                id = self.ipg.add_text_input(parent_id=f"row_{i}", 
                                            placeholder=f"{name}",
                                            width=300.0, 
                                            size=16.0,
                                            line_height_relative=1.3,
                                            padding=[0.0, 0.0, 0.0, 5.0]
                                            )
            self.modal_col_ids.append(id)

        self.ipg.add_button(parent_id="modal_column", label="Exit", on_press=self.exit_modal)
    
    def open_modal(self, tbl_id: int, index: tuple[int, int]):
        df_row = self.df.filter(pl.col("row") == index[0]).to_dict()

        for i, name in enumerate(self.column_names):
            item = df_row.get(name)[0]
            if name == "row":
                self.ipg.update_item(self.modal_col_ids[i], IpgTextParam.Content, f"{item}")
            else:
                self.ipg.update_item(self.modal_col_ids[i], IpgTextInputParam.Value, f"{item}")
            
        self.ipg.update_table(self.table_id, IpgTableParam.ModalShow, True, None)

    def exit_modal(self, btn_id: int):
        self.ipg.update_table(self.table_id, IpgTableParam.ModalShow, False, None)


books = Books()
books.start()
