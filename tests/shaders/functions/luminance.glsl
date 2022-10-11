float luminance(vec3 rgb) {
    return dot(rgb, vec3(0.2125, 0.7154, 0.0721));
}