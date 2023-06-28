#include <ncurses.h>

int main() {
    initscr();

    int height, width, start_y, start_x;

    height = 10;
    width = 20;
    start_y = start_x = 10;

    WINDOW * win = newwin(height, width, 0, 100);

    refresh();

    box(win, 0, 0);
    mvwprintw(win, 1, 1, "Hello World");
    wrefresh(win);

    int c = getch();

    endwin();

    return 0;
}