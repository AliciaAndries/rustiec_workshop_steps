#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Vec_____c_void Vec_____c_void;

/**
 * # Safety
 * The pointer passed to vec must originate from vector_create.
 */
struct Vec_____c_void *vector_create(size_t size);

void vector_add(struct Vec_____c_void *vec, void *element);

void vector_remove_at_index(struct Vec_____c_void *vec, size_t index);

void *vector_at(struct Vec_____c_void *vec, uintptr_t index);

void vector_set(struct Vec_____c_void *vec, size_t index, void *new_value);

void *vector_find(struct Vec_____c_void *vec,
                  void *element_to_match,
                  bool (*match_elements)(void*, void*));

size_t vector_size(struct Vec_____c_void *vec);

void vector_destroy(struct Vec_____c_void *vec);
