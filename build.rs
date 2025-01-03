fn main() {
    slint_build::compile_with_config(
        "ui/MainWindow.slint",
        slint_build::CompilerConfiguration::new()
            .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer)
            .with_style("material-dark".into()),
    )
    .expect("Slint build failed");
}
