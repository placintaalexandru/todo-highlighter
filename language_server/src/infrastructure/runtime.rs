use tower_lsp_server::Client;

use crate::adapters::{
    controllers::lsp::Backend,
    gateways::{color_provider::ColorProvider, ripgrep::RipGrepSearcher},
};

pub fn init_server(client: Client) -> Backend<RipGrepSearcher, ColorProvider> {
    Backend::new(client, RipGrepSearcher::default(), ColorProvider::default())
}
