import random
import os
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
        #          index,Title,Series,Num,Author,Status,Returned,Source,Url
        self.column_widths = [100, 200, 200, 40, 150, 100, 120, 150, 75]
        self.tbl_width = sum(self.column_widths)
        self.tbl_height = 600.0

        self.table_id=0
        self.column_id_names = [ str(i) for i in range(0, len(self.column_widths)) ]
        self.ids = pl.Series
        self.header_control_ids = []

        self.modal_id = 0
        self.modal_col_ids = []
        self.current_modal_row = -1
        self.modal_row = {}
        self.modal_btns = []
        self.delete_count = 0
        self.dp_id = 0

        self.author_sort_list = ["None", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
                          "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"]
                            # I wish I had put a ratings column in but I didn't originally.
                            # I usually can tell the ratings by "Won't finish" or "maybe continue"
                            # The publish date is for when a new book will come out, sometimes I use
                            # the checked date so that I occasionally look and see if the next book is out.
        self.status_list = ["None", "Read", "To Be Read", "Maybe Continue Series", "Final", "Publish Date",
                            "Won't Finish", "Checked"]
        self.source_list = ["None", "Amazon Unlimited", "Amazon", "Library"]
        
        self.author_selected = "None"
        self.status_selected = "None"
        self.source_selected = "None"
        
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
        
        self.df = self.df.sort(["Author", "Series", "Num"])
        self.column_names = self.df.columns
        
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
        # table MUST follow in order, header (if enabled) foolowed by the footer(if enabled)
        # The additions can occur at any time just as long as they are in order.
        # The reason is that during the processing, a vector containing all of the widgets
        # are sent to the table where the widgets are extracted of each group occurs.  Therefore, if the
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
                                    polars_df=self.df,
                                    header_custom_enabled=True,
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
            
            if name == "id": name = ""
            
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
                            # other approaches could be used by not calling ope_modal directly
                        
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
                
            if name == "Source":
                id = self.ipg.add_pick_list(
                                parent_id=f"header{i}",
                                options=self.source_list,
                                on_select=self.filter_books_source,
                                selected=self.source_selected,
                                width_fill=True)
                self.header_control_ids.append(id)
    
    # ******************************create control columns*************************
    def create_control_columns(self):
        for row in self.df.rows():
            # index column
            self.ipg.add_button(
                        parent_id="table",
                        label=f"Edit",
                        width_fill=True,
                        style_id=self.btn_style_id,
                        on_press=self.open_modal,
                        user_data=row[0],
                        )
            # url column
            if row[8] == "":
                self.ipg.add_text(parent_id="table",
                                  content="")
            else:
                self.ipg.add_button(
                            parent_id="table",
                            label=f"Url",
                            width_fill=True,
                            style_id=self.btn_style_id,
                            on_press=self.open_url,
                            user_data=row[0]
                            )
         
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
                    id = self.ipg.add_text(
                            parent_id=f"row_{i}",
                            content="")
                    
                    self.ipg.add_date_picker(
                            parent_id=f"row_{i}", 
                            label="Date", 
                            on_submit=self.change_date,
                            user_data=self.dp_id)
                    
                case "Status":
                    id = self.ipg.add_pick_list(
                            parent_id=f"row_{i}", 
                            options=self.status_list,
                            placeholder=name,
                            user_data=i,
                            on_select=self.on_select)
                    
                case "Source":
                    id = self.ipg.add_pick_list(
                            parent_id=f"row_{i}", 
                            options=self.source_list,
                            placeholder=name,
                            user_data=i,
                            on_select=self.on_select)
                case _:
                    id = self.ipg.add_text_input(parent_id=f"row_{i}", 
                            placeholder=name,
                            width=300.0, 
                            size=16.0,
                            line_height_relative=1.3,
                            padding=[0.0, 0.0, 0.0, 5.0],
                            on_input=self.input_changed,
                            on_submit=self.on_submit,
                            user_data_on_input=i)
                    
            # need to keep the id's for later use in the modal
            self.modal_col_ids.append(id)
        
        # Once all the fields have been added, add the buttons a the bottom
        self.ipg.add_space(parent_id="modal_column", 
                           height=20.0)
        self.ipg.add_button(parent_id="modal_column", 
                            label="Exit & Discard Any Changes", 
                            on_press=self.exit_modal,)
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
            # Get the row index for later use
            self.current_modal_row = id
        self.update_modal_fields()
        
    # ******************************update modal fileds************************************
    def update_modal_fields(self):
        #  get each field and update the corresponding widget
        #  id,Title,Series,Num,Author,Status,Returned,Source,Url
        for i, name in enumerate(self.column_names):
            item = self.modal_row[i]
 
            match name:
                case "id":
                    None
                case "Status":
                    self.ipg.update_item(self.modal_col_ids[i], IpgPickListParam.Selected, item)
                case "Source":
                    self.ipg.update_item(self.modal_col_ids[i], IpgPickListParam.Selected, item)
                case "Returned":
                    self.ipg.update_item(self.modal_col_ids[i], IpgTextParam.Content, item)  
                case _:
                    self.ipg.update_item(self.modal_col_ids[i], IpgTextInputParam.Value, item)

        self.ipg.update_item(self.modal_id, IpgContainerParam.Show, True)

    # ******************************Modal exit************************************
    def exit_modal(self, btn_id: int):
        # update the tables modal_show value to hide the modal
        self.ipg.update_item(self.modal_id, IpgContainerParam.Show, False) 

    # ******************************Modal exit and save************************************
    def exit_and_save(self, btn_id: int):
        self.save_table_backup()
        self.save_table()
        # update the table
        self.update_table()
        # exit
        self.exit_modal(0)

    # ******************************Modal insert new book************************************
    def insert_new(self, btn_id: int):
        # make the hash for the id
        combined = self.modal_row[1] + self.modal_row[4]
        combined.replace(" ", "")
        id = hash(combined)
        
        new_df = pl.DataFrame({
            "id": id, 
            "Title": self.modal_row[1],
            "Series": self.modal_row[2],
            "Num": self.modal_row[3],
            "Author": self.modal_row[4],
            "Status": self.modal_row[5],
            "Returned": "",
            "Source": self.modal_row[7],
            "Url": self.modal_row[8],
            })
        
        self.df = pl.concat([self.df, new_df])
        self.df = self.df.sort(["Author", "Series", "Num"])
        print(self.df.head())
        # self.ipg.add_button(
        #                 parent_id="table",
        #                 label=f"Edit {self.df.height}",
        #                 width_fill=True,
        #                 style_id=self.btn_style_id,
        #                 on_press=self.open_modal,
        #                 )
        self.ipg.update_dataframe(self.table_id, IpgTableParam.PolarsDf, self.df)

    # ******************************Modal delete book************************************
    def delete_book(self, btn_id: int):
        # a way of ensuring the user wants to delete is to give 2 chances 
        # or alternately make another modal
        if self.delete_count == 0:
            self.delete_count += 1
            self.ipg.update_item(btn_id, IpgButtonParam.Label, "Press Again to Delete")
            return
        else:
            self.delete_count = 0
            self.ipg.update_item(btn_id, IpgButtonParam.Label, "Press to Delete")
            self.save_table_backup()
            self.df = self.df.filter((pl.col('Title') != self.modal_row[1]) & (pl.col('Author') != self.modal_row[2]))
            self.ipg.update_dataframe(self.table_id, IpgTableParam.PolarsDf, self.df)
            self.save_table()
            self.exit_modal(btn_id)
    
    # ******************************Modal clear fields************************************
    def clear_fields(self, btn_id):
        self.modal_row = self.fill_with_defaults()
        self.update_modal_fields()
  
  # ******************************Modal save table backup************************************
    def save_table_backup(self):
        cwd = os.getcwd()
        self.df.write_csv(f"{cwd}/python_examples/resources/books_bkup.csv")

    # ******************************Modal save table************************************
    def save_table(self):
        cwd = os.getcwd()
        self.df.write_csv(f"{cwd}/python_examples/resources/books.csv")

    # ******************************Modal all the text inputs on submits************************************
    # The name parameter is the user_data
    def on_submit(self, ti_id: int, value: str, name: str):
        # update the text_input widget so the title shows otherwise the return clears the field
        self.ipg.update_item(ti_id, IpgTextInputParam.Value, value)
        # since the modal row was updated by the input changed, no need here
    def input_changed(self, ti_id: int, value: str, index: int):
        # Update the text_input value as it's typed
        self.ipg.update_item(ti_id, IpgTextInputParam.Value, value)
        # update the modal row
        #index,Title,Series,Num,Author,Status,Returned,Source,Url
        self.modal_row[index] = value

    # ******************************Modal picklists************************************
    def on_select(self, pl_id: int, value: str, index: int):
        # update the picklist widget
        self.ipg.update_item(pl_id, IpgPickListParam.Selected, value)
        # update the modal row
        self.modal_row[index] = value

    # ******************************Modal datepicker************************************
    def change_date(self, btn_id: int, return_date: str, dp_id: int):
        self.ipg.update_item(dp_id, IpgTextParam.Content, return_date)
        # update the modal row
        self.modal_row[6] = return_date

    # ******************************Table url button************************************
    def open_url(self, btn_id: int, url: str):
        webbrowser.open(url)

    # ******************************Table filter by author picklist************************************
    # The filtering resets all filters except the one being used
    # This behavior could be changed by using a filtered df 
    # versus the self.df for displaying.
    def filter_books_author(self, pick_id: int, selected: str):
        if selected == "None":
            self.ipg.update_dataframe(self.table_id, IpgTableParam.PolarsDf, self.df)
            self.reset_filters(pick_id)
            return
        
        # filter the df
        df_filtered = self.df.filter(pl.col('Author').str.to_lowercase().str.starts_with(selected.lower()))
        self.ipg.update_dataframe(self.table_id, IpgTableParam.PolarsDf, df_filtered)
        self.reset_filters(pick_id)

    # ******************************Tabl filter by status************************************
    def filter_books_status(self, pick_id: int, selected: str):
        if selected == "None":
            self.ipg.update_dataframe(self.table_id, IpgTableParam.PolarsDf, self.df)
            self.reset_filters(pick_id)
            return
        
        # filter the df
        df_filtered = self.df.filter(pl.col('Status').str.to_lowercase().str.starts_with(selected.lower()))
        self.ipg.update_dataframe(self.table_id, IpgTableParam.PolarsDf, df_filtered)
        self.reset_filters(pick_id)
    
    # ******************************Table filter by source************************************
    def filter_books_source(self, pick_id: int, selected: str):
        if selected == "None":
            self.ipg.update_dataframe(self.table_id, IpgTableParam.PolarsDf, self.df)
            self.reset_filters(pick_id)
            return
        
        # filter the df
        df_filtered = self.df.filter(pl.col('Source').str.to_lowercase().str.starts_with(selected.lower()))
        self.ipg.update_dataframe(self.table_id, IpgTableParam.PolarsDf, df_filtered)
        self.reset_filters(pick_id)

    # ******************************helper functions************************************
    def find_id_in_dataframe(self, df: pl.DataFrame, id: int) -> bool:
        for col in self.df.columns:
            if (self.df[col] == id).any():
                return True
        return False
    
    def reset_filters(self, selecting_id: int):
        for id in self.header_control_ids:
            if id != selecting_id:
                self.ipg.update_item(id, IpgPickListParam.Selected, "None")
        
    def fill_with_defaults(self):
        #id,Title,Series,Num,Author,Status,Returned,Source,Url
        id = 0
        title = ""
        series = ""
        num = ""
        author = ""
        status = "None"
        returned = ""
        source = "None"
        url = ""
        return [id, title, series, num, author, status, returned, source, url]
    
books = Books()
books.start()
