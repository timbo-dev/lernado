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