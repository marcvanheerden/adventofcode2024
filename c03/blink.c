#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int part2(char input[]) {
    char start[] = "mul( , )";
    char toggle_on[] = "do()";
    char toggle_off[] = "don't()";

    int pos = 0;
    int toggle_pos = 0;

    int product = 1;
    int total = 0;
    char *endptr;
    bool toggle = true;

    while (*input != '\0') {

        if (!toggle) {
            if (toggle_on[toggle_pos] == *input) {
                if (toggle_pos < 3) {
                    toggle_pos++;
                } else {
                    toggle = true;
                    toggle_pos = 0;
                    pos = 0;
                }
            } else {
                toggle_pos = 0;
            }
            input++;
            continue;
        }

        if (toggle_off[toggle_pos] == *input) {
            if (toggle_pos < 6) {
                toggle_pos++;
            } else {
                toggle = false;
                toggle_pos = 0;
                pos = 0;
                input++;
                continue;
            }
        } else {
            toggle_pos = 0;
        }

        if (isdigit(*input)) {
            if (pos == 4) {
                product = strtol(input, &endptr, 10);
                input = endptr;
                pos++;
            } else if (pos == 6) {
                product *= strtol(input, &endptr, 10);
                input = endptr;
                pos++;
            }
        }

        if (*input == start[pos]) {
            if ((pos == 4) || (pos == 6)) {
                pos = 0;
            } else if (pos >= 7) {
                total += product;
                pos = 0;
            } else {
                pos++;
            }

            input++;
            continue;
        }

        pos = 0;
        input++;
    }

    return total;
}

int part1(char input[]) {
    char start[] = "mul( , )";
    int pos = 0;
    int product = 1;
    int total = 0;
    char *endptr;

    while (*input != '\0') {

        if (isdigit(*input)) {
            if (pos == 4) {
                product = strtol(input, &endptr, 10);
                input = endptr;
                pos++;
            } else if (pos == 6) {
                product *= strtol(input, &endptr, 10);
                input = endptr;
                pos++;
            }
        }

        if (*input == start[pos]) {
            if ((pos == 4) || (pos == 6)) {
                pos = 0;
            } else if (pos >= 7) {
                total += product;
                pos = 0;
            } else {
                pos++;
            }

            input++;
            continue;
        }

        pos = 0;
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
    int ans2 = part2(input);
    printf("answer 1: %d \n", ans1);
    printf("answer 2: %d \n", ans2);

    while (true) {
        gpio_put(LED_PIN, 1);
    }
}
