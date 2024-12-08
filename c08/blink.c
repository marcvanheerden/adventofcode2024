#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NODES 1000

typedef struct {
    int row[MAX_NODES];
    int col[MAX_NODES];
    char chr[MAX_NODES];
    int count;
} Nodes;

int gcd(int a, int b) {
    // with thanks to ChatGPT / Euclid
    while (b != 0) {
        int temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

bool node_present(int row, int col, Nodes *nodes) {
    for (int idx = 0; idx < nodes->count; idx++) {
        if ((row == nodes->row[idx]) && (col == nodes->col[idx])) {
            return true;
        }
    }

    return false;
}

int part1(Nodes *nodes, int max_row, int max_col) {

    Nodes antinodes;
    antinodes.count = 0;

    for (int idx1 = 0; idx1 < nodes->count; idx1++) {
        for (int idx2 = 0; idx2 < idx1; idx2++) {
            if (nodes->chr[idx1] == nodes->chr[idx2]) {
                int delta_row = nodes->row[idx1] - nodes->row[idx2];
                int delta_col = nodes->col[idx1] - nodes->col[idx2];

                int antinode_row = nodes->row[idx2] - delta_row;
                int antinode_col = nodes->col[idx2] - delta_col;

                if ((antinode_row >= 0) && (antinode_col >= 0) &&
                    (antinode_row <= max_row) && (antinode_col <= max_col) &&
                    !node_present(antinode_row, antinode_col, &antinodes)) {
                    antinodes.row[antinodes.count] = antinode_row;
                    antinodes.col[antinodes.count] = antinode_col;
                    antinodes.chr[antinodes.count] = nodes->chr[idx1];
                    antinodes.count++;
                }

                antinode_row = nodes->row[idx1] + delta_row;
                antinode_col = nodes->col[idx1] + delta_col;

                if ((antinode_row >= 0) && (antinode_col >= 0) &&
                    (antinode_row <= max_row) && (antinode_col <= max_col) &&
                    !node_present(antinode_row, antinode_col, &antinodes)) {
                    antinodes.row[antinodes.count] = antinode_row;
                    antinodes.col[antinodes.count] = antinode_col;
                    antinodes.chr[antinodes.count] = nodes->chr[idx1];
                    antinodes.count++;
                }
            }
        }
    }

    return antinodes.count;
}

int part2(Nodes *nodes, int max_row, int max_col) {

    Nodes antinodes;
    antinodes.count = 0;

    for (int idx1 = 0; idx1 < nodes->count; idx1++) {
        for (int idx2 = 0; idx2 < idx1; idx2++) {
            if (nodes->chr[idx1] == nodes->chr[idx2]) {
                int delta_row = nodes->row[idx1] - nodes->row[idx2];
                int delta_col = nodes->col[idx1] - nodes->col[idx2];
                int delta_gcd = gcd(delta_row, delta_col);
                delta_row /= delta_gcd;
                delta_col /= delta_gcd;

                int antinode_row;
                int antinode_col;

                for (int multiple = 0; true; multiple++) {
                    antinode_row = nodes->row[idx2] - multiple * delta_row;
                    antinode_col = nodes->col[idx2] - multiple * delta_col;
                    if ((antinode_row >= 0) && (antinode_col >= 0) &&
                        (antinode_row <= max_row) &&
                        (antinode_col <= max_col)) {
                        if (!node_present(antinode_row, antinode_col,
                                          &antinodes)) {
                            antinodes.row[antinodes.count] = antinode_row;
                            antinodes.col[antinodes.count] = antinode_col;
                            antinodes.chr[antinodes.count] = nodes->chr[idx1];
                            antinodes.count++;
                        }
                    } else {
                        break;
                    }
                }

                for (int multiple = 0; true; multiple++) {
                    antinode_row = nodes->row[idx1] + multiple * delta_row;
                    antinode_col = nodes->col[idx1] + multiple * delta_col;
                    if ((antinode_row >= 0) && (antinode_col >= 0) &&
                        (antinode_row <= max_row) &&
                        (antinode_col <= max_col)) {
                        if (!node_present(antinode_row, antinode_col,
                                          &antinodes)) {
                            antinodes.row[antinodes.count] = antinode_row;
                            antinodes.col[antinodes.count] = antinode_col;
                            antinodes.chr[antinodes.count] = nodes->chr[idx1];
                            antinodes.count++;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    return antinodes.count;
}

typedef struct {
    int part1;
    int part2;
} Result;

Result assess_nodes(char input[]) {

    Nodes nodes;
    nodes.count = 0;
    int col = 0;
    int row = 0;
    int max_col = -1;

    while (*input != '\0') {
        if (*input == '\n') {
            max_col = col - 1;
            col = -1;
            row++;
        } else if (*input != '.') {
            nodes.col[nodes.count] = col;
            nodes.row[nodes.count] = row;
            nodes.chr[nodes.count] = *input;
            nodes.count++;
        }

        col++;
        input++;
    }

    Result result;
    result.part1 = part1(&nodes, row - 1, max_col);
    result.part2 = part2(&nodes, row - 1, max_col);

    return result;
}

int main() {
    stdio_init_all();
    const uint LED_PIN = 25;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    Result ans1 = assess_nodes(input);
    printf("answers : %d  %d \n", ans1.part1, ans1.part2);

    while (true) {
        gpio_put(LED_PIN, 1);
    }
}
