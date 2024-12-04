#include "inputs.h"
#include "pico/stdlib.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define WORD "XMAS"
#define WORDREV "SAMX"
#define WORDLEN 4
#define WORD2 "MAS"
#define WORDREV2 "SAM"
#define WORDLEN2 3

int part1(char input[]) {
    // finds words by looking from the most lower-right point of the word
    // assumes all lines are the same length
    char *cursor = input;
    int row = 0;
    int col = 0;
    int maxcol = 0;
    int count = 0;

    while (*cursor != '\0') {
        if (*cursor == '\n') {
            if (row == 0) {
                maxcol = cursor - input;
            }

            row++;
            col = 0;
        }

        bool col_clearance = col >= (WORDLEN - 1);
        bool row_clearance = row >= (WORDLEN - 1);
        bool pos_diag_clearance =
            row_clearance && ((col + WORDLEN - 1) <= maxcol);
        bool neg_diag_clearance = row_clearance && col_clearance;

        if (col_clearance) {
            // check if horizontal word or reverse word ends here
            if ((strncmp(WORD, cursor - WORDLEN + 1, WORDLEN) == 0) ||
                (strncmp(WORDREV, cursor - WORDLEN + 1, WORDLEN) == 0)) {
                count++;
            }
        }

        if (row_clearance) {
            // check if vertical word or reverse word ends here

            bool found = true;
            bool foundrev = true;

            for (int idx = 0; idx < WORDLEN; idx++) {
                char *cmp = cursor - (maxcol + 1) * idx;
                found = found && (*cmp == WORD[idx]);
                foundrev = foundrev && (*cmp == WORDREV[idx]);
            }

            if (found || foundrev) {
                count++;
            }
        }

        if (pos_diag_clearance) {
            // check if positive diagonal (/) word or reverse word ends here
            bool found = true;
            bool foundrev = true;

            for (int idx = 0; idx < WORDLEN; idx++) {
                char *cmp = cursor - (maxcol + 1) * idx + idx;
                found = found && (*cmp == WORD[idx]);
                foundrev = foundrev && (*cmp == WORDREV[idx]);
            }

            if (found || foundrev) {
                count++;
            }
        }

        if (neg_diag_clearance) {
            // check if negative diagonal (\) word or reverse word ends here
            bool found = true;
            bool foundrev = true;

            for (int idx = 0; idx < WORDLEN; idx++) {
                char *cmp = cursor - (maxcol + 1) * idx - idx;
                found = found && (*cmp == WORD[idx]);
                foundrev = foundrev && (*cmp == WORDREV[idx]);
            }

            if (found || foundrev) {
                count++;
            }
        }

        cursor++;
        col++;
    }

    return count;
}

int part2(char input[]) {
    // finds patterns by looking from the most lower-right point of the word
    // assumes all lines are the same length
    char *cursor = input;
    int row = 0;
    int col = 0;
    int maxcol = 0;
    int count = 0;

    while (*cursor != '\0') {
        if (*cursor == '\n') {
            if (row == 0) {
                maxcol = cursor - input;
            }

            row++;
            col = 0;
        }

        bool col_clearance = col >= (WORDLEN2 - 1);
        bool row_clearance = row >= (WORDLEN2 - 1);

        if (row_clearance && col_clearance) {
            // check if pattern ends here

            bool pos_diag = true;
            bool neg_diag = true;
            bool pos_diag_rev = true;
            bool neg_diag_rev = true;

            // trace negative diagonal (\)
            for (int idx = 0; idx < WORDLEN2; idx++) {
                char *cmp = cursor - (maxcol + 1) * idx - idx;
                neg_diag = neg_diag && (*cmp == WORD2[idx]);
                neg_diag_rev = neg_diag_rev && (*cmp == WORDREV2[idx]);
            }

            // trace positive diagonal (/)
            for (int idx = 0; idx < WORDLEN2; idx++) {
                char *cmp = cursor - WORDLEN2 + 1 - (maxcol + 1) * idx + idx;
                pos_diag = pos_diag && (*cmp == WORD2[idx]);
                pos_diag_rev = pos_diag_rev && (*cmp == WORDREV2[idx]);
            }

            if ((neg_diag || neg_diag_rev) && (pos_diag || pos_diag_rev)) {
                count++;
            }
        }

        cursor++;
        col++;
    }

    return count;
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
