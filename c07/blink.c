#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_TERMS 50

uint64_t powllu(uint64_t base, uint64_t exponent) {
    uint64_t result = 1;

    for (int exp = 1; exp <= exponent; exp++) {
        result *= base;
    }

    return result;
}

uint64_t conc(uint64_t pre, uint64_t post) {
    for (uint64_t expo = 1; expo <= 20; expo++) {
        if (post < powllu(10, expo)) {
            return pre * powllu(10, expo) + post;
        }
    }

    return 0;
}

bool calc_possible(uint64_t target, uint64_t current, uint64_t *terms,
                   int term_count, int idx, bool concat) {
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
    uint64_t add_next = calc_possible(target, current + next_term, terms,
                                      term_count, idx, concat);
    uint64_t mul_next = calc_possible(target, current * next_term, terms,
                                      term_count, idx, concat);

    if (concat) {
        uint64_t conc_next = calc_possible(target, conc(current, next_term),
                                           terms, term_count, idx, concat);
        return add_next || mul_next || conc_next;
    }

    return add_next || mul_next;
}

typedef struct {
    uint64_t part1;
    uint64_t part2;
} Result;

Result part1(char input[]) {
    uint64_t target;
    uint64_t terms[MAX_TERMS];
    int term_count = 0;
    char *endptr;
    uint64_t total1 = 0;
    uint64_t total2 = 0;

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

        if (calc_possible(target, terms[0], terms, term_count, 1, false)) {
            total1 += target;
            total2 += target;
        } else if (calc_possible(target, terms[0], terms, term_count, 1,
                                 true)) {
            total2 += target;
        }

        term_count = 0;
        input++;
    }

    Result result;
    result.part1 = total1;
    result.part2 = total2;
    return result;
}

int main() {
    stdio_init_all();
    const uint LED_PIN = 25;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    Result ans1 = part1(input);
    printf("answer: %llu %llu \n", ans1.part1, ans1.part2);

    while (true) {
        gpio_put(LED_PIN, 1);
    }
}
