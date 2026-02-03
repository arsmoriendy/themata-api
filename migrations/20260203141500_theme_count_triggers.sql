CREATE OR REPLACE PROCEDURE proc_inc_theme_count_by(
  theme_ulid BYTEA, inc INTEGER
)
LANGUAGE plpgsql
AS $function$
BEGIN
  UPDATE users SET theme_count = theme_count + inc
  WHERE ulid = (SELECT owner FROM themes WHERE ulid = theme_ulid);

  UPDATE theme_count SET theme_count = theme_count + inc;
END;
$function$;

CREATE OR REPLACE FUNCTION fn_inc_theme_count() RETURNS TRIGGER LANGUAGE plpgsql AS
$function$
BEGIN
  CALL proc_inc_theme_count_by(NEW.ulid, 1);
  RETURN NULL;
END;
$function$;

CREATE OR REPLACE FUNCTION fn_dec_theme_count() RETURNS TRIGGER LANGUAGE plpgsql AS
$function$
BEGIN
  CALL proc_inc_theme_count_by(OLD.ulid, -1);
  RETURN OLD;
END;
$function$;

CREATE OR REPLACE TRIGGER trg_inc_theme_count AFTER INSERT ON themes FOR EACH ROW EXECUTE FUNCTION fn_inc_theme_count();

CREATE OR REPLACE TRIGGER trg_dec_theme_count BEFORE DELETE ON themes FOR EACH ROW EXECUTE FUNCTION fn_dec_theme_count();
