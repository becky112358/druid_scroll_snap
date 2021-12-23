Attempts to apply a Snap feature to the Druid `Scroll` widget.

The problem that every one of these attempts runs into is that the `layout()` is computed after the `event()` and `update()` widget calls. Thus, I can use events and updates to instruct my `Scroll` widget to scroll to the bottom; but the bottom is not extended until after the scrolling. This results in all these scroll-snap feature attempts scrolling to the last-but-one item, and the last item remaining hidden.

Each attempt is available on a branch.

Stuff attempted:

### external wisdom

Watch the point from where new elements are added to the list. (In this case, a button click.) Once the element has been added, send a command instructing the scroll to snap.

This solution would be particularly nice - if it worked - because the external wisdom could probably recognise whether the new element is being added to the middle or the end of the list, and only instruct a snap in the situation where the elements are being added to the end of the list.

### gossip list

Create a `ListSnap` widget which wraps the `List` widget. If the size of the `List` increases, send a `Command` reporting this information.

A `Controller` on the `Scroll` handles the `Command` and snaps the `Scroll`.

### scroll container

Create a `ScrollSnap` widget which wraps the `Scroll` widget. If the size of the `Scroll` contents increases, a flag is set.

In `ScrollSnap` function `update()`, this flag is monitored and used to instruct a snap of the `Scroll`.

### two controllers

One controller is applied to the external `Scroll`, one controller is applied to the internal `List`. If the `List` size changes, it sends a `Command` reporting its new size. The controller applied to the `Scroll` handles this `Command`: if the size has increased and the user is requesting snapping, then the `Scroll` snaps.

### watch child size

The `Scroll` has a controller watching the child size. If it increases, a snap is instructed.
