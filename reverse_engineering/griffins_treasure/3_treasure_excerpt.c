#include <inttypes.h>
#include <stdio.h>
#include <stdbool.h>

int64_t pwd(char* user_input, unsigned int* password)
{
    int32_t i = 0;

    while (true)
    {
        if (user_input[i] == 0)
            return 1;

        printf("%c", password[i] ^ i);
        //if (user_input[i] != password[i] ^ i)
        //    break;

        i += 1;
    }

    return 0;
}

int main() {
    unsigned int v4[28];

    v4[0] = 115;
    v4[1] = 116;
    v4[2] = 114;
    v4[3] = 102;
    v4[4] = 118;
    v4[5] = 118;
    v4[6] = 99;
    v4[7] = 100;
    v4[8] = 122;
    v4[9] = 108;
    v4[10] = 126;
    v4[11] = 123;
    v4[12] = 96;
    v4[13] = 108;
    v4[14] = 103;
    v4[15] = 97;
    v4[16] = 100;
    v4[17] = 116;
    v4[18] = 106;
    v4[19] = 103;
    v4[20] = 100;
    v4[21] = 116;
    v4[22] = 101;
    v4[23] = 100;
    v4[24] = 111;
    v4[25] = 118;
    v4[26] = 104;
    v4[27] = 127;

    char *demo = "                            \0";

    pwd(demo, v4);
}