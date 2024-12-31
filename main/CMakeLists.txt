idf_component_register(SRCS "src/main.c"
                       PRIV_REQUIRES spi_flash motion_sensor
                       REQUIRES "" 
                       INCLUDE_DIRS "include")
