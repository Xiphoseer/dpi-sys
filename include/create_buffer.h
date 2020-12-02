/**
 * @file create_buffer.h
 * @author cbinnig, lthostrup, tziegler
 * @date 2018-08-17
 */

#pragma once

#include <string>
#include "context.h"
#include "err_codes.h"
#include "../utils/Config.h"

/**
 * @brief DPI_Create_buffer initializes a buffer and needs to be called before any buffer operations
 * 
 * @param name - name of the buffer
 * @param node_id - NodeID of the node where the buffer should be located
 * @param context - DPI_Context
 * @param segmentsPerAppender - The number of segments each appender get allocated in the buffer
 * @param participatingAppenders - The number of appenders that are going to append to the buffer
 * @return int - error code
 */
int DPI_Create_buffer(
    const std::string &name,
    NodeID node_id,
    DPI_Context &context,
    size_t segmentsPerAppender = 1,
    size_t participatingAppenders = 1);
