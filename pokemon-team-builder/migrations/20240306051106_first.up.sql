-- defining types
create type Game as enum ('redblue', 'goldsilver', 'rubysapphire', 'diamondpearl', 'blackwhite', 'xy', 'sunmoon', 'swordshield', 'scarletviolet');
create type Gender as enum ('male', 'female', 'unknown');
create type Nature as enum ('adamant', 'bashful', 'bold', 'brave', 'calm', 'careful', 'docile', 'gentle', 'hardy', 'hasty', 'impish', 'jolly', 'lax', 'lonely', 'mild', 'modest', 'naive', 'naughty', 'quiet', 'quirky', 'rash', 'relaxed', 'sassy', 'serious', 'timid');
create type Region as enum ('kanto', 'johto', 'hoenn', 'hisui', 'sinnoh', 'unova', 'kalos', 'alola', 'galar', 'paldea');
create type Stat as enum ('hp', 'attack', 'defense', 'spattack', 'spdefense', 'speed');
create type Tier as enum ('ag', 'uber', 'ou', 'uu', 'ru', 'nu', 'pu', 'zu');
create type Type as enum ('normal', 'fire', 'water', 'grass', 'flying', 'fighting', 'poison', 'electric', 'ground', 'rock', 'psychic', 'ice', 'bug', 'ghost', 'steel', 'dragon', 'dark', 'fairy');

-- defining tables
create table if not exists users (
    id serial primary key,
    name varchar (255) not null,
    email varchar (255) not null unique,
    password varchar (100) not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp
);

create table if not exists ability (
    id serial primary key,
    name varchar (100) not null unique,
    is_main_series boolean default true,
    generation Game not null,
    effect_entry text not null,
    effect_changes text not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp
);

create table if not exists generation (
    id serial primary key,
    name varchar(100) not null unique,
    main_region Region not null,
    types Type[],
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp
);

create table if not exists pokedex (
    id serial primary key,
    name varchar(100) not null unique,
    is_main_series boolean default true,
    description text not null,
    region Region not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp
);

create table if not exists move (
    id serial primary key,
    name varchar(100) not null unique,
    accuracy integer,
    effect_change integer,
    pp integer,
    priority integer not null,
    power integer,
    type Type not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp
);

create table if not exists pokemon (
    id serial primary key,
    name varchar(100) not null unique,
    base_experience integer,
    height real not null,
    weight real not null,
    primary_type Type not null,
    secondary_type Type,
    primary_ability integer not null,
    secondary_ability integer,
    hidden_ability integer,
    is_main_series boolean default true,
    pokedex integer not null,
    origin_region Region not null,
    games Game[] not null,
    form_names varchar(100)[],
    is_mythical boolean default false,
    is_legendary boolean default false,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp,

    foreign key (primary_ability) references ability(id),
    foreign key (secondary_ability) references ability(id),
    foreign key (hidden_ability) references ability(id),
    foreign key (pokedex) references pokedex(id)
);

create table if not exists pokemonstat (
    id serial primary key,
    pokemon integer not null,
    kind Stat not null,
    value integer not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp,

    foreign key (pokemon) references pokemon(id)
);

create table if not exists pokemonsprites (
    id serial primary key,
    pokemon integer not null,
    front_default text,
    front_shiny text,
    front_female text,
    front_shiny_female text,
    back_default text,
    back_shiny text,
    back_female text,
    back_shiny_female text,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp,

    foreign key (pokemon) references pokemon(id)
);

create table if not exists team (
    id serial primary key,
    name varchar(100) not null,
    description text not null,
    user_id integer not null,
    tier Tier not null,
    is_favourite boolean default false,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp,

    foreign key (user_id) references users (id)
);

create table if not exists team_pokemon (
    id serial primary key,
    team integer not null,
    pokemon integer not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp,

    foreign key (team) references team(id),
    foreign key (pokemon) references pokemon(id)
);

create table if not exists move_pokemon (
    id serial primary key,
    move integer not null,
    pokemon integer not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp,

    foreign key (move) references move(id),
    foreign key (pokemon) references pokemon(id)
);