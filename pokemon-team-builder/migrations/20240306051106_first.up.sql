-- defining types
CREATE TYPE Game AS enum (
    'redblue',
    'goldsilver',
    'rubysapphire',
    'diamondpearl',
    'blackwhite',
    'xy',
    'sunmoon',
    'swordshield',
    'scarletviolet'
);

CREATE TYPE Gender AS enum (
    'male',
    'female',
    'unknown'
);

CREATE TYPE Nature AS enum (
    'adamant',
    'bashful',
    'bold',
    'brave',
    'calm',
    'careful',
    'docile',
    'gentle',
    'hardy',
    'hasty',
    'impish',
    'jolly',
    'lax',
    'lonely',
    'mild',
    'modest',
    'naive',
    'naughty',
    'quiet',
    'quirky',
    'rash',
    'relaxed',
    'sassy',
    'serious',
    'timid'
);

CREATE TYPE Region AS enum (
    'kanto',
    'johto',
    'hoenn',
    'hisui',
    'sinnoh',
    'unova',
    'kalos',
    'alola',
    'galar',
    'paldea'
);

CREATE TYPE Stat AS enum (
    'hp',
    'attack',
    'defense',
    'spattack',
    'spdefense',
    'speed'
);

CREATE TYPE Tier AS enum (
    'ag',
    'uber',
    'ou',
    'uu',
    'ru',
    'nu',
    'pu',
    'zu'
);

CREATE TYPE Type AS enum (
    'normal',
    'fire',
    'water',
    'grass',
    'flying',
    'fighting',
    'poison',
    'electric',
    'ground',
    'rock',
    'psychic',
    'ice',
    'bug',
    'ghost',
    'steel',
    'dragon',
    'dark',
    'fairy'
);

-- defining tables
CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    name varchar(255) NOT NULL,
    email varchar(255) NOT NULL UNIQUE,
    password varchar(100) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ability (
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL UNIQUE,
    is_main_series boolean DEFAULT TRUE,
    generation Game NOT NULL,
    effect_entry text NOT NULL,
    effect_changes text NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS generation (
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL UNIQUE,
    main_region Region NOT NULL,
    types Type[],
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS pokedex (
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL UNIQUE,
    is_main_series boolean DEFAULT TRUE,
    description text NOT NULL,
    region Region NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS pokemon_move (
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL UNIQUE,
    accuracy integer,
    effect_change integer,
    pp integer,
    priority integer NOT NULL,
    power integer,
    type Type NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS pokemon (
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL UNIQUE,
    base_experience integer,
    height real NOT NULL,
    weight real NOT NULL,
    primary_type Type NOT NULL,
    secondary_type Type,
    primary_ability integer NOT NULL,
    secondary_ability integer,
    hidden_ability integer,
    is_main_series boolean DEFAULT TRUE,
    pokedex integer NOT NULL,
    origin_region Region NOT NULL,
    games Game[] NOT NULL,
    form_names varchar(100)[],
    is_mythical boolean DEFAULT FALSE,
    is_legendary boolean DEFAULT FALSE,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (primary_ability) REFERENCES ability (id),
    FOREIGN KEY (secondary_ability) REFERENCES ability (id),
    FOREIGN KEY (hidden_ability) REFERENCES ability (id),
    FOREIGN KEY (pokedex) REFERENCES pokedex (id)
);

CREATE TABLE IF NOT EXISTS pokemonstat (
    id serial PRIMARY KEY,
    pokemon integer NOT NULL,
    kind Stat NOT NULL,
    value integer NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (pokemon) REFERENCES pokemon (id)
);

CREATE TABLE IF NOT EXISTS pokemonsprites (
    id serial PRIMARY KEY,
    pokemon integer NOT NULL,
    front_default text,
    front_shiny text,
    front_female text,
    front_shiny_female text,
    back_default text,
    back_shiny text,
    back_female text,
    back_shiny_female text,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (pokemon) REFERENCES pokemon (id)
);

CREATE TABLE IF NOT EXISTS team (
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL,
    description text NOT NULL,
    user_id integer NOT NULL,
    tier Tier NOT NULL,
    is_favourite boolean DEFAULT FALSE,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS team_pokemon (
    id serial PRIMARY KEY,
    team integer NOT NULL,
    pokemon integer NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (team) REFERENCES team (id),
    FOREIGN KEY (pokemon) REFERENCES pokemon (id)
);

CREATE TABLE IF NOT EXISTS move_pokemon (
    id serial PRIMARY KEY,
    move integer NOT NULL,
    pokemon integer NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (MOVE) REFERENCES MOVE (id),
    FOREIGN KEY (pokemon) REFERENCES pokemon (id)
);
