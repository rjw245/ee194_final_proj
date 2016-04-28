#include "sim_api.h"

void SimRoiStart_wrapper() {
    SimRoiStart();
}

void SimRoiEnd_wrapper() {
    SimRoiEnd();
}

void SimMarker_wrapper(int arg0, int arg1) {
    SimMarker(arg0,arg1);
}

