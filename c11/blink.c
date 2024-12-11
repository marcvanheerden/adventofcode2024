#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NODES 2000

typedef struct {
    unsigned long long val;
    unsigned long long count;
} Node;

int count_digits(unsigned long long number) {
    // invariant: number > 0

    unsigned long long place = 10;
    int digits = 1;

    while (number >= place) {
        place *= 10;
        digits++;
    }

    return digits;
}

void split(unsigned long long number, int digits, unsigned long long *front,
           unsigned long long *back) {
    // invariant: number > 0, digits is even

    unsigned long long div = 1;
    for (int zeros = 1; zeros <= digits / 2; zeros++) {
        div *= 10;
    }

    *front = number / div;
    *back = number - (*front * div);
}

unsigned long long part1(char input[], int blinks) {

    Node nodepool[MAX_NODES];
    int freenode = 0;
    char *cursor = input;
    char *endptr;

    while (*cursor != '\n') {
        if (*cursor == ' ') {
            cursor++;
        }

        Node node;
        node.val = strtoull(cursor, &endptr, 10);
        cursor = endptr;
        node.count = 1;

        nodepool[freenode] = node;
        freenode++;
    }

    for (int _blink = 0; _blink < blinks; _blink++) {
        int frozen_node_count = freenode;
        for (int node_idx = 0; node_idx < frozen_node_count; node_idx++) {
            Node node = nodepool[node_idx];

            if (node.val == 0) {
                nodepool[node_idx].val = 1;
                continue;
            }

            int digits = count_digits(node.val);

            if (digits % 2 == 0) {
                unsigned long long front;
                unsigned long long back;
                split(node.val, digits, &front, &back);
                // nodepool[node_idx].val = front;

                int found_front = -1;
                int found_back = -1;
                // check if the value we need to add already exists
                for (int idx2 = 0; idx2 < freenode; idx2++) {
                    if (nodepool[idx2].val == back) {
                        // nodepool[idx2].count += nodepool[node_idx].count;
                        found_back = idx2;
                        break;
                    }
                    if (nodepool[idx2].val == front) {
                        found_front = idx2;
                        break;
                    }
                }

                if (found_front >= 0) {
                    nodepool[node_idx].val = back;
                    nodepool[found_front].count += nodepool[node_idx].count;
                } else if (found_back >= 0) {
                    nodepool[node_idx].val = front;
                    nodepool[found_back].count += nodepool[node_idx].count;
                } else {
                    nodepool[node_idx].val = front;
                    Node newnode;
                    newnode.val = back;
                    newnode.count = nodepool[node_idx].count;
                    nodepool[freenode] = newnode;
                    freenode++;
                }

            } else {
                nodepool[node_idx].val *= 2024;
            }
        }
    }

    unsigned long long count = 0;

    for (int idx = 0; idx < freenode; idx++) {
        count += nodepool[idx].count;
    }

    return count;
}

int main() {
    stdio_init_all();
    const uint LED_PIN = 25;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    unsigned long long ans1 = part1(example1, 25);
    printf("answers : %llu  \n", ans1);

    while (true) {
        gpio_put(LED_PIN, 1);
    }
}
