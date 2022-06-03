# About CAH(Cards Against Humanity)
This project is created to simulate a [CAH](https://crhallberg.com/cah/) match with 4 players by deafult, the cards where extracted from this project [CAH](https://crhallberg.com/cah/), this starter game will be palyed in a terminal to simulate the experiencie of a CAH game

# Flow
There are 4 players, when a round starts the system will shuffle 4 `white` cards to each player and will decide a `black` card (this is the main card to play 'against'), depending on the quantity of blank spaces in the main card the players will have to play 1 or 2 cards from their hand. The players will have to take a vote after the system shows the results (this is the white and the black card together) the player with most votes will win a point and the next round will start (the system then will shuffle again until all the players have 4 cards again) and so on...

# TODO

- [x] Create a `Vec<Players>` not a `Vec<&str>`
- [x] Define all the white cards for each player once their names are input
- [x] Create the shuffle algorithm
- [x] Always show all the cards for each player on the terminal...?
- [x] After a card is delivered (white or black) remove it from the main arrays
- [ ] Implement multiple picks when the black card is `BlackCard.pick = 2`
- [ ] Send the picked cards to a new `Vec`
- [ ] Show each result for each player
- [ ] Score system