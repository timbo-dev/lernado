#include <ncurses.h>

int main() {
    initscr(); // Initialize ncurses

    raw(); // Put the terminal into raw mode

    printw("Press CTRL + C: ");
    refresh();

    int c = getch(); // Read a single character from the user

    printw("\nYou entered: %c\n", c);
    refresh();

    getch(); // Wait for another character before exiting

    endwin(); // Clean up and restore the terminal state

    return 0;
}