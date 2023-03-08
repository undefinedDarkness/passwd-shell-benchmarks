#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_SHELLS 100

int main() {
    FILE *fp;
    char *line = NULL;
    size_t len = 0;
    size_t read;
    int i, count = 0;
    char *shells[MAX_SHELLS];
    int shell_counts[MAX_SHELLS] = {0};

    fp = fopen("passwd", "r");
    if (fp == NULL) {
        fprintf(stderr, "Failed to open file\n");
        exit(EXIT_FAILURE);
    }

    while ((read = getline(&line, &len, fp)) != -1) {
        char *last_colon = strrchr(line, ':');
        char *shell = last_colon + 1;

        for (i = 0; i < count; i++) {
            if (strcmp(shells[i], shell) == 0) {
                shell_counts[i]++;
                break;
            }
        }
        if (i == count) {
            shells[count] = shell;
            shell_counts[count] = 1;
            count++;
        }
    }

    for (i = 0; i < count; i++) {
        printf("%-25s : %5d\n", shells[i], shell_counts[i]);
    }

    fclose(fp);
    if (line) {
        free(line);
    }
    exit(EXIT_SUCCESS);
}
