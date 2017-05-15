#vs

layout (location = 0) in vec2 p;

void main() {
  gl_Position = vec4(p, 0., 1.);
}

#fs

out vec4 frag;

void main() {
  frag = vec4(.8, .5, .8, 1.);
}
