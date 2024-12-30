#include "include/motionsensor.h"

#include "esp_log.h"
#include "include/module.h"

void setup_device(void *arg) {
    uart_config_t uart_config = {
        .baud_rate = 115200,
        .data_bits = UART_DATA_8_BITS,
        .parity = UART_PARITY_DISABLE,
        .stop_bits = UART_STOP_BITS_1,
        .flow_ctrl = UART_HW_FLOWCTRL_DISABLE,
        .source_clk = UART_SCLK_DEFAULT,
    };

    int intr_alloc_flags = 0;

    esp_err_t r;
    r = uart_driver_install((uart_port_t)CONFIG_MOTION_SENSOR_UART_NUM,
                            BUF_SIZE * 2, 0, 0, NULL, intr_alloc_flags);
    if (r != ESP_OK) {
        ESP_LOGE(TAG, "Error installing UART driver");
    }
    r = uart_param_config((uart_port_t)CONFIG_MOTION_SENSOR_UART_NUM,
                          &uart_config);
    if (r != ESP_OK) {
        ESP_LOGE(TAG, "Error configuring UART");
    }
    r = uart_set_pin((uart_port_t)CONFIG_MOTION_SENSOR_UART_NUM,
                     CONFIG_MOTION_SENSOR_UART_TX, CONFIG_MOTION_SENSOR_UART_RX,
                     0, 0);
    if (r != ESP_OK) {
        ESP_LOGE(TAG, "Error setting up pins for UART");
    }
}