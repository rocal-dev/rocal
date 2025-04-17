create table if not exists products (
  id integer primary key,
  name text not null,
  price real not null,
  created_at datetime default current_timestamp
);

insert or ignore into products (id, name, price)
values
  (1, 'Apple', 2.99),
  (2, 'Orange', 1.99),
  (3, 'Banana', 0.99),
  (4, 'Grapes', 3.49),
  (5, 'Strawberry', 4.99),
  (6, 'Watermelon', 5.99),
  (7, 'Blueberry', 6.49),
  (8, 'Pineapple', 3.99),
  (9, 'Mango', 2.49),
  (10, 'Kiwi', 1.49),
  (11, 'Potato Chips', 2.79),
  (12, 'Chocolate Bar', 1.49),
  (13, 'Milk (1L)', 1.99),
  (14, 'Bread Loaf', 2.49),
  (15, 'Eggs (12-pack)', 3.59),
  (16, 'Butter (250g)', 3.19),
  (17, 'Cheddar Cheese', 4.29),
  (18, 'Orange Juice (1L)', 3.89),
  (19, 'Yogurt (500g)', 2.29),
  (20, 'Bottled Water (500ml)', 0.99);
