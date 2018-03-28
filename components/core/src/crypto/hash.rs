// Copyright 2018 The Rio Advancement Inc
use sodiumoxide::crypto::hash::sha512::hash as hash_sodium;
use hex;

/// NOTE: the hashing is keyless
/// TO-DO: digest size must be  = 32 BYTES, but here we index by 10 bytes.
///        We need to start with an empty array of 32 bytes and use that to store the
///        hashed_string as opposed to flatly indexing by 10.
///
/// TO-DO: This method go away eventually as we will move the migration to DieselCli.
pub fn hash_string(data: &str) -> String {
    let hashed = &hash_sodium(data.as_bytes())[..]; //A temporary fix.

    let stripped_len = {
        if hashed.len() > 10 { 10 } else { hashed.len() }
    };

    hex::encode(
        String::from_utf8_lossy(&hashed[..stripped_len]).into_owned(),
    )
}


#[cfg(test)]
mod test {
    use std::env;
    #[allow(unused_imports)]
    use std::fs::{self, File};
    #[allow(unused_imports)]
    use std::io;
    use std::path::PathBuf;

    #[allow(dead_code)]
    fn mk_local_tmpdir() -> PathBuf {
        let dir = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("tmp");
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /*#[test]
    fn hash_file_working() {
        // The expected values were computed using the `b2sum` program from
        // https://github.com/dchest/b2sum using the the `-s=32` option. For example:
        //      b2sum -s=32 signme.dat

        let computed = hash_string("select * from accountstbl");
        let expected = "efbfbd1f2aefbfbdefbfbd5372c8a5efbfbd";
        assert_eq!(computed, expected);
    }*/

}
