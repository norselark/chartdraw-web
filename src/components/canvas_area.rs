use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use wasm_bindgen::{JsCast, JsValue};

use crate::app::{HarmonicCycle, Positions};

use std::f64::consts::PI;

const TAU: f64 = 2. * PI;

lazy_static! {
    static ref ZODIAC_GLYPHS: Vec<&'static str> =
        vec!["♈", "♉", "♊", "♋", "♌", "♍", "♎", "♏", "♐", "♑", "♒", "♓",];
}

pub struct CanvasArea {
    canvas: Option<HtmlCanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    // link: ComponentLink<Self>,
    node_ref: NodeRef,
    harmonic_cycle: HarmonicCycle,
    positions: Positions,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub harmonic_cycle: HarmonicCycle,
    pub positions: Positions,
}

impl Component for CanvasArea {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            canvas: None,
            ctx: None,
            // link,
            node_ref: NodeRef::default(),
            harmonic_cycle: props.harmonic_cycle,
            positions: props.positions,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.canvas = Some(canvas);
        self.ctx = Some(ctx);

        self.render_ctx().unwrap();

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.harmonic_cycle = props.harmonic_cycle;
        self.positions = props.positions;
        self.render_ctx().unwrap();
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <canvas width="512px" height="512px" ref={self.node_ref.clone()} />
        }
    }
}

impl CanvasArea {
    fn render_ctx(&mut self) -> Result<(), JsValue> {
        let ctx: &CanvasRenderingContext2d = self
            .ctx
            .as_ref()
            .expect("Rendering context not initialized!");
        let canvas = self.canvas.as_ref().expect("Canvas not initialized!");
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        ctx.clear_rect(0., 0., width, height);
        ctx.set_transform(width / 2., 0., 0., height / 2., width / 2., height / 2.)?;

        // Set line width
        ctx.set_line_width(0.005);
        ctx.set_fill_style(&"black".into());
        ctx.fill_rect(-1., -1., 2., 2.);

        let cycleoffset = PI / 6.
            * match self.harmonic_cycle {
                HarmonicCycle::Cycle(n) => n as f64 - 1.,
                _ => 0.,
            };
        let start_of_zodiac = self.positions.ascendant - cycleoffset;

        self.draw_common(ctx, cycleoffset, start_of_zodiac)?;

        // 5 deg demarcations
        ctx.set_stroke_style(&"white".into());
        sector_lines(ctx, 0.863, 226. / 256., -start_of_zodiac, 72)?;

        // Twelve sectors
        ctx.set_stroke_style(&"white".into());
        sector_lines(ctx, 181. / 256., 0.863, 0., 12)?;

        // Zodiac sectors
        ctx.set_stroke_style(&"teal".into());
        sector_lines(ctx, 238. / 256., 271. / 256., -start_of_zodiac, 12)?;

        // Sun
        ctx.save();
        ctx.set_stroke_style(&"black".into());
        ctx.set_fill_style(&"yellow".into());
        ctx.rotate(cycleoffset - self.positions.sun - start_of_zodiac)?;
        ctx.begin_path();
        ctx.ellipse(0.5, 0., 0.03, 0.03, 0., 0., TAU)?;
        ctx.fill();
        ctx.stroke();
        ctx.restore();

        // Moon
        ctx.save();
        ctx.set_stroke_style(&"black".into());
        ctx.set_fill_style(&"grey".into());
        ctx.rotate(cycleoffset - self.positions.moon - start_of_zodiac)?;
        ctx.begin_path();
        ctx.ellipse(0.5, 0., 0.03, 0.03, 0., 0., TAU)?;
        ctx.fill();
        ctx.stroke();
        ctx.restore();

        draw_ascendant(ctx, cycleoffset)?;

        Ok(())
    }

    fn draw_common(
        &self,
        ctx: &CanvasRenderingContext2d,
        cycleoffset: f64,
        start_of_zodiac: f64,
    ) -> Result<(), JsValue> {
        ctx.set_stroke_style(&"white".into());

        // Outer ring of planet symbols
        ctx.set_fill_style(&"lightseagreen".into());
        let r = 0.863;
        ctx.begin_path();
        centered_circle(ctx, r)?;
        ctx.fill();
        ctx.stroke();

        // Inner ring of planet symbols
        ctx.set_fill_style(&"lightblue".into());
        let r = 0.703;
        ctx.begin_path();
        centered_circle(ctx, r)?;
        ctx.fill();
        ctx.stroke();

        // Dark blue under horizon
        ctx.set_fill_style(&"blue".into());
        ctx.begin_path();
        ctx.ellipse(0., 0., r, r, 0., cycleoffset - TAU, cycleoffset - PI)?;
        ctx.fill();
        ctx.stroke();

        // Outer ring with 5­ deg demarcations
        let r = 0.883;
        ctx.begin_path();
        centered_circle(ctx, r)?;
        ctx.stroke();

        // Center disk
        ctx.set_stroke_style(&"black".into());
        ctx.set_fill_style(&"lightseagreen".into());
        let r = 56. / 256.;
        ctx.begin_path();
        centered_circle(ctx, r)?;
        ctx.fill();
        ctx.stroke();

        // Zodiac inner ring
        ctx.set_stroke_style(&"teal".into());
        let r = 238. / 256.;
        ctx.begin_path();
        centered_circle(ctx, r)?;
        ctx.stroke();

        // Zodiac outer ring
        let r = 267. / 256.;
        ctx.begin_path();
        centered_circle(ctx, r)?;
        ctx.stroke();

        // Zodiac glyphs
        ctx.save();
        ctx.rotate(-start_of_zodiac + PI / 2.0)?;
        ctx.rotate(PI / 12.)?;

        ctx.set_stroke_style(&"teal".into());
        ctx.set_font("0.1px serif");
        let sector_size = TAU / 12.;
        ctx.set_text_align("center");
        // ctx.set_text_baseline("middle");
        ctx.set_fill_style(&"red".into());
        for glyph in ZODIAC_GLYPHS.iter() {
            ctx.rotate(-sector_size)?;
            ctx.save();
            ctx.translate(0., -243. / 256.)?;
            ctx.fill_text(glyph, 0., 0.)?;
            ctx.restore();
            ctx.set_fill_style(&"teal".into());
        }
        ctx.restore();

        Ok(())
    }
}

fn centered_circle(ctx: &CanvasRenderingContext2d, radius: f64) -> Result<(), JsValue> {
    ctx.ellipse(0., 0., radius, radius, 0., 0., TAU)?;
    Ok(())
}

fn sector_lines(
    ctx: &CanvasRenderingContext2d,
    inner_radius: f64,
    outer_radius: f64,
    offset: f64,
    sectors: u16,
) -> Result<(), JsValue> {
    ctx.save();
    ctx.rotate(offset)?;
    let sector_size = TAU / sectors as f64;
    for _ in 0..sectors {
        ctx.rotate(sector_size)?;
        ctx.begin_path();
        ctx.move_to(0., inner_radius);
        ctx.line_to(0., outer_radius);
        ctx.stroke();
    }
    ctx.restore();
    Ok(())
}

fn draw_ascendant(ctx: &CanvasRenderingContext2d, cycleoffset: f64) -> Result<(), JsValue> {
    ctx.set_stroke_style(&"white".into());
    ctx.set_fill_style(&"white".into());
    ctx.save();
    ctx.rotate(cycleoffset)?;
    ctx.begin_path();
    ctx.move_to(-0.223, 0.);
    ctx.line_to(-0.699, 0.);
    ctx.move_to(0.223, 0.);
    ctx.line_to(0.699, 0.);
    ctx.move_to(-0.859, 0.);
    ctx.line_to(-0.895, 0.);
    ctx.stroke();
    ctx.save();
    ctx.translate(-0.895, 0.)?;
    ctx.begin_path();
    ctx.move_to(0., -0.0117);
    ctx.line_to(-0.0195, -0.0117);
    ctx.line_to(-0.0586, 0.);
    ctx.line_to(-0.0195, 0.0117);
    ctx.line_to(0., 0.0117);
    ctx.line_to(0., -0.0117);
    ctx.stroke();
    ctx.fill();
    ctx.restore();
    ctx.restore();
    Ok(())
}

fn draw_unit(ctx: &CanvasRenderingContext2d) {
    ctx.set_stroke_style(&"red".into());
    ctx.begin_path();
    ctx.move_to(0., 0.);
    ctx.line_to(0.8, 0.);
    ctx.stroke();
}
