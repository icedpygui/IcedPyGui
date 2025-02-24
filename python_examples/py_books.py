import random
import os
from icedpygui import IPG, IpgTableRowHighLight, IpgTableParam, IpgColor
from icedpygui import IpgTextParam, IpgAlignment, IpgHorizontalAlignment, IpgVerticalAlignment, IpgTextInputParam
from icedpygui import IpgButtonParam, IpgPickListParam, IpgDatePickerParam
import polars as pl


# Just to demo how one might use a large table, I have supplied my book list that
# I have read over the last few years.  You may note that I'm a scifi reader but
# am trying to expand more into thrillers and mystery with a touch of other genres.

# Note:: I'll be adding a control row beneath the header for sorting and 
# other types of actions or incorporate it into the header row itself.

# Note: You will need to install polars and have the resource folder with the csv books file.
# I use polars vs pandas because its much faster for larger tables.

class Books:
    def __init__(self):
        self.ipg = IPG()
        self.df = pl.DataFrame
        
        self.wnd_width = 1200.0
        self.wnd_height = 600.0
        
        self.book_list = []
        self.column_names = []
        self.table_list = pl.DataFrame()
        #          index,Title,Series,Num,Author,Status,Returned,Source,Url
        self.column_widths = [75, 200, 200, 40, 150, 100, 100, 150, 100]
        self.tbl_width = sum(self.column_widths)
        self.tbl_height = 600.0

        self.table_id=0
        self.column_id_names = [ str(i) for i in range(0, len(self.column_widths)) ]
        self.ids = pl.Series
        self.list_ids = []

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
        self.sort_selected = "None"
        
        self.btn_style_id = 0

    def start(self):
        self.load()
        self.create_styles()
        self.create_table()
        # self.create_modal()
        self.ipg.start_session()

    def load(self):
        cwd = os.getcwd();
        self.df = pl.read_csv(f"{cwd}/python_examples/resources/books.csv",
                              try_parse_dates=False,
                              missing_utf8_is_empty_string=True)
        self.df = self.df.sort(["Author", "Series", "Num"])
        
        self.column_names = self.df.columns

    def create_styles(self):
        self.btn_style_id = self.ipg.add_button_style(border_radius=[10.0])
    
    # ******************************create table************************************
    def create_table(self):
        self.ipg.add_window(
                    window_id="main", 
                    title="Books",
                    width=self.wnd_width, 
                    height=self.wnd_height,
                    pos_centered=True)

        # Add the container for centering the table
        self.ipg.add_container(
                    window_id="main", 
                    container_id="cont",
                    width_fill=True, 
                    height_fill=True,
                    padding=[10.0])
        
        # Since a modal is needed, add a stack
        # the stack will be as big as the table
        # becuase the default is Shrink
        self.ipg.add_stack(
                    window_id="main",
                    container_id="stack",
                    parent_id="cont")

        # add the table
        # Once the table is created, the next additions to the
        # table MUST follow in order, column names (if enabled), rows, footer(if enabled)
        # The additions can occur at any time just as long as they are in order.
        # The reason is that during the processing, a vector containing all of the widgets
        # are sent to the table where the extraction of each group occurs.  Therefore, if the
        # column names were not added first then the first row would be added in as column names
        # unless the header is disabled.
        # Since we are using a stack as the "modal", we add the table to the stack
        # The container you want to appear will be add later and a show and hide
        # operation will reveal it.  Since it will be sitting on top of the stack,
        # it will be seen when shown.
        self.table_id = self.ipg.add_table(
                                    window_id="main",
                                    table_id="table",
                                    parent_id="stack",
                                    title="Books",
                                    column_widths=self.column_widths,
                                    height=self.tbl_height)
        
        # create columns/header row
        # In ths case we have centered the names
        # The parent id has to match that of the table
        for (i, name) in enumerate(self.column_names):
            self.ipg.add_column(
                        window_id="main",
                        container_id=f"header{i}",
                        parent_id="table",
                        width_fill=True)
            self.ipg.add_text(
                        parent_id=f"header{i}",
                        content=name,
                        align_x=IpgHorizontalAlignment.Center,
                        align_y=IpgVerticalAlignment.Center,
                        width_fill=True)
            
            if name == "Author":
                self.ipg.add_pick_list(
                            parent_id=f"header{i}",
                            options=self.sort_list,
                            on_select=self.filter_books_author,
                            selected=self.sort_selected,
                            width_fill=True)
                
            if name == "Status":
                self.ipg.add_pick_list(
                            parent_id=f"header{i}",
                            options=self.status_list,
                            on_select=self.filter_books_status,
                            selected=self.sort_selected,
                            width_fill=True)
                
            if name == "Source":
                self.ipg.add_pick_list(
                            parent_id=f"header{i}",
                            options=self.source_list,
                            on_select=self.filter_books_source,
                            selected=self.sort_selected,
                            width_fill=True)
                
                
            
        # Add the rows
        # make a new dataframe for ids so that the rows can be edited/filtered/updated
        ids = {}
        scheme = {}
        for i in range(0, len(self.column_widths)):
            ids[str(i)] = []
            scheme[str(i)] = pl.Int64
        
        df_ids = pl.DataFrame(ids, scheme)

        # iter through the rows to create the needed widgets
        # is are loaded into a df
        for i in range(0, len(self.df)):
            row = self.df.row(i)
            ids = {}
            for k in range(0, len(row)):
                ids[str(k)] = []
            for j in range(0, len(row)):
                if j == 0:
                    ids[str(j)] = self.ipg.add_button(
                                    parent_id="table",
                                    label=f"Edit",
                                    width=self.column_widths[0],
                                    style_id=self.btn_style_id,
                                    text_align_x=IpgHorizontalAlignment.Center,
                                    on_press=self.open_modal,
                                    padding=[0.0],
                                    user_data=i)
                elif self.column_names[j] == "Url" and row[j] != "":
                    ids[str(j)] = self.ipg.add_button(
                                    parent_id="table",
                                    label=self.column_names[j],
                                    width=self.column_widths[j],
                                    style_id=self.btn_style_id,
                                    text_align_x=IpgHorizontalAlignment.Center,
                                    on_press=self.show_url,
                                    padding=[0.0],
                                    user_data=row[j])
                else:
                    ids[str(j)] = self.ipg.add_text(
                                    parent_id="table",
                                    content=row[j],
                                    width_fill=True,
                                    align_x=IpgHorizontalAlignment.Center,
                                    align_y=IpgVerticalAlignment.Center,
                                    )
            
            # add this row id ids into the main df
            new_df = pl.DataFrame(ids)
            df_ids = pl.concat([df_ids, new_df])

            # extend the list of ids for easier use in the update methods
            self.list_ids.extend(list(ids.values()))
        
        # finally, concat the ids with the data vertically so that they remain together
        self.df = pl.concat([self.df, df_ids], how="horizontal", rechunk=True)
        # print(self.df.row(0))
        
    # ******************************create modal************************************
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
                                            placeholder=name,
                                            user_data=name,
                                            on_select=self.on_select,
                                            )
            elif name == "Source":
                id = self.ipg.add_pick_list(parent_id=f"row_{i}", 
                                            options=self.source_list,
                                            placeholder=name,
                                            user_data=name,
                                            on_select=self.on_select,
                                            )
            else:
                id = self.ipg.add_text_input(parent_id=f"row_{i}", 
                                            placeholder=name,
                                            width=300.0, 
                                            size=16.0,
                                            line_height_relative=1.3,
                                            padding=[0.0, 0.0, 0.0, 5.0],
                                            on_input=self.input_changed,
                                            on_submit=self.on_submit,
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

    # ******************************open modal************************************
    def open_modal(self, tbl_id: int, index: tuple[int, int]):
        # get the row by filtering it out and converting to a dictionary, for ease of use
        self.modal_row = self.df.filter(pl.col("index") == index[0]).to_dict()
        # Get the row index for later use
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
        self.save_table_backup()
        # delete the row to be updated
        self.df = self.df.filter(pl.col("index") != self.modal_row["index"])
        # make the modal row a df
        modal_df = pl.DataFrame(self.modal_row)
        # drop the index row on both dfs
        self.df.drop_in_place("index")
        modal_df.drop_in_place("index")
        # concat the dfs
        self.df = pl.concat([self.df, modal_df])
        # sort it
        self.df = self.df.sort(["Author", "Series", "Num"])
        # add in the index row
        self.df = self.df.with_row_index()
        self.save_table()
        # update the table
        self.update_table()
        # exit
        self.exit_modal(0)

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
            self.save_table_backup()
            # filters out the deleted row
            self.df = self.df.filter(pl.col("index") != self.current_modal_row)
            # reindex the df
            self.df.drop_in_place("index")
            self.df = self.df.with_row_index()
            self.save_table()
            self.current_modal_row = -1
            self.update_table()
            self.exit_modal(btn_id)
            # reset the button
            self.delete_count == 0
            self.ipg.update_item(btn_id, IpgButtonParam.Label, "Delete Book")
  
    def save_table_backup(self):
        self.df.write_csv("./python_demo/resources/books_bkup.csv")

    def save_table(self):
        self.df.write_csv("./python_demo/resources/books.csv")

    # The name parameter is the user_data
    def on_submit(self, ti_id: int, value: str, name: str):
        print(value)
        # update the text_input widget so the title shows otherwise the return clears the field
        self.ipg.update_item(ti_id, IpgTextInputParam.Value, value)
        # since the modal row was updated by the input changed, no need here
        

    # user_data not used here but still needs to be in the parameter list
    def input_changed(self, ti_id: int, value: str, name: str):
        # Update the text_input value as it's typed
        self.ipg.update_item(ti_id, IpgTextInputParam.Value, value)
        # update the modal row
        self.modal_row[name] = value

    def on_select(self, pl_id: int, value: str, name: str):
        # update the picklist widget
        self.ipg.update_item(pl_id, IpgPickListParam.Selected, value)
        # update the modal row
        self.modal_row[name] = value

    def change_date(self, submit_btn_id: int, return_date):
        self.ipg.update_item(self.dp_id, IpgDatePickerParam.Label, return_date)
        # update the modal row
        self.modal_row["Returned"] = return_date

    def update_table(self):
        self.book_list = []
        self.column_names = self.df.columns

        for name in self.column_names:
            self.book_list.append({name: self.df.get_column(name).to_list()})

        self.ipg.update_item(self.table_id, IpgTableParam.Data, self.book_list)

    def show_url(self, btn_id: int, url: str):
        print(url)

    def filter_books_author(self, pick_id: int, selected: str):
        if selected == "None":
            ids = []
            for id in self.list_ids:
                ids.append((id, True))
            self.ipg.show_item("main", ids)
            return
        
        
        # filter the df
        df = self.df.filter(pl.col('Author').str.to_lowercase().str.starts_with(selected.lower()))
        # select only the columns with the ids
        keepers = df.select(self.column_id_names)
        list_to_keep = []
        
        for column in keepers.iter_columns():
            list_to_keep.extend(column.to_list())
        
        ids = []    
        for id in self.list_ids:
            if id not in list_to_keep:
                ids.append((id, False))
            else:
                # else used because the table might have already been filtered
                ids.append((id, True))
            
        self.ipg.show_item("main", ids)


    def filter_books_status(self, pick_id: int, selected: str):
        print(selected)

    def filter_books_source(self, pick_id: int, selected: str):
        print(selected)
        
    def find_id_in_dataframe(self, df: pl.DataFrame, id: int) -> bool:
        for col in self.df.columns:
            if (self.df[col] == id).any():
                return True
        return False
    
books = Books()
books.start()
