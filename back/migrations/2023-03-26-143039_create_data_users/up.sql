
DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id      INTEGER  PRIMARY KEY AUTOINCREMENT NOT NULL, 
    username     TEXT UNIQUE          NOT NULL,
    password     TEXT                 NOT NULL,
    first_name   TEXT                     NULL,
    last_name    TEXT                     NULL,
    patronymic   TEXT                     NULL,
    block        INTEGER              NOT NULL DEFAULT 0,
    last_active  DATETIME             NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users 
( username     , password   , first_name , last_name   , patronymic    ) VALUES
('Adeyanander' , 'qwerty123', 'Казаков'  , 'Аввакуум'  , 'Юрьевич'     ),
('Washomanj'   , 'qwerty123', 'Мельников', 'Леонтий'   , 'Фролович'    ),
('Puaaaaaaaa'  , 'qwerty123', 'Князев'   , 'Виталий'   , 'Яковович'    ),
('Xueaaaaa'    , 'qwerty123', 'Медведев' , 'Владимир'  , 'Игнатьевич'  ),
('Dicenadiso'  , 'qwerty123', 'Сазонов'  , 'Лаврентий' , 'Альбертович' ),
('Verlitanitok', 'qwerty123', 'Давыдов'  , 'Самуил'    , 'Борисович'   ),
('Saliyann'    , 'qwerty123', 'Давыдова' , 'Владислава', 'Донатовна'   ),
('Orydennan'   , 'qwerty123', 'Логинова' , 'Лада'      , 'Гордеевна'   ),
('Ovanetti'    , 'qwerty123', 'Ларионова', 'Андриана'  , 'Мэлоровна'   );

--------------------------------------------------------------

DROP TABLE IF EXISTS privileges;
CREATE TABLE privileges (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    privilege_name TEXT NOT NULL
);


INSERT INTO privileges 
( privilege_name      ) VALUES
( 'page_counterparty' ),
( 'edit_counterparty' ),
( 'page_drivers'      ),
( 'edit_drivers'      ),
( 'page_trucks'       ),
( 'edit_trucks'       ),
( 'page_users'        ),
( 'edit_users'        ),
( 'page_requests'     ),
( 'edit_requests'     );


DROP TABLE IF EXISTS user_privileges;
CREATE TABLE user_privileges (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL,
    privilege_id INTEGER NOT NULL
);

INSERT INTO user_privileges 
(user_id,privilege_id) VALUES
( 7      , 5         );




