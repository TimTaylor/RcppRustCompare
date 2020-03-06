#include <stdio.h>
#include <stdint.h>

struct graph_buffer
{
    int32_t *array;
    size_t array_length;
    size_t *node_degrees;
    size_t node_degrees_length;
};

struct graph_buffer erdos(size_t n, double p);
struct graph_buffer fast_erdos(size_t n, double p);
void rust_free(struct graph_buffer);
