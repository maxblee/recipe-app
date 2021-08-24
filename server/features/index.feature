Feature: Retrieve Index Page

    Handles the functionality with the index page.

    Scenario: page returns basic information
        When I visit the homepage
        Then I should see some text saying "Hello, world!"
