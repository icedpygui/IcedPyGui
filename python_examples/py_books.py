
import os, uuid
from icedpygui import IPG, IpgTableRowHighLight, IpgTableParam, IpgColor
from icedpygui import IpgTextParam, IpgAlignment, IpgHorizontalAlignment, IpgVerticalAlignment, IpgTextInputParam
from icedpygui import IpgButtonParam, IpgPickListParam, IpgOpaqueParam, IpgDatePickerParam
import polars as pl
import webbrowser

from icedpygui.icedpygui import IpgContainerParam


# Just to demo how one might use a large table, I have supplied my book list that
# I have read over the last few years.  You may note that I'm a scifi reader but
# am trying to expand more into thrillers and mystery with a touch of other genres.

# Note: You will need to install polars and have the resource folder with the csv books file.
# I use polars vs pandas because its much faster for larger tables and since the backend
# is rust, pandas is not available to use anyway.
# 
# I originally used a book list that I had created with dearpygui, so now that I have PDG,
# I'm switching over to this version.  I find using polars in rust a lot more difficult
# and so this is much easier and I still have the speed with large tables.

# I have some finantial table writton with dearpygui, a fairly complex program that
# I'll convert and share later this year.  Iced is missing acouple of widgets that I used
# heavily in the finatial program, so a bit of work will be needed to convert.

# File a bit long because I wanted to keep it in a single file, normally would split up



class Books:
    def __init__(self):
        self.ipg = IPG()
        self.df = pl.DataFrame
        
        self.wnd_width = 1200.0
        self.wnd_height = 600.0
        
        self.book_list = []
        self.column_names = []
        self.table_list = pl.DataFrame()
        #          id,Title,Series,Num,Author,Status,Returned,Source,Url
        self.column_widths = [100, 200, 200, 40, 150, 100, 120, 150, 75]
        self.tbl_width = sum(self.column_widths)
        self.tbl_height = 600.0

        self.table_id=0
        self.column_id_names = [ str(i) for i in range(0, len(self.column_widths)) ]
        self.header_control_ids = []
        self.footer_control_ids = []

        self.modal_id = 0
        self.modal_col_ids = {}
        self.current_modal_row = -1
        self.modal_row = {}
        self.modal_btns = []
        self.delete_count = 0
        self.dp_id = 0

        self.author_sort_list = ["None", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
                          "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"]
        self.status_list = ["None", "Read", "To Be Read", "Maybe Continue Series", "Final", "Publish Date",
                            "Won't Finish", "Checked"]
        self.source_list = ["None", "Amazon Unlimited", "Amazon", "Library"]
        self.returned_list = []
        
        self.author_selected = "None"
        self.status_selected = "None"
        self.source_selected = "None"
        self.returned_selected = "None"
        
        self.btn_style_id = 0
        self.modal_style_id = 0

    def start(self):
        self.load()
        self.create_styles()
        self.create_window()
        self.create_table()
        self.create_control_columns()
        self.create_modal()
        
        self.ipg.start_session()

    # ******************************Load Data************************************ 
    def load(self):
        cwd = os.getcwd()
        self.df = pl.read_csv(f"{cwd}/python_examples/resources/books.csv",
                              try_parse_dates=False,
                              missing_utf8_is_empty_string=True)
        # self.df = self.df.drop("edit_id")
        # self.df = self.df.drop("url_id")
        # self.save_table()
        # quit()
        self.df = self.df.sort(["Author", "Series", "Num"])
        self.column_names = self.df.columns
        
        # Make the dropdown list for the years
        s = self.df.select(
                pl.col('Returned').str.slice(0, 4)
                ).unique().to_series()
        self.returned_list = s.filter(s != "").sort().to_list()
        self.returned_list.insert(0, "None")

    # ******************************create styles************************************
    def create_styles(self):
        self.btn_style_id = self.ipg.add_button_style(border_radius=[10.0])
        
        self.modal_style_id = self.ipg.add_container_style( 
                                    background_color=IpgColor.DARK_GRAY,
                                    text_color=IpgColor.BLACK)
    
    # ******************************create window************************************
    def create_window(self):
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
        # because the default is Shrink
        self.ipg.add_stack(
                    window_id="main",
                    container_id="stack",
                    parent_id="cont")
        
    # ******************************create table************************************
    def create_table(self):
        # Once the table is created, the next additions to the
        # table MUST follow in order, header (if enabled) followed by the footer(if enabled)
        # The header and footer need to be done before the control columns.
        # The reason is that during the processing, a vector containing all of the widgets
        # are sent to the table where the widgets are extracted for each group.
        # Since the vector of elements don't contain any accessible info on what
        # they are, the extraction had to be based on columns numbers and header, 
        # footer, body order.
        # 
        # Since we are using a stack as the "modal", we add the table to the stack
        # The container you want to appear will be add later and a show and hide
        # operation will reveal it.  Since it will be sitting on top of the stack,
        # it will be seen when shown.
        self.table_id = self.ipg.add_table(
                                    window_id="main",
                                    table_id="table",
                                    parent_id="stack",
                                    title="Books",
                                    polars_df=self.df,
                                    header_custom_enabled=True,
                                    footer_enabled=True,
                                    control_columns=[0, 8],
                                    column_widths=self.column_widths,
                                    height=self.tbl_height)
        
        # create header row
        # In ths case we have centered the names
        # The parent id has to match that of the table
        for (i, name) in enumerate(self.column_names):
            self.ipg.add_column(
                        window_id="main",
                        container_id=f"header{i}",
                        parent_id="table",
                        width_fill=True)
            
            if name == "id": 
                name = ""
            
            self.ipg.add_text(
                        parent_id=f"header{i}",
                        content=name,
                        align_x=IpgHorizontalAlignment.Center,
                        align_y=IpgVerticalAlignment.Center,
                        width_fill=True)
            
            if name == "":
                self.new_book_id = self.ipg.add_button(
                            parent_id=f"header{i}",
                            label="New Book",
                            text_align_x=IpgHorizontalAlignment.Center,
                            width_fill=True,
                            on_press=self.open_modal,
                            padding=[0.0],
                            style_id=self.btn_style_id,
                            user_data=999999) 
                            # needed large number to indicate new book
                            # other approaches could be used by not calling open_modal directly
            if name == "Title":
                id = self.ipg.add_text_input(
                                parent_id=f"header{i}",
                                placeholder="Search",
                                width_fill=True,
                                on_input=self.search_titles)
                
            if name == "Series":
                id = self.ipg.add_text_input(
                                parent_id=f"header{i}",
                                placeholder="Search",
                                width_fill=True,
                                on_input=self.search_series)
                
            if name == "Author":
                id = self.ipg.add_pick_list(
                                parent_id=f"header{i}",
                                options=self.author_sort_list,
                                on_select=self.filter_books_author,
                                selected=self.author_selected,
                                width_fill=True)
                self.header_control_ids.append(id)
                
            if name == "Status":
                id = self.ipg.add_pick_list(
                                parent_id=f"header{i}",
                                options=self.status_list,
                                on_select=self.filter_books_status,
                                selected=self.status_selected,
                                width_fill=True)
                self.header_control_ids.append(id)
                
            if name == "Returned":
                id = self.ipg.add_pick_list(
                                parent_id=f"header{i}",
                                options=self.returned_list,
                                on_select=self.filter_books_return_date,
                                selected=self.returned_selected,
                                width_fill=True)
                self.header_control_ids.append(id)
                
            if name == "Source":
                id = self.ipg.add_pick_list(
                                parent_id=f"header{i}",
                                options=self.source_list,
                                on_select=self.filter_books_source,
                                selected=self.source_selected,
                                width_fill=True)
                self.header_control_ids.append(id)
    
        # create the footer
        for (i, name) in enumerate(self.column_names):
            # The entire row must be filled with something
            # so the empty string is used to fill where no widget.
            content = ""
            if name == "Returned":
                content = f"Total = {self.df.height}"
            id = self.ipg.add_text(
                        parent_id="table",
                        content=content,
                        width_fill=True,
                        align_x=IpgHorizontalAlignment.Center,
                        align_y=IpgVerticalAlignment.Center,
                        )
            self.footer_control_ids.append(id)
    
    # ******************************create control columns*************************
    def create_control_columns(self):
        schema = {'edit_id': pl.Int64, "url_id": pl.Int64}
        control_ids = pl.DataFrame(schema=schema)
        for row in self.df.rows():
            # edit column
            edit_id = self.make_edit_button(row[0])
            
            # url column
            url_id = self.make_url_button(row[8], row[0])
            
            control_id = pl.DataFrame({"edit_id": edit_id, "url_id": url_id})
            control_ids = pl.concat([control_ids, control_id])
            
        # add the ids to the main df so that when things rearrange or sorted, the 
        # ids remain with the assigned row.
        self.df = pl.concat([self.df, control_ids], how="horizontal")
        
    # ******************************create modal************************************
    def create_modal(self):

        # add the container to hold and center everything
        # if this is not used, then the modal will be
        # aligned on the left unless you make the
        # modal cover the table
        self.modal_id = self.ipg.add_container(
                                window_id="main", 
                                container_id="modal",
                                parent_id="stack",
                                width_fill=True,
                                height_fill=True,
                                show =False,
                                centered=True)
        
        # add a container to style the background
        # The opaque container is used so that the
        # mouse actions are not seen by the table
        # otherwise if you clicked at the top,
        # the dropdowns in the header would activate
        self.ipg.add_opaque_container(
                    window_id="main", 
                    container_id="bkg",
                    parent_id="modal",
                    width=400.0,
                    height=500.0,
                    centered=True,
                    style_id=self.modal_style_id)
        
        # add a column for the rows to be added from top to bottom
        self.ipg.add_column(
                    window_id="main", 
                    container_id="modal_column",
                    parent_id="bkg",
                    width=400.0, 
                    spacing=2.0,
                    padding=[5.0],
                    align=IpgAlignment.Start)
        
        
        # create all of the text items in left and right columns
        for i, name in enumerate(self.column_names):
            # add a row to help in alignment
            self.ipg.add_row(
                        window_id="main", 
                        container_id=f"row_{i}",
                        parent_id="modal_column", 
                        align=IpgAlignment.Start)
            
            # add the column name to the row
            if name != "id":
                self.ipg.add_text(
                            parent_id=f"row_{i}", 
                            content=f"{name}:",
                            width=75.0,
                            align_x=IpgHorizontalAlignment.Left)
            
            # based on the name and what's needed, add a widget
            # the labels and such will be added when the modal opens
            match name:
                case "id":
                    id = 0
                
                case "Returned":
                    # Since the date_picker updates the text filed
                    # the id of the text is used versus the dp_id
                    id = self.ipg.add_text(
                            parent_id=f"row_{i}",
                            content="")
                    
                    self.ipg.add_date_picker(
                            parent_id=f"row_{i}", 
                            label="Date", 
                            on_submit=self.change_date,
                            user_data=name)
                    
                case "Status":
                    id = self.ipg.add_pick_list(
                            parent_id=f"row_{i}", 
                            options=self.status_list,
                            placeholder=name,
                            user_data=name,
                            on_select=self.on_select)
                    
                case "Source":
                    id = self.ipg.add_pick_list(
                            parent_id=f"row_{i}", 
                            options=self.source_list,
                            placeholder=name,
                            user_data=name,
                            on_select=self.on_select)
                case _:
                    id = self.ipg.add_text_input(parent_id=f"row_{i}", 
                            placeholder=name,
                            width=300.0, 
                            size=16.0,
                            line_height_relative=1.3,
                            padding=[0.0, 0.0, 0.0, 5.0],
                            on_input=self.input_changed,
                            user_data=name)
                    
            # need to keep the id's for later use in the modal
            self.modal_col_ids[name] = id
        
        # Once all the fields have been added, add the buttons a the bottom
        self.ipg.add_space(parent_id="modal_column", 
                           height=20.0)
        self.ipg.add_button(parent_id="modal_column", 
                            label="Exit & Discard Any Changes", 
                            on_press=self.exit_modal)
        self.ipg.add_button(parent_id="modal_column", 
                            label="Exit & Save Changes", 
                            on_press=self.exit_and_save)
        self.ipg.add_button(parent_id="modal_column", 
                            label="Insert as New", 
                            on_press=self.insert_new)
        self.ipg.add_button(parent_id="modal_column", 
                            label="Delete Book", 
                            on_press=self.delete_book,
                            )
        self.ipg.add_button(parent_id="modal_column", 
                            label="Clear Fields", 
                            on_press=self.clear_fields)

    # ******************************open modal************************************
    def open_modal(self, btn_id: int, id: int):
        if id == 999999: # new book indicator
            self.modal_row = self.fill_with_defaults()
        else:
            # get the row by hash
            self.modal_row = self.df.filter(pl.col('id') == id)
        
        self.update_modal_fields()
        
    # ******************************update modal fileds************************************
    def update_modal_fields(self):
        #  get each field and update the corresponding widget
        #  id,Title,Series,Num,Author,Status,Returned,Source,Url
        row = self.modal_row.to_dict()
        for name in self.column_names:
            item = row.get(name)[0]
            match name:
                case "id":
                    None
                case "Status":
                    self.ipg.update_item(self.modal_col_ids.get(name), IpgPickListParam.Selected, item)
                case "Source":
                    self.ipg.update_item(self.modal_col_ids.get(name), IpgPickListParam.Selected, item)
                case "Returned":
                    self.ipg.update_item(self.modal_col_ids.get(name), IpgTextParam.Content, item)  
                case _:
                    self.ipg.update_item(self.modal_col_ids.get(name), IpgTextInputParam.Value, item)

        self.ipg.update_item(self.modal_id, IpgContainerParam.Show, True)

    # ******************************Modal exit************************************
    def exit_modal(self, _btn_id: int):
        # update the tables modal_show value to hide the modal
        self.ipg.update_item(self.modal_id, IpgContainerParam.Show, False) 

    # ******************************Modal exit and save************************************
    def exit_and_save(self, btn_id: int):
        # update the table
        self.update_df_row()
        self.save_table()
        # exit
        self.exit_modal(0)

    # ******************************Modal insert new book************************************
    def insert_new(self, btn_id: int):
        # make the hash for the id
        combined = str(self.modal_row.row(0)[1] + self.modal_row.row(0)[4])
        combined.replace(" ", "")
        id = hash(combined)
        edit_id = self.make_edit_button(id)
        url_id = self.make_url_button(self.modal_row.row(0)[8], id)

        # fill in the df
        new_df = pl.DataFrame({
            "id": id, 
            "Title": self.modal_row.row(0)[1],
            "Series": self.modal_row.row(0)[2],
            "Num": self.modal_row.row(0)[3],
            "Author": self.modal_row.row(0)[4],
            "Status": self.modal_row.row(0)[5],
            "Returned": "",
            "Source": self.modal_row.row(0)[7],
            "Url": self.modal_row.row(0)[8],
            "edit_id": edit_id, 
            "url_id": url_id,
            })

        # Add the new book to the bottom of the table
        self.df = pl.concat([self.df, new_df])
        self.df = self.df.sort(["Author", "Series", "Num"])
        self.update_dataframe(self.df)

    # ******************************Modal delete book************************************
    def delete_book(self, btn_id: int):
        # a way of ensuring the user wants to delete is to give him 2 chances 
        # or alternately make another modal and show a delete message
        if self.delete_count == 0:
            self.delete_count += 1
            self.ipg.update_item(btn_id, IpgButtonParam.Label, "Press Again to Delete")
            return
        else:
            # resetting
            self.delete_count = 0
            self.ipg.update_item(btn_id, IpgButtonParam.Label, "Press to Delete")
            # save a backup copy
            self.save_table_backup()
            # get the hash_id for filtering
            hash_id = self.modal_row.row(0)[0]
            # get the row to be deleted
            delete_row = self.df.filter(pl.col("id") == hash_id)
            # filter out the row to be deleted
            self.df = self.df.filter(pl.col('id') != hash_id)
            # delete the edit button
            btn_id = delete_row.row(0)[9]
            self.ipg.delete_item(window_id="main", wid=btn_id)
            # delete the url button or the empty text widget
            url_id = delete_row.row(0)[10]
            self.ipg.delete_item(window_id="main", wid=url_id)
            # update and save the df
            self.update_dataframe(self.df)    
            self.save_table()
            self.exit_modal(btn_id)
    
    # ******************************Modal clear fields************************************
    def clear_fields(self, btn_id):
        self.modal_row = self.fill_with_defaults()
        self.update_modal_fields()
  
  # ******************************Modal save table backup************************************
    def save_table_backup(self):
        save_table = self.df.select(pl.exclude(["edit_id", "url_id"]))
        cwd = os.getcwd()
        save_table.write_csv(f"{cwd}/python_examples/resources/books_bkup.csv")

    # ******************************Modal save table************************************
    def save_table(self):
        save_table = self.df.select(pl.exclude(["edit_id", "url_id"]))
        cwd = os.getcwd()
        save_table.write_csv(f"{cwd}/python_examples/resources/books.csv")

    # ******************************Modal all the text inputs on submits************************************
    # The name parameter is the user_data
    def input_changed(self, ti_id: int, value: str, name: str):
        self.update_modal_row_item(name, value)

    # ******************************Modal picklists************************************
    def on_select(self, pl_id: int, value: str, name: str):
        self.update_modal_row_item(name, value)

    # ******************************Modal datepicker************************************
    def change_date(self, dp_id: int, return_date: str, name: str):
        # update the returned value
        text_id = self.modal_col_ids.get("Returned")
        self.ipg.update_item(text_id, IpgTextParam.Content, return_date)
        # update the modal row
        self.update_modal_row_item(name, return_date)

    # ******************************Table url button************************************
    def open_url(self, btn_id: int, row_id: int):
        df_filtered = self.df.filter(pl.col('id') == row_id)
        url = df_filtered.row(0)[8]
        webbrowser.open(url)

    # ******************************Table search title column************************************
    def search_titles(self, it_id: int, search=str):
        df_filtered = self.df.filter(pl.col('Title').str.to_lowercase().str.contains(search.lower()))
        self.reset_picklist_filters()
        self.update_dataframe(df_filtered)
        
    # ******************************Table search series column************************************
    def search_series(self, it_id: int, search=str):
        df_filtered = self.df.filter(pl.col('Series').str.to_lowercase().str.contains(search.lower()))
        self.reset_picklist_filters()
        self.update_dataframe(df_filtered)

    # ******************************Table filter by author picklist************************************
    # The filtering resets all filters except the one being used
    # This behavior could be changed by using a filtered df 
    # versus the self.df for displaying.
    def filter_books_author(self, pick_id: int, selected: str):
        if selected == "None":
            self.update_dataframe(self.df)
            self.reset_filters(pick_id)
            return
        
        # filter the df
        df_filtered = self.df.filter(pl.col('Author').str.to_lowercase().str.starts_with(selected.lower()))
        self.update_dataframe(df_filtered)
        self.reset_filters(pick_id)

    # ******************************Table filter by status************************************
    def filter_books_status(self, pick_id: int, selected: str):
        if selected == "None":
            self.update_dataframe(self.df)
            self.reset_filters(pick_id)
            return
        
        # filter the df
        df_filtered = self.df.filter(pl.col('Status').str.to_lowercase().str.starts_with(selected.lower()))
        self.update_dataframe(df_filtered)
        self.reset_filters(pick_id)
    
    # ******************************Table filter by return date************************************
    def filter_books_return_date(self, pick_id: int, selected: str):
        if selected == "None":
            self.update_dataframe(self.df)
            self.reset_filters(pick_id)
            return
        
        # filter the df
        df_filtered = self.df.filter(
            (pl.col('Returned').str.starts_with(selected)) &
            ((pl.col("Status") == "Read") | 
                (pl.col("Status") == "Final") | 
                (pl.col("Status").str.contains("Checked".lower()))))
        # update the total footer text
        self.ipg.update_item(
                    wid=self.footer_control_ids[6], 
                    param=IpgTextParam.Content, 
                    value=f"Total={df_filtered.height}")
        
        self.update_dataframe(df_filtered)
        
        self.reset_filters(pick_id)
    
    # ******************************Table filter by source************************************
    def filter_books_source(self, pick_id: int, selected: str):
        if selected == "None":
            self.update_dataframe(self.df)
            self.reset_filters(pick_id)
            return
        
        # filter the df
        df_filtered = self.df.filter(pl.col('Source') == selected)
        self.update_dataframe(df_filtered)
        self.reset_filters(pick_id)

    # ******************************helper functions************************************
    def reset_filters(self, selecting_id: int):
        for id in self.header_control_ids:
            if id != selecting_id:
                self.ipg.update_item(id, IpgPickListParam.Selected, "None")
                
    def reset_picklist_filters(self):
        for id in self.header_control_ids:
            self.ipg.update_item(id, IpgPickListParam.Selected, "None")
        
    def fill_with_defaults(self):
        return pl.DataFrame({"id":0, "Title":"", "Series":"", "Num":"", "Author":"", 
                             "Status":"None", "Returned":"", "Source":"None", "Url":""})
        
    # used to update a single row df for temp use
    def update_modal_row_item(self, column: str, value: str):
        row_id = self.modal_row.row(0)[0]
        self.modal_row = self.modal_row.lazy().with_columns(
            pl.when(pl.col("id") == row_id).then(pl.lit(value)).otherwise(pl.col(column)).alias(column)
            ).collect()
        
    def update_df_row(self):
        row_id = self.modal_row.row(0)[0]
        for i, name in enumerate(self.column_names):
            if name != "id":
                value = self.modal_row.row(0)[i]
                self.df = self.df.lazy().with_columns(
                    pl.when(pl.col("id") == row_id).then(pl.lit(value)).otherwise(pl.col(name)).alias(name)
                    ).collect()

        self.update_dataframe(self.df)
    
    def make_edit_button(self, hash_value: int) -> int:
        edit_id = self.ipg.add_button(
                        parent_id="table",
                        label=f"Edit",
                        width_fill=True,
                        style_id=self.btn_style_id,
                        on_press=self.open_modal,
                        user_data=hash_value,
                        )
        return edit_id
    
    def make_url_button(self, value: str, hash_value: int) -> int:
        # if the btn is not used, the id will be the text widget
        if value == "":
            id = self.ipg.add_text(
                        parent_id="table",
                        content="")
        else:
            id = self.ipg.add_button(
                        parent_id="table",
                        label=f"Url",
                        width_fill=True,
                        style_id=self.btn_style_id,
                        on_press=self.open_url,
                        user_data=hash_value
                        )
        return id
    
    def update_dataframe(self, df):
        update_df = df.select(pl.exclude(["edit_id", "url_id"]))
        self.ipg.update_dataframe(self.table_id, IpgTableParam.PolarsDf, update_df)
        
        
    
books = Books()
books.start()
