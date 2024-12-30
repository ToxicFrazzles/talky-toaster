idf_component_register(SRCS "src/hello_world_main.c"
                       PRIV_REQUIRES spi_flash motion_sensor
                       REQUIRES "" 
                       INCLUDE_DIRS "include")
