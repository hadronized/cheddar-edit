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

fn main() {
  let mut dev = bootstrap!(960, 540, WindowOpt::default()).unwrap();
  let mut cache = ResCache::new("data");
  let mut handler = Handler;
  let screen = Framebuffer2D::default([dev.width(), dev.height()]);
  let res = [dev.width() as f32, dev.height() as f32, 1. / (dev.width() as f32), 1. / (dev.height() as f32)];
  let quad: Quad = Tess::new(Mode::TriangleFan, &QUAD_TRIS[..], None);

  let mut shader_context: Buffer<ShaderToyContext> = Buffer::new(1);
  let shader = cache.get::<Program<QuadVert, (), ShaderToyUniforms>>("toy.glsl", ()).unwrap();

  shader_context.as_slice_mut().unwrap()[0].res = res;

  while dev.dispatch_events(&mut handler) {
    dev.step(60, |t| {
      cache.sync();

      shader_context.as_slice_mut().unwrap()[0].t = t as f32;

      Pipeline::new(&screen, [0., 0., 0., 1.], &[], &[&shader_context]).enter(|shd_gate| {
        let shader = shader.borrow();
        shd_gate.new(&shader, &[], &[]).enter(|rdr_gate, uniforms| {
          uniforms.context.update(Binding::new(0));

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
  t: f32
}

struct ShaderToyUniforms {
  context: Uniform<Binding>
}

impl UniformInterface for ShaderToyUniforms {
  fn uniform_interface(builder: UniformBuilder) -> program::Result<(Self, Vec<UniformWarning>)> {
    let mut warnings = Vec::new();

    let iface = ShaderToyUniforms {
      context: builder.ask("context").unwrap_or_unbound(&builder, &mut warnings)
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
