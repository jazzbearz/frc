#include <thread>
#include <atomic>
#include <set>
#include <mutex>
#include "th.h"

#define TLOCAL_ARRAY_LEN 1024

static std::atomic<uint32_t> TINC(0);
static std::set<uint32_t> TSET;
static std::mutex T_MUTEX;
thread_local bool is_initialized = false;
thread_local uint32_t TNUM = 0;
thread_local void *TPTR_ARRAY[TLOCAL_ARRAY_LEN];

void initialize_thread_number()
{
    if (!is_initialized)
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
        is_initialized = true;
    }
}

void uninitialize_thread_number()
{
    if (is_initialized)
    {
        std::lock_guard<std::mutex> guard(T_MUTEX);
        TSET.insert(TNUM);
    }
}

unsigned int *get_thread_number()
{
    return &TNUM;
}

void store_local_pointer(int pos, void *ptr)
{
    TPTR_ARRAY[pos] = ptr;
}

void *get_local_pointer(int pos)
{
    return TPTR_ARRAY[pos];
}