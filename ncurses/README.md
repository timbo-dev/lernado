# ncurses

## Description
`ncurses` is a programming library that provides an API for creating text-based user interfaces in terminal-based applications. The name "ncurses" stands for "new curses," as it is an improved version of the original curses library.

With `ncurses`, developers can build interactive and visually appealing interfaces for command-line applications. It offers a wide range of functions and capabilities to control the terminal screen, handle user input, display text in different colors and styles, and create windows, menus, and dialog boxes.

## Key Features

1. **Screen Control**: `ncurses` enables developers to control the appearance and behavior of the terminal screen. It allows for efficient screen updates, cursor positioning, scrolling, and clearing the screen.

2. **User Input Handling**: The library provides functions to handle user input, including keyboard events, mouse events, and function keys. It supports input handling in both raw and cooked mode, allowing for customizable input processing.

3. **Text Formatting**: `ncurses` allows developers to format and display text in various styles, colors, and attributes. It supports features like bold, underline, reverse video, and custom color combinations, enhancing the visual presentation of the text-based interface.

4. **Window Management**: The library offers window management capabilities, allowing the creation of multiple windows within the terminal screen. Developers can control window positioning, resizing, overlapping, and scrolling, enabling the creation of complex layouts.

5. **Menu and Dialog Creation**: `ncurses` facilitates the creation of menus and dialog boxes to interact with users. It provides functions to define menus, handle user selections, and display modal or non-modal dialog boxes.

6. **Multi-platform Support**: `ncurses` is available on various Unix-like systems, including Linux, macOS, and BSD derivatives, making it a portable choice for developing terminal-based applications.

## Why Use ncurses?

There are several reasons to use `ncurses` in your software projects:

1. **Terminal-based Applications**: If you are developing a command-line or terminal-based application that requires an interactive and visually appealing interface, `ncurses` provides the necessary tools and functionality.

2. **Portability**: With `ncurses`, you can write terminal-based applications that can run on different Unix-like systems without significant modifications. This portability allows your software to be deployed across various platforms with minimal effort.

3. **Text-based User Interfaces**: If you prefer working with text-based user interfaces instead of graphical user interfaces (GUIs), `ncurses` allows you to create rich and interactive text-based interfaces that can rival GUIs in terms of functionality and usability.

4. **Retro Computing**: `ncurses` is often used in retro computing projects or for software that runs on legacy systems or low-resource environments where a text-based interface is preferred or necessary.

5. **Integration with Terminal Tools**: `ncurses` integrates well with other terminal tools and utilities. It can be used alongside command-line interfaces, system monitoring tools, text editors, or any application that benefits from an interactive text-based interface.

In conclusion, `ncurses` is a powerful library for building text-based user interfaces in terminal-based applications. It provides extensive control over screen output, user input, formatting, and window management. By utilizing `ncurses`, developers can create visually appealing and interactive applications that run in a terminal environment.

## Ncurses library

<details>
    <summary>
        <h2>
            initscr();
        </h2>
    </summary>

The `initscr()` function is a fundamental command in the `ncurses` library. When developing text-based user interfaces with `ncurses`, this function is used to initialize the library and set up the terminal screen for interface creation.

By calling `initscr()` at the beginning of an `ncurses` program, the necessary data structures and configurations are initialized, allowing subsequent `ncurses` functions to interact with the terminal screen.

## Example

```c
#include <ncurses.h>

int main() {
    // Initialize ncurses
    initscr();

    // Further code and interface creation goes here...

    // End ncurses mode
    endwin();

    return 0;
}
```

In the example above, we have a basic `ncurses` program structure. The `initscr()` function is called to initialize the library and prepare the terminal for interface creation. Following the `initscr()` call, additional code can be written to create the desired interface using various `ncurses` functions.

After the interface creation and program logic, the `endwin()` function is called to clean up and restore the terminal to its original state before exiting the program.

Note that this example only shows the basic structure and usage of `initscr()`. In a real application, you would typically include error handling and implement various interface elements using `ncurses` functions to create a more interactive and visually appealing user interface.

The `initscr()` function sets the foundation for creating text-based user interfaces with `ncurses`. Its initialization of the library and terminal screen setup enables the use of other `ncurses` functions to control screen output, handle user input, and create dynamic interfaces within the terminal environment.
</details>
<details>
    <summary>
        <h2>
            endwin();
        </h2>
    </summary>

This example showcases the usage of the `endwin()` function in an `ncurses` program. The `endwin()` function is responsible for cleaning up and restoring the terminal to its original state when working with the `ncurses` library.

## Example

```c
#include <ncurses.h>

int main() {
    // Initialize ncurses
    initscr();

    // Display a message on the screen
    printw("Hello, ncurses!");

    // Refresh the screen to display the message
    refresh();

    // Wait for user input
    getch();

    // Clean up and restore terminal state
    endwin();

    return 0;
}
```

In this example, we start by calling `initscr()` to initialize the `ncurses` library and prepare the terminal for interface creation.

Using the `printw()` function, we display the message "Hello, ncurses!" on the screen.

The `refresh()` function is then called to update the terminal screen and make the message visible.

After waiting for user input using `getch()`, we use `endwin()` to clean up and restore the terminal to its original state before exiting the program. This ensures that the terminal returns to its normal behavior after running the `ncurses` program.

The `endwin()` function is essential for proper termination of an `ncurses` program, as it ensures that any modifications made to the terminal are reverted, and the terminal can function as usual once the program is finished.
</details>
<details>
    <summary>
        <h2>
            printw();
        </h2>
    </summary>

The `printw()` function in the `ncurses` library is used to display formatted text on the terminal screen. It allows you to print text at the current cursor position or at a specified location within a window.

The function works similarly to `printf()` from the standard C library, but instead of printing to the standard output, it prints to the `ncurses` window or pad.

## Syntax

The syntax for the `printw()` function is as follows:

```c
int printw(const char *format, ...);
```

The `format` parameter is a string that may contain format specifiers, similar to the `printf()` function. The additional arguments are the values to be inserted into the format string based on the format specifiers.

## Example

```c
#include <ncurses.h>

int main() {
    // Initialize ncurses
    initscr();

    // Print a formatted message
    printw("Hello, %s!", "ncurses");

    // Refresh the screen to display the message
    refresh();

    // Wait for user input
    getch();

    // Clean up and restore terminal state
    endwin();

    return 0;
}
```

In this example, after initializing `ncurses` with `initscr()`, we use the `printw()` function to display the message "Hello, ncurses!" on the terminal screen.

The format specifier `%s` is used in the format string to indicate where the string `"ncurses"` should be inserted. In this case, the resulting message will be "Hello, ncurses!".

After printing the message, we call `refresh()` to update the screen and make the message visible.

Finally, we use `getch()` to wait for user input and `endwin()` to clean up and restore the terminal before exiting the program.

Note that you can use various format specifiers in the `printw()` function, just like in `printf()`, to display different types of data such as integers, floats, and characters.

The `printw()` function is a convenient way to display formatted text within an `ncurses` program, enabling you to create dynamic and informative text-based interfaces.

</details>
<details>
    <summary>
        <h2>
            refresh();
        </h2>
    </summary>

The `refresh()` function in the `ncurses` library is used to update the physical screen with the contents of the virtual screen. It is an important function for displaying any changes made to the `ncurses` windows or pads.

When you modify the content of a window or pad using various `ncurses` functions like `printw()`, `mvprintw()`, or `wprintw()`, the changes are not immediately visible on the physical screen. Instead, they are stored in a virtual screen buffer. The `refresh()` function is responsible for updating the physical screen to reflect the changes made to the virtual screen.

## Syntax

The `refresh()` function does not take any arguments. Its syntax is as follows:

```c
int refresh(void);
```

## Example

```c
#include <ncurses.h>

int main() {
    // Initialize ncurses
    initscr();

    // Display a message on the virtual screen
    printw("Hello, ncurses!");

    // Update the physical screen to show the message
    refresh();

    // Wait for user input
    getch();

    // Clean up and restore terminal state
    endwin();

    return 0;
}
```

In this example, after initializing `ncurses` with `initscr()`, we use the `printw()` function to display the message "Hello, ncurses!" on the virtual screen.

To make the message visible on the physical screen, we call the `refresh()` function. This updates the physical screen with the contents of the virtual screen, causing the message to be displayed.

After calling `refresh()`, we use `getch()` to wait for user input and `endwin()` to clean up and restore the terminal before exiting the program.

The `refresh()` function is essential for ensuring that any modifications made to the `ncurses` windows or pads are reflected on the physical screen. It allows you to update the display and show the changes made to the virtual screen.

</details>
<details>
    <summary>
        <h2>
            getch();
        </h2>
    </summary>

The `getch()` function in the `ncurses` library is used to capture a single character of input from the user. It allows you to wait for user input and retrieve the corresponding character code or key code.

## Syntax

The syntax for the `getch()` function is as follows:

```c
int getch(void);
```

The function does not require any parameters and returns an integer value representing the captured character or key code.

## Example

```c
#include <ncurses.h>

int main() {
    // Initialize ncurses
    initscr();

    // Display a prompt
    printw("Press any key to continue...");

    // Refresh the screen to display the prompt
    refresh();

    // Wait for user input
    int ch = getch();

    // Display the captured key code
    printw("\nKey code: %d", ch);

    // Refresh the screen to display the key code
    refresh();

    // Wait for additional user input
    getch();

    // Clean up and restore terminal state
    endwin();

    return 0;
}
```

In this example, after initializing `ncurses` with `initscr()`, we use the `printw()` function to display the prompt "Press any key to continue..." on the terminal screen.

We then call `refresh()` to update the screen and make the prompt visible.

Next, we use `getch()` to wait for user input and capture the key code entered by the user. The captured key code is stored in the variable `ch`.

We then use `printw()` again to display the captured key code on the screen, using the format specifier `%d` to print the integer value.

After displaying the key code, we call `refresh()` again to update the screen and make the key code visible.

Finally, we use another `getch()` to wait for additional user input before cleaning up and restoring the terminal state with `endwin()`.

The `getch()` function is essential for capturing user input in `ncurses` programs, allowing you to implement interactive behavior and respond to user actions within your text-based interface.

</details>
<details>
    <summary>
        <h2>
            move();
        </h2>
    </summary>

The `move()` function in the `ncurses` library is used to move the cursor to a specific position on the terminal screen. It allows you to control where subsequent output will be displayed.

## Syntax

The syntax for the `move()` function is as follows:

```c
int move(int y, int x);
```

The `y` parameter represents the row coordinate, and the `x` parameter represents the column coordinate. Both `y` and `x` are zero-based, meaning the top-left corner of the screen is `(0, 0)`.

The function returns `OK` (a predefined constant with a value of 0) upon success, and `ERR` (another predefined constant with a value of -1) upon failure.

## Example

```c
#include <ncurses.h>

int main() {
    // Initialize ncurses
    initscr();

    // Move the cursor to row 5, column 10
    move(5, 10);

    // Print a message at the moved cursor position
    printw("Moved cursor!");

    // Refresh the screen to display the message
    refresh();

    // Wait for user input
    getch();

    // Clean up and restore terminal state
    endwin();

    return 0;
}
```

In this example, after initializing `ncurses` with `initscr()`, we use the `move()` function to move the cursor to row 5, column 10 on the terminal screen.

We then use the `printw()` function to print the message "Moved cursor!" at the moved cursor position.

After printing the message, we call `refresh()` to update the screen and make the message visible.

We use `getch()` to wait for user input before cleaning up and restoring the terminal state with `endwin()`.

The `move()` function is useful for positioning the cursor to specific locations on the terminal screen, allowing you to precisely control where text and other output will be displayed within your `ncurses` interface.

</details>