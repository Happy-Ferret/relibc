#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int main(int argc, char** argv) {
    char* string = (char*) calloc(20, sizeof(char));
    strcpy(string, "tempXXXXXX");
    mktemp(string);
    printf("%s\n",string);
    free(string);
    return 0;
}
