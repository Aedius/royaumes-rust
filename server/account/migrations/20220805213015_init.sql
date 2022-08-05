CREATE database `account`;

CREATE TABLE `user` (
  `uuid` char(40) NOT NULL,
  `email` varchar(255) NOT NULL,
  `pseudo` varchar(50) NOT NULL,
  `password` varchar(50) NOT NULL,
  `admin` tinyint(1) NOT NULL
) ENGINE='InnoDB' COLLATE 'utf8mb4_general_ci';

ALTER TABLE `user`
ADD PRIMARY KEY `uuid` (`uuid`),
ADD UNIQUE `email` (`email`),
ADD UNIQUE `pseudo` (`pseudo`);
