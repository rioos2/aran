pub use error::{Error, Result};

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format;
pub fn create_table(results: Vec<Vec<String>>, title: Row) {
    let rows = results
        .iter()
        .map(|result| {
            let r = result
                .iter()
                .map(|col| Cell::new(col).style_spec("Fbcl"))
                .collect::<Vec<_>>();
            Row::new(r.clone())
        })
        .collect::<Vec<_>>();

    let mut table = Table::init(rows);

    table.set_titles(title);

    table.set_format(*format::consts::FORMAT_NO_COLSEP);
    table.printstd();
}
