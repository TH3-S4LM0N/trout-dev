use {
    std::{error::Error, future::pending},
    zbus::{ConnectionBuilder, dbus_interface},
    tokio::{runtime::Runtime, time::Instant}
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
    async fn Play(&mut self, playlist: bool, to_play: String) -> String {
        self.count += 1;

        subcommands::play::pre_play(to_play).await;
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
        println!("dbg method");
        let start = Instant::now();
        let res = tokio::spawn(async move {
            ccore::dbg(test_b).await;
        });
        /*loop {
            if res.is_finished() {
                break;
            }
        }*/
        let end = start.elapsed();
        format!("Took {:?} and returned {:?}", end, res)
        
        
        
        
        /*// we enter the tokio runtime explicitly as im unsure 
        // whether we stay in it from `main()` since zbus uses async-std
        let rt = Runtime::new().expect("Failed to create tokio runtime!");
        let _guard = rt.enter();
        let start = Instant::now();
        println!("start");
        let dbg_handler = tokio::spawn(ccore::dbg(test_b));
        loop {
            if dbg_handler.is_finished() {
                println!("breaking");
                break;
            }
        }
        let end = start.elapsed();
        let res = dbg_handler;
        
        format!("Took {:?} and returned {:?}", end, res) */
    } 
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let BackendS = Backend { count: 0 };
    let _ = ConnectionBuilder::session()?
        .name("org.trout.Backend")?
        .serve_at("/org/trout/Backend", BackendS)?
        .build()
        .await?;

    // Do other things or wait forever
    pending::<()>().await;

    Ok(())
}
