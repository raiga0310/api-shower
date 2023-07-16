CREATE TABLE sections (
    id SERIAL PRIMARY KEY,
    building VARCHAR(1) NOT NULL CHECK (building IN('A', 'B', 'C')),
    floor INT NOT NULL CHECK (floor >= 1 AND floor <= 4),
    gender TEXT NOT NULL CHECK (gender IN('male', 'female')),
    total INT NOT NULL,
    available INT NOT NULL,
    occupied INT NOT NULL DEFAULT 0,
    disabled_rooms INT NOT NULL DEFAULT 0,
    CHECK (total >= available + occupied + disabled_rooms)
);

CREATE TABLE usage_history (
    id SERIAL PRIMARY KEY,
    section_id INTEGER REFERENCES sections (id),
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP
);

DO
$$
DECLARE
    buildings CHAR(1)[] := ARRAY['A', 'B', 'C'];
    building CHAR(1);
    floor INTEGER;
    gender TEXT;
BEGIN
    FOREACH building IN ARRAY buildings LOOP
        FOR floor IN 1..4 LOOP
            FOREACH gender IN ARRAY ARRAY ['male', 'female'] LOOP
                INSERT INTO sections (building, floor, gender, total, available)
                VALUES (building, floor, gender, 10, 10);
            END LOOP;
        END LOOP;
    END LOOP;
END
$$;
