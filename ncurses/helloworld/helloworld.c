#include <stdio.h>
#include <ncurses.h>

int main() {
    // Initialize the screen
    // setus up memory and clears the screen
    initscr();

    int x, y;

    x = y = 10;

    // moves the cursor to the specified location
    move(y, x);

    // print a string(const char *) to a window
    printw("Hello World!");

    // refreshes the screen to match whats in memory
    refresh();

    // whats for user input, returns int value of that key
    int c = getch();

    mvprintw(0, 0, "%d", c);

    clear();
    
    getch();

    // deallocates memory and ends ncurses
    endwin();

    return 0;
}