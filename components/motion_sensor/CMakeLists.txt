idf_component_register(SRCS "motionsensor.c" "report_parser.c"
                       PRIV_REQUIRES ""
                       REQUIRES esp_driver_uart esp_driver_gpio
                       INCLUDE_DIRS "include")
