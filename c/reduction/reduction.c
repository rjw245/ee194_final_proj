#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <pthread.h>

typedef struct thread_data
{
    int64_t *input;
    int64_t psum;
    unsigned start;
    unsigned end;
} thread_data;

void *sum_array(void *cl)
{
    thread_data *td = (thread_data *)cl;

    for (unsigned i = td->start; i < td->end; ++i) {
        td->psum += td->input[i]; 
    }

    pthread_exit(NULL);
}

int main(void)
{
    const unsigned inputsize = 1e9;
    const unsigned nthreads = 8;

    const unsigned span = inputsize / nthreads;

    unsigned sum = 0;

    pthread_t threads[nthreads];
    thread_data thread_data[nthreads];

    int64_t *input = (int64_t *)malloc(inputsize * sizeof(int64_t));
    for (unsigned i = 0; i < inputsize; ++i) { input[i] = 1; }

    for (unsigned i = 0; i < nthreads; ++i)
    {
        unsigned start = i * span;
        unsigned end = (i == nthreads - 1) ? inputsize : start + span;

        thread_data[i].input = input;
        thread_data[i].psum  = 0;
        thread_data[i].start = start;
        thread_data[i].end   = end;

        if (pthread_create(&threads[i], NULL, sum_array, &thread_data[i]) == 0)
        {
            printf("Spawned thread\n");
        }
    }

    for (unsigned i = 0; i < nthreads; ++i)
    {
        pthread_join(threads[i], NULL);
        sum += thread_data[i].psum;
    }

    printf("%d\n", sum);

    return 0;
}
