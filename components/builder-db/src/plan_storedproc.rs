// Copyright (c) 2017 RioCorp Inc.

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};

pub struct PlanProcedures;

impl PlanProcedures {
    pub fn new() -> Result<PlanProcedures> {
        Ok(PlanProcedures)
    }
}

impl Migratable for PlanProcedures {
    fn migrate(&self, migrator: &mut Migrator) -> Result<()> {
        debug!("=> START: plansrv");

        // The core plans table
        debug!("=> DONE: plansrv");

        Ok(())
    }
}
