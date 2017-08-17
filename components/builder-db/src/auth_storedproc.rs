// Copyright (c) 2017 RioCorp Inc.

//stored procedures for authentication (users, roles, permissions)

use error::{Result, Error};
use migration::{Migratable, Migrator};

pub struct AuthProcedures;

impl AuthProcedures {
    pub fn new() -> Result<AuthProcedures> {
        Ok(AuthProcedures)
    }
}

impl Migratable for AuthProcedures {
    fn migrate(&self, migrator: &mut Migrator) -> Result<()> {
        debug!("=> START: authsrv");

        // Just make sure you always address the columns by name, not by position.

        // The core user_id_seq table
        migrator.migrate(
            "authsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS user_id_seq;"#,
        )?;
        debug!("=> [✓] user_id_seq");

        // Create table users
        migrator.migrate(
            "authsrv",
            r#"CREATE TABLE  IF NOT EXISTS users (
         id bigint PRIMARY KEY DEFAULT next_id_v1('user_id_seq'),
         email text,
         first_name text,
         last_name text,
         phone text,
         api_key text,
         password text,
         states text,
         approval text,
         suspend text,
         registration_ip_address text,
         updated_at timestamptz,
         created_at timestamptz DEFAULT now())"#,
        )?;

        debug!("=> [✓] users");

        // Insert a new user into the users table
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION insert_user_v1 (
                        email text,
                        first_name text,
                        last_name text,
                        phone text,
                        api_key text,
                        password text,
                        states text,
                        approval text,
                        suspend text,
                        registration_ip_address text
                    ) RETURNS SETOF users AS $$
                            BEGIN
                                RETURN QUERY INSERT INTO users(email, first_name, last_name, phone, api_key, password, states, approval, suspend, registration_ip_address)
                                    VALUES (email, first_name, last_name, phone, api_key, password, states, approval, suspend, registration_ip_address)
                                    RETURNING *;
                                RETURN;
                            END
                        $$ LANGUAGE plpgsql VOLATILE
                        "#,
        )?;
        debug!("=> [✓] fn: insert_user_v1");


        // Select all user from users table
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_users_v1 () RETURNS SETOF users AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM users;
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_users_v1");

        // Select user from users table by id
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_user_v1 (uid bigint) RETURNS SETOF users AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM users WHERE id = uid;
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_user_v1");

        // The core role_id_seq table
        migrator.migrate(
            "authsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS role_id_seq;"#,
        )?;

        debug!("=> [✓] role_id_seq");

        // Create table roles
        migrator.migrate(
            "authsrv",
            r#"CREATE TABLE  IF NOT EXISTS roles (
         id bigint PRIMARY KEY DEFAULT next_id_v1('role_id_seq'),
         name text,
         description text,
         updated_at timestamptz,
         created_at timestamptz DEFAULT now())"#,
        )?;

        debug!("=> [✓] roles");

        // Insert a new role into the roles table
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION insert_role_v1 (
                        name text,
                        description text
                    ) RETURNS SETOF roles AS $$
                            BEGIN
                                RETURN QUERY INSERT INTO roles(name, description)
                                    VALUES (name, description)
                                    RETURNING *;
                                RETURN;
                            END
                        $$ LANGUAGE plpgsql VOLATILE
                        "#,
        )?;
        debug!("=> [✓] fn: insert_role_v1");

        // Select all role from roles table by id
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_roles_v1 () RETURNS SETOF roles AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM roles;
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_roles_v1");

        // Select role from roles table by id
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_role_v1 (rid bigint) RETURNS SETOF roles AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM roles WHERE id = rid;
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_role_v1");

        // The core role_id_seq table
        migrator.migrate(
            "authsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS perm_id_seq;"#,
        )?;

        debug!("=> [✓] perm_id_seq");


        // Create table permissions
        migrator.migrate(
            "authsrv",
            r#"CREATE TABLE IF NOT EXISTS permissions (
         id bigint PRIMARY KEY DEFAULT next_id_v1('perm_id_seq'),
         role_id bigint REFERENCES roles(id),
         name text,
         description text,
         updated_at timestamptz,
         created_at timestamptz DEFAULT now())"#,
        )?;

        debug!("=> [✓] users");

        // Insert a new permission into the permissions table
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION insert_permission_v1 (
                   per_role_id bigint,
                   per_name text,
                   per_description text
               ) RETURNS SETOF permissions AS $$
                    BEGIN
                     IF EXISTS (SELECT true FROM roles WHERE id = per_role_id) THEN
                            RETURN QUERY INSERT INTO permissions (role_id, name, description)
                                   VALUES (per_role_id, per_name, per_description)
                                   ON CONFLICT DO NOTHING
                                   RETURNING *;
                            RETURN;
                            END IF;
                    END
                $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        debug!("=> [✓] fn: insert_permission_v1");

        // Select all permission from permissions table by id
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_permissions_v1 () RETURNS SETOF permissions AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM permissions;
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_permissions_v1");

        // Select permission from permissions table by id
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_permission_v1 (pid bigint) RETURNS SETOF permissions AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM permissions WHERE id = pid;
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_permission_v1");

        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_permission_for_role_v1 (
                  role_id bigint
              ) RETURNS SETOF permissions AS $$
                   BEGIN
                       RETURN QUERY SELECT * FROM permissions WHERE id = role_id AND ignored = false
                         ORDER BY name ASC;
                       RETURN;
                   END
                   $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> DONE: authsrv");

        Ok(())

    }
}
