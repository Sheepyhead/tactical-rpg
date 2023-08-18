# Overview

## Big Picture

### Elevator Pitch

A tactical turn-based RPG set during a peasant revolt in a high fantasy kingdom.

### Target Audience

People who are into tactical games from beginners to advanced players.

### Themes

Workers' rights, revolutionary theory and praxis, anti-monarchism

## Mechanics

### Moment-to-moment Loop

From an overworld map the player chooses one of the several skirmishes available. They then load out their squad and select equipment for them. The squad is placed on a 3d grid-based map and they have to clear one or more objectives for the skirmish in turn based manner, facing various enemy troops.

In the beginning of the skirmish, the map is covered by fog of war, unveiled only by the characters' vision radius. The enemy is unaware of the characters for as long as the characters aren't in direct line of sight and within the enemy characters' vision radius.

Before being discovered the player gets to control each of their characters once, in any order they desire, including interleaving actions between characters. Then once all characters have ended their turn, the enemy takes a turn to move their units in a patrol pattern, or just standing still.

If the enemy discovers the characters on the player's turn, the enemies get to move, but not act otherwise, and then the player gets to finish their current turn. If the enemy discovers the characters on the enemy turn, they get to move, but not act otherwise, and then it's the player's turn.

If the characters have managed to come within attack range of an enemy without being spotted, and make an attack against the enemy, the enemy will then discover the characters and act as normal for the enemies discovering the characters on the player's turn, regardless of whether the attack hits.

Once the enemy has discovered the player characters, the turns proceed as previously described, only now the characters have targets that they may use abilities on, such as attacking them.

Once the skirmish goal has been accomplished, or all player characters are dead or unconscious, the skirmish is over. If the skirmish goal has been failed (such as with a turn-limited skirmish) the player characters get an opportunity to escape by returning to the starting zone or another designated area. Once at least one character is in the designated zone, the skirmish can be ended as a failure with the characters in the designated zone surviving the encounter.

### Game Loop

The player starts a new game or loads an existing game. If the game was saved in a skirmish, they finish the skirmish, if the game was saved in the overworld they pass the time or pick a skirmish to fight.

### Progression Loop

Skirmishes and certain events on the overworld can unlock new equipment, additional characters, or certain training for the characters. An example would be a skirmish to take over a blacksmithing shop, leading to better weapons, or rescuing a wizard from some soldiers, giving you a wizard unit or unlocking wizard training.

### Objects

The nouns that define the world.
- **Class**: A character's class defines their abilities, and what equipment they may use. Characters start without a class, once they gain a level they may pick a class from the ones you have unlocked so far
- **Cover**: An environment-based bonus a character can have. Full cover gives a large bonus to avoiding attacks coming from the direction of the cover (cardinal directions), partial cover gives a smaller bonus.
- **Hit points**: The amount of damage a character can take before being knocked unconscious.
- **Level**: A character's level unlocks more powerful abilities, representing the character gaining experience in their class
- **Skirmish**: A confrontation between a number of player characters and a number of enemy characters, played out on a 3d grid-based battlemap.
- **Unconscious**: A status effect a character receives when reaching 0 hit points. After three turns of being unconscious a character dies permanently. If the skirmish is successful before that time, the character survives with grave injuries, otherwise the character is permanently dead.

#### Buffs
- **Duck**: Doubles all cover bonuses
- **Keep watch**: Takes a reaction shot on an opponent moving around within vision radius and line of sight, or a reaction melee attack to an opponent moving within melee range, whichever occurs first.
- **Sprinting**: A buff that increases character defense by 20 while moving.

### Actions

The verbs that define what players can do.


### Resources

The currency-like pools that resources can build up, spend and convert.
- **Momentum**: A numeric value that indicates how much momentum your revolt has; how popular and well known it is with people who aren't directly part of the revolt
- **Supplies**: The basic currency unit for the game

### Design Invariants

What patterns and restrictions are we imposing on ourselves to make reasoning about the design easier?

### Design Constraints

What problems will be deal-breakers for players? What restrictions are we imposing on ourselves?

### Design Tolerances

What unusual flaws or unconventional choices will our players accept?

### Systems and Hooks

How can we generate interesting complexity?

## Business

### Monetization Strategy

The game code will be open source and freely available. The assets will be closed source and not available. The whole game will be sold as a retail product as a one time purchase from GOG, itch.io, and Steam.

### Marketing Strategy

What channels are we going to use to reach our target audience?

### Marketing Hooks

What game design elements can we focus on to push the marketability of this game?

## Technical

### Technical Strategy

The game will be made in the Bevy engine using the Rust language. It will be released for Windows, Mac, and Linux.

### Technical Constraints

As the Bevy engine is still in its infancy, several severe technical constraints are a concern at time of writing, namely:
1. The lack of fully featured ergonomic GUI
2. The lack of several useful 3d features such as emissive textures

### Technical Tolerances

Writing games in Bevy to compile to all platforms Bevy supports will be mostly trivial, and in theory this game should also be portable to Android/iOS and even web, come widespread WebGPU support.

## Art

### Art Strategy

The game will use moderately detailed 3d art in a classic fantasy style with no strong stylistic variance. Comparable game visuals may be games like Divinity: Original Sin, Grim Dawn

### Art Constraints

The lack of an artist will be an issue...

### Art Tolerances

For placeholders, use as much bought and free assets as we can find.