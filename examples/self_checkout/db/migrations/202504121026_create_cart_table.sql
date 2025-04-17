create table if not exists cart_items (
  id integer primary key,
  product_id integer not null unique,
  number_of_items integer not null default 1,
  foreign key(product_id) references products(id)
);
