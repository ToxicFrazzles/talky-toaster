#pragma once

#include "driver/uart.h"
#include "driver/gpio.h"


#define BUF_SIZE 1024

// const uart_port_t uart_num = CONFIG_MOTION_SENSOR_UART_NUM;

void setup_device(void *arg);
