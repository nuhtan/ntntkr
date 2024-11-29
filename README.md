# Nuhtan's tracker

This will be split into multiple modules:
- Notebooks
    - A list of notebooks with options for sorting.
- Food / Kitchen Inventory
    - This will just be a per item thing, there won't be anything like servings / percentage left.
    - There will be both a section of what is currently in my kitchen and a section for what needs to be bought.
- Financial Tracking
    - Accounts
    - Purchases
    - Budgeting for Planned Purchases
    - Automatic Reocurring Purchases
- Weight
    - Daily Weight
    - Calories burned for a Day
- Sleep Tracking

For weight and sleep tracking, see if there is a way that I can automatically ingest that data from Fitbit.
All of these modules should store data in a server, use the same setup that I experimented with for the financial tracking app setup.
All of these will also need popup tabs for adding new entries.

The application flow will go like so:
Start -> Load Servers from file -> Select / Create Server entry -> Get Users from Server -> Select / Create User -> Select Module
