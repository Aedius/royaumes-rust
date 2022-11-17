Feature: Number feature

    Scenario: if i have some reputation, i can add more
        Given an account with a reputation of 20
        When i try to add 22 reputation
        Then reputation is 42

    Scenario: i cant remove more reputation than that i have
        Given an account with a reputation of 20
        When i try to remove 22 reputation
        Then reputation is 20
        Then i got an error
