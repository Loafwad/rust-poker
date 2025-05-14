CREATE TYPE hand_kind AS ENUM(
    'high_card',
    'one_pair',
    'two_pair',
    'three_of_a_kind',
    'straight',
    'flush',
    'full_house',
    'four_of_a_kind',
    'straight_flush',
    'royal_flush'
);

CREATE TABLE hands(
    id serial PRIMARY KEY,
    cards text NOT NULL,
    hand_kind HAND_KIND NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP
);

