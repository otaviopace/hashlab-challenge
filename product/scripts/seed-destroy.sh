#!/bin/sh

docker-compose run product-migrate-and-seed node_modules/.bin/sequelize db:seed:undo --config src/config/sequelize.js --seeders-path /product/src/ports/sequelize/seeders/
