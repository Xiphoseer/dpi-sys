#include "dpi-sys/include/context.h"

std::unique_ptr<DPI_Context> new_DPI_Context()
{
    return std::unique_ptr<DPI_Context>(new DPI_Context());
}
