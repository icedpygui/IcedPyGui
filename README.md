# IcedPyGui (IPG)
Python wrapper for Rust Iced Gui

<div align="center">

<img src="docs/python_iced.png" width="200px" />

# Iced + Python == IcedPyGui (IPG)

https://github.com/icedpygui/IcedPyGui/assets/163431522/7b485d80-15cc-4f3b-a29d-e264d6b1010d

</div>

## Features

* Examples for all of the currently supported widgets
* Supported Iced widgets
    * Button 
    * Checkbox 
    * Column - container 
    * Container - container
    * Events - keyboard, mouse, timer, and window
    * Modal
    * Pick_list 
    * Progress_bar
    * Radio buttons - mutiple, grouped in one line of code
    * Row - container
    * Scrollable - container (modified in IPG) 
    * Slider
    * Space
    * Text editor - coming soon
    * Text_input
    * Text
    * Tooltip
    * Windows multiple with debug option
* Iced_aw widgets
    * color_picker 

* IPG widgets
    * Date_picker - compact and resizable
    * Selectable_text - all mouse buttons supported
    * Table - Easily loaded with a list of dictionaries

* More to come in the near future


## Installation (PiPy)

* Open one of the example using your favorite IDE.
* Create and activate a virtual environment
 ```python
pip install icedpygui
 ```

## Installation (Rust)
* Install Rust
* Clone the repository and open in your favorite IDE
* Create and activate a virtual environment
* Install maturin and compile the rust code
```python
pip install maturin
maturin develop
```
* Open and run one of the python examples

## Overview

* IcedPyGui is based on Rust Iced https://github.com/iced-rs/iced.
* Some code is used from Iced_aw https://github.com/iced-rs/iced_aw.
* Pyo3 is used as the python wrapper https://github.com/pyo3/pyo3.
* Maturin is used to build and publish the module https://github.com/PyO3/maturin.
* The syntax and the design of the callbacks were inspired by the python wrapper of Dear ImGui, DearPyGui(DPG) https://github.com/hoffstadt/DearPyGui.
* The icon above was a merge of Python and Iced icons by Deep Dream Generator https://deepdreamgenerator.com/

## Intro 
Iced is a really good GUI for Rust but it's still early in the development cycle, more good things will follow.  

Some would probably say it's too early for a python wrap but I though I would give it a try since I wanted a project that would help me improve my Rust skills, which I've only been using a for a short time.

This project is the first I have published and so I expect I'll learn a lot and hopefully you can bare with me.

Rust's strict typing is mostly shielded from the python user but if you venture too far, you'll get burned with an error, so behave yourselves :)

Iced uses a messaging system which acts like a callback and no widget ids are used except for the containers and windows.  This posed a bit of a problem in IPG but in most cases it was solved by mapping an id to the widget.  In cases where this was not possible, the code was pull into IPG and an id was added in with fairly simple changes so it shouldn't be hard to keep those few items updated.

Iced doesn't use the concept of user data being passed around but when using DPG, sometimes the ability to send user data was very helpful.  Therefore, this concept was added to IPG.

The user data is special because it is only passed through to rust and  back out as a PyObject or PyAny.  Therefore any python data can be used since it is never extracted into a rust type.    

# A few important rules or points and then you program with IPG!
   
* Import IPG as indicated below in the demo code.

* Instantiate the Rust structure then add your containers and widgets.

* The last line of code to execute must be ipg.start_seesion().  Any code after that will not be executed because rIced is now running.  You can place it anywhere, just make sure its last executed.  If you start your program and nothing happens, it might mean that you aren't executing start_session() or you forgot to add it in, been there, done that.    

* Every widget needs to have a parent container previously defined and every container needs to have a window and optionally a parent container defined.  If the container is placed into a window then no parent_id is required.

* Therefore at least one window needs to be added first and at least one container needs to be added to the window before any widgets are added.  As long as you have defined a parent, you can add to it.

#### Let's get started with out first prrogram:

Further below you'll find the full code to be tested in your IDE.  Sometimes the these code snippets don't paste properly into the IDE if the parameter names are in them.  But since these snippets are for learning, they need to be there for better understaning.  So look furhter down for the full code to copy and paste.

```python
from icedpygui.icedpygui import IPG
```

Let's instantiate the Rust structure and add containers.  Since this is a short program, I'll dispense with using a class.  See the examples for cases where I used a class but nothing special pertains to IPG when they are used.

```python
ipg = IPG()

ipg.add_window(window_id="main", title="Demo Window", 
                                width=600, height=500, 
                                pos_x=150, pos_y=100)

ipg.add_container("main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)
```

So, the window was added using an id, title, size and position.  Easy so far.  The container which holds only one widget helps center things in the window by using align_x and align_y.  The width and height have 3 options, a default shrink (the container shrinks down to the overall size of the widgets), a width=float and this width_fill=True.  The fill part means that the widget or container will fill the available space of the container it's in, so the above container will be the same 600 by 500 as the window is.  The advantage of using fill is that if you resize the window, it resizes also without you having to recalculate the size based on the window size, which you could do with a resize callback.

To make things a little bit more exciting, let's add 2 widgets in a Column container.

### Adding widgets

```rust
ipg.add_column(window_id="main", container_id="col", parent_id="cont")

ipg.add_button(parent_id="col", label="Press Me!", on_press=button_pressed)

ipg.add_checkbox(parent_id="col", label="Check Me!!!", 
                                    on_checked=checked,
                                    user_data="Some string data")

checked_text_id = ipg.add_text(parent_id="col", 
                                content="This will change when I'm checked")
user_data_text_id = ipg.add_text(parent_id="col", 
                                content="User data here")

```
Ok, the Column was added and a window_id had to be supplied along with a container_id, and a parent_id.  The window_id is needed because if you're using multiple windows, it was decided that the string ids need only to be unique within a window.  This may change in the near future to reducing the number of parameters needed since each widget and container have unique integer ids anyway.  

The button and checkbox widgets were next added and given a parent_id, which is the container_id that you want to put the widget in.  Along with ids, some labels, callbacks, and user_data where supplied.

Hey! What happen to the widget_id?  Well, it's not needed because widgets are not a parent for anything.  The widget gets a integer id once Iced has started, so if you need one, you do the following.

```rust
btn_id = add_button(...)

```
Now when Iced starts, you'll have your id in a variable to be used.  The container and window ids are needed because there needs to be a link between the windows, containers and widgets during the construction phase.  This phase happens before Iced is started, so a known link is needed to be established and strings are easier to remember than integers.

Looking again at the button and checkbox code above, you'll see an on_press and an on_checked parameter.  These are callbacks to functions.  The callback function for these widgets are button_pressed and checked.  We'll do those next.

#### Defining callbacks:

Below we defined two callbacks, button_pressed and checked,  All callbacks have from 2 to 4 parameters.  The button has only two ( 3 if user_data used) and the checkbox can have up to 4 if the user_data is used.

The button is pretty simple, you have the numerical id of the button and the name of the callback.  The button doesn't have any user_data, in this case, and it doesn't have any other data like the checkbox does.  To keep things simple, we are just going to print the id and name.

```rust
def button_pressed(id, name):
    print(id, name)

def checked(id, name, checked, user_data):
    if checked:
        ipg.update_item(id=checked_text_id, 
                        param="content", 
                        value="I'm checked")
    else: 
        ipg.update_item(id=checked_text_id, 
                        param="content", 
                        value="I'm not checked")

    ipg.item_update(id=user_data_text_id,
                        param="content",
                        value=user_data)

```

The checkbox however is a busy widget.  It has to let you know if it's checked or not and it has a pachage it's been carrying around called user_data.  These callback parameters can be called anything you like, order is the key.

The 3rd and 4th parameters for the checkbox are checked, a boolean type, and user_data, a python type.  

We could just print the data but that really never happens in a real programs so let's display the info in a text widget by using the command update_item().

The update_item is going to be your goto function to change the way the gui looks while it's running.  The update_item function takes 3 arguments, an integer id of the widget you want to update, the string name of the parameter of the item, and the value you want to set it too.  Unless you are updating the calling widget, you'll need the id of the widget you want to update.

In this case our variables are global so not an issue.  Normally you would store these in a class or a @dataclass.  The @dataclass is not yet supported.

So in the checked function, we do an if else statement for the boolean and change the text accordingly.  We then put the user data in another text widget.

Finally, we're ready to start the engine on the gui with one last line of code, the magic.

```rust

ipg.start_session()

```
Wow! You'd think I could come up with something more excited than start_session.

Let's put it all together to cut down on the cutting a pasting.

```rust
from icedpygui.icedpygui import IPG


def button_pressed(id, name):
    print(id, name)


def checked(id, name, checked, user_data):
    if checked:
        ipg.update_item(checked_text_id, 
                        "content", 
                        "I'm checked")

    else:
        ipg.update_item(checked_text_id, 
                        "content", 
                        "I'm not checked")

    ipg.update_item(user_data_text_id, "content", user_data)


ipg = IPG()

ipg.add_window(window_id="main", title="Demo Window", 
                                width=600, height=500, 
                                pos_x=150, pos_y=100)

ipg.add_container("main", container_id="cont", align_x="center", 
                  align_y="center", width_fill=True, height_fill=True)

ipg.add_column(window_id="main", container_id="col", parent_id="cont")

ipg.add_button(parent_id="col", label="Press Me!", on_press=button_pressed)

ipg.add_checkbox(parent_id="col", label="Check Me!!!", 
                                    on_checked=checked,
                                    user_data="Some string data")

checked_text_id = ipg.add_text(parent_id="col", 
                                content="This will change when I'm checked")

user_data_text_id = ipg.add_text(parent_id="col", 
                                content="User data here")

ipg.start_session()

```

Hopefully you were able to run the program successfully.  If not, try one of the examples and see if it will work.  All of the examples are run before publishing to make sure the code works.  Unlike standard functions, a GUI is a bit difficult to test since you are visually making sure it all goes as expected. 

The examples are found both in this repositiory but also kept in a separate one to make things easier to access at 

## Contributing / Feedback

Contributions are greatly appreciated! If you want to contribute, please
read our [contributing guidelines] for more details.

Feedback is also welcome! You can create a new topic in [our Discourse forum] or
come chat to [our Discord server].
