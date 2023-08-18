# Demo Specification
## Summary
This is the design document for the demo of this tactical rpg project. It will describe every element that needs to be implemented for this demo to be considered finished and ready for publication.
## Gameplay
The demo will cover a single skirmish for the player. The overworld will not be implemented. The player will start the skirmish with a six character team of varied composition, to show off the various features of the game. The goal of the skirmish will be to kill all enemy forces.
## Map
The skirmish map will be a fantasy city map, with various elevations, but none overlapping each other vertically. Slopes can be traversed as if flat, steep elevations can only be traversed with an ability or if there is a ladder.

The map will contain various full and partial cover objects for both the player an the enemy to use.

The map should be fully modelled and textured in 3d, using placeholder assets either bought or freely available. No animations are necessary for the map. The map should keep the grid in mind and not use designs that won't fit obviously in the grid from a top down perspective.

The map will have a designated starting area for the player characters, as well as several designated spots where enemies might spawn, and a description of the enemy spawns' patrol pattern.

## Characters
Every character should have a rigged, textured, and animated 3d model. It's acceptable to use palette swaps.

Every character has the following stats:
- **Name**: The character's name
- **Class**: The character's class
- **Active effects** *(default none)*: Any buffs, debuffs, and other such temporary or permanent passive effects that the character currently possesses.
- **Hit points (HP)** *(default 4)*: The amount of damage a character can take before falling unconscious.
- **Movement distance** *(default 12)*: The amount of tiles a character can move.
- **Action points (AP)** *(default 2)*: The amount of actions a character can take, some actions may cost more or less than one point.
- **Vision radius** *(Default 27)*: The amount of tiles a character can reveal the map, and see other characters.
- **Defense** *(Default 0)*: The base value subtracted from an attacker's chance to hit
- **Hit chance** *(Default 65%)*: The base chance to hit with an attack

Every character has the following actions:
- **Move** *(1AP)*: Move up to the character's movement distance. If the character spends both AP on a single move, the character has the sprinting buff until it reaches its destination.
- **Melee attack** *(1AP)*: Roll to hit an adjacent character based on your hit chance, deal damage according to equipped melee weapon. Only one attack can be made per turn per character whether it's melee or ranged.
- **Ranged attack** *(1AP)*: Roll to hit a character within ranged weapon range. Deal damage according to equipped ranged weapon. Only one attack can be made per turn per character whether it's melee or ranged.
- **Duck** *(1AP)*: Can only be used while in cover, gives the Duck buff until the character's next turn and ends the character's turn.
- **Keep watch** *(1AP)*: Gets the Keep watch buff until the character's next turn and ends the character's turn.
- **End turn** *(0AP)*: Ends the character's turn. If a character's turn has been ended with this command specifically, it can be undone as long as there's still at least one other character who hasn't ended their turn yet.

Every character has the following slots for equipment:
- **Melee weapon**: The weapon whose stats are used in melee attacks. Every melee weapon has the following stats:
  - **Accuracy**: Adds to the hit chance of the melee attack.
  - **Damage**: A damage range of how much the melee attack deals on hit.
  - **Critical chance**: A chance to deal double damage rolled on hit. 
- **Ranged weapon**: The weapon whose stats are used in ranged attacks. Every ranged weapon has the following stats:
  - **Accuracy**: Adds to the hit chance of the ranged attack.
  - **Damage**: A damage range of how much the ranged attack deals on hit.
  - **Critical chance**: A chance to deal double damage rolled on hit.
  - **Range**: How many tiles away this weapon can hit.
- **Armor**: Provides defensive stats. Every armor has the following stats:
  - **Defense**: Adds to the defense rating of the character.
  - **Fortification**: Adds special hit points to the character that gets subtracted before the character's hit points
- **Item**: Either an item that provides some form of passive bonus, or an activated item that may have limited uses.

## Enemies
Every enemy should have a rigged, textured, and animated 3d model. It's acceptable to use palette swaps. 

All enemies have the same stats as all characters.

All enemies have the same basic actions as all characters.

Some enemies have the same equipment slots as all characters, others may have fewer or none.

Enemies will have an AI system that can let the game make turn decisions for the enemy. This system will include:
- A state-based priority stack that gets recalculated based on the enemy's environment, taking into account:
  - Proximity and number of characters within sight
  - Proximity and number of enemies within sight
  - Equipment and stats of the thinking enemy
  - Skirmish goal(s)
  - Surrounding map:
    - Tiles to run to cover in
    - Opportunities to flank a character
  - A way to tweak the system to be more or less aggressive
- A simple patrol routine AI for when enemies have not yet discovered the characters

## GUI
The GUI will include the following elements:
- **Actions** (Only if a character is currently selected): A horizontal bar in the bottom of the screen with a button for each action the character can do, greying out actions that are temporarily not available. Every action should be numbered and you should be able to pick an action by tapping the number on the keyboard.
- **Character bio** (Only if a character is currently selected): A bottom-left box that should contain a quick view of the character's current stats and active effects, as well as a button to click to see the character's stat window.
- **Stat window**: A window that can be opened to see a character's full stats and equipment. You should be able to hover over things that need a description like active effects and equipment, but not actually change anything from this window.
- **Movement range and line** (Only if a character that can move is currently selected): A colored visual indicator on the map grid of how far a character can move, and which path it would take to move to the currently hovered-over tile. It should use a different color for tiles that are only within sprinting range. If the enemy has not yet discovered the characters it should show whether a move would alert the enemies to the characters' presence.
- **Stat HUD**: A heads up display showing the health, cover status, and active effects icons shown over each character and enemy visible on the map. Should also provide an icon to show whether a character is visible to the enemy if it moves to the hovered space.