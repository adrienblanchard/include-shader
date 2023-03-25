uniform vec2 u_resolution;

#include "tests/shaders/functions/luminance.glsl"
#include "tests/shaders/functions/rand.glsl"

void main() {
   vec2 st = gl_FragCoord.xy / u_resolution.xy;

   gl_FragColor = vec4(vec3(rand(st)), 1.0);
}