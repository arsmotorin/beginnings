#include <stdio.h>

void shadowing() {
    int x = 5;
    printf("The value of x is: %d\n", x);

    {
        int x = 10; // This x shadows the outer x
        printf("The value of inner x is: %d\n", x);
    }

    x = 6;
    printf("The value of x is: %d\n", x);
}

int main() {
    shadowing();
    return 0;
}