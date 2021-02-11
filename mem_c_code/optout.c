#include <stdio.h>

void encrypts(char *key)
{
    while (key != '\0')
    {
        key = (char)(key + 10) % 128;
    }
    printf("%s\n", key);
}

int main()
{
    char key[16];
    strcpy(key, "whatever");
    encrypts(key);
    memset(key, 0, 16);
}