#include <pthread.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

#include "sim_api.h"

#define SIZE 512   // Size by SIZE matrices
uint64_t num_thrd;   // number of threads

uint64_t A[SIZE][SIZE], B[SIZE][SIZE], C[SIZE][SIZE];

// initialize a matrix
void init_matrix(uint64_t m[SIZE][SIZE])
{
  uint64_t i, j, val = 0;
  for (i = 0; i < SIZE; i++)
    for (j = 0; j < SIZE; j++)
      m[i][j] = val++;
}

void print_matrix(uint64_t m[SIZE][SIZE])
{
  uint64_t i, j;
  for (i = 0; i < SIZE; i++) {
    printf("\n\t| ");
    for (j = 0; j < SIZE; j++)
      printf("%2d ", m[i][j]);
    printf("|");
  }
}

// thread function: taking "slice" as its argument
void* multiply(void* slice)
{
  uint64_t s = (uint64_t)slice;   // retrive the slice info
  uint64_t from = (s * SIZE)/num_thrd; // note that this 'slicing' works fine
  uint64_t to = ((s+1) * SIZE)/num_thrd; // even if SIZE is not divisible by num_thrd
  uint64_t i,j,k;

  printf("computing slice %d (from row %d to %d)\n", s, from, to-1);
  for (i = from; i < to; i++)
  {
    for (j = 0; j < SIZE; j++)
    {
      C[i][j] = 0;
      for ( k = 0; k < SIZE; k++)
 C[i][j] += A[i][k]*B[k][j];
    }
  }
  printf("finished slice %d\n", s);
  return 0;
}

uint64_t main(uint64_t argc, char* argv[])
{
  pthread_t* thread;  // pointer to a group of threads
  uint64_t i;

  if (argc!=2)
  {
    printf("Usage: %s number_of_threads\n",argv[0]);
    exit(-1);
  }

  num_thrd = atoi(argv[1]);
  init_matrix(A);
  init_matrix(B);
  thread = (pthread_t*) malloc(num_thrd*sizeof(pthread_t));

  SimRoiStart();
  // this for loop not entered if threadd number is specified as 1
  for (i = 1; i < num_thrd; i++)
  {
    // creates each thread working on its own slice of i
    if (pthread_create (&thread[i], NULL, multiply, (void*)i) != 0 )
    {
      perror("Can't create thread");
      free(thread);
      exit(-1);
    }
  }

  // main thread works on slice 0
  // so everybody is busy
  // main thread does everything if threadd number is specified as 1
  multiply(0);

  // main thead waiting for other thread to complete
  for (i = 1; i < num_thrd; i++)
 pthread_join (thread[i], NULL);

  SimRoiEnd();

  printf("\n\n");
  print_matrix(A);
  printf("\n\n\t       * \n");
  print_matrix(B);
  printf("\n\n\t       = \n");
  print_matrix(C);
  printf("\n\n");

  free(thread);

  return 0;

}
