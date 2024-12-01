#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXLEN 1001

int comp(const void *a, const void *b) { return (*(int *)a - *(int *)b); }

int part1(char input[]) {
    int list1[MAXLEN] = {0};
    int list2[MAXLEN] = {0};
    char *endptr;

    for (int idx = 0; idx < MAXLEN; idx++) {
        list1[idx] = strtol(input, &endptr, 10);
        input = endptr + 3;
        list2[idx] = strtol(input, &endptr, 10);
        input = endptr + 1;

        if (*input == '\0') {
            break;
        }
    }

    qsort(list1, MAXLEN, sizeof(list1[0]), comp);
    qsort(list2, MAXLEN, sizeof(list1[0]), comp);

    int distance = 0;
    for (int idx = 0; idx < MAXLEN; idx++) {
        distance += abs(list2[idx] - list1[idx]);
    }

    return distance;
}

int part2(char input[]) {
    int list1[MAXLEN] = {0};
    int list2[MAXLEN] = {0};
    char *endptr;

    for (int idx = 0; idx < MAXLEN; idx++) {
        list1[idx] = strtol(input, &endptr, 10);
        input = endptr + 3;
        list2[idx] = strtol(input, &endptr, 10);
        input = endptr + 1;

        if (*input == '\0') {
            break;
        }
    }

    int total = 0;
    for (int idx = 0; idx < MAXLEN; idx++) {
        if (list1[idx] == 0) {
            continue;
        }

        for (int idx2 = 0; idx2 < MAXLEN; idx2++) {
            if (list2[idx2] == 0) {
                continue;
            }

            if (list1[idx] == list2[idx2]) {
                total += list1[idx];
            }
        }
    }

    return total;
}

int main() {
    stdio_init_all();
    const uint LED_PIN = 25;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    int ans1 = part1(input);
    printf("answer 1: %d \n", ans1);
    int ans2 = part2(input);
    printf("answer 2: %d \n", ans2);

    while (true) {
        gpio_put(LED_PIN, 1);
    }
}
