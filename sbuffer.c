#include "sbuffer.h"

#include "config.h"
#include "lib/vector.h"

#include <assert.h>
#include <bits/pthreadtypes.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>

#include <linux/futex.h>
#include <sys/syscall.h>
#include <unistd.h>
#include <errno.h>
#include <limits.h>
#include <string.h>

#include <signal.h>

#define BUFFER_SIZE 10
#define MAX_READERS 2

static sensor_data_t* sbuffer_element_create(const sensor_data_t* data) {
    sensor_data_t* element = malloc(sizeof(*data));
    *element = *data;
    return element;
}

struct sbuffer {
    uint num_readers;
    bool producer_active;
    int producer_index;
    int consumers_index[MAX_READERS];
    bool index_read[BUFFER_SIZE];
    void * vector;
    pthread_mutex_t indexes_mutex;
    pthread_cond_t cond;
};

sbuffer_t* sbuffer_create() {
    sbuffer_t* buffer = malloc(sizeof(sbuffer_t));
    assert(buffer != NULL);

    buffer->num_readers = 0;
    buffer->producer_index = 0;
    buffer->producer_active = 1;
    memset(buffer->index_read , 0, BUFFER_SIZE * sizeof(bool));
    pthread_cond_init(&buffer->cond, NULL);
    pthread_mutexattr_t attr;
    pthread_mutexattr_init(&attr);
    pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_RECURSIVE);
    pthread_mutex_init(&buffer->indexes_mutex, &attr);
    memset(buffer->consumers_index, -1, MAX_READERS*sizeof(int));
    
    buffer->vector = vector_create(BUFFER_SIZE);

    return buffer;
}

void sbuffer_destroy(sbuffer_t* buffer) {
    assert(buffer);
    buffer->num_readers = 0;
    vector_destroy(buffer->vector);
    free(buffer);
}

u_char sbuffer_num_readers(sbuffer_t* buffer) {
    assert(buffer);
    pthread_mutex_lock(&buffer->indexes_mutex);
    uint ret = buffer->num_readers;
    pthread_mutex_unlock(&buffer->indexes_mutex);
    return ret;
}

size_t sbuffer_size(sbuffer_t* buffer) {
    assert(buffer);
    int size = 0;
    pthread_mutex_lock(&buffer->indexes_mutex);
    for(uint i = 0; i < MAX_READERS; i++){
        int index = buffer->consumers_index[i];
        if(index != -1){
            int temp;
            if(buffer->producer_index - index < 0){
                temp = buffer->producer_index + vector_size(buffer->vector) - index;
            } else {
                temp = buffer->producer_index - index;
            }
            
            if(temp > size)
                size = temp;
        }
    }
    pthread_mutex_unlock(&buffer->indexes_mutex);
    return size;
}

/*
    Only one producer exists, so cannot be executed multiple times at once.
    Outside of initialization and destruction, buffer->head is only read and modified here.
    It does not require synchronization or atomicity (it could be a pointer outside of the buffer)
*/
void sbuffer_insert_first(sbuffer_t* buffer, const sensor_data_t* data) {
    assert(buffer && data);
    assert(buffer->vector);
    pthread_mutex_lock(&buffer->indexes_mutex);
    // check that no readers are in the way
    for(uint i = 0; i < MAX_READERS; i++) {
        fflush(stdout);
        while(buffer->consumers_index[i] == buffer->producer_index){
            // TODO: need to wait, after the wait check for others, there can only be more in the unchecked 
                // ones because no one can pass the producer
            pthread_cond_wait(&buffer->cond, &buffer->indexes_mutex);
        }
    }

    assert(buffer->producer_index < (int)vector_size(buffer->vector));

    /* void * old_sensor_data = vector_at(buffer->vector, buffer->producer_index);
    assert(old_sensor_data == NULL);
    if(old_sensor_data){
        free(old_sensor_data);
        old_sensor_data = NULL;
    } */    
    vector_set(buffer->vector, buffer->producer_index, sbuffer_element_create(data));
    
    int new_producer_index;
    if(buffer->producer_index + 1 == (int)vector_size(buffer->vector)){
        new_producer_index = 0;
        printf("producer_index is set to 0\n");
        fflush(stdout);
    } else {
        new_producer_index = buffer->producer_index + 1;
    }
    buffer->producer_index = new_producer_index;
    pthread_cond_broadcast(&buffer->cond);
    pthread_mutex_unlock(&buffer->indexes_mutex);
}

void sbuffer_listen(sbuffer_t* buffer, void (*process_reading)(void*, const sensor_data_t*), void* arg) {
    assert(buffer);
    assert(buffer->vector);

    // as soon as this increment hits 2, the connmgr will launch and accept requests
    // TODO: this could be a race condition
    pthread_mutex_lock(&buffer->indexes_mutex);
    buffer->num_readers++;
    int consumer_id = buffer->num_readers - 1;
    int new_consumer_index = -1;

    while (true) {
        
        // the next index of the consumer
        if(new_consumer_index + 1 == (int)vector_size(buffer->vector)){
            new_consumer_index = 0;
        } else {
            new_consumer_index = new_consumer_index + 1;
        }
        // check if there is new data to be read
        while (new_consumer_index == buffer->producer_index && buffer->producer_active)
            pthread_cond_wait(&buffer->cond, &buffer->indexes_mutex);
        
        
        // check if the buffer is closed and empty
        if (new_consumer_index == buffer->producer_index && !buffer->producer_active) {
            buffer->num_readers --;
            pthread_mutex_unlock(&buffer->indexes_mutex);
            return;
        }
        assert(buffer->producer_index != new_consumer_index);

        // New data to be read
        sensor_data_t* new_data = (sensor_data_t*) vector_at(buffer->vector, new_consumer_index);
        assert(consumer_id < MAX_READERS);
        assert(consumer_id >= 0);
        buffer->consumers_index[consumer_id] = new_consumer_index;

        bool read_by_all = buffer->index_read[new_consumer_index];
        buffer->index_read[new_consumer_index] = true;
        

        sensor_data_t new_data_copy = *new_data;
        pthread_mutex_unlock(&buffer->indexes_mutex);
        process_reading(arg, &new_data_copy);
        if(read_by_all) {
            printf("read by all: %d\n", new_consumer_index);
            free(new_data);
            buffer->index_read[new_consumer_index] = false;
            new_data = NULL;
        }
        pthread_mutex_lock(&buffer->indexes_mutex);

    }
    assert(false);
}

// only ever called by producer thread
void sbuffer_close(sbuffer_t* buffer) {
    assert(buffer);
    pthread_mutex_lock(&buffer->indexes_mutex);
    buffer->producer_active = false;
    // TODO: wake up readers
    pthread_cond_broadcast(&buffer->cond);
    pthread_mutex_unlock(&buffer->indexes_mutex);
}
