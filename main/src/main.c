#include "main.h"
#include "esp_log.h"
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
    uint16_t last_distance = 0;
    bool last_detected = false;
    while(1){
        bool detected = motion_sensor_get_detected();
        if(last_detected != detected && detected){
            ESP_LOGI("MAIN", "DETECTED A PERSON!");
        }else if (last_detected != detected && !detected)
        {
            ESP_LOGI("MAIN", "PERSON GONE!");
        }
        last_detected = detected;
        if(!detected){
            vTaskDelay(500/portTICK_PERIOD_MS);
            continue;
        }
        
        uint16_t distance = motion_sensor_get_distance();
        if(distance == last_distance){
            vTaskDelay(500/portTICK_PERIOD_MS);
            continue;
        }
        ESP_LOGI("MAIN", "Distance: % 4dcm", distance);
        last_distance = distance;
        vTaskDelay(500/portTICK_PERIOD_MS);
    }
    vTaskDelay(portMAX_DELAY);
    fflush(stdout);
    esp_restart();
}
