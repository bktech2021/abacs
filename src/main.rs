use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, text::Line, widgets::*};
use std::io::{stdout, Result};

fn main() -> Result<()> {    
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut numstr = String::new();
    let mut terminal_history: Vec<String> = Vec::new();
    terminal_history.push("Abacs".to_owned());
    terminal_history.push("Type 'exit' to close.".to_owned());
    let mut stack: Vec<f32> = Vec::new();

    macro_rules! draw {
        () => {
            let terminal_history_lines = (&mut terminal).size().unwrap().height - 3;
            terminal.draw(|frame| {
                let area = frame.size();
                let rects =
                    Layout::horizontal([Constraint::Percentage(65), Constraint::Percentage(35)])
                        .split(area);
                let mut text = Vec::new();
                let iter = terminal_history
                    .iter()
                    .rev()
                    .take(terminal_history_lines as usize)
                    .rev();
                for line in iter {
                    text.push(Line::from(line.as_str()));
                }
                text.push(Line::from(vec![
                    Span::raw(">>> "),
                    Span::raw(numstr.as_str()),
                    Span::raw("▋"),
                ]));

                let a = Paragraph::new(text)
                    .block(Block::new().title("Terminal").borders(Borders::ALL))
                    .style(Style::new().white().on_black())
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });

                let list = List::new(stack.iter().map(|i| i.to_string()).collect::<Vec<_>>())
                    .block(Block::default().title("List").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .direction(ListDirection::TopToBottom);
                frame.render_widget(a, rects[0]);
                frame.render_widget(list, rects[1]);
            })?;
        };
    }
    event::poll(std::time::Duration::from_millis(16))?;
    draw!();

    loop {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => {
                            break;
                        }

                        KeyCode::Enter => {
                            terminal_history.push(">>> ".to_owned() + numstr.as_str());
                            let maybe_num = numstr.parse::<f32>();
                            match maybe_num {
                                Ok(i) => {
                                    stack.push(i);
                                }
                                Err(_) => {
                                    // probably a command or a float, or a sign
                                    // TODO: implement float

                                    match numstr.as_str() {
                                        "+" => {
                                            if stack.len() >= 2 {
                                                let pop1 = stack.pop().unwrap();
                                                let pop2 = stack.pop().unwrap();
                                                stack.push(pop2 + pop1);
                                            } else {
                                                terminal_history.push(
                                                    "Not enough numbers to do that".to_owned(),
                                                );
                                            }
                                        }
                                        "-" => {
                                            if stack.len() >= 2 {
                                                let pop1 = stack.pop().unwrap();
                                                let pop2 = stack.pop().unwrap();
                                                stack.push(pop2 - pop1);
                                            } else {
                                                terminal_history.push(
                                                    "Not enough numbers to do that".to_owned(),
                                                );
                                            }
                                        }
                                        "*" => {
                                            if stack.len() >= 2 {
                                                let pop1 = stack.pop().unwrap();
                                                let pop2 = stack.pop().unwrap();
                                                stack.push(pop2 * pop1);
                                            } else {
                                                terminal_history.push(
                                                    "Not enough numbers to do that".to_owned(),
                                                );
                                            }
                                        }
                                        "/" => {
                                            if stack.len() >= 2 {
                                                let pop1 = stack.pop().unwrap();
                                                let pop2 = stack.pop().unwrap();
                                                stack.push(pop2 / pop1);
                                            } else {
                                                terminal_history.push(
                                                    "Not enough numbers to do that".to_owned(),
                                                );
                                            }
                                        }
                                        _ => match numstr.to_lowercase().as_str() {
                                            "exit" => break,
                                            "clear" => stack.clear(),
                                            "abs" => {
                                                if stack.len() >= 1 {
                                                    let pop = stack.pop().unwrap();
                                                    stack.push(pop.abs());
                                                } else {
                                                    terminal_history.push(
                                                        "Not enough numbers to do that".to_owned(),
                                                    );
                                                }
                                            }

                                            "avg" => {
                                                let avg = stack.iter().sum::<f32>() as f32
                                                    / stack.len() as f32;
                                                stack.clear();
                                                stack.push(avg);
                                            }

                                            "avg keep" => {
                                                let avg = stack.iter().sum::<f32>() as f32
                                                    / stack.len() as f32;
                                                stack.push(avg);
                                            }

                                            "del" => {
                                                stack.pop();
                                            }

                                            "fact" => {
                                                let pop = stack.pop().unwrap();
                                                if (pop as i32) as f32 == pop {
                                                    let sum: i32 = (1..=pop as i32).product();
                                                    stack.push(sum as f32);
                                                } else {
                                                    let gamma =
                                                        statrs::function::gamma::gamma(pop as f64)
                                                            as f32;
                                                    let sum = pop * gamma;
                                                    stack.push(sum);
                                                }
                                            }

                                            "flip" => {
                                                if stack.len() >= 1 {
                                                    let pop = stack.pop().unwrap() * -1.0;
                                                    stack.push(pop);
                                                } else {
                                                    terminal_history.push(
                                                        "Not enough numbers to do that".to_owned(),
                                                    );
                                                }
                                            }

                                            "int" => {
                                                if stack.len() >= 1 {
                                                    let pop = (stack.pop().unwrap() as i32) as f32;
                                                    stack.push(pop);
                                                } else {
                                                    terminal_history.push(
                                                        "Not enough numbers to do that".to_owned(),
                                                    );
                                                }
                                            }

                                            "log" => {
                                                if stack.len() >= 1 {
                                                    let pop = stack.pop().unwrap().ln();
                                                    stack.push(pop);
                                                } else {
                                                    terminal_history.push(
                                                        "Not enough numbers to do that".to_owned(),
                                                    );
                                                }
                                            }

                                            "log2" => {
                                                if stack.len() >= 1 {
                                                    let pop = stack.pop().unwrap().log2();
                                                    stack.push(pop);
                                                } else {
                                                    terminal_history.push(
                                                        "Not enough numbers to do that".to_owned(),
                                                    );
                                                }
                                            }

                                            "log10" => {
                                                if stack.len() >= 1 {
                                                    let pop = stack.pop().unwrap().log10();
                                                    stack.push(pop);
                                                } else {
                                                    terminal_history.push(
                                                        "Not enough numbers to do that".to_owned(),
                                                    );
                                                }
                                            }

                                            "max" => {
                                                if stack.len() >= 1 {
                                                    let max = stack
                                                        .iter()
                                                        .max_by(|x, y| x.partial_cmp(y).unwrap())
                                                        .unwrap();
                                                    stack.push(*max);
                                                } else {
                                                    terminal_history.push(
                                                        "Not enough numbers to do that".to_owned(),
                                                    );
                                                }
                                            }

                                            "min" => {
                                                if stack.len() >= 1 {
                                                    let max = stack
                                                        .iter()
                                                        .min_by(|x, y| {
                                                            x.abs().partial_cmp(&y.abs()).unwrap()
                                                        })
                                                        .unwrap();
                                                    stack.push(*max);
                                                } else {
                                                    terminal_history.push(
                                                        "Not enough numbers to do that".to_owned(),
                                                    );
                                                }
                                            }

                                            "mod" => {
                                                if stack.len() >= 2 {
                                                    let pop1 = stack.pop().unwrap();
                                                    let pop2 = stack.pop().unwrap();

                                                    if (pop1 as i32) as f32 == pop1 {
                                                        if (pop2 as i32) as f32 == pop2 {
                                                            stack.push(
                                                                (pop2 as i32 % pop1 as i32) as f32,
                                                            );
                                                        } else {
                                                            terminal_history.push(
                                                            "Numbers must be integers to do that"
                                                                .to_owned(),
                                                        );
                                                            stack.push(pop2);
                                                            stack.push(pop1);
                                                        }
                                                    } else {
                                                        terminal_history.push(
                                                            "Numbers must be integers to do that"
                                                                .to_owned(),
                                                        );
                                                        stack.push(pop2);
                                                        stack.push(pop1);
                                                    }
                                                } else {
                                                    terminal_history.push(
                                                        "Not enough numbers to do that".to_owned(),
                                                    );
                                                }
                                            }

                                            "clean" => {
                                                terminal_history.clear();
                                            }
                                            _ => terminal_history
                                                .push("Command not found".to_owned()),
                                        },
                                    }
                                }
                            }
                            numstr = "".to_owned();
                        }

                        KeyCode::Char(c) => {
                            numstr.push(c);
                        }

                        KeyCode::Backspace => {
                            numstr.pop();
                        }

                        _ => {}
                    };
                    draw!();
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
