#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_OBSTACLES 1500
#define MAX_LOCATIONS 10000

typedef struct {
    int row[MAX_OBSTACLES];
    int col[MAX_OBSTACLES];
    int count;
} Obstacles;

typedef struct {
    int row[MAX_LOCATIONS];
    int col[MAX_LOCATIONS];
    int dir[MAX_LOCATIONS]; // direction the guard was facing
    int count;
} Locations;

typedef struct {
    int row;
    int col;
    int dir; // direction the guard is facing
    int maxrow;
    int maxcol;
} Guard;

bool obstacle_detection(int row, int col, Obstacles *obs) {
    for (int idx = 0; idx < obs->count; idx++) {
        if ((obs->row[idx] == row) && (obs->col[idx] == col)) {
            return true;
        } else if (obs->row[idx] > row) {
            // since obstacles are populated in order, once the row is
            // exceeded there will be no matches
            break;
        }
    }

    // check the last one which is the only one that is allowed to be
    // out of row order
    return (obs->row[obs->count - 1] == row) &&
           (obs->col[obs->count - 1] == col);
}

void right_turn(int *delta_row, int *delta_col) {
    if ((*delta_row == -1) && (*delta_col == 0)) {
        *delta_row = 0;
        *delta_col = 1;
    } else if ((*delta_row == 0) && (*delta_col == 1)) {
        *delta_row = 1;
        *delta_col = 0;
    } else if ((*delta_row == 1) && (*delta_col == 0)) {
        *delta_row = 0;
        *delta_col = -1;
    } else if ((*delta_row == 0) && (*delta_col == -1)) {
        *delta_row = -1;
        *delta_col = 0;
    }
}

bool does_it_cycle(Obstacles *obs, Guard guard, Locations *path) {

    int delta_row = -1;
    int delta_col = 0;

    guard.dir = 0;

    path->count = 0;

    while ((guard.row > 0) && (guard.row <= guard.maxrow) && (guard.col > 0) &&
           (guard.col <= guard.maxcol)) {

        bool revisit = false;
        for (int idx = 0; idx < path->count; idx++) {
            if ((path->row[idx] == guard.row) &&
                (path->col[idx] == guard.col)) {
                revisit = true;
                if (path->dir[idx] == guard.dir) {
                    // cycle found
                    return true;
                }
                break;
            }
        }

        if (!revisit) {
            path->row[path->count] = guard.row;
            path->col[path->count] = guard.col;
            path->dir[path->count] = guard.dir;
            path->count++;
        }

        while (obstacle_detection(guard.row + delta_row, guard.col + delta_col,
                                  obs)) {
            right_turn(&delta_row, &delta_col);
            guard.dir = (guard.dir + 1) % 4;
        }

        guard.row += delta_row;
        guard.col += delta_col;
    }

    return false;
}

int path_counter(char input[]) {
    int row = 0;
    int col = 0;
    int row_len = -1;

    Obstacles obs;
    obs.count = 0;

    Guard guard;

    while (*input != '\0') {
        if (*input == '#') {
            obs.row[obs.count] = row;
            obs.col[obs.count] = col;
            obs.count++;
        } else if (*input == '^') {
            guard.row = row;
            guard.col = col;
        } else if (*input == '\n') {
            row_len = col - 1;
            row++;
            col = -1;
        }

        col++;
        input++;
    }

    guard.maxrow = row - 1;
    guard.maxcol = row_len;
    Locations path;
    does_it_cycle(&obs, guard, &path);

    int cycles = 0;
    Locations path2;
    for (int idx = 1; idx < path.count; idx++) {
        obs.row[obs.count] = path.row[idx];
        obs.col[obs.count] = path.col[idx];
        obs.count++;
        if (does_it_cycle(&obs, guard, &path2)) {
            cycles++;
        }
        obs.count--;
    }

    return cycles;
}

int main() {
    stdio_init_all();
    const uint LED_PIN = 25;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    int ans1 = path_counter(input);
    printf("answer 1: %d \n", ans1);

    while (true) {
        gpio_put(LED_PIN, 1);
    }
}
