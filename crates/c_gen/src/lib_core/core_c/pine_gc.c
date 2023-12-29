#include "pine_gc.h"
#include <setjmp.h>
#include <string.h>

#undef LOGLEVEL
#define LOGLEVEL LOGLEVEL_WARNING

KiGc gc;

#define PRIMES_COUNT 30

static const size_t gc_primes[PRIMES_COUNT] = {
    0,       1,       5,       11,      23,       53,       101,      197,       389,       683,
    1259,    2417,    4733,    9371,    18617,    37097,    74093,    148073,    296099,    592019,
    1100009, 2200013, 4400021, 8800019, 17600039, 35200091, 70400203, 140800427, 281600857, 563201731};

static size_t gc_ideal_size(size_t size)
{
    for (size_t i = 0; i < PRIMES_COUNT; ++i)
    {
        if (gc_primes[i] > size)
        {
            return gc_primes[i];
        }
    }
    return gc_primes[PRIMES_COUNT - 1];
}

static Allocation *gc_alloc_new(void *ptr, size_t size)
{
    Allocation *a = (Allocation *)malloc(sizeof(Allocation));
    a->ptr = ptr;
    a->size = size;
    a->tag = GC_FLAG_NONE;
    a->next = NULL;
    return a;
}

static void gc_alloc_delete(Allocation *a)
{
    free(a);
}

static size_t gc_calc_sweep_limit(size_t nitems, size_t nslots, double sweep_factor)
{
    return nitems + sweep_factor * (nslots - nitems);
}

static AllocationMap *gc_alloc_map_new(size_t nslots_init, size_t nslots_min, double sweep_factor,
                                       double load_factor_down, double load_factor_up)
{

    nslots_min = gc_ideal_size(nslots_min);
    if (nslots_init < nslots_min)
    {
        nslots_init = nslots_min;
    }
    else
    {
        nslots_init = gc_ideal_size(nslots_init);
    }

    AllocationMap *am = (AllocationMap *)malloc(sizeof(AllocationMap));
    am->nslots = nslots_init;
    am->nslots_min = nslots_min;
    am->sweep_factor = sweep_factor;
    am->lf_down = load_factor_down;
    am->lf_up = load_factor_up;
    am->allocs = (Allocation **)calloc(am->nslots, sizeof(Allocation *));
    am->nitems = 0;
    am->sweep_limit = gc_calc_sweep_limit(am->nitems, am->nslots, am->sweep_factor);
    return am;
}

static void *malloc_wrapper(size_t size)
{
    return malloc(size);
}

static size_t gc_hash(void *ptr)
{
    uintptr_t ad = (uintptr_t)ptr;
    return (size_t)((13 * ad) ^ (ad >> 15));
}

static double gc_alloc_map_load_factor(AllocationMap *am)
{
    return (double)am->nitems / (double)am->nslots;
}

static void gc_alloc_map_resize(AllocationMap *am, size_t new_capacity)
{
    // Create a new array with the new capacity
    Allocation **resized_allocs = calloc(new_capacity, sizeof(Allocation *));

    // Insert all the allocations into the new array
    // Rehash the allocation and collision chain
    for (size_t i = 0; i < am->nslots; ++i)
    {
        Allocation *alloc = am->allocs[i];
        while (alloc)
        {
            Allocation *next_alloc = alloc->next;
            size_t new_index = gc_hash(alloc->ptr) % new_capacity;
            alloc->next = resized_allocs[new_index];
            resized_allocs[new_index] = alloc;
            alloc = next_alloc;
        }
    }
    free(am->allocs);
    am->nslots = new_capacity;
    am->allocs = resized_allocs;
    am->sweep_limit = gc_calc_sweep_limit(am->nitems, am->nslots, am->sweep_factor);
}

static bool gc_resize_more(AllocationMap *am)
{
    size_t new_size = gc_ideal_size(am->nitems);
    size_t old_size = am->nslots;
    if (new_size > old_size)
    {
        LOG_INFO("Resizing allocation map (cap=%ld, siz=%ld) -> (cap=%ld)", old_size, am->nitems, new_size);
        gc_alloc_map_resize(am, new_size);
        return true;
    }
    return false;
}

static bool gc_resize_less(AllocationMap *am)
{
    size_t new_size = gc_ideal_size(am->nitems);
    size_t old_size = am->nslots;

    if (new_size < am->nslots_min)
    {
        new_size = am->nslots_min;
    }

    if (new_size < old_size)
    {
        LOG_INFO("Resizing allocation map (cap=%ld, siz=%ld) -> (cap=%ld)", old_size, am->nitems, new_size);
        gc_alloc_map_resize(am, new_size);
        return true;
    }
    return false;
}

static bool gc_alloc_map_resize_to_fit(AllocationMap *am)
{
    double load_factor = gc_alloc_map_load_factor(am);
    if (load_factor > am->lf_up)
    {
        return gc_resize_more(am);
    }
    else if (load_factor < am->lf_down)
    {
        return gc_resize_less(am);
    }
    return false;
}

static Allocation *gc_alloc_map_get(AllocationMap *am, void *ptr)
{
    size_t index = gc_hash(ptr) % am->nslots;
    Allocation *cur = am->allocs[index];
    while (cur)
    {
        if (cur->ptr == ptr)
        {
            return cur;
        }
        cur = cur->next;
    }
    return NULL;
}

static Allocation *gc_alloc_map_insert(AllocationMap *am, void *ptr, size_t size)
{
    size_t index = gc_hash(ptr) % am->nslots;
    Allocation *alloc = gc_alloc_new(ptr, size);
    Allocation *cur = am->allocs[index];
    Allocation *prev = NULL;

    while (cur != NULL)
    {
        if (cur->ptr == ptr)
        {
            // found it
            alloc->next = cur->next;
            if (!prev)
            {
                // position 0
                am->allocs[index] = alloc;
            }
            else
            {
                // in the list
                prev->next = alloc;
            }
            gc_alloc_delete(cur);
            LOG_DEBUG("AllocationMap Upsert at ix=%ld", index);
            return alloc;
        }
        prev = cur;
        cur = cur->next;
    }

    /* Insert at the front of the separate chaining list */
    cur = am->allocs[index];
    alloc->next = cur;
    am->allocs[index] = alloc;
    am->nitems++;
    LOG_DEBUG("AllocationMap insert at ix=%ld", index);
    void *p = alloc->ptr;
    if (gc_alloc_map_resize_to_fit(am))
    {
        alloc = gc_alloc_map_get(am, p);
    }
    return alloc;
}

void gc_mark_alloc(KiGc *gc, void *ptr)
{
    Allocation *alloc = gc_alloc_map_get(gc->allocs, ptr);
    // Mark if alloc exists and is not tagged already
    if (alloc && !(alloc->tag & GC_FLAG_MARK))
    {
        alloc->tag |= GC_FLAG_MARK;
        // Mark all pointers in the allocation
        for (char *p = (char *)alloc->ptr; p <= (char *)alloc->ptr + alloc->size - sizeof(char *); ++p)
        {
            gc_mark_alloc(gc, *(void **)p);
        }
    }
}

static void gc_allocation_map_remove(AllocationMap *am, void *ptr, bool allow_resize)
{
    // ignores unknown keys
    size_t index = gc_hash(ptr) % am->nslots;
    Allocation *cur = am->allocs[index];
    Allocation *prev = NULL;
    Allocation *next;
    while (cur != NULL)
    {
        next = cur->next;
        if (cur->ptr == ptr)
        {
            // found it
            if (!prev)
            {
                // first item in list
                am->allocs[index] = cur->next;
            }
            else
            {
                // not the first item in the list
                prev->next = cur->next;
            }
            gc_alloc_delete(cur);
            am->nitems--;
        }
        else
        {
            // move on
            prev = cur;
        }
        cur = next;
    }
    if (allow_resize)
    {
        gc_alloc_map_resize_to_fit(am);
    }
}

/**
 * Marks objects in the stack as reachable in the garbage collector.
 *
 * This function marks objects in the stack as reachable by iterating through
 * the stack memory between the bottom and top pointers.
 */
void gc_mark_stack(KiGc *gc)
{
    // this variable is placed at the top of the stack
    char stk;
    char *bot = gc->bottom;
    char *top = &stk;

    if (bot == top)
    {
        return;
    }

    if (bot > top)
    {
        for (char *p = top; p >= bot; p = ((char *)p) - sizeof(char *))
        {
            gc_mark_alloc(gc, *((void **)p));
        }
    }

    if (bot < top)
    {
        for (char *p = top; p <= bot; p = ((char *)p) + sizeof(char *))
        {
            gc_mark_alloc(gc, *((void **)p));
        }
    }
}

/**
 * Marks root allocations in the garbage collector.
 *
 * Root allocations are typically global variables or explicitly marked objects
 * that serve as entry points for the garbage collector to trace live objects.
 */
void gc_mark_roots(KiGc *gc)
{
    for (size_t i = 0; i < gc->allocs->nslots; ++i)
    {
        Allocation *chunk = gc->allocs->allocs[i];
        while (chunk)
        {
            if (chunk->tag & GC_FLAG_ROOT)
            {
                gc_mark_alloc(gc, chunk->ptr);
            }
            chunk = chunk->next;
        }
    }
}

/**
 * Marks reachable objects in the garbage collector.
 *
 * Initiates the mark phase of the garbage collection process. It marks
 * objects that are reachable from root allocations, as well as objects
 * on the stack.
 */
void gc_mark(KiGc *gc)
{
    gc_mark_roots(gc);
    // Dump registers to stack
    void (*volatile _mark_stack)(KiGc *) = gc_mark_stack;
    jmp_buf env;
    memset(&env, 0, sizeof(jmp_buf));
    setjmp(env);
    _mark_stack(gc);
}

/**
 * Sweeps and frees memory for unreferenced objects in the garbage collector.
 *
 * This function performs the sweep phase of the garbage collection process.
 * It iterates through the allocation map, identifies unreferenced (dead) objects,
 * frees their memory, and updates the allocation map.
 *
 * @return The total number of bytes freed during the sweep phase.
 */
size_t gc_sweep(KiGc *gc)
{
    size_t freed_bytes = 0, freed_allocs = 0;
    for (size_t i = 0; i < gc->allocs->nslots; ++i)
    {
        Allocation *chunk = gc->allocs->allocs[i];
        Allocation *next = NULL;
        // iterate over all allocations in the slot
        while (chunk != NULL)
        {
            if (chunk->tag & GC_FLAG_MARK)
            {
                // still referenced, unmark it
                chunk->tag &= ~GC_FLAG_MARK;
                chunk = chunk->next;
            }
            else
            {
                // not referenced, free it
                freed_bytes += chunk->size;
                freed_allocs++;
                free(chunk->ptr);
                // remove from map
                next = chunk->next;
                gc_allocation_map_remove(gc->allocs, chunk->ptr, false);
                chunk = next;
            }
        }
    }
    LOG_INFO("GC sweep: %lu allocations (%lu bytes)", freed_allocs, freed_bytes);
    gc_alloc_map_resize_to_fit(gc->allocs);
    return freed_bytes;
}

/**
 * Runs the garbage collector.
 *
 * This function runs the garbage collector. It performs the mark and sweep
 * phases of the garbage collection process.
 */
void gc_run(KiGc *gc)
{
    gc_mark(gc);
    gc_sweep(gc);
}

/**
 * Allocates memory using the garbage collector, performing garbage collection if needed.
 *
 * If allocation fails, the program exits with an error code.
 * @param size The size of the memory to be allocated.
 * @return A pointer to the allocated memory.
 */
static void *gc_allocate(KiGc *gc, size_t size)
{
    if (gc->allocs->nitems > gc->allocs->sweep_limit && !gc->paused)
    {
        gc_run(gc);
    }

    void *ptr = malloc_wrapper(size);
    if (ptr)
    {
        Allocation *a = gc_alloc_map_insert(gc->allocs, ptr, size);
        if (a)
        {
            LOG_DEBUG("Allocation inserted", NULL);
            ptr = a->ptr;
            return ptr;
        }
        else
        {
            LOG_CRITICAL("Allocation not inserted", NULL);
            free(ptr);
            ptr = NULL;
        }
    }
    LOG_CRITICAL("Allocation failed", NULL);
    exit(42);
}

static void gc_make_root(KiGc *gc, void *ptr)
{
    Allocation *alloc = gc_alloc_map_get(gc->allocs, ptr);
    if (alloc)
    {
        alloc->tag |= GC_FLAG_ROOT;
    }
}

void *gc_malloc(KiGc *gc, size_t size)
{
    return gc_allocate(gc, size);
}

void *gc_malloc_static(KiGc *gc, size_t size)
{
    void *ptr = gc_malloc(gc, size);
    gc_make_root(gc, ptr);
    return ptr;
}

void gc_start_ext(KiGc *gc, void *bottom, size_t nslots_init, size_t nslots_min, double lf_downsize, double lf_upsize,
                  double sweep_fact)
{
    sweep_fact = sweep_fact;
    gc->paused = false;
    gc->bottom = bottom;
    gc->allocs = gc_alloc_map_new(nslots_init, nslots_min, sweep_fact, lf_downsize, lf_upsize);
}

void gc_start(KiGc *gc, void *bottom)
{
    gc_start_ext(gc, bottom, 1024, 1024, 0.2, 0.8, 0.5);
}

size_t gc_stop(KiGc *gc)
{
    return 0;
}