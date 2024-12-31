#pragma once

#include <stdio.h>
#include <inttypes.h>

static const char* TAG = "MotionSense";

static const uint8_t* COMMAND_HEADER = (uint8_t[]){
    0xFD,
    0xFC,
    0xFB,
    0xFA
};

static const uint8_t* REPORT_HEADER = (uint8_t[]){
    0xF4,
    0xF3,
    0xF2,
    0xF1
};

static const uint8_t* COMMAND_FOOTER = (uint8_t[]){
    0x04,
    0x03,
    0x02,
    0x01
};

static const uint8_t* REPORT_FOOTER = (uint8_t[]){
    0x08,
    0x07,
    0x06,
    0x05
};


typedef enum {
    STATE_UNKNOWN,
    STATE_CONFIGURE,
    STATE_RUNNING,
} motion_sensor_state_t;