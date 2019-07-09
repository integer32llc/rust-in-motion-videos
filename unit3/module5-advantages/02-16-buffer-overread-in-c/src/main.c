#include <stdio.h>
#include <stdlib.h>

int main() {
    int* accts = calloc(2, sizeof(int));

    accts[0] = 50298;
    accts[1] = 7423;

    int bal = accts[11];
    printf("Balance: %d\n", bal);

    free(accts);
    return 0;
}
