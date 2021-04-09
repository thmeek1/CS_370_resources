#include <stdio.h>
#include <unistd.h>


int main(int argc, char *argv[]) {
    printf("In example_program with arguments:\n");
    for (int i = 0; i < argc; i++) {
        printf("%s\n", argv[i]);
    }
    int seconds = 3;

    printf("Sleeping for %d seconds to simulate runtime...\n", seconds);
    sleep(seconds);
    printf("program done\n");

    return 0;
}
