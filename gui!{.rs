gui!{

    tabs {
        "First Tab" {

        }
        "Second Tab" {

        }
        "Third Tab" {

        }
    }

    bb_text!["The quick [b]fox[/b]"]

    // Apply a title.
    [tabs] {
        [tab] first_tab "First" {

        }
    }

    [button] example_button("This is the text.", id = "You can optionally assign an id.") {

    }
}