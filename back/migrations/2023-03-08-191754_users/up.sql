CREATE TABLE users (
    user_id INTEGER  PRIMARY KEY AUTOINCREMENT NOT NULL ,
    name TEXT NOT NULL,
    age INTEGER NOT NULL DEFAULT 0,
    alive BOOLEAN NOT NULL DEFAULT 1
);

INSERT INTO users (name,age) VALUES
  ('Михаил',23),
  ('Борис(Кот)',5),
  ('Саня',-3),
  ('Кирилл',345);