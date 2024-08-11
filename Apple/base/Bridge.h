//
//  Bridge.h
//  winit-ios
//
//

#ifndef Bridge_h
#define Bridge_h

#include <stdint.h>

void start_app();
struct wgpu_canvas;

struct ios_view_obj
{
    void *view;
    // CAMetalLayer
    void *metal_layer;
    int maximum_frames;
    void (*callback_to_swift)(int32_t arg);
};

struct wgpu_canvas *create_wgpu_canvas(struct ios_view_obj object);
void enter_frame(struct wgpu_canvas *data);
//void change_example(struct wgpu_canvas *data, int32_t index);

#endif /* Bridge_h */
