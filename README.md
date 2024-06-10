# Filler docker image

-   To build the image `docker build -t filler .`
-   To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
-   Example of a command in the container `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
-   Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

## Notes

-   `Terminator` is a very strong robot so it's optional to beat him.
-   For M1 Macs use `m1_robots` and `m1_game_engine`.

## How to test with Dockerfile

Naviguer jusqu'à filler puis

```
docker build -t filler .
```

Ensuite

```
docker run -v "/home/mamy/Desktop/filler/solution":/filler/solution -it filler
```

```
cd solution

cargo build --release

cd ..
```

```
./linux_game_engine -f ./maps/map01 -p1 solution/target/release/solution -p2 ./linux_robots/bender
```

## Examples

```
./linux_game_engine -f ./maps/map01 -p1 ./linux_robots/bender -p2 ./linux_robots/terminator
```

<br /><br /><br />

p1: solution

```
./linux_game_engine -f ./maps/map00 -p1 solution/target/release/solution -p2 ./linux_robots/wall_e
```

p2: solution

```
./linux_game_engine -f ./maps/map00 -p1 ./linux_robots/wall_e -p2 solution/target/release/solution
```

<br /><br /><br />

p1: solution

```
./linux_game_engine -f ./maps/map00 -p1 solution/target/release/solution -p2 ./linux_robots/h2_d2
```

p2: solution

```
./linux_game_engine -f ./maps/map00 -p1 ./linux_robots/h2_d2 -p2 solution/target/release/solution
```

<br /><br /><br />

p1: solution

```
./linux_game_engine -f ./maps/map00 -p1 solution/target/release/solution -p2 ./linux_robots/bender
```

p2: solution

```
./linux_game_engine -f ./maps/map00 -p1 ./linux_robots/bender -p2 solution/target/release/solution
```

<br /><br /><br />
