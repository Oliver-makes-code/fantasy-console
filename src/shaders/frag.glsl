#version 440

layout(location = 0) in vec2 uv;

layout(location = 0) uniform sampler2D sam;
layout(location = 1) uniform uint d_width;
layout(location = 2) uniform uint d_height;

out vec4 color;

void main() {
    vec2 p = uv;
    // Get the images per viewport for each axis
    float scale_x = float(d_width) / 320.;
    float scale_y = float(d_height) / 240.;
    // Get the minimum
    float scale = floor(min(scale_x, scale_y));
    // Get the ratio of the two
    scale_x /= scale;
    scale_y /= scale;

    // Scale by the ratio, move to center
    p.x *= scale_x;
    p.x += (1 - scale_x) / 2;

    p.y *= scale_y;
    p.y += (1 - scale_y) / 2;

    color = texture(sam, p);

    // Display black on the borders
    if (p.x < 0 || p.x > 1 || p.y < 0 || p.y > 1)
        color = vec4(0, 0, 0, 1);
}
