#include "mb_report.h"
#include <stdio.h>

#define MB_REPORT_ITERATOR(returnVal, name, ...) \
do { \
    char name_buf[128]; \
    sprintf(name_buf, "%s: %s\n", #name, name? "true" : "false"); \
    size_t name_len = strlen(name_buf); \
    if (mb_report_len + name_len > mb_report_capacity - 1) { \
        mb_report = (char*)realloc(mb_report, mb_report_capacity << 1); \
    } \
    strcat(mb_report, name_buf); \
    mb_report_len += name_len; \
} while(0);

static char* mb_report = (char*)0;
const char* mbReport()
{
    if(mb_report)
        return mb_report;
    
    mb_report = (char*)malloc(0x4000);
    size_t mb_report_len = 0;
    size_t mb_report_capacity = 0;
    memset(mb_report, 0, 0x4000);

    MB_FOR_EACH_DEFINE_FUNCTION(
        MB_REPORT_ITERATOR, 
        MB_REPORT_ITERATOR,
        MB_REPORT_ITERATOR,
        MB_REPORT_ITERATOR, 
        MB_REPORT_ITERATOR, 
        MB_REPORT_ITERATOR,
        MB_REPORT_ITERATOR,
        MB_REPORT_ITERATOR,
        MB_REPORT_ITERATOR,
        MB_REPORT_ITERATOR,
        MB_REPORT_ITERATOR,
        MB_REPORT_ITERATOR
    );
    return mb_report;
}
void mbReportFree()
{
    if (!mb_report)
        return;
    free(mb_report);
    mb_report = (char*)0;
}