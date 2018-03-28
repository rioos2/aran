// Copyright 2018 The Rio Advancement Inc

//stored procedures for authentication (users, origins, origin_members, roles, permissions)

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct AuthProcedures;

impl AuthProcedures {
    pub fn new() -> Result<AuthProcedures> {
        Ok(AuthProcedures)
    }
}

// Just make sure you always address the columns by name, not by position.
impl Migratable for AuthProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("Authprocedure");

        // The core account_id_seq table
        migrator.migrate(
            "authsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS account_id_seq;"#,
        )?;
        ui.para("[✓] accounts");
        // Create table accounts

        migrator.migrate(
            "authsrv",
            r#"CREATE TABLE  IF NOT EXISTS accounts (
         id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('account_id_seq'),
         email text UNIQUE,
         first_name text,
         last_name text,
         phone text,
         api_key text,
         password text,
         approval bool,
         suspend bool,
         roles text[],
         registration_ip_address text,
         trust_level text,
         company_name text,
         object_meta jsonb,
         type_meta jsonb,
         avatar bytea,
         updated_at timestamptz,
         created_at timestamptz DEFAULT now())"#,
        )?;

        ui.para("[✓] accounts");

        migrator.migrate(
            "accountsrv",
            r#"CREATE OR REPLACE FUNCTION select_or_insert_account_v1 (
                  account_email text,
                  account_first_name text,
                  account_last_name text,
                  account_phone text,
                  account_api_key text,
                  account_password text,
                  account_approval bool,
                  account_suspend bool,
                  account_roles text[],
                  account_registration_ip_address text,
                  account_trust_level text,
                  account_company_name text,
                  account_object_meta jsonb,
                  account_type_meta jsonb,
                  account_avatar bytea
                ) RETURNS SETOF accounts AS $$
                    DECLARE
                       existing_account accounts%rowtype;
                    BEGIN
                       SELECT * INTO existing_account FROM accounts WHERE email = account_email LIMIT 1;
                       IF FOUND THEN
                           RETURN NEXT existing_account;
                       ELSE
                             RETURN QUERY INSERT INTO accounts ( email, first_name, last_name, phone, api_key, password, approval, suspend, roles,registration_ip_address,trust_level,company_name,object_meta,type_meta,avatar)
                            VALUES (account_email, account_first_name, account_last_name, account_phone, account_api_key, account_password,
                                account_approval, account_suspend, account_roles,account_registration_ip_address,account_trust_level, account_company_name, account_object_meta, account_type_meta,account_avatar) ON CONFLICT DO NOTHING RETURNING *;
                       END IF;
                       RETURN;
                    END
                $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "accountsrv",
            r#"CREATE OR REPLACE FUNCTION select_only_account_v1 (
                  account_email text,
                  account_first_name text,
                  account_last_name text,
                  account_phone text,
                  account_api_key text,
                  account_password text,
                  account_approval bool,
                  account_suspend bool,
                  account_roles text[],
                  account_registration_ip_address text,
                  account_trust_level text,
                  account_company_name text,
                  account_object_meta jsonb,
                  account_type_meta jsonb,
                  account_avatar bytea

                ) RETURNS SETOF accounts AS $$
                    BEGIN
                       RETURN QUERY SELECT * FROM accounts WHERE email = account_email;
                       RETURN;
                    END
                $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

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

        ui.para("[✓] account_session");

        //sequence ldap id generation
        migrator.migrate(
            "sessionsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS ldap_id_seq;"#,
        )?;
        //sequence ldap_configs table creation

        migrator.migrate(
            "sessionsrv",
            r#"CREATE TABLE  IF NOT EXISTS ldap_configs (
             id bigint PRIMARY KEY DEFAULT next_id_v1('ldap_id_seq'),
             host text,
             port text,
             enforce_starttls bool,
             use_ldaps bool,
             lookup_dn text,
             lookup_password text,
             ca_certs text,
             client_cert text,
             user_search text,
             group_search text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] ldap_configs");
        //ldap config table value insert

        migrator.migrate(
            "sessionsrv",
            r#"CREATE OR REPLACE FUNCTION insert_ldap_config_v1 (
                host text,
                port text,
                enforce_starttls bool,
                use_ldaps bool,
                lookup_dn text,
                lookup_password text,
                ca_certs text,
                client_cert text,
                user_search text,
                group_search text
            ) RETURNS SETOF ldap_configs AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO ldap_configs(host,port,enforce_starttls,use_ldaps,lookup_dn,lookup_password,ca_certs,client_cert,user_search,group_search)
                                        VALUES (host,port,enforce_starttls,use_ldaps,lookup_dn,lookup_password,ca_certs,client_cert,user_search,group_search)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        ui.para("[✓] insert_ldap_config_v1");

        migrator.migrate(
            "sessionsrv",
            r#"CREATE OR REPLACE FUNCTION get_ldap_config_v1 (aid bigint) RETURNS SETOF ldap_configs AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM ldap_configs WHERE id = aid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        //sequence saml provider id generation
        migrator.migrate(
            "sessionsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS saml_provider_id_seq;"#,
        )?;
        //sequence saml_providers table creation

        migrator.migrate(
            "sessionsrv",
            r#"CREATE TABLE  IF NOT EXISTS saml_providers (
                     id bigint PRIMARY KEY DEFAULT next_id_v1('saml_provider_id_seq'),
                     description text,
                     idp_metadata text,
                     sp_base_url text,
                     updated_at timestamptz,
                     created_at timestamptz DEFAULT now()
                     )"#,
        )?;

        ui.para("[✓] saml_providers");
        //saml config table value insert

        migrator.migrate(
            "sessionsrv",
            r#"CREATE OR REPLACE FUNCTION insert_saml_provider_v1 (
                        description text,
                        idp_metadata text,
                        sp_base_url text
                    ) RETURNS SETOF saml_providers AS $$
                                        BEGIN
                                            RETURN QUERY INSERT INTO saml_providers(description,idp_metadata,sp_base_url)
                                                VALUES (description,idp_metadata,sp_base_url)
                                                RETURNING *;
                                            RETURN;
                                        END
                                    $$ LANGUAGE plpgsql VOLATILE
                                    "#,
        )?;

        ui.para("[✓] insert_saml_provider_v1");

        migrator.migrate(
            "sessionsrv",
            r#"CREATE OR REPLACE FUNCTION get_saml_provider_all_v1() RETURNS SETOF saml_providers AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM saml_providers;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "sessionsrv",
            r#"CREATE OR REPLACE FUNCTION get_saml_v1 (sid bigint) RETURNS SETOF saml_providers AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM saml_providers WHERE id = sid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.para("[✓] get_saml_v1");

        //sequence oidc_providers id generation
        migrator.migrate(
            "sessionsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS oidc_provider_id_seq;"#,
        )?;
        //sequence oidc_providers table creation

        migrator.migrate(
            "sessionsrv",
            r#"CREATE TABLE  IF NOT EXISTS oidc_providers (
                     id bigint PRIMARY KEY DEFAULT next_id_v1('oidc_provider_id_seq'),
                     description text,
                     issuer text,
                     base_url text,
                     client_secret text,
                     client_id text,
                     verify_server_certificate bool,
                     ca_certs text,
                     updated_at timestamptz,
                     created_at timestamptz DEFAULT now()
                     )"#,
        )?;

        // ui.para("[✓] oidc_providers");
        //open id config table value insert

        migrator.migrate(
            "sessionsrv",
            r#"CREATE OR REPLACE FUNCTION insert_oidc_provider_v1 (
                description text,
                issuer text,
                base_url text,
                client_secret text,
                client_id text,
                verify_server_certificate bool,
                ca_certs text
                    ) RETURNS SETOF oidc_providers AS $$
                                        BEGIN
                                            RETURN QUERY INSERT INTO oidc_providers(description, issuer, base_url, client_secret, client_id , verify_server_certificate,ca_certs)
                                                VALUES (description, issuer, base_url, client_secret, client_id , verify_server_certificate,ca_certs)
                                                RETURNING *;
                                            RETURN;
                                        END
                                    $$ LANGUAGE plpgsql VOLATILE
                                    "#,
        )?;

        ui.para("[✓] insert_oidc_provider_v1");

        migrator.migrate(
            "sessionsrv",
            r#"CREATE OR REPLACE FUNCTION get_oidc_provider_all_v1() RETURNS SETOF oidc_providers AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM oidc_providers;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "sessionsrv",
            r#"CREATE OR REPLACE FUNCTION get_odic_v1 (oid bigint) RETURNS SETOF oidc_providers AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM oidc_providers WHERE id = oid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.para("[✓] get_odic_v1");

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
                ) RETURNS TABLE(id bigint, email text, token text, api_key text, is_admin bool, is_service_access bool) AS $$
                     DECLARE
                        this_account accounts%rowtype;
                     BEGIN
                        SELECT * FROM accounts WHERE accounts.email = account_email LIMIT 1 INTO this_account;
                        IF FOUND THEN
                            DELETE FROM account_sessions WHERE account_id = this_account.id AND account_sessions.token = account_token AND expires_at < now();
                            IF NOT FOUND THEN
                                RETURN QUERY
                                    SELECT accounts.id, accounts.email, accounts.api_key, account_sessions.token,account_sessions.is_admin,
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

        migrator.migrate(
            "originsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS origin_id_seq;"#,
        )?;

        migrator.migrate(
            "originsrv",
            r#"CREATE TABLE IF NOT EXISTS origins (
                    id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('origin_id_seq'),
                    name text UNIQUE,
                    type_meta jsonb,
                    object_meta jsonb,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
             )"#,
        )?;

        ui.para("[✓] origins");

        migrator.migrate(
            "originsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS origin_mem_id_seq;"#,
        )?;

        migrator.migrate(
            "originsrv",
            r#"CREATE TABLE IF NOT EXISTS origin_members (
                    id bigint PRIMARY KEY DEFAULT next_id_v1('origin_mem_id_seq'),
                    type_meta jsonb,
                    object_meta jsonb,
                    meta_data jsonb,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
                )"#,
        )?;

        ui.para("[✓] origin_members");

        migrator.migrate("originsrv", r#"CREATE SEQUENCE IF NOT EXISTS team_id_seq;"#)?;

        migrator.migrate(
            "originsrv",
            r#"CREATE TABLE IF NOT EXISTS teams (
                    id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('team_id_seq'),
                    name text UNIQUE,
                    type_meta jsonb,
                    object_meta jsonb,
                    meta_data jsonb,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
             )"#,
        )?;

        ui.para("[✓] team");

        migrator.migrate(
            "originsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS team_mem_id_seq;"#,
        )?;

        migrator.migrate(
            "originsrv",
            r#"CREATE TABLE IF NOT EXISTS team_members (
                    id bigint PRIMARY KEY DEFAULT next_id_v1('team_mem_id_seq'),
                    type_meta jsonb,
                    object_meta jsonb,
                    meta_data jsonb,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
                )"#,
        )?;

        ui.para("[✓] team_members");

        migrator.migrate(
            "originsrv",
            r#"CREATE OR REPLACE FUNCTION insert_origin_member_v1 (
                     om_type_meta jsonb,
                     om_obj_meta jsonb,
                     om_meta_data jsonb
                 ) RETURNS void AS $$
                     BEGIN
                         INSERT INTO origin_members ( type_meta, object_meta,meta_data)
                                VALUES (om_type_meta,om_obj_meta,om_meta_data);
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "originsrv",
            r#"INSERT INTO origins (name,object_meta,type_meta) VALUES ('rioos_system','{"name":"rioos_system", "labels": {}, "account": "", "created_at": "", "deleted_at": "", "finalizers": [], "annotations": {}, "cluster_name": "", "initializers": {"result": {"code": 0, "reason": "", "status": "", "details": {"uid": "", "kind": "", "name": "", "group": "", "causes": [], "retry_after_seconds": 0}, "message": "", "type_meta": {"kind": "", "api_version": ""}}, "pending": []}, "owner_references": [{"uid": "", "kind": "", "name": "", "api_version": "", "block_owner_deletion": false}], "deletion_grace_period_seconds": 0}','{"kind":"Origin","api_version":"v1"}')
            ON CONFLICT (name) DO NOTHING"#,
        )?;

        ui.para("[✓] origins: rioos-system");

        migrator.migrate(
            "originsrv",
            r#"CREATE OR REPLACE FUNCTION insert_origin_v1 (
                     origin_name text,
                     origin_type_meta jsonb,
                     origin_object_meta jsonb,
                     origin_mem_type_meta jsonb
                 ) RETURNS SETOF origins AS $$
                     DECLARE
                       existing_origin origins%rowtype;
                       inserted_origin origins;
                     BEGIN
                     SELECT * INTO existing_origin FROM origins WHERE name = origin_name LIMIT 1;
                     IF FOUND THEN
                         RETURN NEXT existing_origin;
                     ELSE
                         INSERT INTO origins (name,type_meta,object_meta)
                                VALUES (origin_name,origin_type_meta,origin_object_meta) ON CONFLICT (name) DO NOTHING RETURNING * into inserted_origin;
                         PERFORM insert_origin_member_v1(origin_mem_type_meta,origin_object_meta, json_build_object('origin',inserted_origin.name)::jsonb);
                         RETURN NEXT inserted_origin;
                         RETURN;
                END IF;
                RETURN;
             END
                 $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "originsrv",
            r#"CREATE OR REPLACE FUNCTION insert_team_member_v1 (
                     om_type_meta jsonb,
                     om_obj_meta jsonb,
                     om_meta_data jsonb
                 ) RETURNS void AS $$
                     BEGIN
                         INSERT INTO team_members ( type_meta, object_meta,meta_data)
                                VALUES (om_type_meta,om_obj_meta,om_meta_data);
                     END
                 $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "originsrv",
            r#"CREATE OR REPLACE FUNCTION insert_team_v1 (
                     team_name text,
                     origin text,
                     team_object_meta jsonb,
                     team_type_meta jsonb,
                     team_meta_data jsonb,
                     team_mem_type_meta jsonb
                 ) RETURNS SETOF teams AS $$
                     DECLARE
                       existing_team teams%rowtype;
                       inserted_team teams;
                     BEGIN
                     SELECT * INTO existing_team FROM teams WHERE name = team_name LIMIT 1;
                     IF FOUND THEN
                         RETURN NEXT existing_team;
                     ELSE
                         INSERT INTO teams (name,type_meta,object_meta,meta_data)
                                VALUES (team_name,team_type_meta,team_object_meta,team_meta_data) ON CONFLICT (name) DO NOTHING RETURNING * into inserted_team;
                                    PERFORM insert_origin_member_v1(team_mem_type_meta,team_object_meta, json_build_object('team',inserted_team.name, 'origin',origin)::jsonb);
                         RETURN NEXT inserted_team;
                         RETURN;
                END IF;
                RETURN;
             END
                 $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "originsrv",
            r#"CREATE OR REPLACE FUNCTION get_origins_v1() RETURNS SETOF origins AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM origins;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "originsrv",
            r#"CREATE OR REPLACE FUNCTION get_origin_v1 (org_name text) RETURNS SETOF origins AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM origins WHERE name = org_name;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

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

        ui.para("[✓] account_origins");

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

        // The core role_id_seq table
        migrator.migrate("authsrv", r#"CREATE SEQUENCE IF NOT EXISTS role_id_seq;"#)?;

        // Create table roles
        migrator.migrate(
            "authsrv",
            r#"CREATE TABLE  IF NOT EXISTS roles (
         id bigint PRIMARY KEY DEFAULT next_id_v1('role_id_seq'),
         name text,
         description text,
         updated_at timestamptz,
         created_at timestamptz DEFAULT now(),
         UNIQUE (name))"#,
        )?;

        ui.para("[✓] roles");

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

        // Select role from roles table by name
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_role_by_name_v1 (rname text) RETURNS SETOF roles AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM roles WHERE name = rname;
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;



        // The core role_id_seq table
        migrator.migrate("authsrv", r#"CREATE SEQUENCE IF NOT EXISTS perm_id_seq;"#)?;

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

        ui.para("[✓] permissions");

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

        // Select role from roles table by id
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_permission_by_role_name_v1 (rname text) RETURNS SETOF permissions AS $$
            DECLARE
               this_role roles%rowtype;
            BEGIN
                SELECT * FROM roles WHERE name = rname LIMIT 1 INTO this_role;
                RETURN QUERY SELECT * FROM permissions WHERE role_id = this_role.id;
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


        // Select role from roles table by name
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_permission_by_email_v1 (email_id text) RETURNS SETOF permissions AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM permissions WHERE role_id IN(SELECT id FROM roles WHERE name = ANY((SELECT roles FROM accounts WHERE email = email_id)::text[]));
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;


        ui.end("AuthProcedure");

        migrator.migrate(
            "originsrv",
            r#"with first_insert as (
                insert into roles(name,description)
                values('role_rios:superuser','Superuser of RIO/OS. God given powers.  instance')
                ON CONFLICT (name) DO NOTHING
                RETURNING id
            )
            insert into permissions (role_id, name ,description)
            values
            ( (select id from first_insert), 'rioos.assembly.get','Read only access to all the users  VMs, Containers'),( (select id from first_insert), 'rioos.assembly.list','Read only access to all the users  VMs, Containers')"#,
        )?;

        migrator.migrate(
            "originsrv",
            r#"with second_insert as (
                insert into roles(name,description)
                values('role_rios:TeamAdmin','TeamOwner of RIO/OS team')
                ON CONFLICT (name) DO NOTHING
                RETURNING id
            )
            insert into permissions (role_id, name ,description)
            values
            ( (select id from second_insert), 'rioos.assembly.get','Read only access to all the users  VMs, Containers'),( (select id from second_insert), 'rioos.assembly.get','Read only access to all the users  VMs, Containers')"#,
        )?;

        // The core otp_id_seq table
        migrator.migrate("authsrv", r#"CREATE SEQUENCE IF NOT EXISTS passticket_id_seq;"#)?;

        // Create table otp
        migrator.migrate(
            "authsrv",
            r#"CREATE TABLE IF NOT EXISTS passtickets (
         id bigint PRIMARY KEY DEFAULT next_id_v1('passticket_id_seq'),
         passticket text,
         created_at timestamptz DEFAULT now())"#,
        )?;

        ui.para("[✓] otp");

        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION insert_passticket_v1 (
                   o_passticket text
                ) RETURNS SETOF passtickets AS $$
                    BEGIN
                       RETURN QUERY INSERT INTO passtickets (passticket) VALUES (o_passticket)
                       RETURNING *;
                   RETURN;
                    END
                $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        // Select otp table by otp
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION get_passticket_v1 (o_passticket text) RETURNS SETOF passtickets AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM passtickets WHERE passticket = o_passticket;
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;


        // Select permission from permissions table by id
        migrator.migrate(
            "authsrv",
            r#"CREATE OR REPLACE FUNCTION remove_passticket_v1 (o_passticket text) RETURNS void AS  $$
                    BEGIN
                       DELETE FROM passtickets WHERE passticket = o_passticket;
                    END
                    $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        Ok(())
    }
}
