#pragma once
#include "esp_task.h"

static void init_timer(TickType_t *timer){
    *timer = xTaskGetTickCount();
}

static bool get_timeout(TickType_t *timer, int period_ms){
    return (period_ms/portTICK_PERIOD_MS) + *timer <= xTaskGetTickCount();
}