use crate::view_board::ViewBoard;
use crate::game::{Game, Move};
use wasm_bindgen::{JsCast};
mod pieces;
mod engine;
mod game;
mod error;
pub use crate::pieces::*;
pub use crate::engine::square::{Position, PositionRow, PositionColumn};


/// # Examples
///
/// ```
/// let x = 5;
/// ```

const BOARD_SIZE: i32 = 400;


mod view_board;
pub use crate::view_board::*;

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


#[derive(Default)]
struct Model {
    game: Game,
    drag_start: Option<usize>
}


// ------ ------
//    Update
// ------ ------

#[derive(Copy, Clone)]
enum Msg {
    Rendered,
    Click(Piece),
    ClickBoard(PositionRow, PositionColumn)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClickBoard(row, column) => {
            web_sys::console::log_2(&(row as i32).into(), &(column as i32).into());
            if let Some(pos) = model.drag_start {
                model.game = model.game.perfome_move(Move::Move(pos, Position{row, column}));
                // model.game.board.pieces[pos].position = Position{row, column};
                model.drag_start = None;
            }
        },
        Msg::Click(piece) => {
            let clicked_pos = model.game.board.pieces.iter().position(|p| p.position == piece.position).unwrap();
            let p = model.game.board.pieces[clicked_pos];
            match model.drag_start {
                Some(pos) => {
                    if model.game.cur_color == p.color {
                        model.drag_start = Some(clicked_pos)
                    } else {
                        // let row = model.game.board.pieces[clicked_pos].position.row;
                        // let column = model.game.board.pieces[clicked_pos].position.column;
                        model.game.board.pieces[pos].position = model.game.board.pieces[clicked_pos].position;
                        model.game = model.game.perfome_move(Move::Capture(pos, clicked_pos));
                        // model.game.board.pieces.remove(clicked_pos);
                        model.drag_start = None;
                    }
                },
                None => {
                    if model.game.cur_color == model.game.board.pieces[clicked_pos].color {
                        model.drag_start = Some(clicked_pos)
                    }
                }
            }

        },
        Msg::Rendered => {
            draw(&model.game.board);
            // We want to call `.skip` to prevent infinite loop.
            // (However infinite loops are useful for animations.)
            orders.after_next_render(|_| Msg::Rendered).skip();
        }
    }
}
// fn perfome_move(model: &mut Model)

fn draw(board: &ViewBoard) {
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
                draw_active(model),
                draw_pieces(model),
                //el_ref(&model.canvas),
                style![
                    St::Width => "100%"
                    St::Height => "100%"
                ]
            ],
        ],
    ]
}


fn draw_active(model: &Model) -> Node<Msg> {
    web_sys::console::log_1(&"draw_pieces".into());
    g![
        C!["selected"],
        IF!(model.drag_start.is_some() => {
            let pos = model.drag_start.unwrap();
            let pos_y = model.game.board.pieces[pos].position.row as i32;
            let pos_x = model.game.board.pieces[pos].position.column as i32;
            let poss_x = (pos_x) * (BOARD_SIZE / 8);
            let poss_y = (7 - pos_y) * (BOARD_SIZE / 8);

            rect![
                attrs!{
                    At::Fill => "#044B94"
                    At::Opacity => "0.4"
                    At::X => px(poss_x),
                    At::Y => px(poss_y),
                    At::Width => px(50),
                    At::Height => px(50)
                },
        ]
        })

    ]
}

fn draw_pieces(model: &Model) -> Node<Msg> {
    web_sys::console::log_1(&"draw_pieces".into());
    g![
        C!["pieces"],
        model.game.board.pieces.iter().map(|p| draw_piece(*p)),
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