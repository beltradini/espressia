# Espressia
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview
Espressia is a visionary espresso simulation system crafted in Rust, designed to elevate the art of coffee extraction into a realm of technological marvel. This project delivers a RESTful API to simulate espresso brewing with precision, allowing users to customize parameters like temperature, pressure, and extraction time while preserving metrics for analysis and inspiration.

## Features
- **Espresso Simulation Utopia**: Simulate brewing with unparalleled realism and control.
- **Customizable Parameters**: Fine-tune temperature, pressure, and time to craft the perfect shot.
- **Precision Validation**: Ensures every input aligns with the science of espresso perfection.
- **Metrics Eternity**: Stores extraction data in a robust format for reflection and growth.
- **RESTful Elegance**: Offers intuitive endpoints to initiate extractions and explore past brilliance.
- **Database Integration**: Metrics are now stored in a high-performance database for seamless retrieval.
- **Recipe Gallery**: Explore and share a curated collection of espresso recipes.
- **Enhanced Logging**: Comprehensive error handling and detailed logs for debugging and insights.

## Installation
To embark on the Espressia journey, follow these steps:

1. Clone the repository:
   ```sh
   git clone https://github.com/beltradini/espressia.git
   cd espressia
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
### POST /start 

Query Parameters:
- **temperature** (default: 93.0°C)
- **pressure** (default: 9.0 bar)
- **time_seconds** (default: 25s)

Example:
```sh
curl -X POST "http://127.0.0.1:3000/start?temperature=95&pressure=9.5&time_seconds=27"
```

## Retrieve Extraction Metrics
### GET /metrics
Example:
```sh
curl -X GET "http://127.0.0.1:3000/metrics"
```
## Future Improvements
- Expand the API to simulate a universe of beverages.
- Introduce machine learning to recommend optimal brewing parameters based on user preferences.
- Develop a mobile application for remote control and monitoring of espresso simulations.
- Add support for multi-user accounts and collaborative recipe sharing.

## License
Espressia is released under the MIT License.