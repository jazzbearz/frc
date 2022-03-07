#pragma once

#ifdef __cplusplus
extern "C"
{
#endif
    extern void initialize_thread_number();
    extern void uninitialize_thread_number();
    extern unsigned int *get_thread_number();
    extern void store_local_pointer(int pos, void *ptr);
    extern void *get_local_pointer(int pos);
#ifdef __cplusplus
}
#endif
