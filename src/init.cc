#include "dpi-sys/include/init.h"

int DPI_Init(DPI_Context &context)
{
    context.registry_client = new RegistryClient();
    return DPI_SUCCESS;
}
