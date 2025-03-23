//#include <stdio.h>
#include "test.h"

extern unsigned long add(unsigned long x);

#define MAGIC 5 + FIVE

void print_num(unsigned long x) {
    printf("%lu\n", x);
}

int main() {
    return add(MAGIC);
}
