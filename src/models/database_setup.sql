--This file will be used to setup the base Table used for testing/logging
--Follow these instruction to setup the user account and schema as required, but change it for whatever new table you require

/* Drop if already exsists */
drop table if exists handler_logs;

/* Create the handler_logs table */
create table
    hanlder_logs (
        log_id SERIAL PRIMAY KEY,
        handler VARCHAR(250) NOT NULL,
        message VARCHAR(250),
        created_at TIMESTAMP DEFAULT now ()
    );

/* Load seed data for testing and verification */
INSERT INTO
    handler_logs (log_id, handler, message)
VALUES
    (
        1,
        "None",
        "This message was created by testing/creation of DB"
    )
