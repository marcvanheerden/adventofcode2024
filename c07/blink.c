#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_TERMS 50

bool calc_possible(uint64_t target, uint64_t current, uint64_t *terms,
                   int term_count, int idx) {
    // stop early if overshot
    if (current > target) {
        return false;
    }

    // terminal condition
    if (idx == term_count) {
        return target == current;
    }

    uint64_t next_term = terms[idx];
    idx++;
    uint64_t add_next =
        calc_possible(target, current + next_term, terms, term_count, idx);
    uint64_t mul_next =
        calc_possible(target, current * next_term, terms, term_count, idx);

    return add_next || mul_next;
}

int part1(char input[]) {
    uint64_t target;
    uint64_t terms[MAX_TERMS];
    int term_count = 0;
    char *endptr;
    int total = 0;

    while (*input != '\0') {
        target = strtoull(input, &endptr, 10);
        input = endptr + 2;

        while (*input != '\n') {
            if (*input == ' ') {
                input++;
            }

            terms[term_count] = strtoull(input, &endptr, 10);
            term_count++;
            input = endptr;
        }

        if (calc_possible(target, terms[0], &terms, term_count, 1)) {
            total += target;
        }

        term_count = 0;
        input++;
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

    while (true) {
        gpio_put(LED_PIN, 1);
    }
}
