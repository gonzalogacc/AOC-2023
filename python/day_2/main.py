from functools import reduce
from typing import List


class Batch:

    def __init__(self, batch_string: str):
        self.red: int = 0
        self.blue: int = 0
        self.green: int = 0
        self.parse_batch(batch_string)

    def parse_batch(self, batch_string):
        for line in batch_string.strip().split(','):
            cuant, color = list(line.strip().split(' '))
            setattr(self, color, int(cuant))

    def is_batch_possible(self, other):
        if self.red > other.red or self.green > other.green or self.blue > other.blue:
            return False
        return True


class Game:

    def __init__(self, game_string: str):
        self.id: int = None
        self.batches: List[Batch] = []
        self.parse_game(game_string)

    def parse_game(self, game_string):
        self.id = int(game_string.strip().split(":")[0].split(" ")[1])
        self.batches = [Batch(sg) for sg in game_string.strip().split(":")[1].split(";")]

    def is_game_possible(self, other: Batch):
        for batch in self.batches:
            if not batch.is_batch_possible(other):
                return False
        return True

    def min_dice_set(self) -> tuple:
        red = max([b.red for b in self.batches])
        green = max([b.green for b in self.batches])
        blue = max([b.blue for b in self.batches])
        return red, green, blue

    def compute_dice_power(self):
        return reduce(lambda x, y: x * y, self.min_dice_set())


def load_games(filename: str) -> List[Game]:
    return [Game(line.strip()) for line in open(filename)]


def game_filter(games: List[Game], bag: Batch) -> int:
    id_sum = 0
    power_sum = 0
    for game in games:
        if game.is_game_possible(bag):
            id_sum += game.id
        power_sum += game.compute_dice_power()
    return id_sum, power_sum


if __name__ == "__main__":
    games = load_games('./input.txt')
    bag = Batch("12 red, 13 green, 14 blue")
    game_sum, power_sum = game_filter(games, bag)
    print(f"Game sum: {game_sum}")
    print(f"Power sum: {power_sum}")
