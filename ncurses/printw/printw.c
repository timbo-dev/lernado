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