#pragma once

#include "config.h"
#include <sys/types.h>

typedef struct sbuffer sbuffer_t;

/**
 * Allocate and initialize a new shared buffer
 */
sbuffer_t* sbuffer_create();

/**
 * Clean up & f all allocated resources
 */
void sbuffer_destroy(sbuffer_t* buffer);

/**
 * Inserts the sensor data in 'data' at the start of 'buffer' (at the 'head')
 * \param buffer a pointer to the buffer that is used
 * \param data a pointer to sensor_data_t data, that will be _copied_ into the buffer
 */
void sbuffer_insert_first(sbuffer_t* buffer, sensor_data_t const* data);

u_char sbuffer_num_readers(sbuffer_t* buffer);

size_t sbuffer_size(sbuffer_t* buffer);

void sbuffer_listen(sbuffer_t* buffer, void (*process_reading)(void*, const sensor_data_t*), void* arg);

/**
 * Closes the buffer. This signifies that no more data will be inserted.
 */
void sbuffer_close(sbuffer_t* buffer);
