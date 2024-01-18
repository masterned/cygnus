use cygnus_models::{
    background,
    characteristics::Characteristics,
    personality::Personality,
    psionics::discipline::{Act, Discipline},
    units::Duration,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    text,
    widgets::{block::Title, Block, BorderType, Borders, List, ListItem, Paragraph, Widget, Wrap},
};

pub struct PersonalityWidget(Personality);

impl From<Personality> for PersonalityWidget {
    fn from(value: Personality) -> Self {
        Self(value)
    }
}

impl Widget for PersonalityWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let l = Layout::default()
            .constraints([Constraint::Ratio(1, 4); 4].as_ref())
            .split(area);

        Paragraph::new(self.0.traits.join("\n"))
            .wrap(Wrap::default())
            .render(l[0], buf);

        Paragraph::new(self.0.ideals.join("\n"))
            .wrap(Wrap::default())
            .render(l[1], buf);

        Paragraph::new(self.0.bonds.join("\n"))
            .wrap(Wrap::default())
            .render(l[2], buf);

        Paragraph::new(self.0.flaws.join("\n"))
            .wrap(Wrap::default())
            .render(l[3], buf);
    }
}

pub struct CharacteristicsWidget(Characteristics);

impl From<Characteristics> for CharacteristicsWidget {
    fn from(value: Characteristics) -> Self {
        Self(value)
    }
}

impl Widget for CharacteristicsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        List::new([
            ListItem::new(text::Line::from(vec![
                text::Span::styled("Alignment: ", Style::default().bold()),
                text::Span::raw(self.0.get_alignment().to_string()),
            ])),
            ListItem::new(text::Line::from(vec![
                text::Span::styled("Gender: ", Style::default().bold()),
                text::Span::raw(
                    self.0
                        .get_gender()
                        .map_or(String::from("--"), |g| g.to_string()),
                ),
            ])),
            ListItem::new(text::Line::from(vec![
                text::Span::styled("Eyes: ", Style::default().bold()),
                text::Span::raw(self.0.get_eye_color()),
            ])),
            ListItem::new(text::Line::from(vec![
                text::Span::styled("Size: ", Style::default().bold()),
                text::Span::raw(self.0.get_size().to_string()),
            ])),
            ListItem::new(text::Line::from(vec![
                text::Span::styled("Height: ", Style::default().bold()),
                text::Span::raw(
                    self.0
                        .get_height()
                        .iter()
                        .map(|h| h.to_string())
                        .collect::<Vec<_>>()
                        .join(" "),
                ),
            ])),
            ListItem::new(text::Line::from(vec![
                text::Span::styled("Faith: ", Style::default().bold()),
                text::Span::raw(
                    self.0
                        .get_faith()
                        .map_or(String::from("--"), |f| f.to_string()),
                ),
            ])),
            ListItem::new(text::Line::from(vec![
                text::Span::styled("Hair: ", Style::default().bold()),
                text::Span::raw(self.0.get_hair_color().to_string()),
            ])),
            ListItem::new(text::Line::from(vec![
                text::Span::styled("Skin: ", Style::default().bold()),
                text::Span::raw(self.0.get_skin_tone().to_string()),
            ])),
            ListItem::new(text::Line::from(vec![
                text::Span::styled("Age: ", Style::default().bold()),
                text::Span::raw(self.0.get_age().to_string()),
            ])),
            ListItem::new(text::Line::from(vec![
                text::Span::styled("Weight: ", Style::default().bold()),
                text::Span::raw(self.0.get_weight().to_string()),
            ])),
        ])
        .render(area, buf);
    }
}

pub struct BackgroundWidget(background::Background);

impl From<background::Background> for BackgroundWidget {
    fn from(value: background::Background) -> Self {
        Self(value)
    }
}

impl Widget for BackgroundWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let b = Block::new()
            .borders(Borders::TOP)
            .title(Title::from("Background"));

        let l = Layout::default()
            .constraints([Constraint::Max(1), Constraint::Min(0)])
            .split(b.inner(area));

        Paragraph::new(self.0.get_name())
            .style(Style::new().bold())
            .render(l[0], buf);

        let feature_widget: BackgroundFeatureWidget = self.0.get_feature().clone().into();
        feature_widget.render(l[1], buf);

        b.render(area, buf);
    }
}

pub struct BackgroundFeatureWidget(background::Feature);

impl From<background::Feature> for BackgroundFeatureWidget {
    fn from(value: background::Feature) -> Self {
        Self(value)
    }
}

impl Widget for BackgroundFeatureWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let l = Layout::new(
            Direction::Vertical,
            [Constraint::Max(1), Constraint::Min(0)],
        )
        .split(area);

        Paragraph::new(format!("Feature: {}", self.0.get_name()))
            .style(Style::new().italic())
            .render(l[0], buf);

        Paragraph::new(self.0.get_description())
            .wrap(Wrap::default())
            .render(l[1], buf);
    }
}

pub struct DisciplineWidget(Discipline);

impl From<Discipline> for DisciplineWidget {
    fn from(value: Discipline) -> Self {
        Self(value)
    }
}

impl Widget for DisciplineWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(self.0.get_name())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let l = Layout::new(
            Direction::Vertical,
            [
                Constraint::Ratio(1, 16),
                Constraint::Ratio(1, 16),
                Constraint::Ratio(2, 16),
                Constraint::Ratio(12, 16),
            ],
        )
        .split(block.inner(area));

        Paragraph::new(self.0.get_order()).render(l[0], buf);

        Paragraph::new(self.0.get_description()).render(l[1], buf);

        Paragraph::new(self.0.get_focus())
            .block(
                Block::new()
                    .title("Psychic Focus")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .render(l[2], buf);

        let acts = self.0.get_acts();

        for act in acts {
            Widget::render(ActWidget::from(act.clone()), l[3], buf);
        }

        block.render(area, buf);
    }
}

pub struct ActWidget(Act);

impl From<Act> for ActWidget {
    fn from(value: Act) -> Self {
        Self(value)
    }
}

impl Widget for ActWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(self.0.get_name())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let l = Layout::new(
            Direction::Vertical,
            [Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)],
        )
        .split(block.inner(area));

        let h = Layout::new(
            ratatui::layout::Direction::Horizontal,
            [Constraint::Ratio(1, 4); 4],
        )
        .split(l[0]);

        let duration = Paragraph::new(
            self.0
                .get_duration()
                .unwrap_or(Duration::Instantaneous)
                .to_string(),
        );
        duration.render(h[0], buf);

        let cost = Paragraph::new(self.0.get_cost().start.to_string());
        cost.render(h[2], buf);

        Paragraph::new("Cast")
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .render(h[3], buf);

        let p = Paragraph::new(self.0.get_description()).wrap(Wrap::default());
        p.render(l[1], buf);

        block.render(area, buf);
    }
}
