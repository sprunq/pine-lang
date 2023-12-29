#include "pine_io.h"

void print_int(int64_t i)
{
    printf("%lld\n", i);
}

void print_bool(uint8_t b)
{
    if (b)
    {
        printf("true\n");
    }
    else
    {
        printf("false\n");
    }
}