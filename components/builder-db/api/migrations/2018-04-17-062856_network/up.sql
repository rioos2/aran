---
---
--- Table:networks
---
CREATE SEQUENCE IF NOT EXISTS net_id_seq;
CREATE TABLE IF NOT EXISTS networks (id bigint PRIMARY KEY DEFAULT next_id_v1('net_id_seq'), network_type text, subnet_ip text, netmask text, gateway text, used_bits smallint[], bridge_hosts JSONB, status JSONB, type_meta JSONB, object_meta JSONB, updated_at timestamptz, created_at timestamptz DEFAULT now());

---
--- Table:networks:create
---
CREATE 
OR REPLACE FUNCTION insert_network_v1 (network_type text, subnet_ip text, netmask text, gateway text, used_bits smallint[], bridge_hosts JSONB, status JSONB, object_meta JSONB, type_meta JSONB) RETURNS SETOF networks AS $$ 
BEGIN
   RETURN QUERY 
   INSERT INTO
      networks(network_type, subnet_ip, netmask, gateway, used_bits, bridge_hosts, status, object_meta, type_meta ) 
   VALUES
      (
         network_type, subnet_ip, netmask, gateway, used_bits, bridge_hosts, status, object_meta, type_meta
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:networks:list_blank
---
CREATE 
OR REPLACE FUNCTION get_networks_v1() RETURNS SETOF networks AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      networks;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:networks:update
---
CREATE 
OR REPLACE FUNCTION update_net_v1(nid bigint, n_network_type text, n_subnet_ip text, n_netmask text, n_gateway text, n_used_bits smallint[], n_bridge_hosts JSONB, n_status JSONB, n_object_meta JSONB) RETURNS SETOF networks AS $$ 
BEGIN
   RETURN QUERY 
   UPDATE
      networks 
   SET
      network_type = n_network_type,
      subnet_ip = n_subnet_ip,
      netmask = n_netmask,
      gateway = n_gateway,
      used_bits = n_used_bits,
      bridge_hosts = n_bridge_hosts,
      status = n_status,
      object_meta = n_object_meta,
      updated_at = now() 
   WHERE
      id = nid RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:networks:show
---
CREATE 
OR REPLACE FUNCTION get_network_v1(nid bigint) RETURNS SETOF networks AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      networks 
   where
      id = nid;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;