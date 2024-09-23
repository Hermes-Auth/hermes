#include "raylib.h"
#include <stdlib.h>

int main(int argc, char **argv) {
  const int width = 800;
  const int height = 100;

  InitWindow(width, height, "Hermes");
  while (!WindowShouldClose()) {
    BeginDrawing();

    ClearBackground(RAYWHITE);

    DrawText("move the ball with arrow keys", 10, 10, 20, DARKGRAY);

    EndDrawing();
  }
  CloseWindow();
  return EXIT_SUCCESS;
}
