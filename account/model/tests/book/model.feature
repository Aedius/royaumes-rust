Feature: Number feature

    Scenario: If i add number to a model i got the sum with previous
        Given a model with nb 20
        When i add the nb 22
        Then nb is 42
