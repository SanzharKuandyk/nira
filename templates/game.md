<!-- Description: Game architecture template focused on systems and connections -->
# Game Blueprint: {PROJECT_NAME}

> **Started:** {DATE}
> **Status:** Planning
> **Approach:** Game Systems (understand how everything connects)

---

## 1. GAME CORE

**Genre:** [platformer | RPG | puzzle | shooter | etc.]

**Core Loop:** (What happens every frame/update)
```
1. Input → [what gets captured]
2. Update → [what changes: physics, AI, timers]
3. Render → [what gets drawn]
4. Audio → [what plays]
```

**Win/Lose:**
- Win condition: [how player wins]
- Lose condition: [how player loses]
- Rewards: [points, unlocks, progression]

**Key Mechanics:**
- [mechanic 1] - [how it works, what it affects]
- [mechanic 2] - [how it works, what it affects]

---

## 2. GAME STATES (The Big Picture Flow)

<!--
  Game states are the major screens/modes.
  Be explicit about what triggers transitions.
-->

**State Machine:**
```
[Boot]
  ↓ (load assets)
[MainMenu]
  ↓ (press start)
[GamePlay]
  ↓ (press pause) ⟷ [Pause] ⟶ (quit) ⟶ [MainMenu]
  ↓ (player dies / wins)
[GameOver]
  ↓ (retry / continue)
[MainMenu] or [GamePlay]
```

**For each state, define:**

### State: MainMenu
- **Active systems:** [input, UI, audio]
- **Inactive systems:** [physics, game logic]
- **Data needed:** [save file exists?, settings]
- **Transitions to:** GamePlay (on start), Settings (on options)

### State: GamePlay
- **Active systems:** [ALL - input, physics, render, audio, AI, etc.]
- **Inactive systems:** [none]
- **Data needed:** [level data, player state, score]
- **Transitions to:** Pause (on ESC), GameOver (on death/win)

### State: Pause
- **Active systems:** [input, UI, partial render]
- **Inactive systems:** [physics, AI, game logic paused]
- **Data needed:** [frozen game state]
- **Transitions to:** GamePlay (resume), MainMenu (quit)

### State: GameOver
- **Active systems:** [UI, input, audio]
- **Inactive systems:** [physics, game logic]
- **Data needed:** [final score, stats]
- **Transitions to:** GamePlay (retry), MainMenu (quit)

---

## 3. SYSTEMS (What Does What)

<!--
  Each system has ONE job. Define what it owns and what it needs from others.
-->

### Input System
**Job:** Capture player input and translate to game actions

**Owns:**
- Keyboard/mouse/gamepad state
- Input mapping (W = jump, Space = attack)

**Needs from other systems:**
- Nothing (it's the entry point)

**Provides to other systems:**
- PlayerInput struct → PlayerController
- UIInput struct → UI system

**Rules:**
- Only reads hardware, never writes game state directly
- Raw input → meaningful actions (not "key W pressed" but "jump requested")

---

### PlayerController System
**Job:** Control player entity based on input

**Owns:**
- Player entity state (position, velocity, health, etc.)

**Needs from other systems:**
- PlayerInput from Input System
- Collision data from Physics System

**Provides to other systems:**
- Player position → Camera System (for following)
- Player state → UI System (for HUD)

**Rules:**
- Only modifies player entity, not world or enemies

---

### Physics System
**Job:** Move entities, detect collisions

**Owns:**
- Entity positions, velocities
- Collision shapes
- Gravity, friction constants

**Needs from other systems:**
- Movement requests from PlayerController, AI
- Level geometry from Level System

**Provides to other systems:**
- Collision events → PlayerController, AI (to react)
- Final positions → Render System

**Rules:**
- Runs at fixed timestep (60 updates/sec)
- Never skips collision checks

---

### Render System
**Job:** Draw everything to screen

**Owns:**
- Camera state
- Sprite/model instances
- Draw order / layers

**Needs from other systems:**
- Entity positions from Physics
- Sprite IDs from Entities
- UI elements from UI System

**Provides to other systems:**
- Nothing (it's the output)

**Rules:**
- Reads state, never modifies it
- Draws at variable framerate (desynced from game logic)

---

### Audio System
**Job:** Play music and sound effects

**Owns:**
- Audio channels
- Volume settings

**Needs from other systems:**
- Sound triggers from game events (collision, player action, etc.)

**Provides to other systems:**
- Nothing (it's an output)

**Rules:**
- Sounds are fire-and-forget events
- Music loops until told to change

---

### AI System (if applicable)
**Job:** Control enemy behavior

**Owns:**
- Enemy state machines
- Pathfinding data

**Needs from other systems:**
- Player position from PlayerController
- Level data from Level System
- Collision data from Physics

**Provides to other systems:**
- Movement requests → Physics
- Attack events → Combat System

**Rules:**
- Each enemy updates independently
- AI has limited "vision" (doesn't know everything)

---

### UI System
**Job:** Show HUD, menus, dialogs

**Owns:**
- UI elements (buttons, text, health bars)
- Menu state

**Needs from other systems:**
- Player stats from PlayerController
- Game state from State Machine

**Provides to other systems:**
- Menu selections → State Machine
- Nothing else (UI doesn't affect game logic)

**Rules:**
- UI never directly modifies game state (only sends events)

---

### Level System
**Job:** Load and manage current level

**Owns:**
- Level geometry (walls, platforms)
- Entity spawn points
- Background/tilemap

**Needs from other systems:**
- Current level ID from State Machine

**Provides to other systems:**
- Collision geometry → Physics
- Entities to spawn → Entity System
- Background data → Render System

**Rules:**
- Levels are immutable once loaded
- Only one level active at a time

---

## 4. ENTITIES & DATA

<!--
  What "things" exist in your game world?
  Define them as data, not classes (easier to reason about).
-->

### Entity: Player
**Data:**
- `position` (x, y) — where player is
- `velocity` (vx, vy) — how fast player is moving
- `health` (int) — current HP
- `sprite_id` (string) — which sprite to draw
- `facing` (left/right) — which direction player faces

**Controlled by:** PlayerController System

**Affected by:** Physics, Combat (if hit)

---

### Entity: Enemy
**Data:**
- `position` (x, y)
- `velocity` (vx, vy)
- `health` (int)
- `ai_state` (patrol/chase/attack)
- `sprite_id` (string)

**Controlled by:** AI System

**Affected by:** Physics, Combat

---

### Entity: Projectile
**Data:**
- `position` (x, y)
- `velocity` (vx, vy)
- `damage` (int)
- `sprite_id` (string)
- `lifetime` (float) — despawn after N seconds

**Controlled by:** Projectile System

**Affected by:** Physics

---

## 5. SYSTEM CONNECTIONS (Data Flow)

<!--
  Show exactly how data flows between systems.
  This is the KEY to understanding your architecture.
-->

```
[Input System]
    ↓ PlayerInput
[PlayerController]
    ↓ movement request
[Physics System] ← level geometry ← [Level System]
    ↓ collision events
[PlayerController] (reacts to collisions)
    ↓ player state
[UI System] (draws HUD)
[Render System] (draws player sprite)
```

**Full frame flow:**
```
1. Input System captures input
2. PlayerController/AI request movements
3. Physics updates all positions, detects collisions
4. Systems react to collisions (player takes damage, etc.)
5. Render System draws everything
6. Audio System plays triggered sounds
```

**Rules:**
- Systems never directly call each other
- Communication via data (shared state or events)
- Clear dependency order (Input → Logic → Physics → Render)

---

## 6. ASSETS & RESOURCES

**Sprites/Textures:**
- Format: [PNG, sprite atlas, etc.]
- Location: `assets/sprites/`
- Loaded when: [at boot, per level, on demand]

**Audio:**
- Format: [WAV, OGG, MP3]
- Location: `assets/audio/`
- Loaded when: [at boot, per level]

**Levels:**
- Format: [JSON, Tiled .tmx, custom]
- Location: `assets/levels/`
- Loaded when: [on level transition]

**Loading Strategy:**
- **Boot:** Load main menu assets, common sprites
- **Level transition:** Unload old level, load new level assets
- **Memory budget:** [max X MB, unload unused assets]

---

## 7. SAVE/LOAD SYSTEM

**What gets saved:**
- Player progress: [current level, unlocks, score]
- Settings: [volume, controls, graphics]
- High scores: [top 10 scores]

**Where:**
- Format: [JSON, binary, cloud save]
- Location: [user directory, cloud service]

**When:**
- Save: [after level complete, on settings change]
- Load: [at boot, when continue pressed]

**Data structure:**
```
SaveFile:
  - current_level: int
  - player_stats: {health, score, etc.}
  - unlocked_levels: [1, 2, 3]
  - settings: {volume, ...}
```

---

## 8. FILE STRUCTURE

```
game/
├── src/
│   ├── main              ← ENTRY: boot sequence, game loop
│   │
│   ├── systems/          ← All game systems
│   │   ├── input         ← Input System
│   │   ├── player        ← PlayerController System
│   │   ├── physics       ← Physics System
│   │   ├── render        ← Render System
│   │   ├── audio         ← Audio System
│   │   ├── ai            ← AI System
│   │   └── ui            ← UI System
│   │
│   ├── entities/         ← Entity data definitions
│   │   ├── player        ← Player entity
│   │   ├── enemy         ← Enemy entity
│   │   └── projectile    ← Projectile entity
│   │
│   ├── states/           ← Game state machine
│   │   ├── state         ← Base state trait
│   │   ├── menu          ← MainMenu state
│   │   ├── gameplay      ← GamePlay state
│   │   └── gameover      ← GameOver state
│   │
│   ├── assets/           ← Asset loading
│   │   ├── loader        ← Asset loader
│   │   └── registry      ← Asset registry
│   │
│   └── util/             ← Shared utilities
│       ├── math          ← Vector math
│       └── collision     ← Collision helpers
│
└── assets/
    ├── sprites/          ← All sprite images
    ├── audio/            ← All audio files
    └── levels/           ← All level data
```

**Dependency rules:**
- Systems can't import other systems (loose coupling)
- Entities are pure data (no logic)
- States own the system orchestration

---

## 9. TASK QUEUE (By System)

<!--
  Organize tasks by which system they touch.
  This makes it clear what needs to be built in what order.
-->

### DONE ✓

- [x] [completed task]

### IN PROGRESS →

**System: [System name]**
- [ ] **[task description]**
  - **Why:** [what problem this solves]
  - **Files:** [which files this touches]
  - **Depends on:** [other tasks or "nothing"]
  - **Test:** [how to verify it works]

### NEXT UP (By System)

**Core / Bootstrap:**
- [ ] Set up game loop (init → update → render)
- [ ] Implement state machine (switch between menu/gameplay/etc.)

**Input System:**
- [ ] Capture keyboard input
- [ ] Map keys to actions

**PlayerController:**
- [ ] Move player left/right
- [ ] Implement jump

**Physics:**
- [ ] Apply gravity to entities
- [ ] Detect collisions with walls

**Render:**
- [ ] Draw sprites at entity positions
- [ ] Implement camera following player

**Audio:**
- [ ] Play background music
- [ ] Play jump sound effect

**Level:**
- [ ] Load level from JSON
- [ ] Spawn entities at spawn points

**UI:**
- [ ] Show health bar
- [ ] Show main menu

### ICEBOX (Future Ideas)

- [ ] [feature for later]

---

## 10. PERFORMANCE & CONSTRAINTS

**Target Performance:**
- FPS: [60 fps, 30 fps]
- Resolution: [1920x1080, variable]
- Max entities: [100 enemies, 500 particles]

**Memory Budget:**
- Total: [512 MB, 1 GB, unlimited]
- Textures: [max X MB]
- Audio: [max Y MB]

**Constraints:**
- Must run on: [PC, web, mobile, console]
- Physics runs at: [fixed 60 Hz]
- Render runs at: [variable, vsync]

**Bottlenecks to watch:**
- Too many collision checks (use spatial partitioning)
- Too many draw calls (use sprite batching)
- Asset loading hitches (preload or stream)
