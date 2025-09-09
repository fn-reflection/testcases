DROP TABLE IF EXISTS plan_change_test;
CREATE TABLE plan_change_test (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  title VARCHAR(255),
  category VARCHAR(255),
  text1 VARCHAR(255),
  text2 TEXT,
  text3 TEXT,
  INDEX idx_category (category, text1),
  INDEX idx_title (title)
);

TRUNCATE TABLE plan_change_test;

-- 100万行レコードを作る
INSERT INTO plan_change_test (title, category, text1, text2, text3)
SELECT
  CONCAT('title', FLOOR(RAND() * 1000000)) AS title,
  CASE
    WHEN MOD(seq,4) = 0 THEN 'category1'
    WHEN MOD(seq,4) = 1 THEN 'category2'
    ELSE CONCAT('category', 3 + MOD(seq, 48))
  END AS category,
  LEFT(REPEAT('loremipsumdolor', 50), 10 + FLOOR(RAND() * 100)) AS text1,
  LEFT(REPEAT('loremipsumdolor', 50), 500 + FLOOR(RAND() * 1000)) AS text2,
  LEFT(REPEAT('loremipsumdolor', 50), 500 + FLOOR(RAND() * 1000)) AS text3

FROM (
  WITH RECURSIVE d AS (
    SELECT 0 AS n
    UNION ALL
    SELECT n + 1 FROM d WHERE n < 999
  ),
  seq AS (
    SELECT a.n * 1000 + b.n AS seq
    FROM d AS a
    CROSS JOIN d AS b
  )
  SELECT seq FROM seq
) AS derived;
