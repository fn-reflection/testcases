CREATE TABLE IF NOT EXISTS tree_nodes (
    id SERIAL PRIMARY KEY,
    parent_id BIGINT UNSIGNED,
    content TEXT NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES tree_nodes(id)
);

INSERT INTO
    tree_nodes(id, parent_id, content)
VALUES
    (1, NULL, '1'),
    (2, 1, '2'),
    (3, 1, '3'),
    (4, 2, '4'),
    (5, 2, '5'),
    (6, 3, '6'),
    (7, 3, '7'),
    (8, 4, '8'),
    (9, 4, '9'),
    (10, 5, '10'),
    (11, 5, '11'),
    (12, 6, '12'),
    (13, 6, '13'),
    (14, 7, '14'),
    (15, 7, '15');

SELECT
    t1.id AS from_id,
    t1.content AS from_content,
    t2.id AS to_id,
    t2.content AS to_content
FROM
    tree_nodes t1
    LEFT OUTER JOIN tree_nodes t2 ON t1.id = t2.parent_id;

WITH RECURSIVE recursive_tree (id, content, parent_id, depth) AS (
    SELECT
        id,
        content,
        parent_id,
        0 AS depth
    FROM
        tree_nodes
    WHERE
        parent_id IS NULL -- root node
    UNION
    ALL
    SELECT
        t.id,
        CONCAT(rt.content, '/', t.content) AS content,
        t.parent_id,
        rt.depth + 1 AS depth
    FROM
        recursive_tree rt
        INNER JOIN tree_nodes t ON rt.id = t.parent_id
)
SELECT
    *
FROM
    recursive_tree;
