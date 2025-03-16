use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem , ListState,  Paragraph, StatefulWidget,
        Widget, Wrap
    },
};
pub fn render_player(area: Rect, buf: &mut ratatui::buffer::Buffer, message: &str, bg_color: Vec<u8>) {
    // Calculer la largeur du fond (prend toute la largeur de l'écran)
    let block_width = area.width;

    // Calculer la position Y pour la ligne 8 à partir du bas
    let new_y = area.y + area.height.saturating_sub(8);

    // La hauteur du fond, s'étendant de la ligne 8 à la ligne 4 en partant du bas
    let block_height = 4; // 4 lignes (entre la ligne 8 et la ligne 4)

    // Créer le bloc de fond avec la couleur spécifiée
    let bg_color_player = Color::Rgb(bg_color[0], bg_color[1], bg_color[2]);
    let block_area = Rect::new(area.x, new_y, block_width, block_height);
    let block = Block::default()
        .style(Style::default().bg(bg_color_player));

    // Définir les dimensions du bloc gauche (1/3 de la largeur)
    let left_block_width = block_width / 3;
    let left_block_area = Rect::new(area.x, new_y, left_block_width, block_height);

    // Définir la zone de texte (2/3 de la largeur)
    let text_area_width = (block_width * 2) / 3;
    let text_area = Rect::new(area.x + left_block_width, new_y, text_area_width, block_height);

    // Créer le paragraphe avec le message centré
    let paragraph = Paragraph::new(format!("Les oracles de la vie\n {}", message))
        .centered()
        .block(Block::default());

    // Créer un bloc gauche de couleur grise (peut être une image ou autre)
    let left_block = Block::default()
        .style(Style::default().bg(Color::Gray));

    // Rendu des blocs
    paragraph.render(text_area, buf);
    left_block.render(left_block_area, buf);
    block.render(block_area, buf);
}

