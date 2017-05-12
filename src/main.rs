#[macro_use]
extern crate spectra;

use spectra::bootstrap::{Action, EventHandler, EventSig, Key, WindowOpt};
use spectra::framebuffer::Framebuffer2D;
use spectra::luminance::buffer::{Binding, Buffer};
use spectra::luminance::pipeline::Pipeline;
use spectra::luminance::shader::program;
use spectra::luminance::tess::{Mode, Tess};
use spectra::resource::ResCache;
use spectra::shader::{Program, Uniform, UniformBuilder, UniformInterface, UniformWarning, UnwrapOrUnbound};
use spectra::texture::{Sampler, TextureImage, TextureRGBA32F, Unit};

fn main() {
  let mut dev = bootstrap!(960, 540, WindowOpt::default()).unwrap();
  let mut cache = ResCache::new("data");
  let mut handler = Handler;
  let screen = Framebuffer2D::default([dev.width(), dev.height()]);
  let res = [dev.width() as f32, dev.height() as f32, 1. / (dev.width() as f32), 1. / (dev.height() as f32)];
  let quad: Quad = Tess::new(Mode::TriangleFan, &QUAD_TRIS[..], None);

  let mut shader_context: Buffer<ShaderToyContext> = Buffer::new(1);
  let shader = cache.get_proxied::<Program<QuadVert, (), ShaderToyUniforms>, _>("toy.glsl", (), move || {
    Program::from_str(DEFAULT_SHADER_SRC).unwrap()
  });

  let textures: Vec<_> = (0..12).into_iter().map(|i| {
    cache.get_proxied::<TextureImage, _>(&format!("tex_{}.png", i), (Sampler::default(), None), move || {
      // default texture is a 2×2 black texture.
      let default_texture = TextureRGBA32F::new([2, 2], 0, &Sampler::default()).unwrap();
      default_texture.clear(false, (0., 0., 0., 1.));

      TextureImage {
        texture: default_texture,
        sampler: Sampler::default(),
        linearizer: None
      }
    })
  }).collect();

  shader_context.as_slice_mut().unwrap()[0].res = res;

  while dev.dispatch_events(&mut handler) {
    dev.step(60, |t| {
      cache.sync();

      let tex_0 = textures[0].borrow();
      let tex_1 = textures[1].borrow();
      let tex_2 = textures[2].borrow();
      let tex_3 = textures[3].borrow();
      let tex_4 = textures[4].borrow();
      let tex_5 = textures[5].borrow();
      let tex_6 = textures[6].borrow();
      let tex_7 = textures[7].borrow();
      let tex_8 = textures[8].borrow();
      let tex_9 = textures[9].borrow();
      let tex_10 = textures[10].borrow();
      let tex_11 = textures[11].borrow();
      let textures = [
        &***tex_0,
        &***tex_1,
        &***tex_2,
        &***tex_3,
        &***tex_4,
        &***tex_5,
        &***tex_6,
        &***tex_7,
        &***tex_8,
        &***tex_9,
        &***tex_10,
        &***tex_11
      ];

      shader_context.as_slice_mut().unwrap()[0].t = t as f32;

      Pipeline::new(&screen, [0., 0., 0., 1.], &textures[..], &[&shader_context]).enter(|shd_gate| {
        let shader = shader.borrow();
        shd_gate.new(&shader, &[], &[]).enter(|rdr_gate, uniforms| {
          uniforms.context.update(Binding::new(0));
          for (i, tex_u) in uniforms.textures.iter().enumerate() {
            tex_u.update(Unit::new(i as u32));
          }

          rdr_gate.new(None, false, &[], &[]).enter(|tess_gate| {
            tess_gate.render((&quad).into(), &[], &[]);
          });
        });
      });
    });
  }
}

struct Handler;

impl EventHandler for Handler {
  fn on_key(&mut self, key: Key, action: Action) -> EventSig {
    match (key, action) {
      (Key::Escape, Action::Release) => EventSig::Aborted,
      _ => EventSig::Handled
    }
  }
}

struct ShaderToyContext {
  res: [f32; 4],
  t: f32,
}

struct ShaderToyUniforms {
  context: Uniform<Binding>,
  textures: Vec<Uniform<Unit>>
}

impl UniformInterface for ShaderToyUniforms {
  fn uniform_interface(builder: UniformBuilder) -> program::Result<(Self, Vec<UniformWarning>)> {
    let mut warnings = Vec::new();

    let context = builder.ask("context").unwrap_or_unbound(&builder, &mut warnings);
    let textures: Vec<_> = (0..12).into_iter().map(|i| {
      builder.ask(&format!("tex_{}", i)).unwrap_or_unbound(&builder, &mut warnings)
    }).collect();

    let iface = ShaderToyUniforms {
      context: context,
      textures: textures
    };

    Ok((iface, warnings))
  }
}

type Quad = Tess<QuadVert>;
type QuadVert = [f32; 2];

const QUAD_TRIS: [QuadVert; 4] = [
  [-1., -1.],
  [ 1., -1.],
  [ 1.,  1.],
  [-1.,  1.]
];

const DEFAULT_SHADER_SRC: &'static str = r#"
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
"#;
