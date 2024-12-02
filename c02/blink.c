#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LEVELS 10

bool assess(int levels[], int length, int ignore) {

    int buffer = 0;
    bool started = false;
    int diffs[MAX_LEVELS - 1];
    int diff_count = 0;

    for (int idx = 0; idx < length; idx++) {
        if (idx == ignore) {
            continue;
        }

        if (started) {
            diffs[diff_count] = levels[idx] - buffer;
            diff_count++;
        }

        buffer = levels[idx];
        started = true;
    }

    bool sign = diffs[0] > 0;

    // check for monotonicity
    for (int idx = 1; idx < diff_count; idx++) {
        if (sign != (diffs[idx] > 0)) {
            return false;
        }
    }

    // check for change bounds
    for (int idx = 0; idx < diff_count; idx++) {
        if (abs(diffs[idx]) < 1 || abs(diffs[idx]) > 3) {
            return false;
        }
    }

    return true;
}

int count_safe_levels(char input[], bool tolerant) {
    char *endptr;
    int level_count = 0;
    int safe_count = 0;
    int levels[MAX_LEVELS] = {0};

    while (*input != '\0') {
        if (isdigit(*input)) {
            levels[level_count] = strtol(input, &endptr, 10);
            input = endptr;
            level_count++;
        } else if (*input == '\n') {
            if (assess(levels, level_count, -1)) {
                safe_count++;
                level_count = 0;
                input++;
                continue;
            }

            if (tolerant) {
                for (int ignore = 0; ignore < level_count; ignore++) {
                    if (assess(levels, level_count, ignore)) {
                        safe_count++;
                        break;
                    }
                }
            }

            level_count = 0;
            input++;
        } else {
            input++;
        }
    }

    return safe_count;
}

int main() {
    stdio_init_all();
    const uint LED_PIN = 25;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    int ans1 = count_safe_levels(input, false);
    int ans2 = count_safe_levels(input, true);
    printf("answer 1: %d \n", ans1);
    printf("answer 2: %d \n", ans2);

    while (true) {
        gpio_put(LED_PIN, 1);
    }
}
