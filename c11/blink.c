#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NODES 200

typedef struct {
    uint64_t val;
    short next;
} Node;

int count_digits(long long number) {
    // invariant: number > 0

    long long place = 10;
    int digits = 1;

    while (number >= place) {
        place *= 10;
        digits++;
    }

    return digits;
}

void split(long long number, int digits, long long *front, long long *back) {
    // invariant: number > 0, digits is even

    long long div = 1;
    for (int zeros = 1; zeros <= digits / 2; zeros++) {
        div *= 10;
    }

    *front = number / div;
    *back = number - (*front * div);
}

long long part1(char input[]) {

    Node nodepool[MAX_NODES];
    int freenode = 0;
    char *cursor = input;
    char *endptr;

    while (*cursor != '\n') {
        if (*cursor == ' ') {
            cursor++;
        }

        Node node;
        node.val = strtoll(cursor, &endptr, 10);
        cursor = endptr;
        node.next = -1;
        if (freenode > 0) {
            nodepool[freenode - 1].next = freenode;
        }
        nodepool[freenode] = node;
        freenode++;
    }

    for (int _blink = 0; _blink < 25; _blink++) {
        int node_idx = 0;
        while (node_idx != -1) {
            Node node = nodepool[node_idx];
            if (node.val == 0) {
                nodepool[node_idx].val = 1;
                node_idx = node.next;
                continue;
            }

            int digits = count_digits(node.val);

            if (digits % 2 == 0) {
                long long front;
                long long back;
                split(node.val, digits, &front, &back);
                nodepool[node_idx].val = front;

                Node newnode;
                newnode.val = back;
                newnode.next = node.next;
                nodepool[node_idx].next = freenode;

                nodepool[freenode] = newnode;
                freenode++;

                node_idx = newnode.next;
            } else {
                nodepool[node_idx].val *= 2024;
                node_idx = node.next;
            }
        }
    }

    return 1;
}

int main() {
    stdio_init_all();
    const uint LED_PIN = 25;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    long long ans1 = part1(example1);
    printf("answers : %lld  \n", ans1);

    while (true) {
        gpio_put(LED_PIN, 1);
    }
}
