CREATE TABLE IF NOT EXISTS naive_trees (
    id SERIAL,
    parent_id BIGINT UNSIGNED,
    content TEXT NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES naive_trees(id)
);

INSERT INTO
    naive_trees(id, parent_id, content)
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
    naive_trees t1
    LEFT OUTER JOIN naive_trees t2 ON t2.parent_id = t1.id;

WITH RECURSIVE RecursiveTrees (id, content, parent_id, depth) AS (
    SELECT
        id,
        content,
        parent_id,
        0 AS depth
    FROM
        naive_trees
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
        RecursiveTrees rt
        INNER JOIN naive_trees t ON rt.id = t.parent_id
)
SELECT
    *
FROM
    RecursiveTrees;
