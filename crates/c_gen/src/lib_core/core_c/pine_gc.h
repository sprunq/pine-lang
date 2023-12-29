#ifndef PINE_GC
#define PINE_GC

#include "log.h"
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef enum Gc_Flag
{
    GC_FLAG_NONE = 0x0,
    GC_FLAG_ROOT = 0x1,
    GC_FLAG_MARK = 0x2,
} Gc_Flag;

typedef struct Allocation
{
    void *ptr;               // pointer to the object in memory
    size_t size;             // allocated size
    Gc_Flag tag;             // gc_state tag
    struct Allocation *next; // a linked list to the next allocation to handle collisions
} Allocation;

typedef struct AllocationMap
{
    size_t nslots;       // number of slots in the map
    size_t nslots_min;   // minimum number of slots in the map
    size_t nitems;       // number of items in the map
    double lf_down;      // load factor to downsize the map
    double lf_up;        // load factor to upsize the map
    double sweep_factor; // sweep factor
    size_t sweep_limit;  // sweep limit
    Allocation **allocs; // array of pointers to allocations
} AllocationMap;

typedef struct KiGc
{
    AllocationMap *allocs;
    bool paused;
    void *bottom;
    size_t min_size;
} KiGc;

extern KiGc gc;

void gc_start(KiGc *gc, void *bottom);

void gc_start_ext(KiGc *gc, void *bottom, size_t nslots_init, size_t nslots_min, double lf_downsize, double lf_upsize,
                  double sweep_fact);

size_t gc_stop(KiGc *gc);

void *gc_malloc(KiGc *gc, size_t size);

void *gc_malloc_static(KiGc *gc, size_t size);

#endif