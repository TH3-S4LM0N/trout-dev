use {
    std::{error::Error, future::pending},
    zbus::{ConnectionBuilder, dbus_interface}
};

mod core;
mod subcommands;
mod sites;

use {
    crate::core as ccore
};

struct Backend {
    count: u64
}

#[allow(non_snake_case)] // as fns here are named to match dbus method counterparts
#[dbus_interface(name = "org.trout.BackendI")]
impl Backend {
    async fn Play(&mut self, playlist: bool, regex: bool, to_play: String) -> String {
        self.count += 1;
        let cfg = ccore::load_cfg().await;

        subcommands::play::pre_play(to_play, playlist, regex, &cfg).await;
        String::new()
    }
    async fn Gen(&mut self, data_dir: String) -> String {
        self.count += 1;
        subcommands::gen::gen(&data_dir).await
    }
    async fn New(&mut self, playlist: String, site: String, link: String) -> String {
        self.count += 1;
        subcommands::new::new(playlist, site, link).await
    }
    
    #[cfg(debug_assertions)]
    async fn dbg(&mut self, test_b: bool) -> String {
        ccore::dbg(test_b).await;

        String::new()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = ccore::load_cfg().await;
    ccore::logger::init_logger(&cfg).await;

    let backend_s = Backend { count: 0 };
    let _ = ConnectionBuilder::session()?
        .name("org.trout.Backend")?
        .serve_at("/org/trout/Backend", backend_s)?
        .build()
        .await?;

    // Do other things or wait forever
    pending::<()>().await;

    Ok(())
}
