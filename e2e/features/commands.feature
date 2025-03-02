Feature: Executing commands
The user is able to execute commands in the <Terminal>
component to change the output on the screen.

  @serial
  Scenario: Clearing the terminal
    Given the user has already executed several commands
    When the user executes the command 'clear'
    Then the output from previous commands disappears
