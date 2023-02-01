int main() {
    volatile char* uart_out = (char*) 0x10000000;

    char* string = "Hello World";
    while(*string){
        *uart_out = *string++;
    }
    return 0;
}