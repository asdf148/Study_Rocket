CREATE TABLE users (
  id int NOT NULL AUTO_INCREMENT,
  name varchar(45) NOT NULL,
  email varchar(45) NOT NULL,
  password varchar(60) NOT NULL,
  created_at timestamp not null default current_timestamp,
  PRIMARY KEY (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3

CREATE TABLE posts (
  id int NOT NULL AUTO_INCREMENT,
  image varchar(100) DEFAULT NULL,
  title varchar(45) NOT NULL,
  content varchar(200) DEFAULT NULL,
  userId int NOT NULL,
  created_at timestamp not null default current_timestamp,
  PRIMARY KEY (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3

CREATE TABLE comments (
  id int NOT NULL AUTO_INCREMENT,
  comment varchar(100) DEFAULT NULL,
  userId int NOT NULL,
  postId int NOT NULL,
  created_at timestamp not null default current_timestamp,
  PRIMARY KEY (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3

ALTER TABLE posts
ADD FOREIGN KEY (userId) REFERENCES users(id);

ALTER TABLE comments
ADD FOREIGN KEY (userId) REFERENCES users(id);

ALTER TABLE comments
ADD FOREIGN KEY (postId) REFERENCES posts(id);