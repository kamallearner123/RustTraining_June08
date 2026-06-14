// vehicle.h

#include <stdint.h>

typedef struct {
    uint32_t id;
    float speed;
} Vehicle;

int process_vehicle(Vehicle* v);
