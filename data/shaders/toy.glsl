// Toy shader

#vs

layout (location = 0) in vec2 p;

void main() {
  gl_Position = vec4(p, 0., 1.);
}

#fs

uniform context {
  vec4 res;
  float t;
};

out vec4 frag;

const float PI = 3.141592;

void main() {
  vec2 hv = gl_FragCoord.xy * res.zw;
  vec2 uv = (hv - .5) * 1.4;

  hv.x = sin(hv.x * hv.y * 100. + 2. * t) + pow(sin(hv.y * 12.), 2.); 
  hv.y = cos(hv.y * 1.4) * .8;

  float scanline = .7;
  
  if (int(gl_FragCoord.y) % 4 == 0) {
    scanline = 1.;
  }

  float vignette = 1. - (uv.x * uv.x + uv.y * uv.y);

  vec3 color = vec3(abs(hv), 0.) * scanline * vignette;
  frag = vec4(color, 1.);
}
