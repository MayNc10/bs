#include <unistd.h>
#define STRING "Hello World!\n"
#define LENGTH 14

int main() {
    write(STDOUT_FILENO, STRING, LENGTH);

    return 0;
}  