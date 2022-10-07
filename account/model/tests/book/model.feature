Feature: Number feature

    Scenario: If i add number to a model i got the sum with previous
        Given a model with nb 20
        When i add the nb 22
        Then nb is 42

    Scenario: I join my first server
        Given a model with nb 1
        When i have joined the server foo with account bar
        Then i have joined 1 server

    Scenario: I can leave with the same account
        Given a model with nb 10
        When i have joined the server tata with account titi
        Then i have joined 1 server
        Then i can leave the server tata with account titi

    Scenario: I cannot rejoin with the same account
        Given a model with nb 10
        When i have joined the server popo with account koko
        Then i cant join the server popo with account koko
