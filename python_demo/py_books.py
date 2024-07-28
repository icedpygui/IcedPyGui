import random

from icedpygui import IPG, IpgTableRowHighLight, IpgTableWidget
import polars as pl
from datetime import datetime, date



class Books:
    def __init__(self):
        self.ipg = IPG()
        self.df = pl.DataFrame
        self.book_list = []
        self.table_list = pl.DataFrame()
        #          Edit,Title,Series,Num,Author,Status,Returned,Source,Url
        self.widths = [100, 200, 200, 40, 150, 100, 100, 150, 100]
        self.popup_tag = None
        self.book = []
        self.changed_book = []
        self.item_component = [0] * 8
        self.item_theme = [0] * 8
        self.item_tags = []

        self.filtered_authors = "None"
        self.filtered_status = "None"
        self.filtered_source = "None"

        self.book_list_types = [pl.UInt32, pl.Utf8, pl.Utf8,
                                pl.Utf8, pl.Utf8, pl.Utf8,
                                pl.Date, pl.Utf8, pl.Utf8]

        self.sort_list = ["None", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
                          "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"]
        self.status_list = ["None", "Read", "To Be Read", "Maybe Continue Series", "Final", "Publish Date",
                            "Won't Finish", "Checked"]
        self.source_list = ["None", "Amazon Unlimited", "Amazon", "Amazon Elizabeth", "Library"]

    def start(self):
        self.load()
        self.create_table()
        self.ipg.start_session()

    def load(self):
        self.df = pl.read_csv("python_demo/resources/books.csv")
        self.book_list = []
        columns = self.df.columns

        for name in columns:
            self.book_list.append({name: self.df.get_column(name).to_list()})

    def create_table(self):
        self.ipg.add_window(window_id="main", title="Books",
                            width=1200, height=600,
                            pos_x=100, pos_y=50,
                            )

        self.ipg.add_container(window_id="main", container_id="table")

        buttons = []
        for _ in range(0, len(self.df)):
            buttons.append(IpgTableWidget.Button)

        self.ipg.add_table(parent_id="table",
                           title="Books",
                           data=self.book_list,
                           data_length=len(self.df),
                           width=1100.0, height=600.0,
                           column_widths=self.widths,
                           row_highlight=IpgTableRowHighLight.Lighter,
                           highlight_amount=0.1,
                           widgets_using_columns={0: buttons},
                           on_button=self.edit_button,
                           )
    
    def edit_button(self, tbl_id: int, wid_index: tuple[int, int]):
        print(tbl_id, wid_index)

    # def edit(self, _sender, _data, user_data):
    #     dpg.show_item(self.popup_tag)
    #     dpg.delete_item(self.popup_tag, children_only=True)
    #
    #     self.book = list(self.book_list.row(user_data))
    #     self.changed_book = list(self.book_list.row(user_data))
    #     # ['row_nr', 'Title', 'Series', 'Num', 'Author', 'Status', 'Returned', 'Source', 'Url']
    #
    #     if dpg.does_item_exist("edit_table"):
    #         dpg.delete_item("edit_table")
    #
    #     with dpg.table(width=-1, tag="edit_table", header_row=False, policy=dpg.mvTable_SizingFixedFit,
    #                    parent=self.popup_tag, borders_outerH=True, borders_outerV=True, borders_innerH=True,
    #                    borders_innerV=True):
    #         dpg.add_table_column(width_stretch=True, init_width_or_weight=.15)
    #         dpg.add_table_column(width_stretch=True, init_width_or_weight=.15)
    #         dpg.add_table_column(width_stretch=True, init_width_or_weight=.70)
    #
    #         columns = self.book_list.columns
    #         self.item_tags = []
    #         for i, item in enumerate(self.book):
    #             with dpg.table_row():
    #                 if columns[i] == "row_nr":
    #                     dpg.add_text(default_value="")
    #                     dpg.add_text(default_value="Row")
    #                     dpg.add_text(default_value=f"{item}")
    #                 else:
    #                     dpg.add_button(label="Reset", callback=self.reset_item, user_data=i - 1)
    #                     dpg.add_text(default_value=columns[i])
    #                     if columns[i] == "Source":
    #                         tag = dpg.add_combo(self.source_list, default_value=item,
    #                                             callback=self.item_changed, width=-1, user_data=i - 1)
    #                     elif columns[i] == "Status":
    #                         tag = dpg.add_combo(self.status_list, default_value=item,
    #                                             callback=self.item_changed, width=-1, user_data=i - 1)
    #                     elif columns[i] == "Returned":
    #                         if str(item).split("-") == 3:
    #                             split = str(item).split("-")
    #                             day = int(split[2])
    #                             month = int(split[1])
    #                             year = int(split[0]) - 1900
    #                         else:
    #                             now = date.today()
    #                             day = now.day
    #                             month = now.month
    #                             year = now.year - 1900
    #                         with dpg.group(horizontal=True):
    #                             tag = dpg.generate_uuid()
    #                             with dpg.group():
    #                                 dpg.add_button(label=item, callback=self.show_date_picker, user_data=tag,
    #                                             tag="date_button")
    #                                 dpg.add_button(label="Accept", show=False, tag="accept_date",
    #                                             callback=self.accept_date, user_data=tag)
    #                                 dpg.add_button(label="Cancel", show=False, tag="cancel_date",
    #                                             callback=self.cancel_date, user_data=tag)
    #                             dpg.add_date_picker(default_value={'month_day': day, 'year': year,
    #                                                             'month': month - 1}, callback=self.set_date,
    #                                                 show=False, tag=tag)
    #
    #                     else:
    #                         tag = dpg.add_input_text(default_value=item, user_data=i - 1, callback=self.item_changed,
    #                                                  width=-1)
    #                     self.item_tags.append(tag)
    #                     dpg.bind_item_theme(tag, self.item_theme[i - 1])
    #
    #     dpg.add_spacer(height=20, parent=self.popup_tag)
    #     with dpg.group(horizontal=True, parent=self.popup_tag):
    #         dpg.add_button(label="Accept Edits", callback=self.accept_edits)
    #         dpg.add_button(label="Add New Book", callback=self.add_new)
    #         # dpg.add_button(label="Cancel", callback=self.cancel)
    #
    #     dpg.add_spacer(height=20, parent=self.popup_tag)
    #     with dpg.group(horizontal=True, parent=self.popup_tag):
    #         dpg.add_button(label="Delete Book", callback=self.delete_book)
    #         dpg.add_button(label="Clear All Fields", callback=self.clear_fields)
    #
    # def cancel(self):
    #     dpg.hide_item(self.popup_tag)
    #     self.reset_theme()
    #
    # def accept_edits(self):
    #     self.delete_book("accepts")
    #     self.add_new("accepts")
    #     self.reset_theme()
    #
    # def add_new(self, sender):
    #     data = list(self.changed_book)
    #     types = list(self.book_list_types)
    #     columns = list(self.book_list.columns)
    #
    #     data_dict = dict(zip(columns, data))
    #     schema = tuple(zip(columns, types))
    #
    #     df = pl.DataFrame(data_dict, schema=schema)
    #     self.book_list = pl.concat([self.book_list, df])
    #     self.book_list = self.book_list.sort(["Author", "Series", "Num"])
    #     self.book_list = drop_and_add_row_nr(self.book_list)
    #     self.book_list.write_csv("./data/books.csv")
    #     self.create_table()
    #     self.reset_theme()
    #     if sender != "accepts":
    #         dpg.hide_item(self.popup_tag)
    #
    # def delete_book(self, sender):
    #     idx = self.book[0]
    #     self.book_list = self.book_list.filter(pl.col("row_nr") != idx)
    #     self.book_list = drop_and_add_row_nr(self.book_list)
    #     self.book_list.write_csv("./data/books.csv")
    #     self.create_table()
    #     if sender != "accepts":
    #         dpg.hide_item(self.popup_tag)
    #
    # def item_changed(self, sender, data, user_data):
    #     dpg.add_theme_color(dpg.mvThemeCol_FrameBg, (135, 0, 0), category=dpg.mvThemeCat_Core,
    #                         parent=self.item_component[user_data])
    #
    #     self.changed_book[user_data + 1] = data
    #
    # def reset_item(self, sender, data, idx):
    #     old = self.book[idx + 1]
    #     self.changed_book[idx + 1] = old
    #     dpg.set_value(self.item_tags[idx], old)
    #
    # def clear_fields(self):
    #     for tag in self.item_tags:
    #         dpg.set_value(tag, "")
    #     self.book = []
    #     self.changed_book = []
    #
    # def show_date_picker(self, sender, data, tag):
    #     dpg.show_item(tag)
    #     dpg.show_item("accept_date")
    #     dpg.show_item("cancel_date")
    #
    # def set_date(self, sender, data):
    #     dt = f"{data['year'] + 1900}-{data['month'] + 1}-{data['month_day']}"
    #     kwargs = {"label": dt}
    #     dpg.configure_item("date_button", **kwargs)
    #     self.changed_book[6] = date(year=data['year'] + 1900, month=data['month'] + 1, day=data['month_day'])
    #
    # def accept_date(self, sender, data, tag):
    #     dpg.hide_item("accept_date")
    #     dpg.hide_item("cancel_date")
    #     dpg.hide_item(tag)
    #
    # def cancel_date(self, sender, data, tag):
    #     dpg.hide_item("accept_date")
    #     dpg.hide_item("cancel_date")
    #     dpg.hide_item(tag)
    #     self.changed_book[6] = self.book[6]
    #     kwargs = {"label": str(self.changed_book[6])}
    #     dpg.configure_item("date_button", **kwargs)


#     def check_what_to_display(self):
#         df = self.book_list.clone()
#         if self.filtered_authors != "None":
#             df = df.filter(pl.col('Author').str.starts_with(self.filtered_authors))
#         if self.filtered_status != "None":
#             df = df.filter(pl.col('Status') == self.filtered_status)
#         if self.filtered_source != "None":
#             df = df.filter(pl.col('Source') == self.filtered_source)
#         return df
#
#     def reset_theme(self):
#         for i in range(0, len(self.book) - 1):
#             dpg.add_theme_color(dpg.mvThemeCol_FrameBg, (190, 50, 50), category=dpg.mvThemeCat_Core,
#                                 parent=self.item_component[i])
#
#     def filter_authors(self, sender, data):
#         self.filtered_authors = data
#         self.create_table()
#
#     def filter_status(self, sender, data):
#         self.filtered_status = data
#         self.create_table()
#
#     def filter_source(self, sender, data):
#         self.filtered_source = data
#         self.create_table()
#
#
# def drop_and_add_row_nr(df):
#     df.drop_in_place("row_nr")
#     df = df.with_row_count()
#     return df


books = Books()
books.start()
