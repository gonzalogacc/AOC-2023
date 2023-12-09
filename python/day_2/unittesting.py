from day_2.main import Batch, Game


def test_batch_parse():
    batch = Batch("3 blue, 4 red")
    assert batch.red == 4
    assert batch.green == 0
    assert batch.blue == 3

    batch = Batch("1 red, 2 green, 6 blue")
    assert batch.red == 1
    assert batch.green == 2
    assert batch.blue == 6

    batch = Batch("2 green")
    assert batch.red == 0
    assert batch.green == 2
    assert batch.blue == 0


def test_game_parse():
    game = Game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
    assert game.id == 1
    assert len(game.batches) == 3

    assert game.batches[0].red == 4
    assert game.batches[0].green == 0
    assert game.batches[0].blue == 3

    assert game.batches[1].red == 1
    assert game.batches[1].green == 2
    assert game.batches[1].blue == 6

    assert game.batches[2].red == 0
    assert game.batches[2].green == 2
    assert game.batches[2].blue == 0

    game = Game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
    assert game.id == 3
    assert len(game.batches) == 3

    assert game.batches[0].red == 20
    assert game.batches[0].green == 8
    assert game.batches[0].blue == 6

    assert game.batches[1].red == 4
    assert game.batches[1].green == 13
    assert game.batches[1].blue == 5

    assert game.batches[2].red == 1
    assert game.batches[2].green == 5
    assert game.batches[2].blue == 0

    # Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    # Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    # Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green


def test_valid_game():
    bag = Batch("100 red, 100 green, 100 blue")
    game = Game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
    assert game.is_game_possible(bag)

    bag = Batch("0 red, 0 green, 0 blue")
    game = Game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
    assert not game.is_game_possible(bag)

    bag = Batch("12 red, 13 green, 14 blue")
    game = Game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
    assert not game.is_game_possible(bag)


def test_game_dice_set():
    game = Game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
    assert game.min_dice_set() == (4, 2, 6)

    game = Game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
    assert game.min_dice_set() == (1, 3, 4)

    game = Game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
    assert game.min_dice_set() == (20, 13, 6)

    game = Game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")
    assert game.min_dice_set() == (14, 3, 15)

    game = Game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
    assert game.min_dice_set() == (6, 3, 2)


def test_dice_power():
    game = Game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
    assert game.compute_dice_power() == 48

    game = Game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
    assert game.compute_dice_power() == 12

    game = Game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
    assert game.compute_dice_power() == 1560

    game = Game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")
    assert game.compute_dice_power() == 630

    game = Game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
    assert game.compute_dice_power() == 36
