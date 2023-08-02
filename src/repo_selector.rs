use std::error::Error;
use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::KeyEventKind;
use tui::{Frame, Terminal};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use walkdir::WalkDir;

use crate::commands::execute_process_and_get_response;
use crate::core::McwContext;

struct StatefulList {
    state: ListState,
    items: Vec<RepoSelectItem>,
}

struct RepoSelectItem {
    repo_path: String,
    selected: Option<bool>,
    git_status: String,
}

impl RepoSelectItem {
    fn new(repo_path: &str) -> Self {
        let cmd = vec![
            "git".to_string(),
            "symbolic-ref".to_string(),
            "--short".to_string(),
            "HEAD".to_string(),
        ];
        let git_state = execute_process_and_get_response(repo_path, &cmd);
        return RepoSelectItem {
            repo_path: repo_path.to_owned(),
            selected: Some(false),
            git_status: git_state,
        };
    }

    fn generate_display(&self) -> String {
        let mut selected_token = ' ';
        if self.selected.unwrap() {
            selected_token = 'X';
        }
        return format!("[{}] ({:15}) {}", selected_token, self.git_status.as_str(), self.repo_path.as_str());
    }
}

impl StatefulList {
    fn with_items(items: Vec<RepoSelectItem>) -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }


    fn toggle_current_item(&mut self) {
        match self.state.selected() {
            None => {}
            Some(idx) => {
                let bing = self.items.get_mut(idx).unwrap();
                let current = bing.selected.clone().take();
                if !current.unwrap() {
                    bing.selected.replace(true);
                } else {
                    bing.selected.replace(false);
                }
            }
        }
    }
}


struct App {
    items: StatefulList,
    //events: Vec<(&'a str, &'a str)>,
}

impl<'a> App {
    fn new(repos: Vec<String>) -> App {
        let mut list_items = Vec::new();
        for x in repos {
            list_items.push(RepoSelectItem::new(x.as_str()));
        }
        let mut app = App {
            items: StatefulList::with_items(list_items),
        };
        app.items.next();
        return app;
    }
}


fn get_sub_git_folders(context: &McwContext) -> Vec<String> {
    let mut folders_contain_git_folders: Vec<String> = Vec::new();
    WalkDir::new(&context.base_path)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_entry(|e|
            {
                e.metadata().unwrap().is_dir()
            }
        )
        .filter_map(|v| v.ok())
        .for_each(|f| {
            let path_as_string = f.path().to_str().unwrap();
            let contains_git_folder = path_as_string.ends_with(".git") && path_as_string != "./.git";
            if contains_git_folder {
                folders_contain_git_folders.push(path_as_string.replace(".git", ""))
            }
        });
    return folders_contain_git_folders;
}

pub fn select_repo_menu(context: &McwContext) -> Result<(), Box<dyn Error>> {
    let git_folders = get_sub_git_folders(&context);
    if git_folders.is_empty() {
        println!("No child git repos found in: {}", context.base_path);
        return Ok(());
    }


    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(git_folders);
    let _res = run_app(&mut terminal, app, context);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;
    return Ok(());
}


fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    context: &McwContext,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            match key.kind {
                KeyEventKind::Release => {}
                KeyEventKind::Repeat => {}
                KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down => app.items.next(),
                        KeyCode::Char('c') => {
                            app.items.items.iter().filter(|p| p.selected.unwrap()).for_each(|r| {
                                context.repositories.borrow_mut().push(r.repo_path.to_owned());
                            });
                            return Ok(());
                        }
                        KeyCode::Char(' ') => app.items.toggle_current_item(),
                        KeyCode::Enter => app.items.toggle_current_item(),
                        KeyCode::Up => app.items.previous(),
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let span = Spans::from(i.generate_display());
            ListItem::new(span).style(Style::default().fg(Color::White).bg(Color::Black))
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Select repos you want to use (UP, DOWN, Toggle with space q to quit, c confirm) "))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
}