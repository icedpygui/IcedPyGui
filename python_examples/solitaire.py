import os
import random
from icedpygui import IPG, IpgColor, IpgStackParam, IpgMousePointer, IpgImageContentFit
from icedpygui import IpgTextParam


def dump(obj):
  for attr in dir(obj):
    print("obj.%s = %r" % (attr, getattr(obj, attr)))
    
    
class solitaire:
    def __init__(self) -> None:
        self.ipg = IPG()
        self.cwd = os.getcwd()
        self.path = self.cwd + "/python_examples/resources/cards/"
        self.card_width: float=100.0
        self.card_height: float=150.0
        self.cards: list=[]
        self.stock: list=[]
        self.waste: list=[]
        self.covers: list=[]
        self.tableau: list=[1, 2, 3, 4, 5, 6, 7]
        self.status_id: int=0
        self.deal_amount: int=3

        self.selected: int=None
        self.destination: int=None
        
        # set the foundation slots using index >=100 to indicate no card present
        self.foundation_top_card: list=[100, 101, 102, 103]

    def start_game(self):
        self.ipg.add_window(window_id="main", title="Solitaire",
                    width=800.0,
                    height=800.0,
                    pos_centered=True,
                    # debug=True
                    )
        self.ipg.add_column(window_id="main",
                            container_id="main_col",
                            width_fill=True,
                            height_fill=True)
        
        self.create_styles()
        self.create_slots()
        self.load_cards()
        self.deal_cards()

        self.ipg.start_session()

    def create_styles(self):
        self.ipg.add_container_style(style_id="stock_style", 
                                    border_color=IpgColor.WHITE,
                                    border_width=2.0)


    def create_slots(self):
        # add row for stock, waste, and foundation cards
        self.ipg.add_row(window_id="main", 
                         container_id="stock_row",
                         parent_id="main_col",
                         height=self.card_height,
                         spacing=10.0
                         )
        
        # add some beginning space
        self.ipg.add_space(parent_id="stock_row",
                           width=20.0)
        
        # add the stock container to the row
        self.ipg.add_container(window_id="main",
                        container_id="stock_pile",
                        parent_id="stock_row",
                        padding=[0.0],
                        style_id="stock_style")
        
        # add the stack in
        self.ipg.add_stack(window_id="main",
                           container_id="stack_stock_pile",
                           parent_id="stock_pile",
                           width=self.card_width,
                           height=self.card_height,)
        
        # add the waste container to the row
        self.ipg.add_container(window_id="main",
                                container_id="waste_pile",
                                parent_id="stock_row",
                                width=self.card_width,
                                height=self.card_height,
                                padding=[0.0],
                                style_id="stock_style")

        # add the stack in
        self.ipg.add_stack(window_id="main",
                           container_id="stack_waste_pile",
                           parent_id="waste_pile",
                           width=self.card_width,
                           height=self.card_height,
                           )

        # add a space between waste and foundation
        self.ipg.add_space(parent_id="stock_row",
                           width=self.card_width
                           )

        # Add the 4 foundation slots
        for i in range(0, 4):
            self.ipg.add_stack(window_id="main",
                           container_id=f"foundation_{100+i}",
                           parent_id="stock_row",
                           width=self.card_width,
                           height=self.card_height,
                           )
            
            self.ipg.add_mousearea(window_id="main",
                                    container_id=f"foundation_mouse_{i}",
                                    parent_id=f"foundation_{100+i}",
                                    mouse_pointer=IpgMousePointer.Grab,
                                    on_press=self.card_selected,
                                    user_data=100+i,
                                    )
            self.ipg.add_container(window_id="main",
                                    container_id=f"foundation_container_{i}",
                                    parent_id=f"foundation_{100+i}",
                                    width=self.card_width,
                                    height=self.card_height,
                                    padding=[0.0],
                                    style_id="stock_style")

        # Add a space between the rows
        self.ipg.add_space(parent_id="main_col", height=20.0)
        self.status_id = self.ipg.add_text(parent_id="main_col", content="Status: Selected None")

        # Add a row for the tableau cards
        self.ipg.add_row(window_id="main",
                         container_id="tableau_row",
                         parent_id="main_col",
                         height=600.0,
                         spacing=10.0
                         )
        
        # Add a space at the beginning of the row
        self.ipg.add_space(parent_id="tableau_row",
                           width=20.0)
        
        # Add the 7 card slots
        for i in range(0, 7):
            # Add in the stacks
            self.ipg.add_stack(window_id="main",
                                container_id=f"tab_stack_{i}",
                                parent_id="tableau_row",
                                width=self.card_width,
                                height=400.0,
                                )
            
    def load_cards(self):
        suites = [
            ("hearts", "RED"),
            ("diamonds", "RED"),
            ("clubs", "BLACK"),
            ("spades", "BLACK"),
        ]
        ranks = [
            ("Ace", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("10", 10),
            ("Jack", 11),
            ("Queen", 12),
            ("King", 13),
        ]

        self.cards = []

        for (suite, color) in suites:
            for (name, value) in ranks:
                d = {"wid": None,
                     "index": None,
                    "suite": suite,
                    "color": color,
                    "name": name,
                    "value": value,
                    "stock_pile": None,
                    "waste_pile": None,
                    "tab_column": None,
                    "tab_index": None,
                    "foundation": None,
                    }
                self.cards.append(d)
        
        random.shuffle(self.cards)

        for i in range(0, 52):
            self.covers.append({
                "wid": None,
                "index": i,
                "tab_column": None,
                "tab_index": None,
            })

    def deal_cards(self):
        card_index = len(self.cards)-1
        for i in range(0, 7):
            last = self.tableau[i]-1
            for j in range(0, self.tableau[i]):
                self.ipg.add_column(window_id="main",
                                    container_id=f"tab_col_1_{card_index}",
                                    parent_id=f"tab_stack_{i}",)
                
                # Add a blank at top to hide the card below
                self.ipg.add_space(parent_id=f"tab_col_1_{card_index}",
                                    height=20*j)
 
                self.cards[card_index]["index"] = card_index
                self.cards[card_index]["tab_column"] = i
                self.cards[card_index]["tab_index"] = j
                card = self.cards[card_index]
                file = f"{self.path}/{card.get('suite')}/{card.get('value')}.png"
                self.cards[card_index]["wid"] = self.ipg.add_image(parent_id=f"tab_col_1_{card_index}", 
                                    image_path=file,
                                    width=self.card_width, 
                                    height=self.card_height,
                                    content_fit=IpgImageContentFit.Fill,
                                    mouse_pointer=IpgMousePointer.Grab,
                                    on_press=self.card_selected,
                                    user_data=card_index,
                                    )
                if j != last:
                    # add the blank over the vard unless last one.
                    self.ipg.add_column(window_id="main",
                                    container_id=f"tab_col_2_{card_index}",
                                    parent_id=f"tab_stack_{i}",)
                
                    # Add a blank at top to hide the card below
                    self.ipg.add_space(parent_id=f"tab_col_2_{card_index}",
                                        height=20*j)

                    self.covers[card_index]["index"] = card_index
                    self.covers[card_index]["tab_column"] = i
                    self.covers[card_index]["tab_index"] = j
                    file = f"{self.path}/card_back.png"
                    self.covers[card_index]["wid"] = self.ipg.add_image(parent_id=f"tab_col_2_{card_index}", 
                            image_path=file,
                            width=self.card_width, 
                            height=self.card_height,
                            content_fit=IpgImageContentFit.Fill,
                            user_data=200+card_index,
                            on_press=self.card_selected,
                            )
    
                card_index -= 1
        
        # add cards left to stock
        for index, card in enumerate(self.cards[0:card_index+1]):
            file = f"{self.path}/{card.get('suite')}/{card.get('value')}.png"
            self.cards[index]["stock_pile"] = i
            self.cards[index]["index"] = index
            self.cards[index]["wid"] = self.stock.append(self.ipg.add_image(parent_id=f"stack_stock_pile", 
                                image_path=file,
                                width=self.card_width, 
                                height=self.card_height,
                                content_fit=IpgImageContentFit.Fill,
                                mouse_pointer=IpgMousePointer.Grabbing,
                                user_data=index,
                                on_press=self.card_selected,
                                ))
            
        # add a cover
        file = f"{self.path}/card_back.png"
        self.ipg.add_image(parent_id=f"stack_stock_pile", 
                            image_path=file,
                            width=self.card_width, 
                            height=self.card_height,
                            content_fit=IpgImageContentFit.Fill,
                            mouse_pointer=IpgMousePointer.Grabbing,
                            user_data=100,
                            on_press=self.new_cards,
                            )

    def card_selected(self, card_id, card_index: int):

        # index >= 200 is cover card
        if card_index >= 200:
            
            cover_index = card_index-200
            column = self.covers[cover_index].get("tab_column")
            if self.covers[cover_index].get("tab_index") == self.tableau[column]-1:
                wid = self.covers[cover_index].get("wid")
                self.ipg.delete_item("main", wid)
                return
            else:
                content = "You can only uncover the last card"
                self.ipg.update_item(self.status_id, IpgTextParam.Content, content)
                return

        # index >= 100 is foundation empty slot
        if card_index >= 100 and bool(self.selected):
            fd_slot = card_index-100
            content = self.move_to_foundation(fd_slot)
            self.ipg.update_item(self.status_id, IpgTextParam.Content, content)
            return
        
        if card_index >= 100 and not bool(self.selected):
            content = "You cannot select a foundation slot first"
            self.ipg.update_item(self.status_id, IpgTextParam.Content, content)
            return

        # if no card selected, set the self.selected
        if not bool(self.selected):
            card = self.cards[card_index]
            content = f"Status: Selected {card.get('name')} {card.get('color')}"
            self.ipg.update_item(self.status_id, IpgTextParam.Content, content)
            self.selected = card_index
            return
        
        # if a card is selected, then this selection is the destination
        if bool(self.selected):
            target_card = self.cards[card_index]
            origin_card = self.cards[self.selected]

            # Check the color
            if target_card.get("color") == origin_card.get("color"):
                content = "You cannot place same colored cards on each other"
                return
            
            # are the card values i part
            if origin_card.get("value") == target_card.get("value")-1:
                content = f"Status: Target card {target_card.get('name')} {target_card.get('suite')}"
                self.move_card(card_index)
            else:
                content = "The value of the selected card must be one less than the target card."

            self.ipg.update_item(self.status_id, IpgTextParam.Content, content)

    def new_cards(self, card_id, card_index):
        if len(self.stock) >= 3:
            ids_to_move = self.stock[-3:] 
            self.stock = self.stock[0:len(self.stock)-3]
        elif len(self.stock) >= 2:
            ids_to_move = self.stock[-2:]
            self.stock = self.stock[0:len(self.stock)-2]
        elif len(self.stock) >= 1:
            ids_to_move = self.stock[-1:]
            self.stock = self.stock[0:len(self.stock)-1]
        else:
            return
        for wid in ids_to_move:
            self.move_card(wid, "stack_waste_pile"), None
            
        self.waste.extend(ids_to_move)
        
    def move_card(self, wid, tar_id, tar_pos):
        self.ipg.move_widget(window_id="main",
                                 widget_id=wid,
                                 target_container_id=tar_id,
                                 target_position=tar_pos,
                                 )
        
    def move_to_foundation(self, fd_slot: int):
        origin_card = self.cards[self.selected]
        
        # get the foundation slot top card value
        fd_index = self.foundation_top_card[fd_slot]

        if fd_index >= 100 and origin_card.get("value") == 1:
            wid = origin_card.get("wid")
            target_id = f"foundation_{fd_index}" 
            self.move_card(wid, target_id, None)
            content = f"Status: Destination foundation_{fd_slot}"

        if fd_index >= 100 and origin_card.get("value") != 1:
            return f"You cannot move the card {origin_card.get('name')} of {origin_card.get('suite')} to the foundation slot {fd_slot}"
        
        # slot not empty
        if fd_index < 100:
            target_card = self.cards.get(fd_index)

            if target_card.get("value") + 1 != origin_card.get("value"):
                return f"You cannot move the card having a value of {origin_card.get('value')} to foundation slot {fd_slot}"
                
        col = self.cards[self.selected]["tab_column"]    
        self.tableau[col] -= 1
        self.cards[self.selected]["foundation"] = fd_index
        self.cards[self.selected]["tab_column"] = None
        self.cards[self.selected]["tab_index"] = None
        self.selected = None
        return content
            

        

game = solitaire()
game.start_game()

