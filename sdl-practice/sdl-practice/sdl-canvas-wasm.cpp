#include <stdio.h>
#include <stdbool.h>
#include <SDL.h>
#include <SDL_image.h>
//#include <emscripten.h>

#define WINDOW_WIDTH    640
#define WINDOW_HEIGHT   480

const unsigned int size = 64;

SDL_Window* window = NULL;
SDL_Renderer* renderer = NULL;

SDL_Point velocity = { 0, 0 };
SDL_Rect sprite = { 0, 0, size, size };
SDL_Texture* texture = NULL;

bool init() {
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        return false;
    }

    SDL_CreateWindowAndRenderer(WINDOW_WIDTH, WINDOW_HEIGHT, 0, &window, &renderer);
    if (window == NULL | renderer == NULL) {
        return false;
    }

    return true;
}

void load_textures() {
    SDL_Surface* surface = IMG_Load("assets/texture.png");
    if (!surface) {
        printf("%s\n", IMG_GetError());
    }
    SDL_SetColorKey(surface, SDL_TRUE, SDL_MapRGB(surface->format, 0x75, 0xFF, 0xFF));
    texture = SDL_CreateTextureFromSurface(renderer, surface);
    SDL_FreeSurface(surface);
}

void process_event(SDL_Event* event) {
    SDL_Keycode key = event->key.keysym.sym;

    if (event->key.type == SDL_KEYDOWN) {
        if (key == SDLK_LEFT || key == SDLK_RIGHT) {
            velocity.x = key == SDLK_LEFT ? -1 : 1;
        }
        if (key == SDLK_UP || key == SDLK_DOWN) {
            velocity.y = key == SDLK_UP ? -1 : 1;
        }
    }
    if (event->key.type == SDL_KEYUP) {
        velocity.x = 0;
        velocity.y = 0;
    }
}

void process_input() {
    SDL_Event event;

    while (SDL_PollEvent(&event)) {
        process_event(&event);
    }
}

void update() {
    sprite.x += velocity.x;
    sprite.y += velocity.y;
}

void draw() {
    SDL_RenderCopy(renderer, texture, NULL, &sprite);
}

void main_loop() {
    process_input();

    SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
    SDL_RenderClear(renderer);

    update();
    draw();

    SDL_RenderPresent(renderer);
}

void destroy() {
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
}

//int main() {
int main(int argc, char* args[]) {
    init();
    load_textures();

    sprite.x = (WINDOW_WIDTH - size) / 2;
    sprite.y = (WINDOW_HEIGHT - size) / 2;

    
    while(1) main_loop();

    // emscripten_set_main_loop(main_loop, -1, 1);

    destroy();
    // return EXIT_SUCCESS;
    return 0;
}