#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

short get_file_id(int idx) {
    if (idx % 2 == 1) {
        return -1;
    }

    return idx / 2;
}

long long part1(char input[]) {
    size_t len = strlen(input) - 1; // drop \n at the end

    int front_cursor = 0;
    int back_cursor = len - 1;
    int frontid = get_file_id(front_cursor);
    int frontlen = input[front_cursor] - '0';
    int backid = get_file_id(back_cursor);
    int backlen = input[back_cursor] - '0';
    long long checksum_idx = 0;

    long long checksum = 0;
    while (front_cursor < back_cursor) {
        if ((backid == -1) || (backlen == 0)) {
            back_cursor--;
            backid = get_file_id(back_cursor);
            backlen = input[back_cursor] - '0';
        } else if (frontlen == 0) {
            front_cursor++;
            frontid = get_file_id(front_cursor);
            frontlen = input[front_cursor] - '0';
        } else if (frontid == -1) {
            frontlen--;
            backlen--;
            checksum += backid * checksum_idx;
            checksum_idx++;
        } else {
            frontlen--;
            checksum += frontid * checksum_idx;
            checksum_idx++;
        }
    }

    if (frontlen < backlen) {
        while (frontlen > 0 && frontid > 0) {
            frontlen--;
            checksum += frontid * checksum_idx;
            checksum_idx++;
        }
    } else {
        while (backlen > 0 && backid > 0) {
            backlen--;
            checksum += backid * checksum_idx;
            checksum_idx++;
        }
    }

    return checksum;
}

#define MAX_NODES 35000

typedef struct __attribute__((packed)) {
    short id;
    char len;
    short next;
    short prev;
} Node;

long long part2(char input[]) {

    Node nodepool[MAX_NODES];
    int freenode = 0;
    char *cursor = input;

    while (*cursor != '\n') {
        Node node;
        node.id = get_file_id(freenode);
        node.len = *cursor - '0';
        node.next = -1;
        if (freenode > 0) {
            node.prev = freenode - 1;
            nodepool[freenode - 1].next = freenode;
        } else {
            node.prev = -1;
        }
        nodepool[freenode] = node;
        freenode++;
        cursor++;
    }

    int back_idx = freenode - 1;

    while (back_idx >= 0) {
        Node backnode = nodepool[back_idx];
        if (backnode.id == -1) {
            back_idx = backnode.prev;
            continue;
        }

        int front_idx = 0;
        Node frontnode = nodepool[front_idx];
        while (front_idx != back_idx) {
            if (frontnode.id == -1 && frontnode.len == backnode.len) {
                // perfect gap, just swap IDs
                nodepool[front_idx].id = backnode.id;
                nodepool[back_idx].id = -1;
                break;
            } else if (frontnode.id == -1 && frontnode.len > backnode.len) {
                // gap with space
                // make a new node to insert
                Node newnode;
                newnode.id = backnode.id;
                newnode.len = backnode.len;
                newnode.next = front_idx;
                newnode.prev = frontnode.prev;

                // insert it and fix links
                nodepool[freenode] = newnode;
                nodepool[frontnode.prev].next = freenode;
                nodepool[frontnode.next].prev = freenode;
                freenode++;

                // adjust size of old gap node
                nodepool[front_idx].len = frontnode.len - backnode.len;

                // make old backnode a gap
                nodepool[back_idx].id = -1;

                break;
            }

            front_idx = frontnode.next;
            frontnode = nodepool[front_idx];
        }
        back_idx = backnode.prev;
    }

    // calculate the checksum

    Node node = nodepool[0];
    long long checksum_idx = 0;
    long long checksum = 0;

    while (node.next != -1) {
        int size = node.len;

        while (size > 0) {
            if (node.id != -1) {
                checksum += node.id * checksum_idx;
            }
            checksum_idx++;
            size--;
        }

        node = nodepool[node.next];
    }

    return checksum;
}

int main() {
    stdio_init_all();
    const uint LED_PIN = 25;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    long long ans1 = part1(input);
    long long ans2 = part2(input);
    printf("answers : %lld  %lld \n", ans1, ans2);

    while (true) {
        gpio_put(LED_PIN, 1);
    }
}
