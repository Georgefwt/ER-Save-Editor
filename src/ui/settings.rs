pub mod settings {
    use crate::App;
    use eframe::egui::{self, Color32, RichText, Ui};
    use rfd::FileDialog;

    pub fn settings(ui: &mut Ui, app: &mut App) {
        ui.heading("Settings");
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("Auto-backup").strong());
            ui.add_space(4.0);
            ui.label(
                RichText::new("Auto-backup is enabled for your own safety.")
                    .color(Color32::from_rgb(120, 200, 120)),
            );

            ui.add_space(8.0);
            ui.label(RichText::new("Backup folder:").strong());

            ui.horizontal(|ui| {
                if ui.button("Select folder...").clicked() {
                    if let Some(folder) = FileDialog::new().pick_folder() {
                        app.backup_folder = Some(folder);
                    }
                }
                if ui
                    .add_enabled(app.backup_folder.is_some(), egui::Button::new("Clear"))
                    .clicked()
                {
                    app.backup_folder = None;
                }
            });

            match &app.backup_folder {
                Some(path) => {
                    ui.label(
                        RichText::new(path.display().to_string())
                            .color(Color32::from_rgb(120, 200, 120)),
                    );
                }
                None => {
                    ui.label(
                        RichText::new("No folder set — saving is blocked until one is selected")
                            .color(Color32::DARK_RED),
                    );
                }
            }

            ui.add_space(4.0);
            ui.label(
                RichText::new(
                    "Backups are saved as <filename sl2 or dat>.YYYY-MM-DD_HH-MM-SS in the chosen folder.",
                )
                .size(10.0)
                .color(Color32::PLACEHOLDER),
            );
        });
    }
}
