CREATE TABLE naive_trees (
    id BIGINT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    parent_id BIGINT UNSIGNED,
    content TEXT NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES nodes(id)
);
