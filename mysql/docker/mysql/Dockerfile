ARG MYSQL_VERSION
FROM mysql:${MYSQL_VERSION}
COPY ./docker/mysql/build/docker-entrypoint-initdb.d/initialize.sql /docker-entrypoint-initdb.d/initialize.sql
