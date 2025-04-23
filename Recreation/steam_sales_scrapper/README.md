# Steam Sales Scrapper
The purpose of this script is to scrape the Steam API to determine whether a game has reached a specified price threshold. If one or more games fall below the user-defined limit, the script will output a string containing the names of the games along with their respective prices.

## Quick Start
1. Nagivate to project folder and run `cargo build`
2. Create `.env` with the following:
    ```
    DOMAIN={your_domain}
    STEAM_API_KEY={your_steam_api_key}
    EMAIL_ADDRESS={your_email_address}
    ```

## Supported Commands
- `SEARCH` := find a game title based on keyphrase and add game with price threshold
    ```bash 
    ```
- `ADD` := add a specified game (title must be exact to work)
    ```bash 
    ```
- `UPDATE` := update price threshold for a specified game
    ```bash 
    ```
- `REMOVE` := remove a specified game
    ```bash 
    ```
- `LIST` := list all the stored price thresholds for selected games 
    ```bash 
    ```
- `CACHE` := update the local stored cache of steam games 
    ```bash 
    ```
- `CHECK` := produce a string containing the games that have reached the desired threshold
    ```bash 
    ```

## TODO
- CLI commands support
- sending email to (stored in .env)