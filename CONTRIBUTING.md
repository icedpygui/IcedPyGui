# Contributing to IcedPyGui

Thanks for taking the time to contribute! All contributions are appreciated, from reporting bugs to implementing new features.

If you're unclear on how to proceed after reading this guide, please contact us on [Discord](https://discord.com/channels/1233081452447666270/1233085181448032458).

## Table of contents

- [Reporting bugs](#reporting-bugs)
- [Suggesting enhancements](#suggesting-enhancements)
- [Contributing to the codebase](#contributing-to-the-codebase)

## Reporting bugs

Before creating a bug report, please check that your bug has not already been reported, and that your bug exists on the latest version of IcedPyGui.
Also, go to the [Discord](https://discord.com/channels/1233081452447666270/1233085181448032458) and ask a question first.
[GitHub issues](https://github.com/icedpygui/IcedPyGui/issues) are used track bugs and suggested enhancements after the Disord forum is used.
You can report a bug by opening a [new issue](https://github.com/icedpygui/IcedPyGui/issues/new).

If you find a closed issue that seems to report the same bug you're experiencing, open a new issue and include a link to the original issue in your issue description.

Please include as many details as possible in your bug report. The information helps the maintainers resolve the issue faster.

## Suggesting enhancements

Go to the [Discord-New-Features](https://discord.com/channels/1233081452447666270/1234893749830680646).
Please describe the behavior you want and why.

## Contributing to the codebase

### Picking an issue

Pick an issue by going through the [issue tracker](https://github.com/icedpygui/IcedPyGui/issues) and finding an issue you would like to work on.
Feel free to pick any issue with an [accepted](https://github.com/icedpygui/IcedPyGui/issues?q=is%3Aopen+is%3Aissue+label%3Aaccepted) label that is not already assigned.
We use the [help wanted](https://github.com/icedpygui/IcedPyGui/issues/issues?q=is%3Aopen+is%3Aissue+label%3A%22help+wanted%22) label to indicate issues that are high on our wishlist.

If you are a first time contributor, you might want to look for issues labeled [good first issue](https://github.com/icedpygui/IcedPyGui/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22).

If you would like to take on an issue, please comment on the issue to let others know.
You may use the issue to discuss possible solutions.

### Working on your issue

IcedPyGui development flow relies on both Rust and Python.  The most common issues or enhancements are going to be:
* adding widgets developed in rust and already working in iced.
* issues with adding more pyo3 enhancements to how rust/python interact.
* modifying widgets that cannot directly be added to icedpygui due to some issue.  These mostly seem to be the iced messaging system where an id is needed and having no lifetime restrictions due to the use of the Mutex storage system.

If you are adding a widget, it's best to first get it working in the pure Rust Iced environment using the currently supported release.  Once this is done, then add it to IcedPyGui.

Create a new git branch from the `main` branch in your local repository, and start coding!

The Rust code is located in the `src` directory, while the Python codebase is located in the `icedpygui` directory.  There is only one python file used for linting purposes and documentation.

#### Adding a widget
To add a widget, make a new folder under `ipg_widgets`, ipg_mywidget.rs.  Also, if this is a modified Iced or Iced_aw widget, create the modified Iced code file under the folder iced_widgets.  

Take a look at the ipg_button to see the ipg and pyo3 imports used.  These will be typically used by all widgets.  Also use the types specified for the id, show, and user_data.

All widgets and such start with Ipg so as not to get confused or cause a conflict with the corresponding iced and iced_aw widgets.

##### Create and implement IpgMyNewWidget

The structure members will always include the id and show.  Containers don't need the user_data but widgets will, if they have a callback.  The remaining members are determined by your widget.  All members need to be initialized, by the calling function or by your own defaults.  

If you are using an Iced or Iced_aw widget, look through the code/docs and determine the defaulting values.  However, sometimes those defaults might not be the behavior you want so change accordingly.  

Some parameters like width, height, and padding have a specific way they are handled.  In the cases of width and height, they come over as 2 parameters (i.e. width: <f32>, width_fill: bool), a f32 and a bool for each.  These are converted to Length with a helper file.  The padding comes over as a Vec<f32> and is converted to a Padding using a helper file.  See the button or another file for an example.

Parameters that require a python class/rust enum equivalent come over as an Option<PyObject>.  This type is converted to their Iced equivalent in your module.  The reason there seems to be duplication is that a python class that is used for linting needs to have a corresponding enum in rust that is defined by pyo3.  Therefore, a conversion routines was needed for all widgets that use these python classes.

##### Add any messaging
Messages for each widget are added in your widget's module.  A single entry will be added in the app module later.  This keeps the app module from growing out of hand and keeps the widget code mostly confined to one module.  The widget will be wrapped with an id so the message in the app module will be MyWidget(usize, any other parameter).  Therefore the typical naming will be MyWidgetMessage.  If the name seems long, abbreviate the MyWidget part.

##### Add Construct method
The widget is constructed using the construct method and should follow the below format.  The IpgMyWidget is the structure that was strored in the mutex which is retrieved after the start_session command is issued in python.
```rust
fn construct_mywidget(mywid: IpgMyWidget) -> Element<'static, app::Message> {
    // your stuff
    let mywidget = MyIpgWidget::new(...) 
    //wrap with id
    mywidget.map(move |message| app::Message::IcedWidget(mywid.id, message))
}
```

##### Adding the callback method
The callback method uses 2 structures containing some generic and specific members.  These are WidgetCallbackIn and WidgetCallbackOut.  The reason for this is that a single function serves to obtain/update all of the widget's data and so a single type was needed to be carried in and out of the callback method.  The callback methods are located in ipg_widgets/callbacks.rs.

In many cases, the generic or a suitable member can be used but if you need more clarity, feel free to add members as needed.

The general usage is to instantiate the structure, add any data to the wci and call the get_set method in the matched message and return any needed data for the callback.
```rust
pub fn mywidget_callback(id: usize, message: MyWidgetMessage) {
    // instantiate the structure
    let mut wci = WidgetCallbackIn::default();
    // id must be used
    wci.id = id;
    // match the message
    match message {
        MyWidgetMessage::OnSomething(something: bool) => {
            // add to wci
            wci.boolean = something
            let mut wco: WidgetCallbackOut = get_set_widget_callback_data(wci);
            wco.id = id;
            wco.event_name = "on_something".to_string();
            process_callback(wco);
        }
    }
}
```
##### Processing the callback

The ipg_button and ipg_checkbox are typical examples of a callback, so see that code.  The general flow is:
* Obtain the MutexGuard of the callbacks HashMap type
* Check to see if it exits using the id and event_name
* Get and match to the callback since it's an Option<PyObject>
* Using the Python gil and matching all the variations of possible data (1-3), call the python method.

##### Add any parameter enums
All widgets should be designed so that all parameters can be updated.  Some parameters or widgets might only rarely be updated and some may be updated routinely, like the text widget.  For completeness, it was decided to add the ability to update all parameters.  This updating is also a good way to show the users how the parameters work and typically, if the updating works, the widgets works properly and it's like a testing of the widget.

As you see in ipg_button, an enum was added including all of the ipg_button parameters.  If you look in /icedpygui/icedpygui.pyi, you'll see a corresponding python class for the IpgButtonParams.  

So add the pub enum IpgMyWidgetParams and the class IpgMyWidgetParams.  Also add reference to the class to the __init__.py file too.

Now add your update method and complete as done in the ipg_button method.  If your widget returns some data like, checkbox does, then look at the ipg_checkbox callback to see how all the possbile permutations are handled when 3 parameters are possible.  

You'll also need to determine the form of your data that is returned.  With a sinple type, you would just return it as a String, boolean, ...  However, if its multiple values thaen you might need a vector or a HashMap that is converted to a dictionary for python.  Checkout the existing widgets for how this is done.
```rust
pub fn mywidget_item_update(mywid: &mut IpgMyWidget,
                            item: PyObject,
                            value: PyObject,
                            )
{
    // your code
}
```
Note that there are a number of generic extraction methods in the helpers file but you may have to add any special ones in your module that pertains only to your widget.  They all follow the same format.

##### Adding to libs.rs
At this point, you need to add the add_mywidget() method to the IPG structure.  The containers and widgets are added alphabetically in their own section.  A widget that can also be a container is added in the container section.  The methods are very similar so use them as examples for you widget.  Note, when you are using the Mutex, try and make sure to drop it a soon as possible.  If you test your widget and find that the program freezes, you probably forgot to drop the Mutex someplace in your code.  Also, you can't have any parameters in a Mutex that have a lifetime.  So don't use &str but String values only.  

Add your widget or container to the match_widget or match_container methods in lib.rs.

Add you enums to the icedpugui pymodule in lib.rs.

##### Adding to the app.rs
In most cases, you only need to add to the message enum, the update method, get_container, and get_widget methods in the app.rs.
You may still have a couple of errors about incomplete matches because some widgets had to be handled differently.  Simply add your widget in and that should pretty much be it.

##### Make the example python file
All of the python examples are found at https://github.com/icedpygui/IcedPyGui-Python-Examples.  You probably have already made an example file so add it to the `python_demo` folder and it will be moved over to the examples repository once the your code is excepted.  Make sure and put enough comments in the file so that a new user understands how to use the widget.

### Pull requests

When you have resolved your issue or made a new feature, [open a pull request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request-from-a-fork) in the IcedPyGui repository.
Please adhere to the following guidelines:

- Start your pull request title with a issue number.
- Use a descriptive title.
- In the pull request description, [link](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue) to the issue you were working on.
- Add any relevant information to the description that you think may help the maintainers review your code.
- Make sure your branch is [rebased](https://docs.github.com/en/get-started/using-git/about-git-rebase) against the latest version of the `main` branch.

After you have opened your pull request, a maintainer will review it and possibly leave some comments.
Once all issues are resolved, the maintainer will merge your pull request, and your work will be part of the next IcedPyGui release!

Keep in mind that your work does not have to be perfect right away!
If you are stuck or unsure about your solution, feel free to open a draft pull request and ask for help.

If you are creating a new widget, a highly commented python example file must also be submitted.

## Contributing to documentation

To be determined.
