CREATE DATABASE ranking;

CREATE TABLE `fighter` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
  `fighter_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=99 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `game` (
  `id` int NOT NULL AUTO_INCREMENT,
  `player_id` bigint NOT NULL,
  `fighter_id_1` int DEFAULT NULL,
  `wins` int DEFAULT '0',
  `loses` int DEFAULT '0',
  `Season` int NOT NULL DEFAULT '2',
  `time_created` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `fighter_id` (`fighter_id_1`),
  KEY `player_id` (`player_id`),
  CONSTRAINT `game_ibfk_2` FOREIGN KEY (`fighter_id_1`) REFERENCES `fighter` (`id`),
  CONSTRAINT `game_ibfk_3` FOREIGN KEY (`player_id`) REFERENCES `player` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1098 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `player` (
  `id` bigint NOT NULL,
  `player_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

