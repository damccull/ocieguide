CREATE TABLE ocieitems(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    nsn TEXT NOT NULL,
    lin TEXT NOT NULL,
    nomenclature TEXT NOT NULL,
    size TEXT,
    unit_of_issue TEXT,
    price DECIMAL
);
