CREATE TABLE shower_sections (
    id SERIAL PRIMARY KEY,
    building VARCHAR(1) NOT NULL CHECK (building IN('A', 'B', 'C')),
    floor INT NOT NULL CHECK (floor >= 1 AND floor <= 4),
    gender TEXT NOT NULL CHECK (gender IN('male', 'female')),
    total_rooms INT NOT NULL,
    in_use_rooms INT NOT NULL DEFAULT 0,
    disabled_rooms INT NOT NULL DEFAULT 0,
    CHECK (total_rooms >= in_use_rooms + disabled_rooms)
);

CREATE TABLE usage_history (
    id SERIAL PRIMARY KEY,
    section_id INTEGER REFERENCES shower_sections (id),
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
            FOR gender IN ARRAY ARRAY['male', 'female'] LOOP
                INSERT INTO shower_sections (building, floor, gender, total_rooms)
                VALUES (building, floor, gender, 10);
            END LOOP;
        END LOOP;
    END LOOP;
END
$$;
