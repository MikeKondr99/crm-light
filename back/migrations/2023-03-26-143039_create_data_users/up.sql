DROP TABLE IF EXISTS users;
-- Your SQL goes here
CREATE TABLE users (
    user_id      INTEGER  PRIMARY KEY AUTOINCREMENT NOT NULL, 
    username     TEXT UNIQUE          NOT NULL,
    password     TEXT                 NOT NULL,
    first_name   TEXT                     NULL,
    last_name    TEXT                     NULL,
    patronymic   TEXT                     NULL,
    block        INTEGER              NOT NULL DEFAULT 0,
    last_active  DATETIME             NOT NULL DEFAULT CURRENT_TIMESTAMP,
    privilege_id INTEGER              NOT NULL DEFAULT 1
);

INSERT INTO users (username, password, first_name,last_name,patronymic) VALUES
('Adeyanander', 'qwerty123', 'Казаков', 'Аввакуум', 'Юрьевич'),
('Washomanj', 'qwerty123', 'Мельников', 'Леонтий', 'Фролович'),
('Puaaaaaaaa', 'qwerty123', 'Князев', 'Виталий', 'Яковович'),
('Xueaaaaa', 'qwerty123', 'Медведев', 'Владимир', 'Игнатьевич'),
('Dicenadiso', 'qwerty123', 'Сазонов', 'Лаврентий', 'Альбертович'),
('Verlitanitok', 'qwerty123', 'Давыдов', 'Самуил', 'Борисович'),
('Saliyann', 'qwerty123', 'Давыдова', 'Владислава', 'Донатовна'),
('Orydennan', 'qwerty123', 'Логинова', 'Лада', 'Гордеевна'),
('Ovanetti', 'qwerty123', 'Ларионова', 'Андриана', 'Мэлоровна');


CREATE TABLE privileges (
    privilege_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    privilege_name TEXT NOT NULL
);

INSERT INTO privileges (privilege_name) VALUES
('Гость'),
('Админ'),
('Бухгалтер'),
('Менеджер');


