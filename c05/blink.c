#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_DEPS 100
#define MAX_PAGES 100
#define MAX_UPDATE_LEN 30

typedef struct {
    int deps[MAX_DEPS];
    int count;
} DepList;

bool isIn(int value, int array[], int size) {
    for (int i = 0; i < size; i++) {
        if (array[i] == value) {
            return true;
        }
    }
    return false;
}

void fix_update(int *update, int size, const DepList deplists[]) {
    // look for incorrect ordered updates, if one is found then swap the values
    // that are in conflict and start checking again, only stop when the full
    // check passes
    for (;;) {
        bool start_again = false;
        for (int idx = 1; idx < size; idx++) {
            for (int prec = 0; prec < idx; prec++) {
                int prec_page = update[prec];
                if (isIn(update[idx], deplists[prec_page].deps,
                         deplists[prec_page].count)) {
                    // swap values where a conflict is found
                    int temp = update[idx];
                    update[idx] = update[prec];
                    update[prec] = temp;
                    // start checking again
                    start_again = true;
                    break;
                }
            }
            if (start_again) {
                break;
            }
        }
        if (!start_again) {
            break;
        }
    }
}

int check_updates(char input[], bool fix) {
    DepList deplists[MAX_PAGES] = {0};

    int page = 0;
    int dep = 0;
    char *endptr;

    // collect dependency lists
    while (*input != '\0') {
        if (*input == '\n') {
            input++;
            break;
        }

        dep = strtol(input, &endptr, 10);
        input = endptr + 1;
        page = strtol(input, &endptr, 10);
        input = endptr + 1;

        int count = deplists[page].count;
        deplists[page].deps[count] = dep;
        deplists[page].count++;
    }

    int update[MAX_UPDATE_LEN] = {-1};
    int update_pages = 0;
    int total = 0;
    bool right_order = true;

    while (*input != '\0') {
        if (*input == '\n') {
            if (right_order && !fix) {
                total += update[update_pages / 2];
            } else if (!right_order && fix) {
                fix_update(&update, update_pages, deplists);
                total += update[update_pages / 2];
            }
            update_pages = 0;
            right_order = true;
            input++;
        }

        // short-circuit update if wrong order found and not fix mode
        if (!right_order && !fix) {
            while (*input != '\n') {
                input++;
            }
            continue;
        }

        if (isdigit(*input)) {
            int page = strtol(input, &endptr, 10);
            input = endptr;

            for (int prec = 0; prec < update_pages; prec++) {
                int prec_page = update[prec];
                if (isIn(page, deplists[prec_page].deps,
                         deplists[prec_page].count)) {
                    right_order = false;
                }
            }

            update[update_pages] = page;
            update_pages++;
            continue;
        }

        input++;
    }

    return total;
}

int main() {
    stdio_init_all();
    const uint LED_PIN = 25;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    int ans1 = check_updates(input, false);
    int ans2 = check_updates(input, true);
    printf("answer 1: %d \n", ans1);
    printf("answer 2: %d \n", ans2);

    while (true) {
        gpio_put(LED_PIN, 1);
    }
}
