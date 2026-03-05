CREATE SEQUENCE transfer_id_seq;

CREATE FUNCTION next_id_bases (count INTEGER, lock_key BIGINT, epoch_offset BIGINT DEFAULT 0, OUT seq_start BIGINT, OUT seq_end BIGINT) AS $$
BEGIN
  IF count <= 0 THEN
    RAISE EXCEPTION 'count must be positive';
  END IF;

  PERFORM pg_advisory_lock(lock_key);

  BEGIN
    SELECT last_value + 1 INTO seq_start FROM transfer_id_seq;
    seq_start := greatest(
      seq_start,
      ((extract(epoch FROM clock_timestamp()) - epoch_offset) * 1000000)::BIGINT
    );
    seq_end := seq_start + count - 1;
    PERFORM setval('transfer_id_seq', seq_end, false);
  EXCEPTION WHEN OTHERS THEN
    PERFORM pg_advisory_unlock(lock_key);
    RAISE;
  END;

  PERFORM pg_advisory_unlock(lock_key);
END;
$$ LANGUAGE plpgsql;
