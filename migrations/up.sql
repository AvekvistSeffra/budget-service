CREATE DATABASE IF NOT EXISTS budget_service;

-- Run the following on the database after it is created

CREATE TYPE income_type AS ENUM ('hourly', 'monthly');
CREATE TYPE currency AS ENUM ('sek', 'eur', 'usd');

CREATE TABLE IF NOT EXISTS person (
    id SERIAL,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS expense (
    id SERIAL,
    person_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    source DECIMAL(10,2) NOT NULL,
    calculated DECIMAL(10,2) GENERATED ALWAYS AS (source * conversion_rate) STORED,
    currency currency NOT NULL,
    category VARCHAR(255) NOT NULL,
    conversion_rate DECIMAL(10,2) NOT NULL DEFAULT 1,
    PRIMARY KEY (id),
    CONSTRAINT fk_person
        FOREIGN KEY(person_id) REFERENCES person(id)
);

CREATE TABLE IF NOT EXISTS income (
    id SERIAL,
    person_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    gross DECIMAL(10,2) NOT NULL,
    rate DECIMAL(10,2) NOT NULL,
    vat DECIMAL(10,2) NOT NULL,
    netto DECIMAL(10,2) GENERATED ALWAYS AS (gross * rate * (1 - vat)) STORED,
    category VARCHAR(255) NOT NULL,
    type income_type NOT NULL,
    currency currency NOT NULL DEFAULT 'sek',
    PRIMARY KEY (id),
    CONSTRAINT fk_person
        FOREIGN KEY(person_id) REFERENCES person(id)
);

CREATE FUNCTION savings(IN uid INTEGER) RETURNS NUMERIC
LANGUAGE SQL
AS $$
SELECT
    SUM(netto) -
    (
        SELECT
            SUM(calculated)
        FROM
            expense
        WHERE
            expense.person_id = income.person_id 
    )
FROM
    income
WHERE
    person_id = uid
GROUP BY
    person_id
$$;
