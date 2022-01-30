ARG MYSQL_VERSION
FROM mysql:${MYSQL_VERSION}
COPY ./build/etc/mysql /etc/mysql
COPY ./build/var/lib/mysql /var/lib/mysql
CMD chmod 700 /var/run/mysqld