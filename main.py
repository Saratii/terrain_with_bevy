import pygame
import sys
import numpy as np

def generate_image():
    # image_array = np.zeros((512, 512, 3), dtype=np.uint8)
    # block_size = 64
    # for y in range(0, image_array.shape[0], block_size):
    #     for x in range(0, image_array.shape[1], block_size):
    #         color = np.random.randint(0, 256, size=(3,), dtype=np.uint8)
    #         image_array[y:y+block_size, x:x+block_size] = color
    # return image_array
    return np.random.randint(0, 256, (512, 512, 3), dtype=np.uint8)

pygame.init()
window_size = (512, 512)
screen = pygame.display.set_mode(window_size)
clock = pygame.time.Clock()
surface = pygame.Surface(window_size)

while True:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            sys.exit()
    image_array = generate_image()
    pygame.surfarray.blit_array(surface, image_array)
    fps = clock.get_fps()
    font = pygame.font.Font(None, 36)
    fps_text = font.render(f"FPS: {int(fps)} - Vsize 1 W/C++", True, (0, 0, 0))
    screen.blit(surface, (0, 0))
    screen.blit(fps_text, (0, 10))
    pygame.display.flip()
    clock.tick(0)

