#pragma once

#include "driver/uart.h"
#include "driver/gpio.h"


#define BUF_SIZE 1024

// const uart_port_t uart_num = CONFIG_MOTION_SENSOR_UART_NUM;

esp_err_t setup_motion_sensor();
