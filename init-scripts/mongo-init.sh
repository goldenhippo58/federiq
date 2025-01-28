#!/bin/bash

echo "Initializing MongoDB with data..."

# Use mongoimport to load the data into MongoDB
mongoimport --host localhost --db testdb --collection users --file /docker-entrypoint-initdb.d/mongo_data.json --jsonArray

echo "MongoDB initialization complete!"
