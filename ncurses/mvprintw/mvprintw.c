#include <ncurses.h>

int main() {
    // Initialize ncurses
    initscr();

    // Move the cursor to row 5, column 10 and print a formatted message
    mvprintw(5, 10, "Position: (%d, %d)", 5, 10);

    // Refresh the screen to display the message
    refresh();

    // Wait for user input
    getch();

    // Clean up and restore terminal state
    endwin();

    return 0;
}