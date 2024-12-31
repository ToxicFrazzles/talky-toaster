#include "include/module.h"
#include "include/report.h"

report_t* allocate_report() {
    report_t* report = (report_t*)malloc(sizeof(report_t));
    report->gate_energies = (uint16_t*)malloc(16 * sizeof(uint16_t));
    return report;
}

void deallocate_report(report_t* report) {
    free(report->gate_energies);
    free(report);
}

void parse_report(uint8_t* buffer, int buffer_len, report_t* report) {
    int offset = 4;
    int len = len = buffer[4] | buffer[5] << 8;
    report->detected = buffer[6] == 1;
    report->distance = buffer[7] | buffer[8] << 8;

    if(report->gate_energies == NULL) return;
    for(int i=0; i<16; ++i){
        report->gate_energies[i] = buffer[9+2*i] | buffer[10+2*i] << 8;
    }
}
