-- Add migration script here
DROP TABLE IF EXISTS cart_products;
DROP TABLE IF EXISTS carts;
DROP TABLE IF EXISTS order_products;
DROP TABLE IF EXISTS reviews;
DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS orders;
DROP TABLE IF EXISTS users;

DROP TYPE IF EXISTS category;
create type category as enum ('smartphones', 'laptops', 'tablets', 'cameras', 'iems', 'headphones', 'videogames', 'consoles', 'musicalbums', 'movies', 'figures', 'playingcards');
DROP TYPE IF EXISTS user_role;
create type user_role as enum ('admin', 'customer');

CREATE TABLE IF NOT EXISTS users
(
    id serial primary key,
    username varchar (255) not null,
    email varchar (255) unique not null,
    password varchar (255) not null,
    address varchar (255) not null,
    users_role user_role not null,
    created_at timestamptz default current_timestamp,
    update_at timestamptz default current_timestamp
);

CREATE TABLE IF NOT EXISTS carts
(
    id serial primary key,
    user_id int not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp,
    foreign key (user_id) references users (id)
);

CREATE TABLE IF NOT EXISTS orders
(
    id serial primary key,
    user_id int not null,
    destination varchar (255) not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp,
    foreign key (user_id) references users (id)
);

CREATE TABLE IF NOT EXISTS products
(
    id serial primary key,
    title varchar (255) not null,
    description varchar (255) not null,
    price float not null,
    quantity int not null,
    category category not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp
);

CREATE TABLE IF NOT EXISTS reviews
(
    id serial primary key,
    user_id int not null,
    product_id int not null,
    rating float not null,
    review varchar (255) not null,
    created_at timestamptz default current_timestamp,
    updated_at timestamptz default current_timestamp,
    foreign key (user_id) references users (id),
    foreign key (product_id) references products (id)
);

CREATE TABLE IF NOT EXISTS cart_products
(
    id serial primary key,
    cart_id int,
    product_id int,
    foreign key (cart_id) references carts (id),
    foreign key (product_id) references products (id)
);

CREATE TABLE IF NOT EXISTS order_products
(
    id serial primary key,
    order_id int,
    product_id int,
    foreign key (order_id) references orders (id),
    foreign key (product_id) references products (id)
);