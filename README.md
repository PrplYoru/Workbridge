# Workbridge

Workbridge is a web application designed to bridge the gap between recent graduates and companies. It provides a platform for both parties to connect and interact. The backend of the application is built with Rust, utilizing Actix-web as the web framework. The frontend is developed using Vue.js.

## Badges

[![CodeScene Code Health](https://codescene.io/projects/53238/status-badges/code-health)](https://codescene.io/projects/53238) <br>
[![CodeScene System Mastery](https://codescene.io/projects/53238/status-badges/system-mastery)](https://codescene.io/projects/53238)

## Features

Workbridge offers a variety of features including:

- User Authentication: Ensures that users are who they claim to be.
- User Roles: Differentiates between different types of users (e.g., graduates, companies).
- Company Profiles: Allows companies to create and manage their profiles.
- User Profiles: Allows users to create and manage their profiles.
- Password Hashing: Enhances security by storing passwords in a hashed format.
- JWT Authentication: Provides secure transmission of information between parties.
- User Registration: Allows new users to register.
- Company Registration: Allows new companies to register.
- User and Company Search and Filtering: Helps users and companies find each other more easily.

## Prerequisites

Before you begin, ensure you have the following installed(links):

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)
- [Yarn](https://classic.yarnpkg.com/en/docs/install)
- [XAMPP](https://www.apachefriends.org/index.html)

## Installation

Follow these steps to get Workbridge up and running:

1. Clone the repository:
```bash
$ git clone https://github.com/PrplYoru/Workbridge.git
```
2. Navigate into the project directory:
``` bash
$ cd diplomati_vue
```
3. Build the cargo project:
``` bash
$ cargo build
```
4. Navigate into the frontend directory and install the dependencies:
``` bash
$ cd vue/diplomati && yarn install
```
5. Start the XAMPP server and create a database named `diplomati`.

6. Start the server using cargo in the root directory:
``` bash
$ cargo run
```
7. The server should now be running on `localhost:8000`. Open your web browser and navigate to this address to start using Workbridge.
