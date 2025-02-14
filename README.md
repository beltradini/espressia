# Mastrena 3.0
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview
Mastrena 3.0 is an espresso machine simulation system built with Rust. This project provides a RESTful API to simulate espresso extraction processes with customizable parameters such as temperature, pressure, and extraction time. The system also records extraction metrics for future analysis.

## Features
- **Espresso Extraction Simulation**: Simulates the brewing process with adjustable settings.
- **Customizable Parameters**: Users can define temperature, pressure, and extraction time.
- **Validation Mechanism**: Ensures input values fall within realistic ranges.
- **Metrics Storage**: Records extraction data in JSON format for future reference.
- **REST API**: Provides endpoints to start an extraction and retrieve past records.

## Installation
To set up Mastrena 3.0, follow these steps:

1. Clone the repository:
   ```sh
   git clone https://github.com/beltradini/mastrena-3.0.git
   cd mastrena-3.0
   ```

2. Install Rust (if not already installed):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Build and run the project:
   ```sh
   cargo run
   ```

## API Endpoints
### Start an Espresso Extraction
**POST** `/start`

Query Parameters:
- `temperature` (optional, default: 93.0°C)
- `pressure` (optional, default: 9.0 bar)
- `time_seconds` (optional, default: 25s)

Example:
```sh
curl -X POST "http://127.0.0.1:3000/start?temperature=95&pressure=9.5&time_seconds=27"
```

### Retrieve Extraction Metrics
**GET** `/metrics`

Example:
```sh
curl -X GET "http://127.0.0.1:3000/metrics"
```

## Future Improvements
- Implement a database for better storage and retrieval of metrics.
- Add support for different espresso recipes.
- Enhance error handling and logging.
- Expand API functionalities with additional beverage simulations.

## License
Mastrena 3.0 is released under the MIT License.

