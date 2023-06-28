#include <ncurses.h>

int main() {
    // Initialize ncurses
    initscr();

    // Create a new window
    WINDOW *new_window = newwin(10, 20, 5, 10);

    // Check if the new window was created successfully
    if (new_window == NULL) {
        printw("Failed to create a new window!");
        refresh();
        getch();
        endwin();
        return 1;
    }

    // Print a message in the new window
    mvwprintw(new_window, 2, 5, "Hello, new window!");

    // Refresh the standard window to display the message
    refresh();

    // Refresh the new window to display the message
    wrefresh(new_window);

    // Wait for user input
    getch();

    // Clean up and restore terminal state
    endwin();

    return 0;
}
