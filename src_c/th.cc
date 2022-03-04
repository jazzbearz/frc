#include <thread>
#include <atomic>
#include <set>
#include <mutex>
#include "th.h"

static std::atomic<uint32_t> TINC(0);
static std::set<uint32_t> TSET;
static std::mutex T_MUTEX;
thread_local uint32_t TNUM = 0;
thread_local void *TPTR = NULL;

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
void store_local_pointer(void *ptr)
{
    TPTR = ptr;
}

void *get_local_pointer()
{
    return TPTR;
}