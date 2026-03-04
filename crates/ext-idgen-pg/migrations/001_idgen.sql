CREATE SEQUENCE transfer_id_seq;

CREATE FUNCTION next_transfer_ids (count INTEGER, lock_key BIGINT, OUT seq_start BIGINT, OUT seq_end BIGINT) AS $$
BEGIN
  PERFORM pg_advisory_lock(lock_key);

  BEGIN
    seq_start := greatest(
      nextval('transfer_id_seq'),
      (extract(epoch FROM clock_timestamp()) * 1000000)::BIGINT
    );
    seq_end := seq_start + count - 1;
    PERFORM setval('transfer_id_seq', seq_end);
  EXCEPTION WHEN OTHERS THEN
    PERFORM pg_advisory_unlock(lock_key);
    RAISE;
  END;

  PERFORM pg_advisory_unlock(lock_key);
END;
$$ LANGUAGE plpgsql;
