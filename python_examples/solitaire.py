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
        self.tableau: list=[1, 2, 3, 4, 5, 6, 7]
        self.status_id: int=0
        self.deal_amount: int=3

        self.selected: dict=None

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
                           height=self.card_height,)

        # add a space between waste and foundation
        self.ipg.add_space(parent_id="stock_row",
                           width=self.card_width)

        # Add the 4 foundation slots
        for i in range(0, 4):
            self.ipg.add_mousearea(window_id="main",
                                    container_id=f"foundation_mouse_{i}",
                                    parent_id="stock_row",
                                    mouse_pointer=IpgMousePointer.Grab,
                                    on_press=self.card_selected,
                                    user_data=(f"foundation_{i}", None),
                                    )
            self.ipg.add_container(window_id="main",
                                    container_id=f"foundation_pile_{i}",
                                    parent_id=f"foundation_mouse_{i}",
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
                d = {"suite": suite,
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

    def deal_cards(self):
        count = len(self.cards)-1
        for i in range(0, 7):
            last = self.tableau[i]-1
            for j in range(0, self.tableau[i]):
                self.ipg.add_column(window_id="main",
                                    container_id=f"tab_col_1_{count}",
                                    parent_id=f"tab_stack_{i}",)
                
                # Add a blank at top to hide the card below
                self.ipg.add_space(parent_id=f"tab_col_1_{count}",
                                    height=20*j)
 
                card = self.cards[count]
                card["tab_column"] = i
                card["tab_index"] = j 
                file = f"{self.path}/{card.get('suite')}/{card.get('value')}.png"
                self.ipg.add_image(parent_id=f"tab_col_1_{count}", 
                                    image_path=file,
                                    width=self.card_width, 
                                    height=self.card_height,
                                    content_fit=IpgImageContentFit.Fill,
                                    mouse_pointer=IpgMousePointer.Grab,
                                    on_press=self.card_selected,
                                    user_data=("card", card),
                                    )
                if j != last:
                    # add the blank over the vard unless last one.
                    self.ipg.add_column(window_id="main",
                                    container_id=f"tab_col_2_{count}",
                                    parent_id=f"tab_stack_{i}",)
                
                    # Add a blank at top to hide the card below
                    self.ipg.add_space(parent_id=f"tab_col_2_{count}",
                                        height=20*j)
                    
                    file = f"{self.path}/card_back.png"
                    self.ipg.add_image(parent_id=f"tab_col_2_{count}", 
                            image_path=file,
                            width=self.card_width, 
                            height=self.card_height,
                            content_fit=IpgImageContentFit.Fill,
                            user_data=(f"cover_{count}", None),
                            on_press=self.card_selected,
                            )
    
                count -= 1
        
        # add cards left to stock
        for card in self.cards[0:count]:
            file = f"{self.path}/{card.get('suite')}/{card.get('value')}.png"
            self.ipg.add_image(parent_id=f"stack_stock_pile", 
                                image_path=file,
                                width=self.card_width, 
                                height=self.card_height,
                                content_fit=IpgImageContentFit.Fill,
                                mouse_pointer=IpgMousePointer.Grabbing,
                                user_data=("card", card),
                                on_press=self.card_selected,
                                )
            
        # add a cover
        file = f"{self.path}/card_back.png"
        self.ipg.add_image(parent_id=f"stack_stock_pile", 
                            image_path=file,
                            width=self.card_width, 
                            height=self.card_height,
                            content_fit=IpgImageContentFit.Fill,
                            mouse_pointer=IpgMousePointer.Grabbing,
                            user_data=("cover", None),
                            on_press=self.card_selected,
                            )

    def card_selected(self, card_id, info):
        content = ""
        if info[0] == "card" and not bool(self.selected):
            content = f"Status: Selected {info[1].get('name')} {info[1].get('color')}"
            self.selected = info[1]
        elif info[0] == "card" and bool(self.selected):
            # Check the color
            if self.selected.get("color") == info[1].get("color"):
                content = "You cannot place same colored cards on each other"
            else:
                if self.selected.get("value") == info[1].get("value") -1:
                    content = f"Status: Selected Target {info[0]} {info[1].get('color')}"
                else:
                    content = "The value of the selected card must be one less than the target card."

            self.selected = None
        self.ipg.update_item(self.status_id, IpgTextParam.Content, content)

    def new_cards(self):
        print()
        
        
        
        

game = solitaire()
game.start_game()

