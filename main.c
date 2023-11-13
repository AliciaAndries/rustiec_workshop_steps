#ifndef _GNU_SOURCE
    #define _GNU_SOURCE
#endif

#include "config.h"
#include "connmgr.h"
#include "datamgr.h"
#include "sbuffer.h"
#include "sensor_db.h"

#include <assert.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <wait.h>
#include <inttypes.h>

static int print_usage() {
    printf("Usage: <command> <port number> \n");
    return -1;
}

static void datamgr_callback(void* unused, const sensor_data_t* data) {
    (void) unused;
    // printf("\tDatamgr received: sensor id = %" PRIu16 " - temperature = %g - timestamp = %ld\n", data->id, data->value, data->ts);
    datamgr_process_reading(data);
}

static void* datamgr_run(void* buffer) {
    datamgr_init();
    sbuffer_listen(buffer, datamgr_callback, NULL);
    datamgr_free();
    return NULL;
}

static void storagemgr_callback(void* db, const sensor_data_t* data) {
    assert(db != NULL);
    // printf("\tStoragemgr received: sensor id = %" PRIu16 " - temperature = %g - timestamp = %ld\n", data->id, data->value, data->ts);
    storagemgr_insert_sensor(db, data->id, data->value, data->ts);
}

static void* storagemgr_run(void* buffer) {
    DBCONN* db = storagemgr_init_connection(1);
    assert(db != NULL);
    sbuffer_listen(buffer, storagemgr_callback, db);
    storagemgr_disconnect(db);
    return NULL;
}

int main(int argc, char* argv[]) {
    if (argc != 2)
        return print_usage();
    char* strport = argv[1];
    char* error_char = NULL;
    int port_number = strtol(strport, &error_char, 10);
    if (strport[0] == '\0' || error_char[0] != '\0')
        return print_usage();
    sbuffer_t* buffer = sbuffer_create();

    pthread_t datamgr_thread;
    ASSERT_ELSE_PERROR(pthread_create(&datamgr_thread, NULL, datamgr_run, buffer) == 0);

    pthread_t storagemgr_thread;
    ASSERT_ELSE_PERROR(pthread_create(&storagemgr_thread, NULL, storagemgr_run, buffer) == 0);

    // wait for both readers to properly initialize, to avoid unlikely race condition
    while (sbuffer_num_readers(buffer) != 2);
    printf("both readers are present\n");
    // main server loop
    connmgr_listen(port_number, buffer);


    sbuffer_close(buffer);
    

    // now just for fun, we actively wait for the readers to finish, 
    // logging the remaining size of the buffer every second
    while (sbuffer_num_readers(buffer)>0) {
        printf("Remaining sbuffer size: %lu\n", sbuffer_size(buffer));
        sleep(1);
    }

    pthread_join(datamgr_thread, NULL);
    pthread_join(storagemgr_thread, NULL);
    // due to the pthread joins, the readers are guaranteed to be gone now
    sbuffer_destroy(buffer);
    printf("Shut down\n");
    return 0;
}