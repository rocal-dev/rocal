create table if not exists sales (
  id integer primary key,
  created_at datetime default current_timestamp
);

create table if not exists sales_items (
  id integer primary key,
  sales_id integer not null,
  product_id integer not null,
  product_name text not null,
  product_price real not null,
  number_of_items integer not null,
  foreign key(sales_id) references sales(id),
  foreign key(product_id) references products(id)
);
