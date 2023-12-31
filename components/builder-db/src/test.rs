// Copyright 2018 The Rio Advancement Inc

//! Shared code for testing crates that use this database layer.
//!
//! The design uses a database with dynamically created schemas per test, that automatically handle

use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};
use std::sync::{Once, ONCE_INIT};

pub use protocol::sharding::SHARD_COUNT;

pub static INIT_TEMPLATE: Once = ONCE_INIT;
pub static TEST_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

pub mod postgres {
    use std::path::PathBuf;
    use std::process::{Child, Command, Stdio};
    use std::sync::{Once, ONCE_INIT};
    use std::thread;

    struct Postgres {
        inner: Child,
    }

    static POSTGRES: Once = ONCE_INIT;

    pub fn start() {
        POSTGRES.call_once(|| {
            thread::spawn(move || {
                let mut postgres = Postgres::new();
                let _ = postgres.inner.wait();
            });
        });
    }

    impl Postgres {
        fn new() -> Postgres {
            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("db");
            let start_path = root_path.join("start.sh");
            let child = Command::new("sudo")
                .arg("-E")
                .arg(start_path)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .env("DB_TEST_DIR", root_path)
                .current_dir("/tmp")
                .spawn()
                .expect("Failed to launch core/postgresql");
            Postgres { inner: child }
        }
    }
}

pub mod init {
    use std::sync::{Once, ONCE_INIT};

    use config::DataStore;
    use pool::Pool;

    static INIT: Once = ONCE_INIT;
    pub fn create_database() {
        INIT.call_once(|| {
            let mut config = DataStore::default();
            config.database = "template1".to_string();
            config.pool_size = 1;
            let pool = Pool::new(&config, vec![0]).expect("Failed to create pool");
            let conn = pool.get_raw().expect("Failed to get connection");
            let _ = conn.execute("DROP DATABASE IF EXISTS builder_db_test_template", &[]);
            let _ = conn.execute("CREATE DATABASE builder_db_test_template", &[]);
        })
    }
}

#[macro_export]
#[allow(unused_must_use)]
macro_rules! datastore_test {
    ($datastore:ident) => {
        {
            use std::sync::atomic::Ordering;
            use $crate::config::DataStore;
            use $crate::pool::Pool;
            use $crate::test::{postgres, SHARD_COUNT, INIT_TEMPLATE, TEST_COUNT};

            postgres::start();

            INIT_TEMPLATE.call_once(|| {
                let mut config = DataStore::default();
                config.database = "template1".to_string();
                config.pool_size = 1;
                let pool = Pool::new(&config, vec![0]).expect("Failed to create pool");
                let conn = pool.get_raw().expect("Failed to get connection");
                conn.execute("DROP DATABASE IF EXISTS builder_db_test_template", &[]).expect("Failed to drop existing template database");
                conn.execute("CREATE DATABASE builder_db_test_template", &[]).expect("Failed to create template database");
                config.database = "builder_db_test_template".to_string();
                let template_pool = Pool::new(&config, (0..SHARD_COUNT).collect()).expect("Failed to create pool");
                let ds = $datastore::from_pool(template_pool).expect("Failed to create data store from pool");
                ds.setup().expect("Failed to migrate data");
            });
            let test_number = TEST_COUNT.fetch_add(1, Ordering::SeqCst);
            let db_name = format!("builder_db_test_{}", test_number);
            let mut config = DataStore::default();
            config.database = "template1".to_string();
            config.pool_size = 1;
            let create_pool = Pool::new(&config, vec![0]).expect("Failed to create pool");
            let conn = create_pool.get_raw().expect("Failed to get connection");
            let drop_db = format!("DROP DATABASE IF EXISTS {}", db_name);
            let create_db = format!("CREATE DATABASE {} TEMPLATE builder_db_test_template", db_name);
            conn.execute(&drop_db, &[]).expect("Failed to drop test database");
            conn.execute(&create_db, &[]).expect("Failed to create test database from template");

            config.database = db_name;
            config.pool_size = 5;
            let pool = Pool::new(&config, (0..SHARD_COUNT).collect()).expect("Failed to create pool");
            $datastore::from_pool(pool).expect("Failed to create data store from pool")
        }
    }
}

/// The `with_pool` macro injects a `Pool` instance thats dynamically configured to use the test
/// database, and set to create a new schema for the test.
#[macro_export]
macro_rules! with_pool {
    ($pool:ident, $code:block) => {
        use std::sync::atomic::Ordering;
        use $crate::config::DataStore;
        use $crate::pool::Pool;
        use $crate::test::{init, postgres, SHARD_COUNT, TEST_COUNT};

        postgres::start();
        init::create_database();
        let test_number = TEST_COUNT.fetch_add(1, Ordering::SeqCst);
        let db_name = format!("builder_db_test_{}", test_number);
        let mut config = DataStore::default();
        config.database = "template1".to_string();
        config.pool_size = 1;
        let create_pool = Pool::new(&config, vec![0]).expect("Failed to create pool");
        let conn = create_pool.get_raw().expect("Failed to get connection");
        let drop_db = format!("DROP DATABASE IF EXISTS {}", db_name);
        let create_db = format!("CREATE DATABASE {} TEMPLATE builder_db_test_template", db_name);
        conn.execute(&drop_db, &[]).expect("Failed to drop test database");
        conn.execute(&create_db, &[]).expect("Failed to create test database from template");

        config.database = db_name;
        config.pool_size = 5;
        let $pool = Pool::new(&config, (0..SHARD_COUNT).collect()).expect("Failed to create pool");
        $code
    }
}
