DROP TABLE IF EXISTS attempts

CREATE TABLE attempts {
       id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
       successfull BOOLEAN NOT NULL DEFAULT FALSE 
}