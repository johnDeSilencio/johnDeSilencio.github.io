Feature: Initial page load
The initial page of the website loads as expected.

  @serial
  Scenario: The website is opened for the first time
    When I open the website
    Then "Nicholas R. Smith" is in the title of the website
