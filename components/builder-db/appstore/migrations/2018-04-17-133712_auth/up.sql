-- Your SQL goes here

CREATE SEQUENCE IF NOT EXISTS account_id_seq;


CREATE TABLE IF NOT EXISTS accounts (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('account_id_seq'),
                                                                          email text UNIQUE,
                                                                                     first_name text, last_name text, phone text, api_key text, password text, approval bool,
                                                                                                                                                               suspend bool,
                                                                                                                                                               ROLES text[], registration_ip_address text, trust_level text, company_name text, object_meta JSONB,
                                                                                                                                                                                                                                                            type_meta JSONB,
                                                                                                                                                                                                                                                                      avatar BYTEA,
                                                                                                                                                                                                                                                                             updated_at timestamptz,
                                                                                                                                                                                                                                                                             created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_account_v1 (account_email text, account_first_name text, account_last_name text, account_phone text, account_api_key text, account_password text, account_approval bool, account_suspend bool, account_roles text[], account_registration_ip_address text, account_trust_level text, account_company_name text, account_object_meta JSONB, account_type_meta JSONB, account_avatar BYTEA) RETURNS
SETOF accounts AS $$
                 BEGIN
                        RETURN QUERY INSERT INTO accounts ( email, first_name, last_name, phone, api_key, password, approval, suspend, roles,registration_ip_address,trust_level,company_name,object_meta,type_meta,avatar)
                         VALUES (account_email, account_first_name, account_last_name, account_phone, account_api_key, account_password,
                             account_approval, account_suspend, account_roles,account_registration_ip_address,account_trust_level, account_company_name, account_object_meta, account_type_meta,account_avatar) ON CONFLICT DO NOTHING RETURNING *;
                    RETURN;
                 END
             $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_accounts_v1 () RETURNS
SETOF accounts AS $$
                                     BEGIN
                                       RETURN QUERY SELECT * FROM accounts;
                                       RETURN;
                                     END
                                     $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_account_by_id_v1 (UID bigint) RETURNS
SETOF accounts AS $$
                                                   BEGIN
                                                     RETURN QUERY SELECT * FROM accounts WHERE id = uid;
                                                     RETURN;
                                                   END
                                                   $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_account_by_email_v1 (account_email text) RETURNS
SETOF accounts AS $$
                                                                 BEGIN
                                                                    RETURN QUERY SELECT * FROM accounts WHERE email = account_email;
                                                                    RETURN;
                                                                 END
                                                             $$ LANGUAGE PLPGSQL STABLE;


CREATE TABLE IF NOT EXISTS account_sessions (account_id bigint REFERENCES accounts(id),
                                                                          token text, device JSONB,
                                                                                             provider text, is_admin bool DEFAULT FALSE,
                                                                                                                                  is_service_access bool DEFAULT FALSE,
                                                                                                                                                                 created_at timestamptz DEFAULT now(),
                                                                                                                                                                                                expires_at timestamptz DEFAULT now() + interval '1 day');


CREATE OR REPLACE FUNCTION insert_account_session_v1 (a_account_id bigint, account_token text, account_provider text, device JSONB) RETURNS
SETOF account_sessions AS $$
BEGIN
RETURN QUERY INSERT INTO account_sessions (account_id, token, provider, device)
                                                                                                               VALUES (a_account_id, account_token, account_provider, device)
                                                                                                               RETURNING *;
                                                                                               RETURN;
                                                                                            END
                                                                                        $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_session_v1 (acc_id bigint, acc_device JSONB) RETURNS
SETOF account_sessions AS $$
      BEGIN
      RETURN QUERY SELECT * FROM account_sessions WHERE account_id = acc_id AND acc_device = device LIMIT 1;
      RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_account_session_by_email_token_v1 (account_email text, account_token text) RETURNS TABLE(id bigint, email text, api_key text,token text) AS $$
      DECLARE
      this_account accounts%rowtype;
      BEGIN
      SELECT * FROM accounts WHERE accounts.email = account_email LIMIT 1 INTO this_account;
      IF FOUND THEN
      DELETE FROM account_sessions WHERE account_id = this_account.id AND account_sessions.token = account_token AND expires_at < now();
      RETURN QUERY
      SELECT accounts.id, accounts.email, accounts.api_key, account_sessions.token
      FROM accounts
      INNER JOIN account_sessions ON  accounts.id =account_sessions.account_id
      WHERE accounts.id = this_account.id
      AND account_sessions.token = account_token;
      END IF;
      RETURN;
      END
      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_logout_v1 (account_email text, account_token text,acc_device JSONB) RETURNS
SETOF accounts AS $$
      DECLARE
        this_account accounts%rowtype;
      BEGIN
          SELECT * FROM accounts WHERE accounts.email = account_email LIMIT 1 INTO this_account;
            IF FOUND THEN
              DELETE FROM account_sessions WHERE account_id = this_account.id AND account_sessions.token = account_token AND account_sessions.device = acc_device;
              RETURN QUERY
              SELECT * from accounts WHERE accounts.id = this_account.id;
            END IF;
              RETURN;
            END
            $$ LANGUAGE PLPGSQL VOLATILE;


CREATE SEQUENCE IF NOT EXISTS ldap_id_seq;


CREATE TABLE IF NOT EXISTS ldap_configs (id bigint PRIMARY KEY DEFAULT next_id_v1('ldap_id_seq'),
                                                                       HOST text, port text, enforce_starttls bool,
                                                                                             use_ldaps bool,
                                                                                             lookup_dn text, lookup_password text, ca_certs text, client_cert text, user_search text, group_search text, updated_at timestamptz,
                                                                                                                                                                                                         created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_ldap_config_v1 (HOST text, port text, enforce_starttls bool, use_ldaps bool, lookup_dn text, lookup_password text, ca_certs text, client_cert text, user_search text, group_search text) RETURNS
SETOF ldap_configs AS $$
                          BEGIN
                              RETURN QUERY INSERT INTO ldap_configs(host,port,enforce_starttls,use_ldaps,lookup_dn,lookup_password,ca_certs,client_cert,user_search,group_search)
                                  VALUES (host,port,enforce_starttls,use_ldaps,lookup_dn,lookup_password,ca_certs,client_cert,user_search,group_search)
                                  RETURNING *;
                              RETURN;
                          END
                      $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_ldap_config_v1 (aid bigint) RETURNS
SETOF ldap_configs AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM ldap_configs WHERE id = aid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE SEQUENCE IF NOT EXISTS saml_provider_id_seq;


CREATE TABLE IF NOT EXISTS saml_providers (id bigint PRIMARY KEY DEFAULT next_id_v1('saml_provider_id_seq'),
                                                                         description text, idp_metadata text, sp_base_url text, updated_at timestamptz,
                                                                                                                                created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_saml_provider_v1 (description text, idp_metadata text, sp_base_url text) RETURNS
SETOF saml_providers AS $$
                                                                                                                                                                   BEGIN
                                                                                                                                                                       RETURN QUERY INSERT INTO saml_providers(description,idp_metadata,sp_base_url)
                                                                                                                                                                           VALUES (description,idp_metadata,sp_base_url)
                                                                                                                                                                           RETURNING *;
                                                                                                                                                                       RETURN;
                                                                                                                                                                   END
                                                                                                                                                               $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_saml_provider_all_v1() RETURNS
SETOF saml_providers AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM saml_providers;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_saml_v1 (sid bigint) RETURNS
SETOF saml_providers AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM saml_providers WHERE id = sid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE SEQUENCE IF NOT EXISTS oidc_provider_id_seq;


CREATE TABLE IF NOT EXISTS oidc_providers (id bigint PRIMARY KEY DEFAULT next_id_v1('oidc_provider_id_seq'),
                                                                         description text, issuer text, base_url text, client_secret text, client_id text, verify_server_certificate bool,
                                                                                                                                                           ca_certs text, updated_at timestamptz,
                                                                                                                                                                          created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_oidc_provider_v1 (description text, issuer text, base_url text, client_secret text, client_id text, verify_server_certificate bool, ca_certs text) RETURNS
SETOF oidc_providers AS $$
                                  BEGIN
                                      RETURN QUERY INSERT INTO oidc_providers(description, issuer, base_url, client_secret, client_id , verify_server_certificate,ca_certs)
                                          VALUES (description, issuer, base_url, client_secret, client_id , verify_server_certificate,ca_certs)
                                          RETURNING *;
                                      RETURN;
                                  END
                              $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_oidc_provider_all_v1() RETURNS
SETOF oidc_providers AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM oidc_providers;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_odic_v1 (oid bigint) RETURNS
SETOF oidc_providers AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM oidc_providers WHERE id = oid;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE SEQUENCE IF NOT EXISTS origin_id_seq;


CREATE TABLE IF NOT EXISTS origins (id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('origin_id_seq'),
                                                                         name text UNIQUE,
                                                                                   type_meta JSONB,
                                                                                             object_meta JSONB,
                                                                                                         created_at timestamptz DEFAULT now(),
                                                                                                                                        updated_at timestamptz);


CREATE SEQUENCE IF NOT EXISTS origin_mem_id_seq;


CREATE TABLE IF NOT EXISTS origin_members (id bigint PRIMARY KEY DEFAULT next_id_v1('origin_mem_id_seq'),
                                                                         type_meta JSONB,
                                                                                   object_meta JSONB,
                                                                                               meta_data JSONB,
                                                                                                         created_at timestamptz DEFAULT now(),
                                                                                                                                        updated_at timestamptz);


CREATE SEQUENCE IF NOT EXISTS team_mem_id_seq;


CREATE TABLE IF NOT EXISTS team_members (id bigint PRIMARY KEY DEFAULT next_id_v1('team_mem_id_seq'),
                                                                       type_meta JSONB,
                                                                                 object_meta JSONB,
                                                                                             meta_data JSONB,
                                                                                                       created_at timestamptz DEFAULT now(),
                                                                                                                                      updated_at timestamptz);


CREATE OR REPLACE FUNCTION insert_origin_member_v1 (om_type_meta JSONB, om_obj_meta JSONB, om_meta_data JSONB) RETURNS void AS $$
               BEGIN
                   INSERT INTO origin_members ( type_meta, object_meta,meta_data)
                          VALUES (om_type_meta,om_obj_meta,om_meta_data);
               END
           $$ LANGUAGE PLPGSQL VOLATILE;


INSERT INTO origins (name, object_meta, type_meta)
VALUES ('rioos_system',
        '{"name":"rioos_system", "labels": {}, "account": "", "created_at": "", "deleted_at": "", "finalizers": [], "annotations": {}, "cluster_name": "", "initializers": {"result": {"code": 0, "reason": "", "status": "", "details": {"uid": "", "kind": "", "name": "", "group": "", "causes": [], "retry_after_seconds": 0}, "message": "", "type_meta": {"kind": "", "api_version": ""}}, "pending": []}, "owner_references": [{"uid": "", "kind": "", "name": "", "api_version": "", "block_owner_deletion": false}], "deletion_grace_period_seconds": 0}',
        '{"kind":"Origin","api_version":"v1"}') ON CONFLICT (name) DO NOTHING;


CREATE OR REPLACE FUNCTION insert_origin_v1 (origin_name text, origin_type_meta JSONB, origin_object_meta JSONB) RETURNS
SETOF origins AS $$
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
                           PERFORM insert_origin_member_v1('{"kind":"OriginMember","api_version":"v1"}',origin_object_meta, json_build_object('origin',inserted_origin.name)::jsonb);
                           RETURN NEXT inserted_origin;
                           RETURN;
                  END IF;
                  RETURN;
               END
                   $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION insert_team_member_v1 (om_type_meta JSONB, om_obj_meta JSONB, om_meta_data JSONB) RETURNS void AS $$
               BEGIN
                   INSERT INTO team_members ( type_meta, object_meta,meta_data)
                          VALUES (om_type_meta,om_obj_meta,om_meta_data);
               END
           $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_origins_v1() RETURNS
SETOF origins AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM origins;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_origin_v1 (org_name text) RETURNS
SETOF origins AS $$
                  BEGIN
                    RETURN QUERY SELECT * FROM origins WHERE name = org_name;
                    RETURN;
                  END
                  $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION list_origin_members_v1 (om_origin_id bigint) RETURNS TABLE(account_name text) AS $$
              BEGIN
                  RETURN QUERY SELECT origin_members.account_name FROM origin_members WHERE origin_id = om_origin_id
                    ORDER BY account_name ASC;
                  RETURN;
              END
              $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION check_account_in_origin_members_v1 (om_origin_name text, om_account_id bigint) RETURNS TABLE(is_member bool) AS $$
              BEGIN
                  RETURN QUERY SELECT true FROM origin_members WHERE origin_name = om_origin_name AND account_id = om_account_id;
                  RETURN;
              END
              $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION list_origin_by_account_id_v1 (o_account_id bigint) RETURNS TABLE(origin_name text) AS $$
              BEGIN
                  RETURN QUERY SELECT origin_members.origin_name FROM origin_members WHERE account_id = o_account_id
                    ORDER BY origin_name ASC;
                  RETURN;
              END
              $$ LANGUAGE PLPGSQL STABLE;


CREATE TABLE IF NOT EXISTS account_origins (account_id bigint, account_email text, origin_id bigint, origin_name text, created_at timestamptz DEFAULT now(),
                                                                                                                                                      updated_at timestamptz,
                                                                                                                                                      UNIQUE(account_id, origin_id));


CREATE OR REPLACE FUNCTION insert_account_origin_v1 (o_account_id bigint, o_account_email text, o_origin_id bigint, o_origin_name text) RETURNS void AS $$
                                                                                                                                                                    BEGIN
                                                                                                                                                                       INSERT INTO account_origins (account_id, account_email, origin_id, origin_name) VALUES (o_account_id, o_account_email, o_origin_id, o_origin_name);
                                                                                                                                                                    END
                                                                                                                                                                $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_account_origins_v1 (in_account_id bigint) RETURNS
SETOF account_origins AS $$
              BEGIN
                 RETURN QUERY SELECT * FROM account_origins WHERE account_id = in_account_id;
                 RETURN;
              END
          $$ LANGUAGE PLPGSQL STABLE;


CREATE SEQUENCE IF NOT EXISTS role_id_seq;


CREATE TABLE IF NOT EXISTS ROLES (id bigint PRIMARY KEY DEFAULT next_id_v1('role_id_seq'),
                                                                name text, description text, updated_at timestamptz,
                                                                                             created_at timestamptz DEFAULT now(),
                                                                                                                            UNIQUE (name));


CREATE OR REPLACE FUNCTION insert_role_v1 (name text, description text) RETURNS
SETOF ROLES AS $$
                      BEGIN
                          RETURN QUERY INSERT INTO roles(name, description)
                              VALUES (name, description)
                              RETURNING *;
                          RETURN;
                      END
                  $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_roles_v1 () RETURNS
SETOF ROLES AS $$
              BEGIN
                RETURN QUERY SELECT * FROM roles;
                RETURN;
              END
              $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_role_v1 (rid bigint) RETURNS
SETOF ROLES AS $$
              BEGIN
                RETURN QUERY SELECT * FROM roles WHERE id = rid;
                RETURN;
              END
              $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_role_by_name_v1 (rname text) RETURNS
SETOF ROLES AS $$
              BEGIN
                RETURN QUERY SELECT * FROM roles WHERE name = rname;
                RETURN;
              END
              $$ LANGUAGE PLPGSQL STABLE;


CREATE SEQUENCE IF NOT EXISTS perm_id_seq;


CREATE TABLE IF NOT EXISTS permissions (id bigint PRIMARY KEY DEFAULT next_id_v1('perm_id_seq'),
                                                                      role_id bigint REFERENCES roles(id),
                                                                                                name text, description text, updated_at timestamptz,
                                                                                                                             created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_permission_v1 (per_role_id bigint, per_name text, per_description text) RETURNS
SETOF permissions AS $$
                                                                                                                                            BEGIN
                                                                                                                                             IF EXISTS (SELECT true FROM roles WHERE id = per_role_id) THEN
                                                                                                                                                    RETURN QUERY INSERT INTO permissions (role_id, name, description)
                                                                                                                                                           VALUES (per_role_id, per_name, per_description)
                                                                                                                                                           ON CONFLICT DO NOTHING
                                                                                                                                                           RETURNING *;
                                                                                                                                                    RETURN;
                                                                                                                                                    END IF;
                                                                                                                                            END
                                                                                                                                        $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_permissions_v1 () RETURNS
SETOF permissions AS $$
              BEGIN
                RETURN QUERY SELECT * FROM permissions;
                RETURN;
              END
              $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_permission_v1 (pid bigint) RETURNS
SETOF permissions AS $$
              BEGIN
                RETURN QUERY SELECT * FROM permissions WHERE id = pid;
                RETURN;
              END
              $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_permission_for_role_v1 (rid bigint) RETURNS
SETOF permissions AS $$
             BEGIN
                 RETURN QUERY SELECT * FROM permissions WHERE role_id = rid
                   ORDER BY name ASC;
                 RETURN;
             END
             $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_permission_by_role_name_v1 (rname text) RETURNS
SETOF permissions AS $$
                   DECLARE
                      this_role roles%rowtype;
                   BEGIN
                       SELECT * FROM roles WHERE name = rname LIMIT 1 INTO this_role;
                       RETURN QUERY SELECT * FROM permissions WHERE role_id = this_role.id;
                       RETURN;
                           END
                           $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION get_specfic_permission_role_v1 (perm_id bigint, rid bigint) RETURNS
SETOF permissions AS $$
             BEGIN
                 RETURN QUERY SELECT * FROM permissions WHERE role_id = rid AND id = perm_id
                   ORDER BY name ASC;
                 RETURN;
             END
             $$ LANGUAGE PLPGSQL STABLE;


CREATE SEQUENCE IF NOT EXISTS passticket_id_seq;


CREATE TABLE IF NOT EXISTS passtickets (id bigint PRIMARY KEY DEFAULT next_id_v1('passticket_id_seq'),
                                                                      passticket text, created_at timestamptz DEFAULT now());


CREATE OR REPLACE FUNCTION insert_passticket_v1 (o_passticket text) RETURNS
SETOF passtickets AS $$
                                                                                     BEGIN
                                                                                        RETURN QUERY INSERT INTO passtickets (passticket) VALUES (o_passticket)
                                                                                        RETURNING *;
                                                                                    RETURN;
                                                                                     END
                                                                                 $$ LANGUAGE PLPGSQL VOLATILE;


CREATE OR REPLACE FUNCTION get_passticket_v1 (o_passticket text) RETURNS
SETOF passtickets AS $$
                                                                                               BEGIN
                                                                                                 RETURN QUERY SELECT * FROM passtickets WHERE passticket = o_passticket;
                                                                                                 RETURN;
                                                                                               END
                                                                                               $$ LANGUAGE PLPGSQL STABLE;


CREATE OR REPLACE FUNCTION remove_passticket_v1 (o_passticket text) RETURNS void AS $$
                                                                                                             BEGIN
                                                                                                                DELETE FROM passtickets WHERE passticket = o_passticket;
                                                                                                             END
                                                                                                             $$ LANGUAGE PLPGSQL VOLATILE
