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

    // Clear the screen
    clear();

    // Refresh the screen to show the cleared screen
    refresh();

    // Wait for additional user input
    getch();

    // Clean up and restore terminal state
    endwin();

    return 0;
}