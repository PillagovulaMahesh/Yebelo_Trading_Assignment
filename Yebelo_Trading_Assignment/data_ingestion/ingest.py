import csv
import json
from kafka import KafkaProducer

# Redpanda / Kafka broker
BROKER = "localhost:9092"
TOPIC = "trade-data"

# Create Kafka producer
producer = KafkaProducer(
    bootstrap_servers=BROKER,
    value_serializer=lambda v: json.dumps(v).encode("utf-8")
)

# Path to CSV file
CSV_FILE = "trades_data.csv"

with open(CSV_FILE, newline='', encoding='utf-8') as csvfile:
    reader = csv.DictReader(csvfile)
    for row in reader:
        # Convert numeric fields to float if needed
        for key in ["amount_in_sol", "amount_in_token", "price_in_sol"]:
            if key in row and row[key]:
                try:
                    row[key] = float(row[key])
                except ValueError:
                    row[key] = 0.0

        # Publish to Redpanda
        producer.send(TOPIC, value=row)

# Wait for all messages to be sent
producer.flush()
print(f"Finished publishing CSV rows to topic '{TOPIC}'")
