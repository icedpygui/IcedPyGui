import random

from icedpygui import IPG, IpgTableRowHighLight, IpgTableWidget, IpgTableParam, IpgColor
from icedpygui import IpgTextParam, IpgAlignment, IpgHorizontalAlignment, IpgVerticalAlignment, IpgTextInputParam
from icedpygui import IpgButtonParam, IpgPickListParam, IpgDatePickerParam
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
        #          index,Title,Series,Num,Author,Status,Returned,Source,Url
        self.widths = [75, 200, 200, 40, 150, 100, 100, 150, 100]

        self.table_id=0

        self.modal_col_ids = []
        self.current_modal_row = -1
        self.modal_row = {}
        self.modal_btns = []
        self.delete_count = 0
        self.dp_id = 0

        self.sort_list = ["None", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
                          "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"]
        self.status_list = ["None", "Read", "To Be Read", "Maybe Continue Series", "Final", "Publish Date",
                            "Won't Finish", "Checked"]
        self.source_list = ["None", "Amazon Unlimited", "Amazon", "Library"]

        self.book_list_types = {"index": pl.UInt32, "Title":pl.Utf8, "Series": pl.Utf8,
                                "Num": pl.Utf8, "Author": pl.Utf8, "Status": pl.Utf8,
                                "Returned": pl.Date, "Source": pl.Utf8, "Url": pl.Utf8}

    def start(self):
        self.load()
        self.create_table()
        self.create_modal()
        self.ipg.start_session()

    def load(self):
        self.df = pl.read_csv("./python_demo/resources/books.csv",
                              try_parse_dates=False,
                              missing_utf8_is_empty_string=True)
        self.df.sort(["Author", "Series", "Num"])
        self.df.cast({"Returned": pl.Utf8})
        
        self.book_list = []
        self.column_names = self.df.columns

        for name in self.column_names:
            if name == "Returned":
                str_dates = []
                for n in self.df.get_column(name).to_list():
                    str_dates.append(f"{n}")
                
                self.book_list.append({name: str_dates})
            else:
                self.book_list.append({name: self.df.get_column(name).to_list()})
            
        
    def create_table(self):
        self.ipg.add_window(window_id="main", title="Books",
                            width=1200, height=600,
                            pos_x=100, pos_y=50,
                            )

        # add the table
        self.table_id = self.ipg.add_table(window_id="main",
                                            table_id="table",
                                            title="Books",
                                            data=self.book_list,
                                            data_length=len(self.df),
                                            width=sum(self.widths), height=600.0,
                                            column_widths=self.widths,
                                            row_highlight=IpgTableRowHighLight.Lighter,
                                            highlight_amount=0.1,
                                            button_fill_columns=[0],
                                            on_button=self.open_modal,
                                            )
    # create the modal
    def create_modal(self):
        # add some styling to the modal container
        self.ipg.add_container_style(style_id="modal_style", 
                                     background_color=IpgColor.DARK_GRAY,
                                     text_color=IpgColor.BLACK,
                                     )
        # add the container to hold everything
        self.ipg.add_container(window_id="main", 
                               container_id="modal",
                               parent_id="table",
                               width=400.0, height=400.0,
                               style_id="modal_style")
        
        # add a column for the rows to be added from top to bottom
        self.ipg.add_column(window_id="main", 
                            container_id="modal_column",
                            parent_id="modal", spacing=2.0,
                            align_items=IpgAlignment.Start)
        
        
        # create all of the text items in left and right columns
        for i, name in enumerate(self.column_names):
            # add a row to help in alignment
            self.ipg.add_row(window_id="main", 
                                container_id=f"row_{i}",
                                parent_id="modal_column", 
                                align_items=IpgAlignment.Start)
            
            # add the column name to the row
            self.ipg.add_text(parent_id=f"row_{i}", 
                                content=f"{name}:",
                                width=75.0,
                                horizontal_alignment=IpgHorizontalAlignment.Left
                                )
            # based on the name and what's needed, add a widget
            # the labels and such will be added when the modal opens
            if name == "index":
                id = self.ipg.add_text(parent_id=f"row_{i}",
                                    content="",
                                    )
            elif name == "Returned":
                id = self.ipg.add_date_picker(parent_id=f"row_{i}", label=name, on_submit=self.change_date)
                self.dp_id = id
            elif name == "Status":
                id = self.ipg.add_pick_list(parent_id=f"row_{i}", 
                                            options=self.status_list,
                                            placeholder="Status",
                                            user_data=name)
            elif name == "Source":
                id = self.ipg.add_pick_list(parent_id=f"row_{i}", 
                                            options=self.source_list,
                                            placeholder="Source",
                                            user_data=name)
            else:
                id = self.ipg.add_text_input(parent_id=f"row_{i}", 
                                            placeholder=name,
                                            width=300.0, 
                                            size=16.0,
                                            line_height_relative=1.3,
                                            padding=[0.0, 0.0, 0.0, 5.0],
                                            user_data=name
                                            )
            # need to keep the id's for later use in the modal
            self.modal_col_ids.append(id)
        
        # Once all the fields have been added, add the buttons a the bottom
        self.ipg.add_button(parent_id="modal_column", 
                            label="Exit & Discard Any Changes", 
                            on_press=self.exit_modal,)
        self.ipg.add_button(parent_id="modal_column", 
                            label="Exit & Save", 
                            on_press=self.exit_and_save)
        self.ipg.add_button(parent_id="modal_column", 
                            label="Insert New", 
                            on_press=self.insert_new)
        self.ipg.add_button(parent_id="modal_column", 
                            label="Delete Book", 
                            on_press=self.delete_book)

    # The modal button returns the table_id and the row, column tuple
    def open_modal(self, tbl_id: int, index: tuple[int, int]):
        # get the row by filtering it out and converting to  a dictionary, for ease of use
        self.modal_row = self.df.filter(pl.col("index") == index[0]).to_dict()
        # Get the row index for leter use
        self.current_modal_row = index[0]

        #  get each field and update the corresponding widget
        for i, name in enumerate(self.column_names):
            item = self.modal_row.get(name)[0]
            if name == "index":
                self.ipg.update_item(self.modal_col_ids[i], IpgTextParam.Content, f"{item}")
            elif name == "Status":
                self.ipg.update_item(self.modal_col_ids[i], IpgPickListParam.Selected, f"{item}")
            elif name == "Source":
                self.ipg.update_item(self.modal_col_ids[i], IpgPickListParam.Selected, f"{item}")
            elif name == "Returned":
                self.ipg.update_item(self.modal_col_ids[i], IpgDatePickerParam.Label, f"{item}")
            else:
                self.ipg.update_item(self.modal_col_ids[i], IpgTextInputParam.Value, f"{item}")

        # update the table's modal_show value to show the modal
        # The table update is different than the item_update, the last
        # parameter is for when the data needs updating
        self.ipg.update_item(self.table_id, IpgTableParam.ModalShow, True)

    def exit_modal(self, btn_id: int):
        # update the tables modal_show value to hide the modal
        self.ipg.update_item(self.table_id, IpgTableParam.ModalShow, False)

    def exit_and_save(self, btn_id: int):
        print("save")

    def insert_new(self, btn_id: int):
        print("insert")

    def delete_book(self, btn_id: int):
        # a way of ensuring the user wants to delete is to give 2 chances 
        # or alternately make another modal
        if self.delete_count == 0:
            self.delete_count += 1
            self.ipg.update_item(btn_id, IpgButtonParam.Label, "Press Again to Delete")
            return
        # This if is probably not needed
        if self.current_modal_row != -1:
            # make a backup copy
            self.df.write_csv("./python_demo/resources/books_bkup.csv")
            # filters out the deleted row
            self.df = self.df.filter(pl.col("index") != self.current_modal_row)
            # reindex the df
            self.df.drop_in_place("index")
            self.df = self.df.with_row_index()
            # save the file
            self.df.write_csv("./python_demo/resources/books.csv")
            self.current_modal_row = -1
            self.update_table()
            self.exit_modal(btn_id)
            # let's update the table
            self.update_table()
    
    def change_date(self, submit_btn_id: int, return_date):
        self.ipg.update_item(self.dp_id, IpgDatePickerParam.Label, return_date)

    def update_table(self):
        self.book_list = []
        self.column_names = self.df.columns

        for name in self.column_names:
            self.book_list.append({name: self.df.get_column(name).to_list()})

        self.ipg.update_item(self.table_id, IpgTableParam.Data, self.book_list)

books = Books()
books.start()
