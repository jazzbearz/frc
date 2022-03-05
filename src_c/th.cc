#include <thread>
#include <atomic>
#include <set>
#include <mutex>
#include "th.h"

static std::atomic<uint32_t> TINC(0);
static std::set<uint32_t> TSET;
static std::mutex T_MUTEX;
thread_local uint32_t TNUM = 0;
thread_local void *TPTR_0 = NULL;
thread_local void *TPTR_1 = NULL;
thread_local void *TPTR_2 = NULL;
thread_local void *TPTR_3 = NULL;
thread_local void *TPTR_4 = NULL;
thread_local void *TPTR_5 = NULL;
thread_local void *TPTR_6 = NULL;
thread_local void *TPTR_7 = NULL;
thread_local void *TPTR_8 = NULL;
thread_local void *TPTR_9 = NULL;

void initialize_thread_number()
{
    {
        std::lock_guard<std::mutex> guard(T_MUTEX);
        if (TSET.size() > 0)
        {
            auto begin = TSET.begin();
            TNUM = *begin;
            TSET.erase(TNUM);
            return;
        }
    }
    auto tnum = TINC.fetch_add(1, std::memory_order::memory_order_relaxed);
    TNUM = tnum;
}

void uninitialize_thread_number()
{
    std::lock_guard<std::mutex> guard(T_MUTEX);
    TSET.insert(TNUM);
}

unsigned int *get_thread_number()
{
    return &TNUM;
}

void store_local_pointer_0(void *ptr)
{
    TPTR_0 = ptr;
}

void *get_local_pointer_0()
{
    return TPTR_0;
}

void store_local_pointer_1(void *ptr)
{
    TPTR_1 = ptr;
}

void *get_local_pointer_1()
{
    return TPTR_1;
}

void store_local_pointer_2(void *ptr)
{
    TPTR_2 = ptr;
}

void *get_local_pointer_2()
{
    return TPTR_2;
}

void store_local_pointer_3(void *ptr)
{
    TPTR_3 = ptr;
}

void *get_local_pointer_3()
{
    return TPTR_3;
}

void store_local_pointer_4(void *ptr)
{
    TPTR_4 = ptr;
}

void *get_local_pointer_4()
{
    return TPTR_4;
}

void store_local_pointer_5(void *ptr)
{
    TPTR_5 = ptr;
}

void *get_local_pointer_5()
{
    return TPTR_5;
}

void store_local_pointer_6(void *ptr)
{
    TPTR_6 = ptr;
}

void *get_local_pointer_6()
{
    return TPTR_6;
}

void store_local_pointer_7(void *ptr)
{
    TPTR_7 = ptr;
}

void *get_local_pointer_7()
{
    return TPTR_7;
}

void store_local_pointer_8(void *ptr)
{
    TPTR_8 = ptr;
}

void *get_local_pointer_8()
{
    return TPTR_8;
}

void store_local_pointer_9(void *ptr)
{
    TPTR_9 = ptr;
}

void *get_local_pointer_9()
{
    return TPTR_9;
}