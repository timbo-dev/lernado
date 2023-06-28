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