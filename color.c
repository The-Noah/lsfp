#include <windows.h>

void enable_color(){
  HANDLE hOutput = GetStdHandle(STD_OUTPUT_HANDLE);
  DWORD dwMode;
  GetConsoleMode(hOutput, &dwMode);

  dwMode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
  if(!SetConsoleMode(hOutput, dwMode)){
    // TODO have some sort of error handling
  }
}
