---
--- Table:passtickets
---
CREATE SEQUENCE IF NOT EXISTS passticket_id_seq;
CREATE TABLE IF NOT EXISTS passtickets (id bigint PRIMARY KEY DEFAULT next_id_v1('passticket_id_seq'), passticket text, created_at timestamptz DEFAULT now());

---
--- Table:passtickets:create
---
CREATE 
OR REPLACE FUNCTION insert_passticket_v1 (o_passticket text) RETURNS SETOF passtickets AS $$ 
BEGIN
   RETURN QUERY 
   INSERT INTO
      passtickets (passticket) 
   VALUES
      (
         o_passticket
      )
      RETURNING *;
RETURN;
END
$$ LANGUAGE PLPGSQL VOLATILE;

---
--- Table:passtickets:show
---
CREATE 
OR REPLACE FUNCTION get_passticket_v1 (o_passticket text) RETURNS SETOF passtickets AS $$ 
BEGIN
   RETURN QUERY 
   SELECT
      * 
   FROM
      passtickets 
   WHERE
      passticket = o_passticket;
RETURN;
END
$$ LANGUAGE PLPGSQL STABLE;

---
--- Table:passtickets:remove
---
CREATE 
OR REPLACE FUNCTION remove_passticket_v1 (o_passticket text) RETURNS void AS $$ 
BEGIN
   DELETE
   FROM
      passtickets 
   WHERE
      passticket = o_passticket;
END
$$ LANGUAGE PLPGSQL VOLATILE