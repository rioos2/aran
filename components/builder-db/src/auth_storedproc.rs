// Copyright (c) 2017 RioCorp Inc.

//stored procedures for authentication (users, origins, origin_members, roles, permissions)

use error::Result;
use migration::{Migratable, Migrator};

pub struct AuthProcedures;

impl AuthProcedures {
    pub fn new() -> Result<AuthProcedures> {
        Ok(AuthProcedures)
    }
}

// Just make sure you always address the columns by name, not by position.
impl Migratable for AuthProcedures {
    fn migrate(&self, migrator: &mut Migrator) -> Result<()> {
        debug!("=> START: authsrv");

        // The core account_id_seq table
        migrator.migrate(
            "authsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS account_id_seq;"#,
        )?;
        debug!("=> [✓] account_id_seq");

        // Create table accounts
        migrator.migrate(
            "authsrv",
            r#"CREATE TABLE  IF NOT EXISTS accounts (
         id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('account_id_seq'),
         name text,
         email text UNIQUE,
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

        debug!("=> [✓] accounts");

        // Insert a new account into the accounts table
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION insert_account_v1 (
                        name text,
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
                    ) RETURNS SETOF accounts AS $$
                            BEGIN
                                RETURN QUERY INSERT INTO accounts(name, email, first_name, last_name, phone, api_key, password, states, approval, suspend, registration_ip_address)
                                    VALUES (name, email, first_name, last_name, phone, api_key, password, states, approval, suspend, registration_ip_address)
                                    RETURNING *;
                                RETURN;
                            END
                        $$ LANGUAGE plpgsql VOLATILE
                        "#,
        )?;

        debug!("=> [✓] fn: insert_account_v1");


        migrator.migrate(
            "accountsrv",
            r#"CREATE OR REPLACE FUNCTION select_or_insert_account_v1 (
                  account_name text,
                  account_email text,
                  account_first_name text,
                  account_last_name text,
                  account_phone text,
                  account_api_key text,
                  account_password text,
                  account_states text,
                  account_approval text,
                  account_suspend text,
                  account_registration_ip_address text
                ) RETURNS SETOF accounts AS $$
                    DECLARE
                       existing_account accounts%rowtype;
                    BEGIN
                       SELECT * INTO existing_account FROM accounts WHERE email = account_email LIMIT 1;
                       IF FOUND THEN
                           RETURN NEXT existing_account;
                       ELSE
                           RETURN QUERY INSERT INTO accounts (name, email, first_name, last_name, phone, api_key, password, states, approval, suspend, registration_ip_address)
                            VALUES (account_name, account_email, account_first_name, account_last_name, account_phone, account_api_key, account_password, account_states,
                                account_approval, account_suspend, account_registration_ip_address) ON CONFLICT DO NOTHING RETURNING *;
                       END IF;
                       RETURN;
                    END
                $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        debug!("=> [✓] fn: select_or_insert_account_v1");

        // Select all account from accounts table
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_accounts_v1 () RETURNS SETOF accounts AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM accounts;
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_accounts_v1");

        // Select account from accounts table by id
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION  get_account_by_id_v1 (uid bigint) RETURNS SETOF accounts AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM accounts WHERE id = uid;
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn:  get_account_by_id_v1");

        // Select account from accounts table by email
        migrator.migrate(
            "accountsrv",
            r#"CREATE OR REPLACE FUNCTION get_account_by_email_v1 (
                   account_email text
                ) RETURNS SETOF accounts AS $$
                    BEGIN
                       RETURN QUERY SELECT * FROM accounts WHERE email = account_email;
                       RETURN;
                    END
                $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_account_by_email_v1");


        migrator.migrate(
            "authsrv",
            r#"CREATE TABLE IF NOT EXISTS account_sessions (
                        account_id bigint REFERENCES accounts(id),
                        token text,
                        provider text,
                        is_admin bool DEFAULT false,
                        is_service_access bool DEFAULT false,
                        created_at timestamptz DEFAULT now(),
                        expires_at timestamptz DEFAULT now() + interval '1 day',
                        UNIQUE (account_id)
                        )"#,
        )?;
        debug!("=> [✓] fn: account_sessions_v1");

        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION insert_account_session_v1 (
                    a_account_id bigint,
                    account_token text,
                    account_provider text,
                    account_is_admin bool,
                    account_is_service_access bool
                 ) RETURNS SETOF account_sessions AS $$
                     BEGIN
                        RETURN QUERY INSERT INTO account_sessions (account_id, token, provider, is_admin, is_service_access)
                                        VALUES (a_account_id, account_token, account_provider, account_is_admin, account_is_service_access)
                                        ON CONFLICT (account_id) DO UPDATE
                                        SET token = account_token, expires_at = now() + interval '1 day', provider = account_provider, is_admin = account_is_admin, is_service_access = account_is_service_access
                                        RETURNING *;
                        RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_account_session_v1 (
                    account_email text,
                    account_token text
                ) RETURNS TABLE(id bigint, email text, name text, token text, api_key text, is_admin bool, is_service_access bool) AS $$
                     DECLARE
                        this_account accounts%rowtype;
                     BEGIN
                        SELECT * FROM accounts WHERE accounts.email = account_email LIMIT 1 INTO this_account;
                        IF FOUND THEN
                            DELETE FROM account_sessions WHERE account_id = this_account.id AND account_sessions.token = account_token AND expires_at < now();
                            IF NOT FOUND THEN
                                RETURN QUERY
                                    SELECT accounts.id, accounts.email, accounts.api_key,
                                           accounts.name, account_sessions.token,
                                           account_sessions.is_admin,
                                           account_sessions.is_service_access
                                      FROM accounts
                                        INNER JOIN account_sessions ON account_sessions.account_id = accounts.id
                                      WHERE accounts.id = this_account.id
                                        AND account_sessions.token = account_token;
                            END IF;
                        END IF;
                        RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        debug!("=> [✓] fn: get_account_session_v1");

        migrator.migrate(
            "originsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS origin_id_seq;"#,
        )?;

        debug!("=> [✓] origin_id_seq");

        migrator.migrate(
            "originsrv",
            r#"CREATE TABLE IF NOT EXISTS origins (
                    id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('origin_id_seq'),
                    name text UNIQUE,
                    owner_id bigint,
                    type_meta text,
                    object_meta text,
                    session_sync bool DEFAULT false,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
             )"#,
        )?;

        debug!("=> [✓] origins");

        migrator.migrate(
            "originsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS origin_mem_id_seq;"#,
        )?;

        debug!("=> [✓] origin_id_seq");

        migrator.migrate(
            "originsrv",
            r#"CREATE TABLE IF NOT EXISTS origin_members (
                    id bigint PRIMARY KEY DEFAULT next_id_v1('origin_mem_id_seq'),
                    origin_id bigint REFERENCES origins(id),
                    origin_name text ,
                    account_id bigint REFERENCES accounts(id),
                    account_name text,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
                )"#,
        )?;

        debug!("=> [✓] fn: origin_members");

        migrator.migrate(
            "originsrv",
            r#"CREATE OR REPLACE FUNCTION insert_origin_member_v1 (
                     om_origin_id bigint,
                     om_origin_name text,
                     om_account_id bigint,
                     om_account_name text
                 ) RETURNS void AS $$
                     BEGIN
                         INSERT INTO origin_members (origin_id, origin_name, account_id, account_name)
                                VALUES (om_origin_id, om_origin_name, om_account_id, om_account_name);
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        debug!("=> [✓] fn: insert_origin_member_v1");

        migrator.migrate(
            "originsrv",
            r#"CREATE OR REPLACE FUNCTION insert_origin_v1 (
                     origin_name text,
                     origin_owner_id bigint,
                     origin_owner_name text,
                     origin_type_meta text,
                     origin_object_meta text
                 ) RETURNS SETOF origins AS $$
                     DECLARE
                       inserted_origin origins;
                     BEGIN
                         INSERT INTO origins (name, owner_id,type_meta,object_meta)
                                VALUES (origin_name, origin_owner_id,origin_type_meta,origin_object_meta) RETURNING * into inserted_origin;
                         PERFORM insert_origin_member_v1(inserted_origin.id, origin_name, origin_owner_id, origin_owner_name);
                         RETURN NEXT inserted_origin;
                         RETURN;
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        debug!("=> [✓] fn: insert_origin_v1");

        migrator.migrate(
            "originsrv",
            r#"CREATE OR REPLACE FUNCTION list_origin_members_v1 (
                   om_origin_id bigint
                 ) RETURNS TABLE(account_name text) AS $$
                    BEGIN
                        RETURN QUERY SELECT origin_members.account_name FROM origin_members WHERE origin_id = om_origin_id
                          ORDER BY account_name ASC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: list_origin_members_v1");

        migrator.migrate(
            "originsrv",
            r#"CREATE OR REPLACE FUNCTION check_account_in_origin_members_v1 (
                   om_origin_name text,
                   om_account_id bigint
                 ) RETURNS TABLE(is_member bool) AS $$
                    BEGIN
                        RETURN QUERY SELECT true FROM origin_members WHERE origin_name = om_origin_name AND account_id = om_account_id;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: check_account_in_origin_members_v1");

        migrator.migrate(
            "originsrv",
            r#"CREATE OR REPLACE FUNCTION list_origin_by_account_id_v1 (
                   o_account_id bigint
                 ) RETURNS TABLE(origin_name text) AS $$
                    BEGIN
                        RETURN QUERY SELECT origin_members.origin_name FROM origin_members WHERE account_id = o_account_id
                          ORDER BY origin_name ASC;
                        RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: list_origin_by_account_id_v1");

        migrator.migrate(
            "accountsrv",
            r#"CREATE TABLE IF NOT EXISTS account_origins (
                       account_id bigint,
                       account_email text,
                       origin_id bigint,
                       origin_name text,
                       created_at timestamptz DEFAULT now(),
                       updated_at timestamptz,
                       UNIQUE(account_id, origin_id)
                       )"#,
        )?;

        debug!("=> [✓] account_origins");

        migrator.migrate(
            "accountsrv",
            r#"CREATE OR REPLACE FUNCTION insert_account_origin_v1 (
                   o_account_id bigint,
                   o_account_email text,
                   o_origin_id bigint,
                   o_origin_name text
                ) RETURNS void AS $$
                    BEGIN
                       INSERT INTO account_origins (account_id, account_email, origin_id, origin_name) VALUES (o_account_id, o_account_email, o_origin_id, o_origin_name);
                    END
                $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        debug!("=> [✓] fn: insert_account_origin_v1");

        migrator.migrate(
            "accountsrv",
            r#"CREATE OR REPLACE FUNCTION get_account_origins_v1 (
                   in_account_id bigint
                ) RETURNS SETOF account_origins AS $$
                    BEGIN
                       RETURN QUERY SELECT * FROM account_origins WHERE account_id = in_account_id;
                       RETURN;
                    END
                $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_account_origin_v1");

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
                 rid bigint
              ) RETURNS SETOF permissions AS $$
                   BEGIN
                       RETURN QUERY SELECT * FROM permissions WHERE role_id = rid
                         ORDER BY name ASC;
                       RETURN;
                   END
                   $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_specfic_permission_role_v1 (
                 perm_id bigint,
                 rid bigint
              ) RETURNS SETOF permissions AS $$
                   BEGIN
                       RETURN QUERY SELECT * FROM permissions WHERE role_id = rid AND id = perm_id
                         ORDER BY name ASC;
                       RETURN;
                   END
                   $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> DONE: authsrv");

        Ok(())

    }
}
