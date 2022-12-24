pub fn machine_config(&mut self, ui: &mut Ui) {
    ui.heading("Machine Configuration");
    let style = self.style.as_mut().unwrap();

    ui.collapsing("Aesthetics", |ui| {
        ui.separator();
        ui.label("Edit shit here");
        ui.checkbox(&mut style.tabs_are_draggable, "Tabs are draggable");
    });
}
