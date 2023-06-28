#include <ncurses.h>

int main() {
    initscr(); // Initialize ncurses

    int height, width, start_y, start_x;

    height = 10;
    width = 20;
    start_y = start_x = 0;

    WINDOW *win = newwin(height, width, start_y, start_x); // Create a new window with specified dimensions and starting position

    refresh(); // Refresh the standard screen

    box(win, 0, 0); // Draw a box around the window
    mvwprintw(win, 1, 1, "Hello World"); // Print "Hello World" at row 1, column 1 within the window
    wrefresh(win); // Refresh the window to display the changes

    int c = getch(); // Wait for user input and store the pressed key in 'c'

    endwin(); // Clean up and restore the terminal state

    return 0;
}