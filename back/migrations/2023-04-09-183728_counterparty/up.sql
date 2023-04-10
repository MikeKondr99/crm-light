

DROP TABLE IF EXISTS counterparties;
CREATE TABLE counterparties (
    id           INTEGER  PRIMARY KEY AUTOINCREMENT NOT NULL, 
    inn          TEXT                               NOT NULL,
    name         TEXT                               NOT NULL,
    vat_id       INTEGER                            NOT NULL,
    kpp          TEXT                               NOT NULL,
    ogrn         TEXT                               NOT NULL,
    bik          TEXT                               NOT NULL,
    role_id      INTEGER                            NOT NULL,
    status_id    INTEGER                            NOT NULL
);


INSERT INTO counterparties
( inn          , ogrn          , kpp       , bik        , name          , vat_id , role_id , status_id ) VALUES
('922185262656','1056947132835','121343121','994906076' ,'LOL&CO'       , 3      , 2       , 3         ),
('195834152088','7125580103708','012201171','372913021' ,'Коробки INC'  , 2      , 1       , 2         ),
('688170949084','8133326518999','842001965','437984421' ,'OAO Сложно'   , 1      , 2       , 1         ),
('026324565460','7086512542065','832345552','972494740' ,'OOO Случайный', 2      , 1       , 1         ),
('050790344949','7024669742937','397143746','685809243' ,'Старая школа' , 3      , 1       , 2         ),
('468761505872','6067707216186','808701454','316209987' ,'ИП Хемлог'    , 1      , 1       , 3         ),
('602209067208','3040248035803','598443069','644411511' ,'OOO Генерация', 3      , 2       , 3         ),
('145549591550','4038008665922','621443895','104575011' ,'ИП Лошкарев'  , 1      , 2       , 1         );

DROP TABLE IF EXISTS roles;

CREATE TABLE roles (
    id   INTEGER  PRIMARY KEY AUTOINCREMENT NOT NULL, 
    name TEXT                               NOT NULL
);

INSERT INTO roles
( name         ) VALUES
( 'Перевозчик' ),
( 'Заказчик'   );

DROP TABLE IF EXISTS counterparty_statuses;

CREATE TABLE counterparty_statuses (
    id    INTEGER  PRIMARY KEY AUTOINCREMENT NOT NULL, 
    name  TEXT                               NOT NULL
);

INSERT INTO counterparty_statuses
( name         ) VALUES
( 'Активен'    ),
( 'Блокирован' ),
( 'Заморожен'  );

DROP TABLE IF EXISTS vats;
CREATE TABLE vats (
    id    INTEGER  PRIMARY KEY AUTOINCREMENT NOT NULL, 
    name  TEXT                               NOT NULL,
    value INTEGER                           NULL
);

INSERT INTO vats
(  name     , value ) VALUES
( 'С ндс 20', 20    ),
( 'С ндс 0' , 0     ),
( 'Без ндс' , null  );

