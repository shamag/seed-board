use crate::board::Board;
use wasm_bindgen::{JsCast};
mod pieces;
pub use crate::pieces::*;

const BOARD_SIZE: i32 = 400;


mod board;
pub use crate::board::*;

use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Rendered);
    Model::default()
}


macro_rules! stop_and_prevent {
    { $event:expr } => {
        {
            $event.stop_propagation();
            $event.prevent_default();
        }
     };
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    board: Board,
    drag_start: Option<usize>
}

// ------ Color -------

#[derive(Copy, Clone, PartialEq, Eq)]
enum Color {
    A,
    B,
}

impl Color {
    const fn as_str(&self) -> &str {
        match self {
            Self::A => "white",
            Self::B => "green",
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::A
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Copy, Clone)]
enum Msg {
    Rendered,
    ChangeColor,
    Click(Piece),
    ClickBoard(PositionRow, PositionColumn)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClickBoard(row, column) => {
            web_sys::console::log_2(&(row as i32).into(), &(column as i32).into());
            if let Some(pos) = model.drag_start {
                model.board.pieces[pos].position = PiecePosition{row, column}
            }
        },
        Msg::Click(piece) => {
            for (i, p) in model.board.pieces.iter_mut().enumerate() {
                if p.position == piece.position {
                    model.drag_start = Some(i);

                    // p.position.column = PositionColumn::B;
                    // p.position.row = PositionRow::Three;
                }
            }

        },
        Msg::Rendered => {
            draw(&model.board);
            // We want to call `.skip` to prevent infinite loop.
            // (However infinite loops are useful for animations.)
            orders.after_next_render(|_| Msg::Rendered).skip();
        }
        Msg::ChangeColor => {
            // model.fill_color = if model.fill_color == Color::A {
            //     Color::B
            // } else {
            //     Color::A
            // };
        }
    }
}

fn draw(board: &Board) {
    // let canvas = canvas.get().expect("get canvas element");
    // let ctx = seed::canvas_context_2d(&canvas);

    // ctx.rect(0., 0., 200., 100.);
    // ctx.set_fill_style(&JsValue::from_str(fill_color.as_str()));
    // ctx.fill();

    // ctx.move_to(0., 0.);
    // ctx.line_to(200., 100.);
    // ctx.stroke();
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        div![
            style! {
                St::Display => "flex",
                St::Height => px(BOARD_SIZE),
                St::Width => px(BOARD_SIZE),
                St::BackgroundImage => "url('./board.svg')"
            },
            svg![
                ev(Ev::Click, move |event| {
                    stop_and_prevent!(event);
                    let ev: web_sys::MouseEvent = event.unchecked_into();
                    let position_y = ev.offset_x() * 8 / BOARD_SIZE ;
                    let position_x = 7 - ev.offset_y() * 8 / BOARD_SIZE ;
                    web_sys::console::log_3(&"click_board".into(), &position_x.into(), &position_y.into());
                    Msg::ClickBoard(position_x.into(), position_y.into())
                }),
                g![
                    C!["board"],
                    r#use![
                        attrs!{
                            At::X => "0"
                            At::Class => "b1"
                            At::Href => "/board.svg#board"
                        }
                    ]
                ],
                draw_pieces(model),
                //el_ref(&model.canvas),
                style![
                    St::Width => "100%"
                    St::Height => "100%"
                ]
            ],
        ],
        button!["Change color", ev(Ev::Click, |_| Msg::ChangeColor)],
    ]
}

fn draw_pieces(model: &Model) -> Node<Msg> {
    web_sys::console::log_1(&"draw_pieces".into());
    g![
        C!["pieces"],
        model.board.pieces.iter().map(|p| draw_piece(*p)),
    ]
}

fn draw_piece(piece: Piece) -> Node<Msg> {
        let (offset_x, offset_y) = piece.board_position();
        let translate = format!("translate({}%, {}%) scale(1.2)",offset_x, offset_y);
        g![
            r#use![
                attrs!{
                    At::Href => format!("/sprites.svg#{}", piece.name())
                },
                style!{
                    St::Transform => translate
                }
            ],
            ev(Ev::Click, move |event| {
                web_sys::console::log_1(&"click_piece".into());
                stop_and_prevent!(event);
                Msg::Click(piece.clone())
            }),
        ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}