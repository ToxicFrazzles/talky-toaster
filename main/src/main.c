#include "main.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "esp_chip_info.h"
#include "esp_flash.h"
#include "esp_system.h"
#include "motionsensor.h"

void app_main(void)
{
    printf("Hello world!\n");

    setup_motion_sensor();
    vTaskDelay(portMAX_DELAY);
    fflush(stdout);
    esp_restart();
}
