This project is a game of life simulation inspired by the video [Artificial Life. The battle of clans](https://www.youtube.com/watch?v=q2uuMY37JuA&t=987s) ([Original Video in Russian](https://www.youtube.com/watch?v=_Aow6P3oBAg)).

### Rules of the game
* Leaf
  * A leaf can photosynthesize to gain energy
* Sprout
  * Sprout cells hold the genome of the organism and execute genome commands
  * Each step holds a slight probability of random mutation
  * A sprout can create 1-3 branches each step, then turns into wood
* Wood
  * Connects energy collecting cells to energy consuming sprouts
* Seed Class 
  * Consume very little energy
  * After organism death, seed becomes a sprout
* Antenna class
  * Can survive above charge toxicity limit
  * Converts charge to energy
* Root class
  * Can survive above matter toxicity limit
  * Converts organic matter to energy

* Lifespan of a cell is dependent on the amount of energy spent on its creation