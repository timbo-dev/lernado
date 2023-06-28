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