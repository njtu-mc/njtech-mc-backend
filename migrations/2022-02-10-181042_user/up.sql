-- Your SQL goes here
create table user
(
    id          int auto_increment primary key,
    created_at  DATETIME     NOT NULL default CURRENT_TIMESTAMP comment '创建时间',
    updated_at  DATETIME     NOT NULL default CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP comment '最后修改时间',
    `name`      varchar(30)  NOT NULL default '' comment '姓名',
    email       varchar(30)  NOT NULL default '' comment '邮箱, 如果有 njtech.edu.cn 邮箱可以直接注册',
    referrer_id int          NOT NULL default 0 comment '推荐人 id，没有南工邮箱需要',
    mc_id       varchar(100) NOT NULL default '' comment 'minecraft id',

    KEY `created_at` (created_at),
    KEY `updated_at` (updated_at),
    unique email (email)
);
