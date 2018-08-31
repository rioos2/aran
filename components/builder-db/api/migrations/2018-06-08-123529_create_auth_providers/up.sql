---
---
--- Table:ldap_configs
---
CREATE SEQUENCE IF NOT EXISTS ldap_id_seq;
CREATE TABLE IF NOT EXISTS ldap_configs (id bigint PRIMARY KEY DEFAULT next_id_v1('ldap_id_seq'), HOST text, port text, enforce_starttls bool, use_ldaps bool, lookup_dn text, lookup_password text, ca_certs text, client_cert text, user_search text, group_search text, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:ldap_configs:create
---
CREATE 
OR REPLACE FUNCTION insert_ldap_config_v1 (HOST text, port text, enforce_starttls bool, use_ldaps bool, lookup_dn text, lookup_password text, ca_certs text, client_cert text, user_search text, group_search text) RETURNS SETOF ldap_configs AS $$ 
BEGIN
   RETURN QUERY 
   INSERT INTO
      ldap_configs(host, port, enforce_starttls, use_ldaps, lookup_dn, lookup_password, ca_certs, client_cert, user_search, group_search) 
   VALUES
      (
         host,
         port,
         enforce_starttls,
         use_ldaps,
         lookup_dn,
         lookup_password,
         ca_certs,
         client_cert,
         user_search,
         group_search
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:ldap_configs:show
---
CREATE 
OR REPLACE FUNCTION get_ldap_config_v1 (aid bigint) RETURNS SETOF ldap_configs AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      ldap_configs 
   WHERE
      id = aid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:saml_providers
---
CREATE SEQUENCE IF NOT EXISTS saml_provider_id_seq;
CREATE TABLE IF NOT EXISTS saml_providers (id bigint PRIMARY KEY DEFAULT next_id_v1('saml_provider_id_seq'), description text, idp_metadata text, sp_base_url text, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:saml_providers:create
---
CREATE 
OR REPLACE FUNCTION insert_saml_provider_v1 (description text, idp_metadata text, sp_base_url text) RETURNS SETOF saml_providers AS $$ 
BEGIN
   RETURN QUERY 
   INSERT INTO
      saml_providers(description, idp_metadata, sp_base_url) 
   VALUES
      (
         description,
         idp_metadata,
         sp_base_url
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:saml_providers:list_blank
---
CREATE 
OR REPLACE FUNCTION get_saml_provider_all_v1() RETURNS SETOF saml_providers AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      saml_providers;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:saml_providers:show
---
CREATE 
OR REPLACE FUNCTION get_saml_v1 (sid bigint) RETURNS SETOF saml_providers AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      saml_providers 
   WHERE
      id = sid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:oidc_providers
---
CREATE SEQUENCE IF NOT EXISTS oidc_provider_id_seq;
CREATE TABLE IF NOT EXISTS oidc_providers (id bigint PRIMARY KEY DEFAULT next_id_v1('oidc_provider_id_seq'), description text, issuer text, base_url text, client_secret text, client_id text, verify_server_certificate bool, ca_certs text, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:oidc_providers:create
---
CREATE 
OR REPLACE FUNCTION insert_oidc_provider_v1 (description text, issuer text, base_url text, client_secret text, client_id text, verify_server_certificate bool, ca_certs text) RETURNS SETOF oidc_providers AS $$ 
BEGIN
   RETURN QUERY 
   INSERT INTO
      oidc_providers(description, issuer, base_url, client_secret, client_id , verify_server_certificate, ca_certs) 
   VALUES
      (
         description,
         issuer,
         base_url,
         client_secret,
         client_id,
         verify_server_certificate,
         ca_certs
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:oidc_providers:list_blank
---
CREATE 
OR REPLACE FUNCTION get_oidc_provider_all_v1() RETURNS SETOF oidc_providers AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      oidc_providers;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:oidc_providers:show
---
CREATE 
OR REPLACE FUNCTION get_odic_v1 (oid bigint) RETURNS SETOF oidc_providers AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      oidc_providers 
   WHERE
      id = oid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;
