//! path_builds

use std::f32::consts::PI;

use iced::{widget::canvas::{self, Frame, Path, Stroke}, Color, Point, Radians, Vector};
use charts_rs_mod::{GuiArrow, GuiAxis, GuiBubble, GuiCircle, GuiGrid, GuiLegend, GuiLine,
GuiPie, GuiPolygon, GuiPolyline, GuiRect, GuiSmoothLine, GuiSmoothLineFill,
GuiStraightLine, GuiStraightLineFill, GuiText};


pub fn build_arrow(
        ar: &GuiArrow,
        frame: &mut Frame,
        )
{

}

pub fn build_axis(
        ax: &GuiAxis,
        frame: &mut Frame,
        )
{

}

pub fn build_bubble(
        bu: &GuiBubble,
        frame: &mut Frame,
        )
{

}

pub fn build_circle(
        cir: &GuiCircle,
        frame: &mut Frame,
        ) 
{
    let path = Path::new(|p| {
        p.circle(Point::new(cir.cx, cir.cy), cir.r);
    });

    let color = match cir.stroke_color {
        Some(c) => Color::from_rgba8(c.r, c.g, c.b, c.a as f32),
        None => Color::TRANSPARENT,
    };

    let stroke = 
        Stroke {
            style: canvas::stroke::Style::Solid(color),
            width: cir.stroke_width,
            ..Stroke::default()
        };
    
    if cir.fill.is_some() {
        let c = cir.fill.unwrap();
        frame.fill(&path, Color::from_rgba8(c.r, c.g, c.b, c.a as f32))
    }

    frame.stroke(&path, stroke)

}

pub fn build_grid(
        grid: &GuiGrid,
        frame: &mut Frame,
        )
{

}

pub fn build_legend(
        leg: &GuiLegend,
        frame: &mut Frame,
        )
{

}


pub fn build_line(
        line: &GuiLine, 
        frame: &mut Frame
        ) 
{
    let path = Path::new(|p| {
        p.move_to(line.points[0]);
        p.line_to(line.points[1]);
    });
            
}

pub fn build_pie(
        pie: &GuiPie,
        frame: &mut Frame,
        )
{

}

pub fn build_polygon(
        pg: &GuiPolygon, 
        frame: &mut Frame,   
        ) 
{
    let path = Path::new(|p| {
        let points = &pg.points;
        for (index, point) in points.iter().enumerate() {
            if index == 0 {
                p.move_to(*point);
            } else {
                p.line_to(*point);
            }
        }
        p.line_to(points[0]);
    });

}

pub fn build_polyline(
        pl: &GuiPolyline, 
        frame: &mut Frame,   
        )
{
    let path = Path::new(|p| {
        for (index, point) in pl.points.iter().enumerate() {
            if index == 0 {
                p.move_to(*point);
            } else {
                p.line_to(*point);
            }
        }
    });
            
}

pub fn build_rect(
        rect: &GuiRect,
        frame: &mut Frame,
        )
{

}

pub fn build_smooth_line(
        sl: &GuiSmoothLine,
        frame: &mut Frame,
        )
{

}

pub fn build_smooth_line_fill(
        slf: &GuiSmoothLineFill,
        frame: &mut Frame,
        )
{

}

pub fn build_straight_line(
        sl: &GuiStraightLine,
        frame: &mut Frame,
        )
{

}

pub fn build_straight_line_fill(
        slf: &GuiStraightLineFill,
        frame: &mut Frame,
        )
{

}

pub fn build_text (
        txt: &GuiText, 
        frame: &mut Frame,
        ) {

        let mut text = canvas::Text {
                    content: txt.content.clone(),
                    position: Point::ORIGIN,
                    color: txt.color,
                    size: txt.size,
                    line_height: txt.line_height,
                    font: txt.font,
                    horizontal_alignment: txt.horizontal_alignment,
                    vertical_alignment: txt.vertical_alignment,
                    shaping: txt.shaping,
                };
                  
}
