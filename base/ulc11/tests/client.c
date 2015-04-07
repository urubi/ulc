#define _POSIX_SOURCE

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <assert.h>

#include <stdio.h>

typedef void ExecBlob;

ExecBlob*   ulc11_new(const char *path);
void        ulc11_free(ExecBlob *e);

uint8_t*    ulc11_data_load(ExecBlob *e, uint64_t *length);
uint8_t*    ulc11_data_free(uint8_t *data);
bool        ulc11_data_store(ExecBlob *e, uint8_t *data, uint64_t length);
bool        ulc11_data_strip(ExecBlob *e);

int main(int argc, char **argv){
    ExecBlob *e;
    uint64_t length;
    uint8_t *data;
    int out;
    
    if (argc == 2) {
        assert(e = ulc11_new(NULL));
        assert(ulc11_data_strip(e));
        ulc11_free(e);
    }
    else {
        assert(e = ulc11_new(NULL));
        assert(data = ulc11_data_load(e, &length));
        
        if (!length) out = 0;
        else out = data[0];
        
        fprintf(stdout, "%d", out);
        out += 1;
        
        assert(ulc11_data_store(e, (uint8_t[1]) {out}, 1));
        
        ulc11_data_free(data);
        ulc11_free(e);
    }
    return 0;
}
