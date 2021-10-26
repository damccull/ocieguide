CREATE TABLE ocieitems(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    nsn TEXT,
    lin TEXT,
    nomenclature TEXT,
    size TEXT,
    unit_of_issue TEXT,
    price DECIMAL
);
